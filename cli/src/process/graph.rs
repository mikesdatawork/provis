use termimad::ProgressBar;

/// Render a bar graph with percentage (for detailed display)
pub fn render_bar(usage: f32, width: usize, ascii_mode: bool) -> String {
    if ascii_mode {
        let count = (usage / 100.0 * width as f32).round() as usize;
        let bar: String = "█".repeat(count);
        let no_bar: String = "░".repeat(width - count);
        format!("`{bar}{no_bar}` {usage:.1}%")
    } else {
        let pb = ProgressBar::new(usage / 100.0, width);
        format!("`{:<width$}` {:.1}%", pb, usage, width = width)
    }
}

/// Render a compact bar graph without percentage
pub fn render_bar_compact(usage: f32, width: usize, ascii_mode: bool) -> String {
    if ascii_mode {
        let count = (usage / 100.0 * width as f32).round() as usize;
        let bar: String = "█".repeat(count);
        let no_bar: String = "░".repeat(width - count);
        format!("`{bar}{no_bar}`")
    } else {
        let pb = ProgressBar::new(usage / 100.0, width);
        format!("`{:<width$}`", pb, width = width)
    }
}
