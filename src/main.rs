use std::{default, io};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};

use tui_big_text::{BigText, PixelSize};

use ratatui::layout::{Alignment, Constraint, Layout, Rect};
use ratatui::style::{Color, Style, Stylize};
use ratatui::symbols::Marker;
use ratatui::text::{Line as TextLine, Span, Text};
use ratatui::{DefaultTerminal, Frame};
use ratatui::widgets::{Block, Borders, Paragraph, Widget};

fn main() -> io::Result<()> {
    ratatui::run(|terminal| App::default().run(terminal))
}

#[derive(Debug)]
pub struct App {
    exit: bool,
    appstate: AppState,
}

#[derive(Debug, Default)]
pub enum AppState {
    #[default]
    TitleScreen,
    Active,
    Dead
}

impl Default for App {
    fn default() -> Self {
        App { exit: false, appstate: AppState::TitleScreen }
    }
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }

        Ok(())
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn draw(&self, frame: &mut Frame) {
        match self.appstate {
            AppState::TitleScreen => self.show_title(frame),
            AppState::Active => self.play_snake(frame),
            _ => {}
        }
    }

    fn show_title(&self, frame: &mut Frame) -> ()  {
        let verticle_chunks = Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints([
                Constraint::Ratio(1, 3),
                Constraint::Fill(1),
                Constraint::Ratio(1, 3),
            ]).split(frame.area());

        let horizontal_chunks = Layout::default()
            .direction(ratatui::layout::Direction::Horizontal)
            .constraints([
                Constraint::Ratio(1, 3),
                Constraint::Fill(1),
                Constraint::Ratio(1, 3),
            ]).split(verticle_chunks[1]);

        let style = Style::new() 
            .bold();


        let text = BigText::builder()
                .pixel_size(PixelSize::HalfWidth) // See if I can make it full width. It cuts out atm 
                .style(style)
                .lines(vec![
                    "Snake-Rs".red().into(),
                ])
                .build();

        frame.render_widget(text, horizontal_chunks[1]);
    }

    fn play_snake(&self, frame: &mut Frame) {
        let area = Rect::new(5, 2, 50, 20);

        let block = Block::default()
            .title("Floating Rect")
            .borders(Borders::all());

        frame.render_widget(block, area);
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event);
            }
            _ => {}
        }

        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            _ => { self.exit() }
        }
    }
}

// impl Widget for &App {
//     fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer)
//     where
//         Self: Sized
//     {
//         let title = Paragraph::new("Hello World");

//         title.render(area, buf);
//     }
// }