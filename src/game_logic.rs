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

#[derive(Debug, Clone, Copy)]
pub struct CanvasPosition {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone)]
pub struct Snake {
    pub head: SnakeBody,
    pub body: Vec<SnakeBody>,
    pub direction: Direction,
}

impl Default for Snake {
    fn default() -> Self {
        let head = CanvasPosition::new(5.0, 5.0);
        let first_piece = SnakeBody::new(
            CanvasPosition::new(head.x - 1.0, head.y),
            CanvasPosition::new(head.x - 1.0, head.y),
        );

        let second_piece = SnakeBody::new(
            CanvasPosition::new(head.x - 2.0, head.y),
            CanvasPosition::new(head.x - 2.0, head.y),
        );

        Snake {
            head: SnakeBody::new(head, head),
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

    fn move_snake_body(&mut self) {
        let mut prev: Option<&mut SnakeBody> = None;
        for mut part in self.body.iter_mut() {
            if let Some(p) = prev.take() {
                part.last_position = part.current_position;
                part = p;
                continue;
            }

            part.last_position = part.current_position;
            part.current_position = self.head.last_position;
            prev = Some(part);
        }
    }

    pub fn move_snake_head(&mut self, food_location: Food) -> AppState {
        match self.direction {
            Direction::Up => {
                if self.head.current_position.y < 1.0 {
                    return AppState::Dead;
                }

                self.move_snake_body();
                let alive = self
                    .set_head(
                        self.head.current_position.x,
                        self.head.current_position.y + 1.0,
                    )
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
                self.move_snake_body();
                let alive = self
                    .set_head(
                        self.head.current_position.x,
                        self.head.current_position.y - 1.0,
                    )
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
                if self.head.current_position.x < 1.0 {
                    return AppState::Dead;
                }

                self.move_snake_body();
                let alive = self
                    .set_head(
                        self.head.current_position.x - 1.0,
                        self.head.current_position.y,
                    )
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
                self.move_snake_body();
                let alive = self
                    .set_head(
                        self.head.current_position.x + 1.0,
                        self.head.current_position.y,
                    )
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
        let coliding = {
            self.head.current_position.x == food_pos.x && self.head.current_position.y == food_pos.y
        };

        return coliding;
    }

    fn set_head(&mut self, x: f64, y: f64) -> Result<bool> {
        // Figure out a way to verify its a valid positition in the future w/ idomatic rust w/o adding additional argument
        // https://users.rust-lang.org/t/current-best-practice-for-parent-child-struct-relationship/84542/3
        // https://www.sitepoint.com/rust-global-variables/

        self.head.last_position = self.head.current_position;

        self.head.current_position.x = x;
        self.head.current_position.y = y;

        Ok(true)
    }

    fn get_head(&self) -> (f64, f64) {
        return (self.head.current_position.x, self.head.current_position.y);
    }
}
