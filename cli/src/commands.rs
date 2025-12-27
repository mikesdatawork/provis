use {
    std::io::{self, Write},
    termimad::{
        MadSkin,
        minimad::{OwningTemplateExpander, TableBuilder, Col, Alignment},
    },
};

pub fn print_commands<W: Write>(
    w: &mut W,
    color: bool,
    ascii: bool,
) -> io::Result<()> {
    writeln!(w, "\nProvis Command Reference\n")?;

    let mut expander = OwningTemplateExpander::new();
    expander.set_default("");

    // View Modes
    add_command(&mut expander, "--help", "Show detailed help", "provis --help");
    add_command(&mut expander, "--commands", "Show this command reference", "provis --commands");
    add_command(&mut expander, "--version", "Show version", "provis --version");
    add_command(&mut expander, "(default)", "Disk/filesystem view", "provis");
    add_command(&mut expander, "--processes, -p", "Process monitoring view", "provis -p");
    add_command(&mut expander, "--size-on-disk", "Directory size analysis", "provis --size-on-disk");

    // Directory Scanning Options
    add_section(&mut expander, "DIRECTORY SCANNING");
    add_command(&mut expander, "--recursive", "Scan all nested folders", "provis --size-on-disk --recursive");
    add_command(&mut expander, "--depth N", "Scan N levels deep (default: 1)", "provis --size-on-disk --depth 3");
    add_command(&mut expander, "--limit N", "Show top N results (default: 20)", "provis --size-on-disk --limit 50");
    add_command(&mut expander, "--root", "Scan from filesystem root", "provis --size-on-disk --root");
    add_command(&mut expander, "--all, -a", "Show hidden dirs and build artifacts", "provis --size-on-disk -a");
    add_command(&mut expander, "PATH", "Start scan from path", "provis --size-on-disk /home");

    // Disk View Options
    add_section(&mut expander, "DISK VIEW");
    add_command(&mut expander, "--all, -a", "Show all mount points", "provis -a");
    add_command(&mut expander, "--list-cols", "List available columns", "provis --list-cols");
    add_command(&mut expander, "--cols, -c", "Select columns to display", "provis -c +inodes");
    add_command(&mut expander, "--sort, -s", "Sort by column", "provis -s free-desc");
    add_command(&mut expander, "--filter, -f", "Filter filesystems", "provis -f 'size>100G'");
    add_command(&mut expander, "--units, -u", "Size units (SI/binary/bytes)", "provis -u binary");

    // Display Options
    add_section(&mut expander, "DISPLAY");
    add_command(&mut expander, "--color", "Color mode (auto/yes/no)", "provis --color yes");
    add_command(&mut expander, "--ascii", "Use ASCII characters only", "provis --ascii");

    // Output Formats
    add_section(&mut expander, "OUTPUT");
    add_command(&mut expander, "--json, -j", "JSON output", "provis -j");
    add_command(&mut expander, "--csv", "CSV output", "provis --csv");
    add_command(&mut expander, "--csv-separator", "CSV separator character", "provis --csv --csv-separator ';'");

    // Build table
    let mut tbl = TableBuilder::default();
    tbl.col(Col::new("Flag", "${flag}").align_content(Alignment::Left))
        .col(Col::new("Description", "${desc}").align_content(Alignment::Left))
        .col(Col::new("Example", "${example}").align_content(Alignment::Left));

    // Display
    let mut skin = if color {
        MadSkin::default()
    } else {
        MadSkin::no_style()
    };

    if ascii {
        skin.limit_to_ascii();
    }

    skin.write_owning_expander_md(w, &expander, &tbl)?;

    writeln!(w, "\nFor detailed help: provis --help")?;
    writeln!(w, "Documentation: https://dystroy.org/dysk\n")?;

    Ok(())
}

fn add_command(expander: &mut OwningTemplateExpander, flag: &str, desc: &str, example: &str) {
    expander
        .sub("rows")
        .set("flag", flag)
        .set("desc", desc)
        .set("example", example);
}

fn add_section(expander: &mut OwningTemplateExpander, section: &str) {
    expander
        .sub("rows")
        .set("flag", "")
        .set("desc", format!("**{}**", section))
        .set("example", "");
}
