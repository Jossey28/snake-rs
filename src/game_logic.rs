use std::time::Duration;

use ratatui::layout::Position;

use crate::App;
use crate::AppState;

#[derive(Debug, Default, Clone, Copy)]
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
            head: Position { x: 0, y: 0 },
            body: None, 
            tail: None, 
            direction: Direction::Up, 
        }
    }
}

impl Snake {
    pub fn move_snake(&mut self, app: App) -> AppState {
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

#[derive(Debug, Clone)]
pub struct Settings {
    pub terminal_width: u16,
    pub terminal_height: u16,

    pub tick_rate: Duration,
}

impl Default for Settings {
    fn default() -> Self {
        Settings { terminal_width: 0, terminal_height: 0, tick_rate: Duration::from_millis(250) }
    }
}