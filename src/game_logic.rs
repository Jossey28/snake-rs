use std::time::Duration;

use ratatui::layout::Position;
use ratatui::layout::Rect;
use ratatui::widgets::Widget;
use ratatui::style::Color;


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

        if current_direction == target_direction { return; }
        if ( current_direction == Direction::Left ) && ( target_direction == Direction::Right) {
            return;
        }
        if ( current_direction == Direction::Right ) && ( target_direction == Direction::Left) {
            return;
        } 
        if ( current_direction == Direction::Up ) && ( target_direction == Direction::Down) {
            return;
        }
        if ( current_direction == Direction::Down ) && ( target_direction == Direction::Up) {
            return;
        }

        self.direction = target_direction;
    }

    pub fn move_snake(&mut self, app: App) -> AppState {
        match self.direction {
            Direction::Up => {
                if self.head.y > 0 {
                    self.head.y -= 1;

                    if self.is_at_food(app.food) {
                      return AppState::Coliding;  
                    } 

                    return AppState::Active;
                } else {
                    return AppState::Dead;
                }
            },
            Direction::Down =>{ 
                if self.head.y + 1 < app.settings.terminal_height {
                    self.head.y += 1;

                    if self.is_at_food(app.food) {
                      return AppState::Coliding;  
                    } 
                    
                    return AppState::Active;
                } else {
                    return AppState::Dead;
                }
            },
            Direction::Left => {
                if self.head.x > 0 {
                    self.head.x -= 1;

                    if self.is_at_food(app.food) {
                      return AppState::Coliding;  
                    } 

                    return AppState::Active;
                } else {
                    return AppState::Dead;
                }
            },
            Direction::Right => {
                if self.head.x + 1 < app.settings.terminal_width {
                    self.head.x += 1;

                    if self.is_at_food(app.food) {
                      return AppState::Coliding;  
                    } 

                    return AppState::Active;
                } else {
                    return AppState::Dead;
                }  
            },
        }
    }

    fn is_at_food(&self, food: Position) -> bool {
        let coliding = {
            self.head == food
        };

        return coliding;
    }
}

impl Widget for &Snake {
    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let terminal_color = buf
            .cell_mut(Position::new(area.width - 1, area.height - 1))
            .expect("Invalid buffer for term color")
            .bg;

        let snake_head = buf
            .cell_mut(self.head)
            .expect("invalid snake head position");
        // snake_head.set_char('▀');
        match self.direction {
            Direction::Left => snake_head.set_char('◀'), // https://cloford.com/resources/charcodes/utf-8_geometric.htm
            Direction::Right => snake_head.set_char('▶'), // Starting @ UTF8+9654
            Direction::Up => snake_head.set_char('▲'),   // Or "BLACK UP-POINTING TRIANGLE"
            Direction::Down => snake_head.set_char('▼'),
        };
        snake_head.set_fg(Color::Green);
        snake_head.set_bg(terminal_color);

    }
}


#[derive(Debug, Clone)]
pub struct Settings {
    pub terminal_width: u16,
    pub terminal_height: u16,

    pub tick_rate: Duration,
}

impl Default for Settings {
    fn default() -> Self {
        Settings { terminal_width: 0, terminal_height: 0, tick_rate: Duration::from_millis(50) }
    }
}