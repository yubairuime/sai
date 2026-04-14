use std::fmt;

use crate::parser::Position;

#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub position: Position,
    pub msg: String,
}

impl Diagnostic {
    pub fn new(position: Position, msg: &str) -> Self {
        Self {
            position: position,
            msg: msg.to_string(),
        }
    }
}

impl fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}:{}\n{}", self.position.file, self.position.line, self.position.col, self.msg)
    }
}

pub fn render_diagnostic(source: Option<&str>, diagnostic: &Diagnostic, use_color: bool) -> String {
    let red = if use_color { "\x1b[31m" } else { "" };
    let blue = if use_color { "\x1b[34m" } else { "" };
    let bold = if use_color { "\x1b[1m" } else { "" };
    let reset = if use_color { "\x1b[0m" } else { "" };

    let header = format!(
        "{bold}{red}error{reset}{bold}:{}{reset} {}:{}:{}",
        if use_color { format!("{reset}") } else { String::new() },
        diagnostic.position.file,
        diagnostic.position.line,
        diagnostic.position.col,
    );

    let message = format!("{bold}{}{reset}", diagnostic.msg);

    let Some(source) = source else {
        return format!("{header}\n{message}");
    };

    let Some(line) = source.lines().nth(diagnostic.position.line.saturating_sub(1)) else {
        return format!("{header}\n{message}");
    };

    let gutter = diagnostic.position.line.to_string();
    let caret_col = diagnostic.position.col.saturating_sub(1);
    let caret_padding: String = line.chars().take(caret_col).map(|ch| if ch == '\t' { '\t' } else { ' ' }).collect();

    format!("{header}\n{message}\n {blue}{gutter}{reset} | {line}\n {} | {red}{caret_padding}^{reset}", " ".repeat(gutter.len()))
}

pub fn render_diagnostics(source: Option<&str>, diagnostics: &[Diagnostic], use_color: bool) -> Vec<String> {
    diagnostics.iter().map(|diagnostic| render_diagnostic(source, diagnostic, use_color)).collect()
}
