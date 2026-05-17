mod event_handler;
mod game_logic;
mod ui;

use std::time::Instant;

use color_eyre::eyre::Result;
use crossterm::event::{KeyCode, KeyEvent};

use ratatui::layout::Position;
use ratatui::{DefaultTerminal, Frame};

use crate::event_handler::{GameEvent, GameEventHandler};
use crate::game_logic::{Direction, Snake, Food, GlobalSettings};

fn main() -> Result<()> {
    color_eyre::install()?;

    let mut terminal = ratatui::init();
    let mut app = App::default();
    let events = GameEventHandler::new(app.settings.tick_rate);
    let app_result = app.run(&mut terminal, events);

    ratatui::restore();
    app_result
}

#[derive(Debug, Default, Clone)]
pub struct App {
    exit: bool,
    appstate: AppState,
    food: Food,

    snake: Snake,
    settings: GlobalSettings,

    last_tick: Option<Instant>,
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum AppState {
    #[default]
    TitleScreen,
    Active,
    Dead,
    Coliding,
}

impl App {
    pub fn run(
        &mut self,
        terminal: &mut DefaultTerminal,
        events: GameEventHandler,
    ) -> std::prelude::v1::Result<(), color_eyre::eyre::Error> {
        self.last_tick = Some(Instant::now());
        self.food = Food::default();

        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;

            match events.next()? {
                GameEvent::Tick => {
                    if self.appstate == AppState::Active {
                        let food = &self.food;
                        let app_state = self.snake.move_snake(food.into());
                        self.appstate = app_state;
                    }
                }
                GameEvent::Key(key_event) => self.handle_key_event(key_event)?,
                GameEvent::Mouse(_) => {}
                GameEvent::Resize(_, _) => {}
            }
        }

        Ok(())
    }

    fn handle_collision(&mut self) {    
        self.food = Food::from((fastrand::u16(0..self.settings.terminal_width), fastrand::u16(0..self.settings.terminal_height)));
        self.appstate = AppState::Active;
    } 

    fn exit(&mut self) {
        self.exit = true;
    }

    fn draw(&mut self, frame: &mut Frame) {
        self.settings.terminal_height = frame.area().height;
        self.settings.terminal_width = frame.area().width;

        match self.appstate {
            AppState::TitleScreen => ui::show_title(frame),
            AppState::Dead => ui::show_title(frame),
            AppState::Active => {
                frame.render_widget(&self.snake, frame.area());
                frame.render_widget(&self.food, frame.area());
                },
            AppState::Coliding => self.handle_collision(),
        }
    }

    fn start_game(&mut self) {
        self.snake = Snake::default();
        self.appstate = AppState::Active;
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) -> Result<()> {
        let active = {
            self.appstate == AppState::Active
        };

        match key_event.code {
            KeyCode::Enter if !active => self.start_game(),
            KeyCode::Char('q') if active => self.appstate = AppState::TitleScreen,
            KeyCode::Esc => self.exit(),

            KeyCode::Char('w') | KeyCode::Up if active => self.snake.change_direction(Direction::Up),
            KeyCode::Char('a') | KeyCode::Left if active => self.snake.change_direction(Direction::Left),
            KeyCode::Char('s') | KeyCode::Down if active => self.snake.change_direction(Direction::Down),
            KeyCode::Char('d') | KeyCode::Right if active => self.snake.change_direction(Direction::Right),

            _ => {},
        };

        Ok(())
    }
}