use color_eyre::Result;

use color_eyre::eyre::Ok;

use crate::AppState;

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

            for (_index, val) in (0..10).enumerate() {
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

    fn is_at_food(&self, food_pos: Food) -> bool {
        let coliding = {
            self.head.current_position.x == food_pos.x && self.head.current_position.y == food_pos.y
        };

        return coliding;
    }

    fn set_head(&mut self, x: f64, y: f64) {
        // Figure out a way to verify its a valid positition in the future w/ idomatic rust w/o adding additional argument
        // https://users.rust-lang.org/t/current-best-practice-for-parent-child-struct-relationship/84542/3
        // https://www.sitepoint.com/rust-global-variables/

        self.head.last_position = self.head.current_position;

        self.head.current_position.x = x;
        self.head.current_position.y = y;
    }

    pub fn get_head(&self) -> (f64, f64) {
        return (self.head.current_position.x, self.head.current_position.y);
    }

    pub fn is_head_in_body(&self) -> bool {
        self.body.contains(&self.head)
    }

    pub fn add_to_tail(&mut self) {
        todo!()
    }
}
