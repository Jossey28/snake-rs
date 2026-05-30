use ratatui::Frame;
use ratatui::layout::Alignment;
use ratatui::layout::Rect;
use ratatui::style::Color;
use ratatui::style::Style;
use ratatui::symbols;
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;
use ratatui::widgets::Tabs;
use tui_big_text::BigText;
use tui_big_text::PixelSize;

use ratatui::prelude::Stylize;

use crate::AvailableTabs;

pub fn show_title(frame: &mut Frame, area: Rect) -> () {
    let style = Style::new().bold();

    let text = BigText::builder()
        .pixel_size(PixelSize::Full) // See if I can make it full width. It cuts out atm
        .style(style)
        .lines(vec!["Snake-Rs".red().into()])
        .alignment(Alignment::Center)
        .build();

    frame.render_widget(text, area);
}

pub fn display_menu(frame: &mut Frame, area: Rect, tab: AvailableTabs) -> () {
    let tabs = Tabs::new(vec!["Tab 1", "Tab 2", "Tab 3"])
        .style(Color::White)
        .highlight_style(Style::default().magenta().on_black().bold())
        .select(tab as usize)
        .divider(symbols::DOT)
        .padding(" ", " ");

    frame.render_widget(tabs, area);
}

fn render_tabs(frame: &mut Frame, area: Rect, tab: AvailableTabs) {
    match tab {
        _ => todo!()
    } 
}

pub fn display_score(frame: &mut Frame, area: Rect, count: i64) {
    let title = Line::from_iter([
        Span::from("Score: ").bold(),
        Span::from(format!("{}", count).blue()),
    ]);

    frame.render_widget(title, area);
}
