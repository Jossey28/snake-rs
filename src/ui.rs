use color_eyre::owo_colors::style;
use ratatui::Frame;
use ratatui::layout::Alignment;
use ratatui::layout::Constraint;
use ratatui::layout::Layout;
use ratatui::layout::Rect;
use ratatui::style::Color;
use ratatui::style::Style;
use ratatui::symbols;
use ratatui::text::Text;
use ratatui::text::{Line, Span};
use ratatui::widgets::Block;
use ratatui::widgets::BorderType;
use ratatui::widgets::BorderType::Rounded;
use ratatui::widgets::Borders;
use ratatui::widgets::Padding;
use ratatui::widgets::Paragraph;
use ratatui::widgets::Tabs;
use tui_big_text::BigText;
use tui_big_text::PixelSize;

use ratatui::prelude::Stylize;

use crate::AvailableTabs;

pub fn show_title(frame: &mut Frame, area: Rect) -> () {
    let [_, center, _] = Layout::vertical([
        Constraint::Fill(1),
        Constraint::Length(8),
        Constraint::Fill(1),
    ])
    .areas(area);

    let style = Style::new().bold();

    let text = BigText::builder()
        .pixel_size(PixelSize::Full) // See if I can make it full width. It cuts out atm
        .style(style)
        .lines(vec!["Snake-Rs".red().into()])
        .alignment(Alignment::Center)
        .build();

    frame.render_widget(text, center);
}

pub fn display_menu(frame: &mut Frame, area: Rect, tab: AvailableTabs) -> () {
    let tabs = Tabs::new(vec!["Home", "Settings", "Credits"])
        .style(Color::White)
        .highlight_style(Style::default().magenta().on_black().bold())
        .select(tab as usize)
        .divider(symbols::DOT)
        .padding(" ", " ");

    let area = area.centered_horizontally(Constraint::Length(1));

    frame.render_widget(tabs, area);
}

pub fn display_tabs(frame: &mut Frame, area: Rect, tab: AvailableTabs) {
    let lines = match tab {
        AvailableTabs::Instructions => Text::from_iter([
            Span::from("Hello! Welcome to my snake very own snake clone ;)")
                .bold()
                .green(),
            Span::from("Click tab to view the other menus.")
                .bold()
                .cyan(),
            Span::from("Click Enter to get started. Make sure to have fun!")
                .bold()
                .light_red(),
        ]),
        AvailableTabs::Settings => Text::from_iter([Span::from("Settings stuff").bold().cyan()]),
        AvailableTabs::Credits => Text::from_iter([Span::from("Credits stuff").bold().dark_gray()]),
    };

    let block = Paragraph::new(lines)
        .alignment(Alignment::Center)
        .block(
            Block::bordered()
                .padding(Padding::new(0, 0, (area.height / 2) - 3, 0))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().bg(Color::DarkGray))
                .title("Title"),
        )
        .style(Style::new().white());

    frame.render_widget(block, area);
}

pub fn display_score(frame: &mut Frame, area: Rect, count: i64) {
    let title = Line::from_iter([
        Span::from("Score: ").bold(),
        Span::from(format!("{}", count).blue()),
    ]);

    frame.render_widget(title, area);
}
