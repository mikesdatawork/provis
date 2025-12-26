//! Process display module
//! 
//! Handles formatting and displaying process information

use {
    super::{ProcessCollector, render_bar},
    crate::Args,
    std::io::{self, Write},
    termimad::{
        MadSkin,
        minimad::{OwningTemplateExpander, TableBuilder},
    },
};

/// Display process information as a table
pub fn display_processes<W: Write>(
    w: &mut W,
    args: &Args,
) -> io::Result<()> {
    let mut collector = ProcessCollector::new();
    collector.refresh();
    
    // Get top 10 by CPU
    let processes = collector.get_top_by_cpu(10);
    
    if processes.is_empty() {
        writeln!(w, "No processes found")?;
        return Ok(());
    }
    
    // Create table with expander
    let mut expander = OwningTemplateExpander::new();
    expander.set_default("");
    
    for process in &processes {
        let cpu_bar = render_bar(process.cpu_usage, 10, args.ascii);
        let mem_bar = render_bar(process.mem_percent, 10, args.ascii);
        
        expander
            .sub("rows")
            .set("pid", process.pid)
            .set("name", &process.name)
            .set("cpu", format!("{:.1}%", process.cpu_usage))
            .set_md("cpu-bar", &cpu_bar)
            .set("mem", format!("{:.1}%", process.mem_percent))
            .set_md("mem-bar", &mem_bar);
    }
    
    // Build table structure using minimad::Col
    let mut tbl = TableBuilder::default();
    tbl.col(termimad::minimad::Col::new("PID", "${pid}"));
    tbl.col(termimad::minimad::Col::new("Process", "${name}"));
    tbl.col(termimad::minimad::Col::new("CPU%", "${cpu}"));
    tbl.col(termimad::minimad::Col::new("CPU Usage", "${cpu-bar}"));
    tbl.col(termimad::minimad::Col::new("MEM%", "${mem}"));
    tbl.col(termimad::minimad::Col::new("Memory Usage", "${mem-bar}"));
    
    // Create skin
    let mut skin = if args.color() {
        MadSkin::default()
    } else {
        MadSkin::no_style()
    };
    
    if args.ascii {
        skin.limit_to_ascii();
    }
    
    // Write table
    skin.write_owning_expander_md(w, &expander, &tbl)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
}
