mod game_logic;
mod ui;

use std::time::{Duration, Instant};

use color_eyre::eyre::{Context, Result};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};

use ratatui::layout::{Position, Rect};
use ratatui::style::Color;
use ratatui::widgets::Widget;
use ratatui::{DefaultTerminal, Frame};

use crate::game_logic::Direction;

fn main() -> Result<()> {
    color_eyre::install()?;

    let mut terminal = ratatui::init();
    let app_result = App::default().run(&mut terminal);

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
}

impl App {
    pub fn run(
        &mut self,
        terminal: &mut DefaultTerminal,
    ) -> std::prelude::v1::Result<(), color_eyre::eyre::Error> {
        self.last_tick = Some(Instant::now());

        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;

            self.snake.move_snake(self.clone());
        }

        Ok(())
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
        }
    }

    fn handle_events(&mut self) -> Result<()> {
        let frame_timeout: Duration = Duration::from_secs_f64(1.0 / 60.0); // Run at 60fps    

        if event::poll(frame_timeout)? {
            match event::read()? {
                Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                    self.handle_key_event(key_event)
                        .wrap_err_with(|| format!("handling key event failed:\n{key_event:#?}"));
                }
                _ => {} // For some reason it doesn't work with Ok(()). Fix later.
            };
        }

        let last_tick = self.last_tick.unwrap();
        if last_tick.elapsed() >= self.settings.tick_rate {
            self.snake.move_snake(self.clone());
        }

        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) -> Result<()> {
        match key_event.code {
            KeyCode::Enter => Ok(self.appstate = AppState::Active),
            KeyCode::Char('q') => Ok(self.appstate = AppState::TitleScreen),
            KeyCode::Esc => Ok(self.exit()),

            KeyCode::Char('w') | KeyCode::Up => Ok(self.snake.direction = Direction::Up),
            KeyCode::Char('a') | KeyCode::Left => Ok(self.snake.direction = Direction::Left),
            KeyCode::Char('s') | KeyCode::Down => Ok(self.snake.direction = Direction::Down),
            KeyCode::Char('d') | KeyCode::Right => Ok(self.snake.direction = Direction::Right),

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
