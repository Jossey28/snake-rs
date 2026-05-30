mod event_handler;
mod game_logic;
mod ui;

use std::default;
use std::sync::Arc;
use std::time::{Duration, Instant};

use color_eyre::eyre::Result;
use crossterm::event::{KeyCode, KeyEvent};

use rand::Rng;
use rand::prelude::ThreadRng;
use ratatui::layout::{Constraint, Layout, Offset, Position};
use ratatui::style::Color;
use ratatui::symbols::Marker;
use ratatui::widgets::Widget;
use ratatui::widgets::canvas::{Canvas, Line, Map, MapResolution, Points, Rectangle};
use ratatui::{DefaultTerminal, Frame};

use crate::event_handler::{GameEvent, GameEventHandler};
use crate::game_logic::{Direction, Food, Snake};

use itertools::Itertools;

fn main() -> Result<()> {
    color_eyre::install()?;

    let mut terminal = ratatui::init();
    let mut app: App = App::default();

    let tick_rate = Duration::from_millis(10);
    let events = GameEventHandler::new(tick_rate);
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

    screen_width: u16,
    screen_height: u16,

    canvas_max_width: f64,
    canvas_max_height: f64,

    score: i64,

    last_tick: Option<Instant>,
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum AppState {
    #[default]
    TitleScreen,
    Active,
    Dead,
}

#[derive(PartialEq, Eq, Debug)]
pub enum CollisionType {
    Ceiling,
    Floor,
    LeftWall,
    RightWall,
    Food,
    Body,
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
                        self.snake.move_snake_head();

                        let coliding = self.check_colision();
                        // print!("{:#?}", coliding);
                        if coliding.is_some() {
                            self.handle_colision(coliding.unwrap());
                        }
                    }
                }
                GameEvent::Key(key_event) => self.handle_key_event(key_event)?,
                GameEvent::Mouse(_) => {}
                GameEvent::Resize(_, _) => {}
            }
        }

        Ok(())
    }

    fn check_colision(&self) -> Option<CollisionType> {
        if self.snake.head.current_position.x < 1.0 {
            return Some(CollisionType::LeftWall);
        }

        if self.snake.head.current_position.x > self.canvas_max_width - 1.0 {
            return Some(CollisionType::RightWall);
        }

        if self.snake.head.current_position.y < 1.0 {
            return Some(CollisionType::Ceiling);
        }

        if self.snake.head.current_position.y > self.canvas_max_height - 1.0 {
            return Some(CollisionType::Floor);
        }

        if self.snake.is_head_in_body() {
            return Some(CollisionType::Body);
        }

        let eating_food: bool = {
            let tolerance = 1.0;

            let snake_x = self.snake.head.current_position.x;
            let snake_y = self.snake.head.current_position.y;

            let food_x = self.food.x;
            let food_y = self.food.y;

            if (snake_x - food_x).abs() < tolerance && (snake_y - food_y).abs() < tolerance {
                true
            } else {
                false
            }
        };

        if eating_food {
            return Some(CollisionType::Food);
        }
        None
    }

    fn handle_colision(&mut self, collision: CollisionType) {
        match collision {
            CollisionType::Food => {
                self.regen_food();
                self.increment_score();
                self.snake.add_to_tail();
            }
            _ => self.appstate = AppState::Dead,
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn regen_food(&mut self) {
        let rand_x: i64 = rand::random_range(0..self.canvas_max_width as i64);
        let rand_y: i64 = rand::random_range(0..self.canvas_max_height as i64);

        self.food.x = rand_x as f64;
        self.food.y = rand_y as f64;
    }

    fn increment_score(&mut self) {
        self.score += 1;
    }

    fn draw(&mut self, frame: &mut Frame) {
        self.screen_height = frame.area().height;
        self.screen_width = frame.area().width;

        match self.appstate {
            AppState::TitleScreen | AppState::Dead => {
                let area = Layout::vertical([Constraint::Fill(1), Constraint::Fill(1)])
                    .split(frame.area());

                ui::show_title(frame, area[0]);
                ui::display_menu_title(frame, area[1]);
                ui::display_menu(frame, area[1] + Offset::new(0, 2));
            }
            AppState::Active => {
                let vertical =
                    Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]).spacing(1);
                let horizontal = Layout::horizontal([Constraint::Percentage(100)]).spacing(1);

                let [top, main] = frame.area().layout(&vertical);
                let [area] = main.layout(&horizontal);

                ui::display_score(frame, top, self.score);
                frame.render_widget(self, area);
            }
            _ => {}
        }
    }

    fn start_game(&mut self) {
        self.snake = Snake::default();
        self.appstate = AppState::Active;
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) -> Result<()> {
        let active = { self.appstate == AppState::Active };

        match key_event.code {
            KeyCode::Enter if !active => self.start_game(),
            KeyCode::Char('q') if active => self.appstate = AppState::TitleScreen,
            KeyCode::Esc => self.exit(),

            KeyCode::Char('w') | KeyCode::Up if active => {
                self.snake.change_direction(Direction::Up)
            }
            KeyCode::Char('a') | KeyCode::Left if active => {
                self.snake.change_direction(Direction::Left)
            }
            KeyCode::Char('s') | KeyCode::Down if active => {
                self.snake.change_direction(Direction::Down)
            }
            KeyCode::Char('d') | KeyCode::Right if active => {
                self.snake.change_direction(Direction::Right)
            }

            _ => {}
        };

        Ok(())
    }
}

impl Widget for &mut App {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let widget_aspect_ratio = (area.height as f64 * 2.0) / area.width as f64;
        let x_max = 150.0;
        let y_max = x_max * widget_aspect_ratio;

        self.canvas_max_width = x_max;
        self.canvas_max_height = y_max;

        let canvas = Canvas::default()
            .x_bounds([0.0, x_max])
            .y_bounds([0.0, y_max])
            .marker(Marker::Dot)
            .paint(move |ctx| {
                ctx.draw(&Line {
                    // Top Bar
                    x1: x_max,
                    y1: y_max,
                    x2: 0.0,
                    y2: y_max,
                    color: Color::Green,
                });

                ctx.draw(&Line {
                    // Bottom Bar
                    x1: x_max,
                    y1: 0.0,
                    x2: 0.0,
                    y2: 0.0,
                    color: Color::Green,
                });

                ctx.draw(&Line {
                    // Left Bar
                    x1: 0.0,
                    y1: y_max,
                    x2: 0.0,
                    y2: 0.0,
                    color: Color::Green,
                });

                ctx.draw(&Line {
                    // Right Bar
                    x1: x_max,
                    y1: 0.0,
                    x2: x_max,
                    y2: y_max,
                    color: Color::Green,
                });

                ctx.layer();
                ctx.marker(Marker::HalfBlock);
                ctx.draw(&Line {
                    // Apple
                    x1: self.food.x,
                    x2: self.food.x,

                    y1: self.food.y,
                    y2: self.food.y,

                    color: Color::LightYellow,
                });

                ctx.layer(); // Begin Foreground
                ctx.marker(Marker::HalfBlock);

                ctx.draw(&Line {
                    // Head
                    x1: self.snake.head.current_position.x,
                    x2: self.snake.head.current_position.x,
                    y1: self.snake.head.current_position.y,
                    y2: self.snake.head.current_position.y,

                    color: Color::Red,
                });

                for part in self.snake.body.iter() {
                    ctx.draw(&Line {
                        // Body points
                        x1: part.current_position.x,
                        x2: part.current_position.x,
                        y1: part.current_position.y,
                        y2: part.current_position.y,

                        color: Color::Blue,
                    });
                }
            });

        canvas.render(area, buf);
    }
}
