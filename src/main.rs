use std::time::Duration;

use color_eyre::eyre::{Context, Ok, Result};

use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyCode},
    layout::{Constraint, Direction, Layout},
    widgets::{Paragraph, Wrap},
};

fn main() -> Result<()> {
    color_eyre::install()?;

    let terminal = ratatui::init();
    let app_result = run(terminal).context("App loop failed");
    ratatui::restore();

    app_result
}

fn run(mut terminal: DefaultTerminal) -> Result<()> {
    loop {
        terminal.draw(draw)?;
        if should_quit()? {
            break;
        }
    }
    Ok(())
}

fn draw(frame: &mut Frame) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(frame.area());

    let title = Paragraph::new("Title");
    frame.render_widget(title, chunks[0]);

    let extract = Paragraph::new("Text").wrap(Wrap { trim: true });
    frame.render_widget(extract, chunks[1]);
}

fn should_quit() -> Result<bool> {
    if event::poll(Duration::from_millis(250)).context("Event poll failed")? {
        if let Event::Key(key) = event::read().context("Event read failed")? {
            return Ok(KeyCode::Char('q') == key.code);
        }
    }
    Ok(false)
}
