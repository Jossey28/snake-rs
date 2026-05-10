use ratatui::Frame;
use ratatui::layout::Layout;
use ratatui::layout::Constraint;
use ratatui::style::Style;

use tui_big_text::BigText;
use tui_big_text::PixelSize;

use ratatui::prelude::Stylize;

pub fn show_title(frame: &mut Frame) -> ()  {
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