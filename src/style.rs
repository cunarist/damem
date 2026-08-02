//! Colors for terminal output.
//!
//! `anstream` strips these when the output is piped or the terminal cannot
//! render them, so `damem recall > context.md` stays plain text.

use anstyle::{AnsiColor, Color, Style};

pub const PROBLEM: Style = fg(AnsiColor::Red);
pub const SUGGESTION: Style = fg(AnsiColor::Blue);
pub const OK: Style = fg(AnsiColor::Green);
pub const PATH: Style = Style::new().bold();
pub const DIM: Style = Style::new().dimmed();
pub const HEADING: Style = fg(AnsiColor::Cyan).bold();

const fn fg(color: AnsiColor) -> Style {
  Style::new().fg_color(Some(Color::Ansi(color)))
}

/// Clap's own help, in the same colors as everything else.
pub fn clap_styles() -> clap::builder::Styles {
  clap::builder::Styles::styled()
    .header(HEADING)
    .usage(HEADING)
    .literal(PATH)
    .placeholder(DIM)
}
