use crate::event::{AppEvent, Event, EventHandler};

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::DefaultTerminal;
use ratatui::widgets::ListState;
use std::fs::{self, DirEntry};
use std::path::PathBuf;

#[derive(Debug)]
pub struct App {
    pub running: bool,
    pub events: EventHandler,
    pub list_state: ListState,
    pub current_dir: PathBuf,
    pub entries: Vec<DirEntry>,
}

impl Default for App {
    fn default() -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));

        let current_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));

        Self {
            running: true,
            events: EventHandler::new(),
            list_state,
            current_dir,
            entries: Vec::new(),
        }
    }
}

impl App {
    pub fn new() -> Self { Self::default() }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        self.refresh()?;

        while self.running {
            terminal.draw(|frame| frame.render_widget(&mut self, frame.area()))?;
            self.handle_events()?;
        }
        Ok(())
    }

    pub fn handle_events(&mut self) -> color_eyre::Result<()> {
        match self.events.next()? {
            Event::Tick => self.tick(),
            Event::Crossterm(event) => match event {
                crossterm::event::Event::Key(key_event)
                if key_event.kind == crossterm::event::KeyEventKind::Press =>
                    {
                        self.handle_key_event(key_event)?
                    }
                _ => {}
            },
            Event::App(app_event) => match app_event {
                AppEvent::Quit => self.quit(),

                AppEvent::NextItem => self.next_item(),
                AppEvent::PreviousItem => self.previous_item(),
                AppEvent::PreviousFolder => self.previous_folder()?,
                AppEvent::Select => self.select()?,

                AppEvent::InputFieldOpenBash => self.input_field_open_bash(),
                AppEvent::Escape => self.escape(),
            },
        }
        Ok(())
    }

    pub fn handle_key_event(&mut self, key_event: KeyEvent) -> color_eyre::Result<()> {
        match key_event.code {
            KeyCode::Char('c' | 'C') if key_event.modifiers == KeyModifiers::CONTROL => {
                self.events.send(AppEvent::Quit)
            }

            KeyCode::Down => self.events.send(AppEvent::NextItem),
            KeyCode::Up => self.events.send(AppEvent::PreviousItem),
            KeyCode::Right | KeyCode::Enter => self.events.send(AppEvent::Select),
            KeyCode::Left => self.events.send(AppEvent::PreviousFolder),

            KeyCode::Char(char) => self.events.send(AppEvent::InputFieldOpenBash),
            KeyCode::Esc => self.events.send(AppEvent::Escape),
            _ => {}
        }
        Ok(())
    }

    pub fn refresh(&mut self) -> color_eyre::Result<()> {
        let mut entries: Vec<DirEntry> = fs::read_dir(&self.current_dir)?
            .collect::<Result<_, _>>()?;

        self.entries = entries;

        Ok(())
    }

    pub fn tick(&self) {}

    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn next_item(&mut self) {
        self.list_state.select_next();
    }

    pub fn previous_item(&mut self) {
        self.list_state.select_previous();
    }

    pub fn previous_folder(&mut self) -> color_eyre::Result<()> {
        if let Some(parent) = self.current_dir.parent() {
            self.current_dir = parent.to_path_buf();
            self.refresh()?;
        }
        Ok(())
    }

    pub fn select(&mut self) -> color_eyre::Result<()> {
        // when/if i also add bash this will be the same keybind to execute that command, so add that here.
        // (note: currently the right arrow will ALSO run this function, so I should praobably do something
        // about making sure that right arrow can still be used to navigate folders while typing bash.)

        if let Some(i) = self.list_state.selected() {
            if let Some(entry) = self.entries.get(i) {
                let is_dir = entry.file_type()?.is_dir();
                if is_dir {
                    self.current_dir = entry.path();
                    self.refresh()?;
                } else {
                    // file opening logic. i still don't rlly know how i wanna do this ngl.
                    // for now im js gonna do nothing and the user's gotta do `nvim <file>` or smt
                }
            }
        }
        Ok(())
    }

    pub fn input_field_open_commands(&mut self) {
        // Clear input field
        // Replace bash indicator "$" with command indicator ":"
        // Move cursor to input box & highlight the box
    }

    pub fn input_field_open_bash(&mut self) {
        // Clear input field
        // Replace command indicator ":" with bash indicator "$" (if applicable)
        // Move cursor to input box & highlight the box
    }

    pub fn escape(&mut self) {
        // If inside input field, exit input field
        // If inside output window, exit output window
    }
}