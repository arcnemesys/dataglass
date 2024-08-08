#![allow(unused)]
use dataglass::app::{download_episodes, music_for_programming, App};
use dataglass::event::{Event, EventHandler};
use dataglass::handler::handle_key_events;
use dataglass::tui::Tui;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use reqwest::{get, Client};
use rodio::{DeviceTrait, OutputStream, Sink};
use rss::Channel;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::error::Error;
use std::io;
use std::io::BufReader;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut app = App::new();

    let mfp_episodes = music_for_programming().await.unwrap();
    app.episodes = mfp_episodes;
    download_episodes(&app).await.unwrap();
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    while app.running {
        tui.draw(&mut app)?;
        match tui.events.next()? {
            Event::Key(key_event) => handle_key_events(key_event, &mut app)?,
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        }
    }

    tui.exit()?;
    Ok(())
}
