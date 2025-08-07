use color_eyre::eyre::{Context, Ok, Result};
// use crossterm::event;
// use crossterm::terminal::Clear;
// use std::time::Duration;
// use ratatui::{
//     DefaultTerminal, Frame,
//     crossterm::event::{self, Event, KeyCode},
//     layout::{Constraint, Direction, Layout},
//     widgets::{Paragraph, Wrap},
// };
use quick_xml::de::from_str;
use reqwest::blocking::Client;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
struct Feed {
    entry: Vec<Entry>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
struct Entry {
    author: Author,
    title: String,
    content: String,
    id: String,
    link: Link,
    updated: String,
    published: String,
}

#[derive(Deserialize, Debug)]
struct Author {
    name: String,
    uri: String,
}

#[derive(Deserialize, Debug)]
struct Link {
    #[serde(rename = "@href")]
    href: String,
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let client = Client::new();
    let xml = client
        .get("https://www.reddit.com/r/physicsmemes/top/.rss")
        .header("User-Agent", "Digest/0.1 by u/rengine199")
        .send()?
        .text()?;

    let feed: Feed = from_str(&xml)?;

    for entry in feed.entry {
        println!("{:#?}", entry.author);
        println!("{:#?}", entry.title);
        println!("{:#?}", entry.content);
    }

    // let terminal = ratatui::init();
    // let app_result = run(terminal).context("App loop failed");
    // ratatui::restore();

    // app_result
    Ok(())
}

// fn run(mut terminal: DefaultTerminal) -> Result<()> {
//     loop {
//         terminal.draw(|f| draw(f))?;
//         if should_quit()? {
//             break;
//         }
//     }
//     Ok(())
// }

// fn draw(frame: &mut Frame) {
//     let chunks = Layout::default()
//         .direction(Direction::Vertical)
//         .constraints([Constraint::Length(3), Constraint::Min(0)])
//         .split(frame.area());

//     let title = Paragraph::new("test");
//     frame.render_widget(title, chunks[0]);

//     let extract = Paragraph::new("test").wrap(Wrap { trim: true });
//     frame.render_widget(extract, chunks[1]);
// }

// fn should_quit() -> Result<bool> {
//     if event::poll(Duration::from_millis(250)).context("Event poll failed")? {
//         if let Event::Key(key) = event::read().context("Event read failed")? {
//             return Ok(KeyCode::Char('q') == key.code);
//         }
//     }
//     Ok(false)
// }
