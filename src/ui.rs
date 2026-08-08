use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect, Layout, Constraint, Direction},
    style::{Color, Style, Stylize},
    widgets::{Block, List, BorderType, Paragraph, Widget, StatefulWidget},
};
use crate::app::App;

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(0)])
            .split(area);

        let header_block = Block::bordered()
            .title_alignment(Alignment::Left)
            .border_type(BorderType::Rounded);

        let file_block = Block::bordered()
            .title_alignment(Alignment::Left)
            .border_type(BorderType::Rounded);

        Paragraph::new("$ ")
            .fg(Color::White)
            .bg(Color::Black)
            .left_aligned()
            .block(header_block)
            .render(layout[0], buf);

        let items: Vec<String> = self.entries.iter()
            .map(|e| e.file_name().to_string_lossy().into_owned())
            .collect();
        let list = List::new(items)
            .block(file_block)
            .highlight_style(Style::new())
            .highlight_symbol("> ")
            .repeat_highlight_symbol(true);

        StatefulWidget::render(list, layout[1], buf, &mut self.list_state);
    }
}