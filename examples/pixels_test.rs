use std::io;

use crossterm::event::{self, KeyCode};

use ratatui::layout::{Position, Rect};
use ratatui::style::Color;
use ratatui::{DefaultTerminal, Frame};
use ratatui::widgets::Widget;

fn main () -> io::Result<()> {
    ratatui::run(
        |terminal|
        App::default().run(terminal)
    )
}

#[derive(Default)]
struct App {
    exit: bool,
    terminal_width: u16,
    terminal_height: u16,
    mode: u8, 
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit { 
            terminal.draw(
                |frame|
                self.draw(frame)
            )?;

            if let Some(key) = event::read()?.as_key_press_event() {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => self.exit(),
                    KeyCode::Char('1') => self.mode = 1,
                    KeyCode::Char('2') => self.mode = 2,
                    _ => self.exit()
                }
            }
        }

        Ok(())
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn draw(&mut self, frame: &mut Frame){
        self.terminal_height = frame.area().height;
        self.terminal_width = frame.area().width;

        frame.render_widget(&*self, frame.area());
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized
    {
        // let boxes: Vec<Rect> = Vec::new();

        for (xi, x) in (area.left()..area.right()).enumerate() {
            for (yi, y) in (area.top()..area.bottom()).enumerate() {
                static POSSIBLE_COLORS: [Color; 3] = [Color::Red, Color::Green, Color::Blue];

                let buffer_unsafe = buf.cell_mut(Position::new(x, y));
                let buffer = buffer_unsafe.unwrap();

                match self.mode {
                    0 | 1 => buffer.set_bg(POSSIBLE_COLORS[xi % 3]),
                    2 => buffer.set_bg(POSSIBLE_COLORS[yi % 3]),
                    _ => buffer.set_bg(POSSIBLE_COLORS[xi % 3])
                };
            }
        }
    }
}
