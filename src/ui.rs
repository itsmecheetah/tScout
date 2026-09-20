use ratatui::{layout::{Alignment, Layout, Constraint, Direction}, style::{Color, Style}, widgets::{Block, List, BorderType, Paragraph}, text::Line, Frame};
use ratatui::style::Modifier;
use ratatui::text::Span;
use crate::app::{App, InputMode};

impl App {
    pub fn render(&mut self, frame: &mut Frame) {
        let area = frame.area();
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(0)])
            .split(area);

        let header_block = Block::bordered()
            .title(Line::from(Span::styled("tScout", Style::default().add_modifier(Modifier::BOLD), )))
            .title(Line::from("(≧▽≦)").alignment(Alignment::Right))
            .border_type(BorderType::Rounded);

        let file_block = Block::bordered()
            .title(self.current_dir.to_string_lossy())
            .title(Line::from(if self.show_hidden { "showing hidden files" } else { "" }).alignment(Alignment::Right))
            .border_type(BorderType::Rounded);

        let input_style = match self.input_mode {
            InputMode::Normal => Color::DarkGray.into(),
            InputMode::Command => Style::default(),
        };
        let file_style = match self.input_mode {
            InputMode::Normal => Style::default(),
            InputMode::Command => Color::DarkGray.into(),
        };

        frame.render_widget(
            Paragraph::new(format!("$ {}", self.input.value()))
                .style(input_style)
                .block(header_block),
            layout[0],
        );

        let width = area.width.max(3) - 3;
        let scroll = self.input.visual_scroll(width as usize);

        if self.input_mode == InputMode::Command {
            let x = self.input.visual_cursor().max(scroll) - scroll + 3;
            frame.set_cursor_position((area.x + x as u16, area.y + 1))
        }

        let file_items: Vec<String> = self.entries.iter()
            .map(|e| e.file_name().to_string_lossy().into_owned())
            .collect();

        let mut dir_items: Vec<String> = self.dir_entries.iter()
            .map(|e| e.file_name().to_string_lossy().into_owned())
            .map(|e| format!("{e}/"))
            .collect();

        dir_items.extend(file_items);

        let list = List::new(dir_items)
            .block(file_block)
            .style(file_style)
            .highlight_style(Style::new())
            .highlight_symbol("> ")
            .repeat_highlight_symbol(true);

        frame.render_stateful_widget(list, layout[1], &mut self.list_state);
    }
}