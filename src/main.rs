use reqwest::get;
// use rodio::{source::Source, Decoder, OutputStream, Sink};
use dataglass::app::App;
use dataglass::event::{Event, EventHandler};
use dataglass::handler::handle_key_events;
use dataglass::tui::Tui;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use rss::Channel;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::error::Error;
use std::io;
use std::io::BufReader;


pub async fn music_for_programming_test() -> Result<(), Box<dyn Error>> {
    let url = "https://musicforprogramming.net/rss.xml";
    let response = get(url).await.unwrap();

    if response.status().is_success() {
        let content = response.text().await?;
        let channel = Channel::read_from(content.as_bytes())?;

        println!("Title: {}", channel.title());
        println!("Description: {}", channel.description());

        for item in channel.items() {

            let title = item.title().unwrap().to_owned();
            // let episode = Episode {
            //     title,
            // }
            println!("Item: {}", item.title().unwrap_or_default());
            println!("\n");
            println!("Item Categories: {:?}", item.categories());
            println!("\n");
            println!("Item Comments: {:?}", item.comments().unwrap());
            println!("\n");
            // println!("Item Content: {:?}", item.content().unwrap());
            println!("\n");
            println!("Item ItunesExt {:?}", item.itunes_ext().unwrap());
            println!("\n");
            println!("Item Pub date: {:?}", item.pub_date());
            println!("\n");
            println!("Link: {}", item.link().unwrap_or_default());
            println!("\n");
            println!("Description: {}", item.description().unwrap_or_default());
            println!("------");
        }
    } else {
        println!("Failed to fetch the RSS feed: HTTP {}", response.status());
    }

    Ok(())
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let mut app = App::new();

    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    // Start the main loop.
    while app.running {
        // Render the user interface.
        tui.draw(&mut app)?;
        // Handle events.
        match tui.events.next()? {
            Event::Key(key_event) => handle_key_events(key_event, &mut app)?,
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        }
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
