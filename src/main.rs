use std::fmt::format;
use std::io;
use std::time::{Duration, SystemTime};

use color_eyre::eyre::{Context, Result};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};

use tui_big_text::{BigText, PixelSize};

use rand::{Rng, RngExt};

use ratatui::layout::{Alignment, Constraint, Layout, Rect};
use ratatui::style::{Color, Style, Stylize};
use ratatui::symbols::Marker;
use ratatui::text::{Line as TextLine, Span, Text};
use ratatui::{DefaultTerminal, Frame};
use ratatui::widgets::{Block, Borders, Paragraph, Widget};

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
    food: Food,
    snake: Snake,
    settings: Settings,
}

#[derive(Debug, Default, Clone)]
pub struct Food {
    x: u16,
    y: u16,
}

#[derive(Debug, Clone)]
pub struct Snake {
    head: Rect,
    body: Option<Vec<Rect>>,
    tail: Option<Rect>,
    direction: Direction,
}

impl Default for Snake {
    fn default() -> Self {
        Snake { 
            head: Rect::new(25, 25, 2, 1),
            body: None, 
            tail: None, 
            direction: Direction::Down, 
        }
    }
}

impl Snake {
    fn move_snake(&mut self, app: App) -> AppState {
        match self.direction {
            Direction::Up => {
                if self.head.y > 0 {
                    self.head.y -= 1;
                    return AppState::Active;
                } else {
                    return AppState::Dead;
                }
            },
            Direction::Down =>{ 
                if self.head.y + 1 < app.settings.terminal_height {
                    self.head.y += 1;
                    return AppState::Active;
                } else {
                    return AppState::Dead;
                }
            },
            Direction::Left => {
                if self.head.x > 0 {
                    self.head.x -= 1;
                    return AppState::Active;
                } else {
                    return AppState::Dead;
                }
            },
            Direction::Right => {
                if self.head.x + 1 < app.settings.terminal_width {
                    self.head.x += 1;
                    return AppState::Active;
                } else {
                    return AppState::Dead;
                }  
            },
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum AppState {
    #[default]
    TitleScreen,
    Active,
    Dead
}

#[derive(Debug, Default, Clone, Copy)]
pub enum Direction {
    #[default]
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Default, Clone)]
pub struct Settings {
    terminal_width: u16,
    terminal_height: u16,
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> std::prelude::v1::Result<(), color_eyre::eyre::Error> {
        let frame_timeout = Duration::from_secs_f64(1.0 / 60.0); // Run at 60fps    
        event::poll(frame_timeout)?;

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
            AppState::TitleScreen => self.show_title(frame),
            AppState::Dead => self.show_title(frame),
            AppState::Active => self.play_snake(frame),
            _ => {}
        }
    }

    fn show_title(&self, frame: &mut Frame) -> ()  {
        let verticle_chunks = Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints([
                Constraint::Ratio(1, 3),
                Constraint::Fill(1),
                Constraint::Ratio(1, 3),
            ]).split(frame.area());

        let horizontal_chunks = Layout::default()
            .direction(ratatui::layout::Direction::Horizontal)
            .constraints([
                Constraint::Ratio(1, 3),
                Constraint::Fill(1),
                Constraint::Ratio(1, 3),
            ]).split(verticle_chunks[1]);

        let style = Style::new() 
            .bold();


        let text = BigText::builder()
                .pixel_size(PixelSize::HalfWidth) // See if I can make it full width. It cuts out atm 
                .style(style)
                .lines(vec![
                    "Snake-Rs".red().into(),
                ])
                .build();

        frame.render_widget(text, horizontal_chunks[1]);
    }

    fn play_snake(&mut self, frame: &mut Frame) {
        let current_appstate = self.snake.move_snake(self.clone());
        if current_appstate != AppState::Active {
            return
        }

        let block = Block::default().style(Style::new().bg(Color::LightGreen));

        frame.render_widget(block, self.snake.head);
    }

    fn regen_food(&mut self, frame: &mut Frame) {
        let mut rng = rand::rng();

        self.food.y = rng.random_range(..self.settings.terminal_height - 10);
        self.food.x = rng.random_range(..self.settings.terminal_width - 10);

        let area = Rect::new(self.food.x, self.food.y, 2, 1); // Width is double height due to terminal shenanigans.

        let block = Block::default().style(Style::new().bg(Color::Red));

        frame.render_widget(block, area);
    }

    fn handle_events(&mut self) -> Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self
                .handle_key_event(key_event)
                .wrap_err_with(|| format!("handling key event failed:\n{key_event:#?}"));
        }
        _ => {} // For some reason it doesn't work with Ok(()). Fix later.
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

            _ => { Ok(self.exit()) }
        }
    }
}