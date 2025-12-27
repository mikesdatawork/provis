use {
    crate::{
        Args,
        directory::DirectoryScanner,
        process::graph::render_bar_compact,
    },
    std::{
        io::{self, Write},
        path::PathBuf,
    },
    termimad::{
        MadSkin,
        CompoundStyle,
        crossterm::style::Color::AnsiValue,
        minimad::{OwningTemplateExpander, TableBuilder, Col, Alignment},
    },
};

static USED_COLOR: u8 = 209;

pub fn display_directories<W: Write>(
    w: &mut W,
    args: &Args,
) -> io::Result<()> {
    let base_path = if args.root {
        PathBuf::from("/")
    } else if let Some(ref path) = args.path {
        path.clone()
    } else {
        std::env::current_dir()?
    };

    writeln!(w, "Scanning directory: {}", base_path.display())?;
    if !args.all {
        writeln!(w, "(Skipping hidden and build directories - use --all to show everything)")?;
    }
    writeln!(w, "Please wait...")?;
    w.flush()?;

    let mut scanner = DirectoryScanner::new(&base_path);
    
    if args.recursive {
        scanner = scanner.recursive();
    } else {
        scanner = scanner.max_depth(args.depth);
    }

    scanner = scanner.show_all(args.all);

    let entries = match scanner.scan() {
        Ok(e) => e,
        Err(err) => {
            eprintln!("\nError scanning directory: {}", err);
            return Ok(());
        }
    };

    if entries.is_empty() {
        writeln!(w, "\nNo subdirectories found.")?;
        return Ok(());
    }

    let limit = match args.limit.as_str() {
        "all" => entries.len(),
        "20" => 20.min(entries.len()),
        _ => 10.min(entries.len()),
    };

    let display_entries = &entries[..limit];
    let max_size = entries.first().map(|e| e.size).unwrap_or(1);

    let mut expander = OwningTemplateExpander::new();
    expander.set_default("");

    for entry in display_entries {
        let size_pct = (entry.size as f32 / max_size as f32) * 100.0;
        let size_display = format_bytes(entry.size);
        let path_display = entry.path.display().to_string();

        expander
            .sub("rows")
            .set_md("bar", render_bar_compact(size_pct, 10, args.ascii))
            .set("size", size_display)
            .set("path", path_display);
    }

    let mut tbl = TableBuilder::default();
    tbl.col(Col::new("Usage", "${bar}").align_content(Alignment::Left))
        .col(Col::new("Size", "${size}").align_content(Alignment::Right))
        .col(Col::new("Path", "${path}").align_content(Alignment::Left));

    let mut skin = if args.color() {
        make_colored_skin()
    } else {
        MadSkin::no_style()
    };

    if args.ascii {
        skin.limit_to_ascii();
    }

    writeln!(w, "\nTop {} Directories by Size:\n", limit)?;
    skin.write_owning_expander_md(w, &expander, &tbl)
}

fn make_colored_skin() -> MadSkin {
    MadSkin {
        inline_code: CompoundStyle::with_fg(AnsiValue(USED_COLOR)),
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
    } else if bytes < 1024 * 1024 * 1024 * 1024 {
        format!("{:.1}G", bytes as f64 / (1024.0 * 1024.0 * 1024.0))
    } else {
        format!("{:.1}T", bytes as f64 / (1024.0 * 1024.0 * 1024.0 * 1024.0))
    }
}
