// use color_eyre::eyre::Ok;
use ratatui::{DefaultTerminal, Frame, crossterm};

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    ratatui::run(app)?;
    Ok(())
}

fn app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    loop {
        terminal.draw(render)?;
        if crossterm::event::read()?.is_key_press() {
            break Ok(());
        }s
    }
}

fn render(frame: &mut Frame) {
    frame.render_widget("Hello World", frame.area());
}