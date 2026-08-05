use crate::event::{AppEvent, Event, EventHandler};

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::DefaultTerminal;

#[derive(Debug)]
pub struct App {
    pub running: bool,
    pub counter: u8,
    pub events: EventHandler,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            counter: 0,
            events: EventHandler::new(),
        }
    }
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        while self.running {
            terminal.draw(|frame| frame.render_widget(&self, frame.area()))?;
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
                AppEvent::PreviousFolder => self.previous_folder(),
                AppEvent::Select => self.select(),

                AppEvent::InputFieldOpenCommands => self.input_field_open_commands(),
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

            KeyCode::Char(':') => self.events.send(AppEvent::InputFieldOpenCommands),
            KeyCode::Char(char) => self.events.send(AppEvent::InputFieldOpenBash), // How can I capture any input (aside from ':') and pass it as a parameter to that listener so that I can have the first character ready in the bash section of the file explorer?
            KeyCode::Esc => self.events.send(AppEvent::Escape),
            _ => {}
        }
        Ok(())
    }
    pub fn tick(&self) {}

    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn next_item(&mut self) {
        // Select next item
    }

    pub fn previous_item(&mut self) {
        // Select previous item
    }

    pub fn previous_folder(&mut self) {
        // Go to previous folder (duh)
    }

    pub fn select(&mut self) {
        // If inside input field:
            // If bash selected:
                // Run bash command
                // If output, open output window and add text there
            // If command selected:
                // Run that command
                // Possibly open output window depending on command
        // Else:
            // If folder selected:
                // Repopulate list with all items in the selected folder
            // If file selected:
                // Open file with an application (either via another list or just a default idk)
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