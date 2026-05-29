use std::time::Duration;

use color_eyre::Result;

use color_eyre::eyre::Ok;
use ratatui::layout::Position;
use ratatui::layout::Rect;
use ratatui::style::Color;
use ratatui::widgets::Widget;
use ratatui::widgets::canvas::Canvas;

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

#[derive(Debug, Clone, Copy)]
pub struct Food {
    x: f64,
    y: f64,
}

impl Default for Food {
    fn default() -> Self {
        Food { x: 10.0, y: 10.0 }
    }
}

impl From<(f64, f64)> for Food {
    fn from(value: (f64, f64)) -> Self {
        let x = value.0;
        let y = value.1;

        Food { x, y }
    }
}

// impl Into<Position> for &Food {
//     fn into(self) -> Position {
//         Position {
//             x: self.x as u16,
//             y: self.y as u16,
//         }
//     }
// }

// impl Widget for &Food {
//     fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer)
//     where
//         Self: Sized,
//     {
//         if self.x >= area.right().into() || self.y >= area.bottom().into() {
//             return;
//         }

//         let food_pos: Position = self.into();
//         let food_location = buf.cell_mut(food_pos).expect("invalid food position");

//         let is_top_pixel = self.y % 2 == 0;
//         if is_top_pixel {
//             let bottom_color = if food_location.symbol() == "▄" {
//                 food_location.fg
//             } else {
//                 Color::Reset
//             };

//             food_location.set_char('▀');
//             food_location.set_fg(Color::Red);
//             food_location.set_bg(bottom_color);
//         } else {
//             let top_color = if food_location.symbol() == "▀" {
//                 food_location.fg
//             } else {
//                 Color::Reset
//             };

//             food_location.set_char('▀');
//             food_location.set_fg(Color::Red);
//             food_location.set_bg(top_color);
//         }
//     }
// }
#[derive(Debug, Clone, Copy)]
pub struct CanvasPosition {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone)]
pub struct SnakeBody {
    pub current_position: CanvasPosition,
    pub last_position: CanvasPosition,
}

impl SnakeBody {
    fn new(current_position: CanvasPosition, last_position: CanvasPosition) -> Self {
        Self { current_position, last_position}
    }
}

impl CanvasPosition {
    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone)]
pub struct Snake {
    pub head: CanvasPosition,
    pub body: Vec<SnakeBody>,
    pub direction: Direction,
}

impl Default for Snake {
    fn default() -> Self {
        let head = CanvasPosition::new(5.0, 5.0);
        let first_piece = SnakeBody::new(CanvasPosition::new(head.x -1.0 , head.y), CanvasPosition::new(head.x - 1.0, head.y));
        let second_piece = SnakeBody::new(CanvasPosition::new(head.x - 2.0, head.y), CanvasPosition::new(head.x - 2.0, head.y));

        Snake {
            head: head,
            body: vec![first_piece, second_piece],
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

    pub fn move_snake(&mut self, food_location: Food) -> AppState {
        match self.direction {
            Direction::Up => {
                if self.head.y < 1.0 {
                    return AppState::Dead;
                }

                let alive = self
                    .set_head(self.head.x, self.head.x - 1.0)
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
                    .set_head(self.head.x, self.head.x + 1.0)
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
                if self.head.x < 1.0 {
                    return AppState::Dead;
                }

                let alive = self
                    .set_head(self.head.x - 1.0, self.head.y)
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
                    .set_head(self.head.x + 1.0, self.head.y)
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

    fn is_at_food(&self, food_pos: Food) -> bool {
        let coliding = { self.head.x == food_pos.x && self.head.y == food_pos.y };

        return coliding;
    }

    fn set_head(&mut self, x: f64, y: f64) -> Result<bool> {
        // Figure out a way to verify its a valid positition in the future w/ idomatic rust w/o adding additional argument
        // https://users.rust-lang.org/t/current-best-practice-for-parent-child-struct-relationship/84542/3
        // https://www.sitepoint.com/rust-global-variables/

        self.head.x = x;
        self.head.y = y;

        Ok(true)
    }

    fn get_head(&self) -> (f64, f64) {
        return (self.head.x, self.head.y);
    }
}

// impl Widget for &Snake {
//     fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer)
//     where
//         Self: Sized,
//     {
//         let (x, y) = self.get_head();
//         if x >= area.right().into() || y >= area.bottom().into() {
//             return;
//         }

//         let snake_head = buf
//             .cell_mut(self.head)
//             .expect("invalid snake head position");
//         // snake_head.set_char('▀');
//         match self.direction {
//             Direction::Left => snake_head.set_char('◀'), // https://cloford.com/resources/charcodes/utf-8_geometric.htm
//             Direction::Right => snake_head.set_char('▶'), // Starting @ UTF8+9654
//             Direction::Up => snake_head.set_char('▲'),   // Or "BLACK UP-POINTING TRIANGLE"
//             Direction::Down => snake_head.set_char('▼'),
//         };

//         snake_head.set_fg(Color::Green);
//         snake_head.set_bg(Color::Reset);
//     }
// }
