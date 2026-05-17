use std::time::Duration;

use color_eyre::Result;

use color_eyre::eyre::Ok;
use ratatui::layout::Position;
use ratatui::layout::Rect;
use ratatui::style::Color;
use ratatui::widgets::Widget;

use crate::App;
use crate::AppState;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum Direction {
    #[default]
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone)]
pub struct Food {
    x: u16,
    y: u16,
}

impl Default for Food {
    fn default() -> Self {
        Food { x: 10, y: 10 }
    }
}

impl From<(u16, u16)> for Food {
    fn from(value: (u16, u16)) -> Self {
        let x = value.0;
        let y = value.1;

        Food { x, y }
    }
}

impl Into<Position> for &Food {
    fn into(self) -> Position {
        Position {
            x: self.x,
            y: self.y,
        }
    }
}

impl Widget for &Food {
    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        if self.x >= area.right() || self.y >= area.bottom() {
            return;
        }

        let food_pos: Position = self.into();
        let food_location = buf.cell_mut(food_pos).expect("invalid food position");

        let is_top_pixel = self.y % 2 == 0;
        if is_top_pixel {
            let bottom_color = if food_location.symbol() == "▄" {
                food_location.fg
            } else {
                Color::Reset
            };

            food_location.set_char('▀');
            food_location.set_fg(Color::Red);
            food_location.set_bg(bottom_color);
        } else {
            let top_color = if food_location.symbol() == "▀" {
                food_location.fg
            } else {
                Color::Reset
            };

            food_location.set_char('▀');
            food_location.set_fg(Color::Red);
            food_location.set_bg(top_color);
        }
    }
}

#[derive(Debug, Clone)]
pub struct Snake {
    pub head: Position,
    body: Option<Vec<Position>>,
    tail: Option<Position>,
    pub direction: Direction,
}

impl Default for Snake {
    fn default() -> Self {
        Snake {
            head: Position { x: 5, y: 5 },
            body: None,
            tail: None,
            direction: Direction::Down,
        }
    }
}

impl Snake {
    pub fn change_direction(&mut self, target_direction: Direction) {
        let current_direction = self.direction;

        if current_direction == target_direction {
            return;
        }
        if (current_direction == Direction::Left) && (target_direction == Direction::Right) {
            return;
        }
        if (current_direction == Direction::Right) && (target_direction == Direction::Left) {
            return;
        }
        if (current_direction == Direction::Up) && (target_direction == Direction::Down) {
            return;
        }
        if (current_direction == Direction::Down) && (target_direction == Direction::Up) {
            return;
        }

        self.direction = target_direction;
    }

    pub fn move_snake(&mut self, food_location: Position) -> AppState {
        match self.direction {
            Direction::Up => {
                if self.head.y == 0 {
                    return AppState::Dead;
                }

                let alive = self
                    .set_head(self.head.x, self.head.y - 1)
                    .unwrap_or_else(|_| false);

                if !alive {
                    return AppState::Dead;
                }

                if self.is_at_food(food_location) {
                    return AppState::Coliding;
                }

                return AppState::Active;
            }
            Direction::Down => {
                let alive = self
                    .set_head(self.head.x, self.head.y + 1)
                    .unwrap_or_else(|_| false);

                if !alive {
                    return AppState::Dead;
                }

                if self.is_at_food(food_location) {
                    return AppState::Coliding;
                }

                return AppState::Active;
            }
            Direction::Left => {
                if self.head.x == 0 {
                    return AppState::Dead;
                }

                let alive = self
                    .set_head(self.head.x - 1, self.head.y)
                    .unwrap_or_else(|_| false);

                if !alive {
                    return AppState::Dead;
                }

                if self.is_at_food(food_location) {
                    return AppState::Coliding;
                }

                return AppState::Active;
            }
            Direction::Right => {
                let alive = self
                    .set_head(self.head.x + 1, self.head.y)
                    .unwrap_or_else(|_| false);

                if !alive {
                    return AppState::Dead;
                }

                if self.is_at_food(food_location) {
                    return AppState::Coliding;
                }

                return AppState::Active;
            }
        }
    }

    fn is_at_food(&self, food_pos: Position) -> bool {
        let coliding = { self.head == food_pos };

        return coliding;
    }

    fn set_head(&mut self, x: u16, y: u16) -> Result<bool> {
        // Figure out a way to verify its a valid positition in the future w/ idomatic rust w/o adding additional argument
        // https://users.rust-lang.org/t/current-best-practice-for-parent-child-struct-relationship/84542/3
        // https://www.sitepoint.com/rust-global-variables/

        self.head.x = x;
        self.head.y = y;

        Ok(true)
    }

    fn get_head(&self) -> (u16, u16) {
        return (self.head.x, self.head.y);
    }
}

impl Widget for &Snake {
    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let (x, y) = self.get_head();
        if x >= area.right() || y >= area.bottom() {
            return;
        }

        let snake_head = buf
            .cell_mut(self.head)
            .expect("invalid snake head position");
        // snake_head.set_char('▀');
        match self.direction {
            Direction::Left => snake_head.set_char('◀'), // https://cloford.com/resources/charcodes/utf-8_geometric.htm
            Direction::Right => snake_head.set_char('▸'), // Starting @ UTF8+9654
            Direction::Up => snake_head.set_char('▴'),   // Or "BLACK UP-POINTING TRIANGLE"
            Direction::Down => snake_head.set_char('▼'),
        };

        snake_head.set_fg(Color::Green);
        snake_head.set_bg(Color::Reset);
    }
}

#[derive(Debug, Clone)]
pub struct GlobalSettings {
    pub terminal_width: u16,
    pub terminal_height: u16,

    pub tick_rate: Duration,
}

impl Default for GlobalSettings {
    fn default() -> Self {
        GlobalSettings {
            terminal_width: 0,
            terminal_height: 0,
            tick_rate: Duration::from_millis(50),
        }
    }
}
