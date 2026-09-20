use crate::event::{AppEvent, Event, EventHandler};

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::DefaultTerminal;
use ratatui::widgets::ListState;
use std::fs::{self, DirEntry};
use std::path::PathBuf;
use std::process::Command;
use tui_input::backend::crossterm::EventHandler as InputEventHandler;
use tui_input::Input;

#[derive(Debug)]
pub struct App {
    pub running: bool,
    pub events: EventHandler,
    pub list_state: ListState,
    pub current_dir: PathBuf,
    pub dir_entries: Vec<DirEntry>,
    pub entries: Vec<DirEntry>,
    pub show_hidden: bool,
    pub input: Input,
    pub input_mode: InputMode,
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
            dir_entries: Vec::new(),
            entries: Vec::new(),
            show_hidden: false,
            input: Default::default(),
            input_mode: InputMode::Normal,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RefreshMode {
    Reset,
    Retain,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputMode {
    Normal,
    Command,
}

impl App {
    pub fn new() -> Self { Self::default() }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        self.refresh(RefreshMode::Reset)?;

        while self.running {
            terminal.draw(|frame| self.render(frame))?;
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

                AppEvent::ToggleHidden => self.toggle_hidden()?,

                AppEvent::InputFieldOpenCommand(key_event) => self.input_field_open_command(key_event),
                AppEvent::Escape => self.escape(),
                AppEvent::Execute => self.execute()?,
            },
        }
        Ok(())
    }

    pub fn handle_key_event(&mut self, key_event: KeyEvent) -> color_eyre::Result<()> {
        match self.input_mode {
            InputMode::Normal => match key_event.code {
                KeyCode::Char('c' | 'C') if key_event.modifiers == KeyModifiers::CONTROL => {
                self.events.send(AppEvent::Quit)
                }

                KeyCode::Down => self.events.send(AppEvent::NextItem),
                KeyCode::Up => self.events.send(AppEvent::PreviousItem),
                KeyCode::Right | KeyCode::Enter => self.events.send(AppEvent::Select),
                KeyCode::Left => self.events.send(AppEvent::PreviousFolder),

                KeyCode::Char('.') => self.events.send(AppEvent::ToggleHidden),

                KeyCode::Char(_char) => self.events.send(AppEvent::InputFieldOpenCommand(key_event)),
                KeyCode::Esc => self.events.send(AppEvent::Escape),
                _ => {}
            }
            InputMode::Command => match key_event.code {
                KeyCode::Enter => self.events.send(AppEvent::Execute),
                KeyCode::Esc => self.events.send(AppEvent::Escape),
                _ => {
                    self.input.handle_event(&crossterm::event::Event::Key(key_event));
                }
            }
        }
        Ok(())
    }

    pub fn refresh(&mut self, refresh_mode: RefreshMode) -> color_eyre::Result<()> {
        let mut entries: Vec<DirEntry> = fs::read_dir(&self.current_dir)?
            .collect::<Result<_, _>>()?;

        let mut dir_entries: Vec<DirEntry> = fs::read_dir(&self.current_dir)?
            .collect::<Result<_, _>>()?;

        entries.retain(|e| !e.file_type().unwrap().is_dir());
        dir_entries.retain(|e| e.file_type().unwrap().is_dir());

        if !self.show_hidden {
            entries.retain(|e| !e.file_name().to_string_lossy().starts_with('.'));
        }

        entries.sort_by_key(|e| {
            e.file_name().to_string_lossy().to_lowercase()
        });

        dir_entries.sort_by_key(|e| {
            e.file_name().to_string_lossy().to_lowercase()
        });

        self.entries = entries;
        self.dir_entries = dir_entries;

        if refresh_mode == RefreshMode::Reset {
            self.list_state.select(Some(0));
        }

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
            self.refresh(RefreshMode::Reset)?;
        }
        Ok(())
    }

    pub fn select(&mut self) -> color_eyre::Result<()> {
        if let Some(i) = self.list_state.selected() {
            if let Some(entry) = self.entries.get(i) {
                let is_dir = entry.file_type()?.is_dir();
                if is_dir {
                    self.current_dir = entry.path();
                    self.refresh(RefreshMode::Reset)?;
                } else {
                    if cfg!(target_os = "windows") {
                        // todo windows file selection
                        // when i finish writing the linux file-opener i'll boot into windows and do this one
                        // but for now it'll js not work on windows lmao
                    } else {
                        let editor = std::env::var("EDITOR")?;
                        Command::new(editor).arg(entry.path()).status()?;
                    }
                }
            }
        }
        Ok(())
    }

    pub fn toggle_hidden(&mut self) -> color_eyre::Result<()> {
        self.show_hidden = !self.show_hidden;
        self.refresh(RefreshMode::Retain)
    }

    pub fn input_field_open_command(&mut self, key_event: KeyEvent) {
        self.input_mode = InputMode::Command;
        self.input.handle_event(&crossterm::event::Event::Key(key_event));
    }

    pub fn escape(&mut self) {
        self.input_mode = InputMode::Normal;
    }

    pub fn execute(&mut self) -> std::io::Result<()> {
        if cfg!(target_os = "windows") {
            Command::new("cmd").args(["/C", &self.input.value_and_reset()]).current_dir(&self.current_dir).status()?;
        } else {
            Command::new("sh").args(["-c", &self.input.value_and_reset()]).current_dir(&self.current_dir).status()?;
        }
        let _ = self.refresh(RefreshMode::Retain);
        Ok(())
    }
}