use ratatui::{widgets::Paragraph, text::Span};

pub fn render() -> Paragraph<'static> {
    Paragraph::new(Span::raw("Logs Content"))
}