use ratatui::Frame;
use ratatui::layout::Alignment;
use ratatui::layout::Rect;
use ratatui::style::Style;

use ratatui::widgets::Paragraph;
use tui_big_text::BigText;
use tui_big_text::PixelSize;

use ratatui::prelude::Stylize;

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

pub fn display_menu_title(frame: &mut Frame, area: Rect) -> () {
    // let top_text = Line::from("value").bold().alignment(Alignment::Center);
    let text = BigText::builder()
        .pixel_size(PixelSize::Octant) // See if I can make it full width. It cuts out atm
        // .style(style)
        .lines(vec!["Click tab to switch tabs".gray().into()])
        .alignment(Alignment::Center)
        .build();

    frame.render_widget(text, area);
}
pub fn display_menu(frame: &mut Frame, area: Rect) -> () {
    let menu = Paragraph::new("imagine menu").alignment(Alignment::Center);

    frame.render_widget(menu, area);
}
