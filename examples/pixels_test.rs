use std::io;
use std::time::{Duration, SystemTime};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};

use tui_big_text::{BigText, PixelSize};

use rand::{Rng, RngExt};

use ratatui::layout::{Alignment, Constraint, Layout, Position, Rect};
use ratatui::style::{Color, Style, Stylize};
use ratatui::symbols::Marker;
use ratatui::text::{Line as TextLine, Span, Text};
use ratatui::{DefaultTerminal, Frame};
use ratatui::widgets::{Block, Borders, Paragraph, Widget};

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

        for (_xi, x) in (area.left()..area.right()).enumerate() {
            for (_yi, y) in (area.top()..area.bottom()).enumerate() {
                dbg!(&buf);

                Paragraph::new(format!("{} {}", x, y))
                    .block(Block::bordered())
                    .render(area, buf);
            }
        }
    }
}
