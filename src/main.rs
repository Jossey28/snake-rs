mod event_handler;
mod game_logic;
mod ui;

use std::time::{Duration, Instant};

use color_eyre::eyre::Result;
use crossterm::event::{KeyCode, KeyEvent};

use ratatui::layout::{Constraint, Layout, Offset};
use ratatui::style::Color;
use ratatui::symbols::Marker;
use ratatui::widgets::Widget;
use ratatui::widgets::canvas::{Canvas, Line};
use ratatui::{DefaultTerminal, Frame};

use crate::event_handler::{GameEvent, GameEventHandler};
use crate::game_logic::{Direction, Food, GameSettings, Snake};

fn main() -> Result<()> {
    color_eyre::install()?;

    let mut terminal = ratatui::init();
    let mut app: App = App::default();

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

    screen_width: u16,
    screen_height: u16,

    canvas_max_width: f64,
    canvas_max_height: f64,

    score: i64,

    last_tick: Option<Instant>,

    current_tab: AvailableTabs,

    settings: GameSettings,
}

#[derive(Debug, Default, Clone, Copy)]
pub enum AvailableTabs {
    #[default]
    Instructions = 0,
    Settings = 1,
    Credits = 2,
}

impl AvailableTabs {
    fn next(self) -> Self {
        match self {
            AvailableTabs::Instructions => AvailableTabs::Settings,
            AvailableTabs::Settings => AvailableTabs::Credits,
            AvailableTabs::Credits => AvailableTabs::Instructions,
        }
    }
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
            let tolerance = 1.5;

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
        let safe_distance = 10;

        let rand_x: i64 = rand::random_range(0..self.canvas_max_width as i64)
            .clamp(0, self.canvas_max_width as i64 - safe_distance);
        let rand_y: i64 = rand::random_range(0..self.canvas_max_height as i64)
            .clamp(0, self.canvas_max_height as i64 - safe_distance);

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
            // TODO! Create a death counter with persistent scrore
            AppState::TitleScreen => {
                let area = Layout::vertical([Constraint::Fill(1), Constraint::Fill(1)])
                    .split(frame.area());

                ui::show_title(frame, area[0]);
                ui::display_tabs(frame, area[1] + Offset::new(0, 1), self.current_tab, &mut self.settings);
                ui::display_menu(frame, area[1] + Offset::new(1, 0), self.current_tab);
            }
            AppState::Dead => {
                self.appstate = AppState::TitleScreen;
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
        }
    }

    fn start_game(&mut self) {
        self.snake = Snake::default();
        self.appstate = AppState::Active;
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) -> Result<()> {
        if key_event.code == KeyCode::Esc {
            self.exit();
        }

        match self.appstate {
            AppState::Active => match key_event.code {
                KeyCode::Char('q') => self.appstate = AppState::TitleScreen,

                KeyCode::Char('w') | KeyCode::Up => self.snake.change_direction(Direction::Up),
                KeyCode::Char('a') | KeyCode::Left => self.snake.change_direction(Direction::Left),
                KeyCode::Char('s') | KeyCode::Down => self.snake.change_direction(Direction::Down),
                KeyCode::Char('d') | KeyCode::Right => {
                    self.snake.change_direction(Direction::Right)
                }
                _ => {}
            },

            AppState::TitleScreen => match key_event.code {
                KeyCode::Enter => self.start_game(),

                KeyCode::Tab => {
                    self.current_tab = self.current_tab.next();
                }
                _ => {}
            },

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
                    color: self.settings.wall_color,
                });

                ctx.draw(&Line {
                    // Bottom Bar
                    x1: x_max,
                    y1: 0.0,
                    x2: 0.0,
                    y2: 0.0,
                    color: self.settings.wall_color,
                });

                ctx.draw(&Line {
                    // Left Bar
                    x1: 0.0,
                    y1: y_max,
                    x2: 0.0,
                    y2: 0.0,
                    color: self.settings.wall_color,
                });

                ctx.draw(&Line {
                    // Right Bar
                    x1: x_max,
                    y1: 0.0,
                    x2: x_max,
                    y2: y_max,
                    color: self.settings.wall_color,
                });

                ctx.layer();
                ctx.marker(Marker::HalfBlock);
                ctx.draw(&Line {
                    // Apple
                    x1: self.food.x,
                    x2: self.food.x,

                    y1: self.food.y,
                    y2: self.food.y,

                    color: self.settings.food_color,
                });

                ctx.layer(); // Begin Foreground
                ctx.marker(Marker::HalfBlock);

                ctx.draw(&Line {
                    // Head
                    x1: self.snake.head.current_position.x,
                    x2: self.snake.head.current_position.x,
                    y1: self.snake.head.current_position.y,
                    y2: self.snake.head.current_position.y,

                    color: self.settings.head_color,
                });

                for part in self.snake.body.iter() {
                    ctx.draw(&Line {
                        // Body points
                        x1: part.current_position.x,
                        x2: part.current_position.x,
                        y1: part.current_position.y,
                        y2: part.current_position.y,

                        color: self.settings.body_color,
                    });
                }
            });

        canvas.render(area, buf);
    }
}
