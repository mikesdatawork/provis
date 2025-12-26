use {
    crate::{
        Args,
        process::{
            stats::ProcessCollector,
            graph::render_bar_compact,
        },
    },
    std::{
        io::{self, Write},
        thread,
        time::Duration,
    },
    termimad::{
        MadSkin,
        CompoundStyle,
        crossterm::style::Color::AnsiValue,
        minimad::{OwningTemplateExpander, TableBuilder, Col, Alignment},
    },
};

// Color scheme from dysk
static USED_COLOR: u8 = 209;  // Red for usage

pub fn display_processes<W: Write>(
    w: &mut W,
    args: &Args,
) -> io::Result<()> {
    // Create collector and do initial refresh
    let mut collector = ProcessCollector::new();
    collector.refresh();
    
    // Wait 200ms for CPU and I/O measurements
    thread::sleep(Duration::from_millis(200));
    
    // Refresh again to get CPU and I/O deltas
    collector.refresh();
    
    // Get top processes by CPU
    let processes = collector.get_top_by_cpu(10);
    
    // Build table data
    let mut expander = OwningTemplateExpander::new();
    expander.set_default("");
    
    for process in &processes {
        // Format I/O as separate read and write
        let read_display = format_bytes(process.io_read_bytes);
        let write_display = format_bytes(process.io_write_bytes);
        
        expander
            .sub("rows")
            .set("pid", process.pid.to_string())
            .set("name", &process.name)
            .set_md("cpu_pct", format!("~~{:.1}%~~", process.cpu_usage))
            .set_md("cpu_bar", render_bar_compact(process.cpu_usage, 10, args.ascii))
            .set_md("mem_pct", format!("~~{:.1}%~~", process.mem_percent))
            .set_md("mem_bar", render_bar_compact(process.mem_percent, 10, args.ascii))
            .set("disk_read", read_display)
            .set("disk_write", write_display);
    }
    
    // Build table structure
    let mut tbl = TableBuilder::default();
    tbl.col(Col::new("PID", "${pid}").align_content(Alignment::Right))
        .col(Col::new("Process", "${name}").align_content(Alignment::Left))
        .col(Col::new("CPU%", "${cpu_pct}").align_content(Alignment::Right))
        .col(Col::new("CPU Usage", "${cpu_bar}").align_content(Alignment::Left))
        .col(Col::new("MEM%", "${mem_pct}").align_content(Alignment::Right))
        .col(Col::new("Memory Usage", "${mem_bar}").align_content(Alignment::Left))
        .col(Col::new("Disk Read", "${disk_read}").align_content(Alignment::Right))
        .col(Col::new("Disk Write", "${disk_write}").align_content(Alignment::Right));
    
    // Display with color if enabled
    let mut skin = if args.color() {
        make_colored_skin()
    } else {
        MadSkin::no_style()
    };
    
    if args.ascii {
        skin.limit_to_ascii();
    }
    
    writeln!(w, "\nTop 10 Processes by CPU Usage:\n")?;
    skin.write_owning_expander_md(w, &expander, &tbl)
}

fn make_colored_skin() -> MadSkin {
    MadSkin {
        strikeout: CompoundStyle::with_fg(AnsiValue(USED_COLOR)),  // red percentages
        inline_code: CompoundStyle::with_fg(AnsiValue(USED_COLOR)), // red bars
        ..Default::default()
    }
}

fn format_bytes(bytes: u64) -> String {
    if bytes < 1024 {
        format!("{}B", bytes)
    } else if bytes < 1024 * 1024 {
        format!("{:.1}K", bytes as f64 / 1024.0)
    } else if bytes < 1024 * 1024 * 1024 {
        format!("{:.1}M", bytes as f64 / (1024.0 * 1024.0))
    } else {
        format!("{:.1}G", bytes as f64 / (1024.0 * 1024.0 * 1024.0))
    }
}
