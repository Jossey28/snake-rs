use std::time::Duration;

use ratatui::{style::Color, widgets::StatefulWidget};

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum Direction {
    #[default]
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Food {
    pub x: f64,
    pub y: f64,
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CanvasPosition {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SnakeBody {
    pub current_position: CanvasPosition,
    pub last_position: CanvasPosition,
}

impl SnakeBody {
    fn new(current_position: CanvasPosition, last_position: CanvasPosition) -> Self {
        Self {
            current_position,
            last_position,
        }
    }
}

impl CanvasPosition {
    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

impl Into<(f64, f64)> for CanvasPosition {
    fn into(self) -> (f64, f64) {
        (self.x, self.y)
    }
}

#[derive(Debug, Clone)]
pub struct Snake {
    pub head: SnakeBody,
    pub body: Vec<SnakeBody>,
    pub direction: Direction,
}

impl Default for Snake {
    fn default() -> Self {
        let head = CanvasPosition::new(20.0, 5.0);

        let body_parts: Vec<SnakeBody> = {
            let mut tmp: Vec<SnakeBody> = vec![];

            for (_index, val) in (0..50).enumerate() {
                let pos = CanvasPosition::new(head.x - (val as f64), head.y);
                tmp.push(SnakeBody::new(pos, pos));
            }
            tmp
        };

        Snake {
            head: SnakeBody::new(head, head),
            body: body_parts,
            direction: Direction::Right,
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

    fn move_snake_body(&mut self) {
        let mut prev = self.head.last_position;
        for part in self.body.iter_mut() {
            let old_pos = part.current_position;
            part.current_position = prev;
            part.last_position = old_pos;
            prev = old_pos;
            continue;
        }
    }

    pub fn move_snake_head(&mut self) {
        match self.direction {
            Direction::Up => {
                self.move_snake_body();

                self.set_head(
                    self.head.current_position.x,
                    self.head.current_position.y + 1.0,
                );
            }
            Direction::Down => {
                self.move_snake_body();

                self.set_head(
                    self.head.current_position.x,
                    self.head.current_position.y - 1.0,
                );
            }
            Direction::Left => {
                self.move_snake_body();

                self.set_head(
                    self.head.current_position.x - 1.0,
                    self.head.current_position.y,
                );
            }
            Direction::Right => {
                self.move_snake_body();

                self.set_head(
                    self.head.current_position.x + 1.0,
                    self.head.current_position.y,
                );
            }
        }
    }

    fn set_head(&mut self, x: f64, y: f64) {
        // Figure out a way to verify its a valid positition in the future w/ idomatic rust w/o adding additional argument
        // https://users.rust-lang.org/t/current-best-practice-for-parent-child-struct-relationship/84542/3
        // https://www.sitepoint.com/rust-global-variables/

        self.head.last_position = self.head.current_position;

        self.head.current_position.x = x;
        self.head.current_position.y = y;
    }

    pub fn is_head_in_body(&self) -> bool {
        let tolerance = 1.0;

        let head_x = self.head.current_position.x.clone();
        let head_y = self.head.current_position.y.clone();

        for part in self.body.iter() {
            if (part.current_position.x - head_x).abs() < tolerance
                && (part.current_position.y - head_y).abs() < tolerance
            {
                return true;
            }
        }

        return false;
    }

    pub fn add_to_tail(&mut self) {
        self.body.push(SnakeBody::new(
            self.body.last().unwrap().last_position,
            self.body.last().unwrap().last_position,
        ));
    }
}

#[derive(Debug, Clone)]
pub struct GameSettings {
    pub active_row: usize,

    pub highscore: u64,

    pub invisible: bool,

    // pub tick_rate: Duration,

    pub head_color: Color,
    pub body_color: Color,
    pub wall_color: Color,
    pub food_color: Color,
}

impl Default for GameSettings {
    fn default() -> Self {
        GameSettings {
            active_row: 0,

            highscore: 0,

            invisible: false,

            // tick_rate: Duration::from_millis(10),

            head_color: Color::Red,
            body_color: Color::Blue,
            wall_color: Color::Green,
            food_color: Color::LightYellow,
        }
    }
}

impl GameSettings {
    pub const TOTAL_ROWS: usize = 5;

    pub fn move_down(&mut self) {
        self.active_row = (self.active_row + 1) % Self::TOTAL_ROWS;
    }

    pub fn move_up(&mut self) {
        if self.active_row == 0 {
            self.active_row = Self::TOTAL_ROWS - 1;
        } else {
            self.active_row -= 1;
        }
    }

    pub fn switch_color_for_field(&mut self) {
        let colors: [Color; 8] = [
            Color::Blue,
            Color::Cyan,
            Color::DarkGray,
            Color::Green,
            Color::LightGreen,
            Color::LightRed,
            Color::LightYellow,
            Color::Red,
        ];
        match self.active_row {
            2 => {
                let i = colors
                    .iter()
                    .position(|&col| col == self.head_color)
                    .unwrap();
                let new_color = colors.get((i + 1) % colors.len()).unwrap();
                self.head_color = *new_color;
            } // Head
            3 => {
                let i = colors
                    .iter()
                    .position(|&col| col == self.body_color)
                    .unwrap();
                let new_color = colors.get((i + 1) % colors.len()).unwrap();
                self.body_color = *new_color;
            } // Body
            4 => {
                let i = colors
                    .iter()
                    .position(|&col| col == self.wall_color)
                    .unwrap();
                let new_color = colors.get((i + 1) % colors.len()).unwrap();
                self.wall_color = *new_color;
            } // Wall
            5 => {
                let i = colors
                    .iter()
                    .position(|&col| col == self.food_color)
                    .unwrap();
                let new_color = colors.get((i + 1) % colors.len()).unwrap();
                self.food_color = *new_color;
            } // Food
            _ => unreachable!(),
        }
    }

    pub fn toggle_mortality(&mut self) {
        self.invisible = !self.invisible;
    }

    // pub fn increment_time(&mut self, increase: bool) {
    //     if increase {
    //         let mut tick_rate = self.tick_rate.as_millis();
    //         tick_rate += 1;
    //         self.tick_rate = Duration::from_millis(tick_rate.try_into().unwrap());
    //     } else {
    //         let mut tick_rate = self.tick_rate.as_millis();
    //         if tick_rate > 2 {
    //             tick_rate -= 1;
    //             self.tick_rate = Duration::from_millis(tick_rate.try_into().unwrap());
    //         }
    //     }
    // }
}
