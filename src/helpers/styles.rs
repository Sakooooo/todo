use anstyle::{AnsiColor, Color, Style};

pub const BOLD: Style = Style::new().bold();
pub const CATEGORY: Style = Style::new()
    .bold()
    .fg_color(Some(Color::Ansi(AnsiColor::Blue)));
pub const FOLDER: Style = Style::new()
    .bold()
    .fg_color(Some(Color::Ansi(AnsiColor::Cyan)));
pub const DEADLINE: Style = Style::new()
    .bold()
    .fg_color(Some(Color::Ansi(AnsiColor::BrightRed)));
pub const SCHEDULED: Style = Style::new()
    .bold()
    .fg_color(Some(Color::Ansi(AnsiColor::Blue)));
