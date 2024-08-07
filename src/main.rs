use reqwest::{get, Client};
// use rodio::{source::Source, Decoder, OutputStream, Sink};
use dataglass::app::{music_for_programming, App};
use dataglass::event::{Event, EventHandler};
use dataglass::handler::handle_key_events;
use dataglass::tui::Tui;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use rodio::{OutputStream, Sink};
use rss::Channel;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::error::Error;
use std::io;
use std::io::BufReader;
use std::num::NonZeroUsize;
use std::time::Duration;
use stream_download::http::HttpStream;
use stream_download::storage::memory::MemoryStorageProvider;
use stream_download::{storage::adaptive::AdaptiveStorageProvider, Settings, StreamDownload};
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut app = App::new();

    let mfp_episodes = music_for_programming().await.unwrap();
    app.episodes = mfp_episodes;
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;
    stream_episode(app.clone(), app.episodes[0].audio_url.clone())
        .await
        .unwrap();
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

pub async fn stream_episode(app: App, url: String) -> Result<(), Box<dyn Error>> {
    let (_stream, handle) = OutputStream::try_default()?;
    let sink = Sink::try_new(&handle)?;
    let prefetch_bytes = 192 / 8 * 1024 * 10;
    let settings = Settings::default().prefetch_bytes(prefetch_bytes);
    let adaptive_storage = AdaptiveStorageProvider::new(
        MemoryStorageProvider,
        NonZeroUsize::new((settings.get_prefetch_bytes() * 2) as usize).unwrap(),
    );
    let stream = HttpStream::new(app.client, url.parse()?).await?;

    let reader = StreamDownload::from_stream(stream, adaptive_storage, settings).await?;

    sink.append(rodio::Decoder::new(reader)?);

    let handle = tokio::task::spawn_blocking(move || {
        sink.sleep_until_end();
    });
    handle.await?;
    Ok(())
    // app.client
    // let settings = Settings::default().pre
}
