use std::fmt::format;

use color_eyre::owo_colors::style;
use ratatui::Frame;
use ratatui::layout::Alignment;
use ratatui::layout::Constraint;
use ratatui::layout::Layout;
use ratatui::layout::Offset;
use ratatui::layout::Rect;
use ratatui::style::Color;
use ratatui::style::Modifier;
use ratatui::style::Style;
use ratatui::symbols;
use ratatui::text::Text;
use ratatui::text::{Line, Span};
use ratatui::widgets::Block;
use ratatui::widgets::BorderType;
use ratatui::widgets::Borders;
use ratatui::widgets::Padding;
use ratatui::widgets::Paragraph;
use ratatui::widgets::Tabs;
use tui_big_text::BigText;
use tui_big_text::PixelSize;

use ratatui::prelude::Stylize;

use crate::AvailableTabs;
use crate::game_logic::GameSettings;

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

pub fn display_tabs(frame: &mut Frame, area: Rect, tab: AvailableTabs, state: &mut GameSettings) {
    match tab {
        AvailableTabs::Instructions => instruction_tab(frame, area),
        AvailableTabs::Settings => settings_tab(frame, area, state),
        // AvailableTabs::Credits => credits_tab(frame, area),
    };
}

fn instruction_tab(frame: &mut Frame, area: Rect) {
    let outer = Block::bordered()
        .title("Settings")
        .border_type(BorderType::Rounded);

    // let [content_area] = Layout::vertical([Constraint::Fill(1)]).areas(outer.inner(area));

    let [_, center, footer] = Layout::vertical([
        Constraint::Fill(1),
        Constraint::Length(8),
        Constraint::Fill(1),
    ])
    .areas(outer.inner(area));

    let intro_lines = vec![
        "Hello! Welcome to my snake very own".bold().green().into(),
        "snake clone ;)".bold().green().into(),
        "Click tab to view the settings menu.".bold().cyan().into(),
        "Click Enter to get started.".bold().light_red().into(),
        "Make sure to have fun!".bold().light_red().into(),
    ];

    let intro = BigText::builder()
        .alignment(Alignment::Center)
        .pixel_size(PixelSize::Octant)
        .lines(intro_lines)
        .build();

    let instructions_lines = vec![
        Line::from_iter(vec!["Start game -> Enter"]),
        Line::from_iter(vec!["Quit game to Title -> Q "]),
        Line::from_iter(vec!["Exit game -> Esc "]),
    ];

    let instructions = Paragraph::new(instructions_lines).alignment(Alignment::Center);

    // let block = Paragraph::new(lines)
    //     .alignment(Alignment::Center)
    //     .block(
    //         Block::bordered()
    //             .padding(Padding::new(0, 0, (area.height / 2) - 3, 0))
    //             .borders(Borders::ALL)
    //             .border_type(BorderType::Rounded)
    //             .title("Instructions"),
    //     )
    //     .style(Style::new().white());

    frame.render_widget(outer, area);
    frame.render_widget(intro, center);
    frame.render_widget(instructions, footer + Offset::new(0, 3));
}

fn settings_tab(frame: &mut Frame, area: Rect, state: &mut GameSettings) {
    let outer = Block::bordered()
        .title("Settings")
        .border_type(BorderType::Rounded);

    let [header_area, notice_area, content_area] =
        Layout::vertical(Constraint::from_percentages([30, 5, 65])).areas(outer.inner(area));

    let line = match state.active_row {
        0 => "Invisible Mode",
        // 1 => "Tick Rate (Speed)",
        1 => "Snake Head Color",
        2 => "Snake Body Color",
        3 => "Walls Color",
        4 => "Food Color",
        _ => unreachable!(),
    };

    let notice = Paragraph::new(
        "up arrow (↑) and (↓) down arrow to choose setting ; (-->) right arrow to modify settings",
    )
    .alignment(Alignment::Center)
    .style(Style::default().fg(Color::Cyan));

    // let settings_title = B::new(line).alignment(Alignment::Center).bg(Color::LightCyan);
    let header = BigText::builder()
        .pixel_size(PixelSize::Octant)
        .lines(vec![line.into()])
        .alignment(Alignment::Center)
        .build();

    let mut builder = BigText::builder();
    let content = match state.active_row {
        0 => {
            if state.invisible {
                builder.lines(vec!["Currently Invisible".into()])
            } else {
                builder.lines(vec!["Currently Mortal".into()])
            }
        }
        // 1 => {
        //     let tick_rate = state.tick_rate.as_millis().to_string();
        //     let lines = vec![
        //         "Currently at".into(),
        //         format!("{}ms", tick_rate).bold().into(),
        //     ];

        //     builder.lines(lines)
        // }
        1 => {
            let color = state.head_color;
            let lines = vec![
                "Head color currently set to:".into(),
                format!("{}", color).fg(color).bold().into(),
            ];

            builder.lines(lines)
        }
        2 => {
            let color = state.body_color;
            let lines = vec![
                "Body color currently set to:".into(),
                format!("{}", color).fg(color).bold().into(),
            ];

            builder.lines(lines)
        }
        3 => {
            let color = state.wall_color;
            let lines = vec![
                "Wall color currently set to:".into(),
                format!("{}", color).fg(color).bold().into(),
            ];

            builder.lines(lines)
        }
        4 => {
            let color = state.food_color;
            let lines = vec![
                "Food color currently set to:".into(),
                format!("{}", color).fg(color).bold().into(),
            ];

            builder.lines(lines)
        }
        _ => unreachable!(),
    };

    let content = content
        .alignment(Alignment::Center)
        .pixel_size(PixelSize::Octant)
        .build();
    frame.render_widget(outer, area);
    frame.render_widget(header, header_area + Offset::new(0, 2));
    frame.render_widget(notice, notice_area);
    frame.render_widget(content, content_area + Offset::new(0, 3));
}

fn credits_tab(frame: &mut Frame, area: Rect) {
    let lines = Text::from_iter([Span::from("Credits stuff").bold().dark_gray()]);
    let block = Paragraph::new(lines)
        .alignment(Alignment::Center)
        .block(
            Block::bordered()
                .padding(Padding::new(0, 0, (area.height / 2) - 3, 0))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .title("Credits"),
        )
        .style(Style::new().white());

    frame.render_widget(block, area);
}

pub fn display_score(frame: &mut Frame, area: Rect, count: i64) {
    let score = Line::from(vec!["Score: ".bold().into(), format!("{}", count).blue().into()]);

    let display = Paragraph::new(score).alignment(Alignment::Center);
    frame.render_widget(display, area);
}
