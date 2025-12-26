use {
    crate::{
        Args,
        process::{
            stats::ProcessCollector,
            graph::render_bar,
        },
    },
    std::{
        io::{self, Write},
        thread,
        time::Duration,
    },
    termimad::{
        MadSkin,
        minimad::{OwningTemplateExpander, TableBuilder, Col, Alignment},
    },
};

pub fn display_processes<W: Write>(
    w: &mut W,
    args: &Args,
) -> io::Result<()> {
    // Create collector and do initial refresh
    let mut collector = ProcessCollector::new();
    collector.refresh();
    
    // Wait 200ms for CPU measurements
    thread::sleep(Duration::from_millis(200));
    
    // Refresh again to get CPU deltas
    collector.refresh();
    
    // Get top processes by CPU
    let processes = collector.get_top_by_cpu(10);
    
    // Build table data
    let mut expander = OwningTemplateExpander::new();
    expander.set_default("");
    
    for process in &processes {
        expander
            .sub("rows")
            .set("pid", process.pid.to_string())
            .set("name", &process.name)
            .set("cpu_pct", format!("{:.1}%", process.cpu_usage))
            .set_md("cpu_bar", render_bar(process.cpu_usage, 10, args.ascii))
            .set("mem_pct", format!("{:.1}%", process.mem_percent))
            .set_md("mem_bar", render_bar(process.mem_percent, 10, args.ascii));
    }
    
    // Build table structure
    let mut tbl = TableBuilder::default();
    tbl.col(Col::new("PID", "${pid}").align_content(Alignment::Right))
        .col(Col::new("Process", "${name}").align_content(Alignment::Left))
        .col(Col::new("CPU%", "${cpu_pct}").align_content(Alignment::Right))
        .col(Col::new("CPU Usage", "${cpu_bar}").align_content(Alignment::Left))
        .col(Col::new("MEM%", "${mem_pct}").align_content(Alignment::Right))
        .col(Col::new("Memory Usage", "${mem_bar}").align_content(Alignment::Left));
    
    // Display with color if enabled
    let mut skin = if args.color() {
        MadSkin::default()
    } else {
        MadSkin::no_style()
    };
    
    if args.ascii {
        skin.limit_to_ascii();
    }
    
    writeln!(w, "\nTop 10 Processes by CPU Usage:\n")?;
    skin.write_owning_expander_md(w, &expander, &tbl)
}
