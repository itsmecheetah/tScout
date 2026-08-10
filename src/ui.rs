use ratatui::{buffer::Buffer, layout::{Alignment, Rect, Layout, Constraint, Direction}, style::{Color, Style, Stylize}, widgets::{Block, List, BorderType, Paragraph, Widget, StatefulWidget}, text::Line, Frame};
use crate::app::{App, InputMode};

impl App {
    pub fn render(&mut self, frame: &mut Frame) {
        let area = frame.area();
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(0)])
            .split(area);

        let header_block = Block::bordered()
            .title("bash")
            .title_alignment(Alignment::Left)
            .border_type(BorderType::Rounded);

        let file_block = Block::bordered()
            .title(self.current_dir.to_string_lossy())
            .title(Line::from(if self.show_hidden { "showing hidden files" } else { "" }).alignment(Alignment::Right))
            .border_type(BorderType::Rounded);

        let input_style = match self.input_mode {
            InputMode::Normal => Style::default(),
            InputMode::Bash => Color::Yellow.into(),
        };

        frame.render_widget(
            Paragraph::new(format!("$ {}", self.input.value()))
                .style(input_style)
                .block(header_block),
            layout[0],
        );

        let width = area.width.max(3) - 3;
        let scroll = self.input.visual_scroll(width as usize);

        if self.input_mode == InputMode::Bash {
            let x = self.input.visual_cursor().max(scroll) - scroll + 3;
            frame.set_cursor_position((area.x + x as u16, area.y + 1))
        }

        let items: Vec<String> = self.entries.iter()
            .map(|e| e.file_name().to_string_lossy().into_owned())
            .collect();
        let list = List::new(items)
            .block(file_block)
            .highlight_style(Style::new())
            .highlight_symbol("> ")
            .repeat_highlight_symbol(true);

        frame.render_stateful_widget(list, layout[1], &mut self.list_state);
    }
}