//! Graph rendering module for resource usage visualization
//!
//! Provides functions to render resource usage as bar graphs

/// Render a horizontal bar graph for a percentage value
///
/// # Arguments
/// * `usage` - Current usage (0.0 to 100.0)
/// * `width` - Width of the bar in characters
/// * `ascii_mode` - If true, use ASCII characters instead of Unicode
///
/// # Returns
/// String representation of the bar graph
pub fn render_bar(usage: f32, width: usize, ascii_mode: bool) -> String {
    let usage = usage.clamp(0.0, 100.0);
    let filled = ((usage / 100.0) * width as f32).round() as usize;
    let filled = filled.min(width);
    
    if ascii_mode {
        let bar = "#".repeat(filled);
        let empty = "-".repeat(width - filled);
        format!("[{}{}] {:>5.1}%", bar, empty, usage)
    } else {
        let bar = "█".repeat(filled);
        let empty = "░".repeat(width - filled);
        format!("[{}{}] {:>5.1}%", bar, empty, usage)
    }
}

/// Render a compact bar without percentage text
pub fn render_bar_compact(usage: f32, width: usize, ascii_mode: bool) -> String {
    let usage = usage.clamp(0.0, 100.0);
    let filled = ((usage / 100.0) * width as f32).round() as usize;
    let filled = filled.min(width);
    
    if ascii_mode {
        let bar = "#".repeat(filled);
        let empty = "-".repeat(width - filled);
        format!("[{}{}]", bar, empty)
    } else {
        let bar = "█".repeat(filled);
        let empty = "░".repeat(width - filled);
        format!("[{}{}]", bar, empty)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_bar_ascii() {
        let bar = render_bar(50.0, 10, true);
        assert!(bar.contains("#####"));
        assert!(bar.contains("50.0%"));
    }

    #[test]
    fn test_render_bar_unicode() {
        let bar = render_bar(75.0, 10, false);
        assert!(bar.contains("█"));
        assert!(bar.contains("75.0%"));
    }

    #[test]
    fn test_render_bar_clamp() {
        let bar = render_bar(150.0, 10, true);
        assert!(bar.contains("100.0%"));
    }
}
