use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect, Layout, Constraint, Direction},
    style::{Color, Stylize},
    widgets::{Block, BorderType, Paragraph, Widget},
};

use crate::app::App;

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(1), Constraint::Length(3), Constraint::Min(0)])
            .split(area);

        let title_block = Block::new()
            .title("tScout - type :h for a list of keybinds")
            .title_alignment(Alignment::Center);

        let header_block = Block::bordered()
            .title_alignment(Alignment::Left)
            .border_type(BorderType::Rounded);

        let file_block = Block::bordered()
            .title_alignment(Alignment::Left)
            .border_type(BorderType::Rounded);

        let title_text = Paragraph::new("dummy text")
            .fg(Color::White)
            .bg(Color::Black)
            .block(title_block)
            .render(area, buf);

        let header_text = Paragraph::new("$ ")
            .fg(Color::White)
            .bg(Color::Black)
            .left_aligned()
            .block(header_block)
            .render(layout[1], buf);

        let file_text = Paragraph::new("Desktop\nDocuments\nDownloads\nMusic\nPictures\nProjects\nPublic")
            .fg(Color::White)
            .bg(Color::Black)
            .left_aligned()
            .block(file_block)
            .render(layout[2], buf);
    }
}