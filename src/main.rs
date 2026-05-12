mod event_handler;
mod game_logic;
mod ui;

use std::time::Instant;

use color_eyre::eyre::Result;
use crossterm::event::{KeyCode, KeyEvent};

use ratatui::layout::{Position, Rect};
use ratatui::style::Color;
use ratatui::widgets::Widget;
use ratatui::{DefaultTerminal, Frame};

use crate::event_handler::{GameEvent, GameEventHandler};
use crate::game_logic::{Direction, Snake};

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
    food: Position,

    snake: game_logic::Snake,
    settings: game_logic::Settings,

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
        self.food = Position { x: 10, y: 10 };

        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;

            match events.next()? {
                GameEvent::Tick => {
                    if self.appstate == AppState::Active {
                        let app_state = self.snake.move_snake(self.clone());
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
        self.food = Position { x: fastrand::u16(0..self.settings.terminal_width), y: fastrand::u16(0..self.settings.terminal_height) };
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
            AppState::Active => frame.render_widget(&*self, frame.area()),
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
            KeyCode::Enter if !active => Ok(self.start_game()),
            KeyCode::Char('q') if active => Ok(self.appstate = AppState::TitleScreen),
            KeyCode::Esc => Ok(self.exit()),

            KeyCode::Char('w') | KeyCode::Up if active => Ok(self.snake.direction = Direction::Up),
            KeyCode::Char('a') | KeyCode::Left if active => Ok(self.snake.direction = Direction::Left),
            KeyCode::Char('s') | KeyCode::Down if active => Ok(self.snake.direction = Direction::Down),
            KeyCode::Char('d') | KeyCode::Right if active => Ok(self.snake.direction = Direction::Right),

            _ => Ok(self.exit()),
        }
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let terminal_color = buf
            .cell_mut(Position::new(area.width - 1, area.height - 1))
            .expect("Invalid buffer for term color")
            .bg;

        let snake_head = buf
            .cell_mut(self.snake.head)
            .expect("invalid snake head position");
        // snake_head.set_char('▀');
        match self.snake.direction {
            Direction::Left => snake_head.set_char('◀'), // https://cloford.com/resources/charcodes/utf-8_geometric.htm
            Direction::Right => snake_head.set_char('▶'), // Starting @ UTF8+9654
            Direction::Up => snake_head.set_char('▲'),   // Or "BLACK UP-POINTING TRIANGLE"
            Direction::Down => snake_head.set_char('▼'),
        };
        snake_head.set_fg(Color::Green);
        snake_head.set_bg(terminal_color);

        let food_location = buf.cell_mut(self.food).expect("invalid food position");
        food_location.set_char('▀');
        food_location.set_fg(Color::Red);
        food_location.set_bg(terminal_color);
    }
}
