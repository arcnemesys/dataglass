use anyhow::{Context, Error, Result};
use futures::{TryStream, TryStreamExt};
use futures_util::stream::StreamExt;
use ratatui::widgets::ListState;
use reqwest::get;
use rodio::{DeviceTrait, OutputStream, Sink};
use rss::Channel;
use std::{error::Error as StdErr, io::Read, num::NonZeroUsize, time::Duration};
use stream_download::{
    http::{reqwest::Client, HttpStream},
    source::SourceStream,
    storage::{adaptive::AdaptiveStorageProvider, memory::MemoryStorageProvider, StorageProvider},
    Settings, StreamDownload, StreamState,
};
use tokio::fs::{create_dir_all, File};
use tokio::io::AsyncWriteExt;
pub type AppResult<T> = Result<T, Box<dyn StdErr>>;

#[derive(Debug, Clone)]
pub struct Episode {
    pub title: String,
    pub audio_url: String,
    pub author: String,
    pub duration: String,
    pub key_words: String,
    pub pub_date: String,
    pub link: String,
}

#[derive(Debug, Clone)]
pub struct App {
    pub episodes: Vec<Episode>,
    pub current_track: Option<Episode>,
    pub playback_state: PlaybackState,
    pub episode_list_state: ListState,
    pub menu_list_state: ListState,
    pub selected_episode: usize,
    pub running: bool,
    pub client: Client,
    pub selected_list: SelectedList,
}

#[derive(Clone, Debug)]
pub enum SelectedList {
    Episodes,
}

impl App {
    pub fn new() -> Self {
        let mut episode_list_state = ListState::default();
        episode_list_state.select(Some(0));
        let mut menu_list_state = ListState::default();
        menu_list_state.select(Some(0));
        let client = Client::new();

        Self {
            episodes: Vec::new(),
            current_track: None,
            selected_episode: 0,
            playback_state: PlaybackState::Stopped,
            episode_list_state,
            menu_list_state,
            running: true,
            client,
            selected_list: SelectedList::Episodes,
        }
    }
    pub fn quit(&mut self) {
        self.running = false;
    }

    // pub async fn stream_episode(&self, url: &String) -> Result<(), Box<dyn Error>> {
    //     let (_stream, handle) = OutputStream::try_default()?;
    //     let sink = Sink::try_new(&handle)?;
    //     let prefetch_bytes = 192 / 8 * 1024 * 10;
    //     let settings = Settings::default().prefetch_bytes(prefetch_bytes);
    //     let adaptive_storage = AdaptiveStorageProvider::new(
    //         MemoryStorageProvider,
    //         NonZeroUsize::new((settings.get_prefetch_bytes() * 2) as usize).unwrap(),
    //     );
    //     let stream = HttpStream::new(self.client.clone(), url.parse()?).await?;

    //     let reader = StreamDownload::from_stream(stream, adaptive_storage, settings).await?;

    //     sink.append(rodio::Decoder::new(reader)?);

    //     let handle = tokio::task::spawn(move || {
    //         sink.sleep_until_end();
    //     });
    //     handle.await?;
    //     Ok(())
    // }
}

#[derive(Debug, Clone)]
pub enum PlaybackState {
    Playing,
    Paused,
    Stopped,
    Muted,
}

#[derive(Debug)]
enum Message {
    NextEpisode,
    PrevEpisode,
    Play,
    Pause,
    Stop,
    Save,
    Seek,
}

const MFP_FEED: &str = "https://musicforprogramming.net/rss.xml";

pub async fn music_for_programming() -> Result<Vec<Episode>, Error> {
    let url = "https://musicforprogramming.net/rss.xml";
    let response = get(url).await.unwrap();
    let mut episodes = Vec::new();

    if response.status().is_success() {
        let content = response.text().await.unwrap();
        let channel = Channel::read_from(content.as_bytes())?;

        for item in channel.items() {
            let title = item.title().unwrap().to_owned();
            let audio_url = item.comments().unwrap().to_owned();
            let itunes_ext = item.itunes_ext().unwrap().to_owned();
            let author = &itunes_ext.author.unwrap().clone();
            let duration = &itunes_ext.duration.unwrap();
            let keywords = &itunes_ext.keywords.unwrap();
            let pub_date = item.pub_date().unwrap().to_owned();
            let link = item.clone().link.unwrap();
            let episode = Episode {
                title,
                audio_url,
                author: author.to_owned(),
                duration: duration.to_owned(),
                key_words: keywords.to_owned(),
                pub_date,
                link,
            };
            episodes.push(episode);
        }
    } else {
        println!("Failed to fetch the RSS feed: HTTP {}", response.status());
    }

    Ok(episodes)
}

pub async fn stream_episode(app: &mut App, url: &String) -> Result<(), Error> {
    let (_stream, handle) = OutputStream::try_default()?;
    let sink = Sink::try_new(&handle)?;
    let prefetch_bytes = 192 / 8 * 1024 * 10;
    let settings = Settings::default().prefetch_bytes(prefetch_bytes);
    let adaptive_storage = AdaptiveStorageProvider::new(
        MemoryStorageProvider,
        NonZeroUsize::new((settings.get_prefetch_bytes() * 2) as usize).unwrap(),
    );
    // let stream = HttpStream::new(app.client.clone(), url.parse()?).await?;

    let mut reader =
        StreamDownload::new::<HttpStream<Client>>(url.parse()?, adaptive_storage, settings).await?;

    let mut buf = Vec::new();

    reader.read_to_end(&mut buf)?;
    sink.append(rodio::Decoder::new(reader)?);

    let handle = tokio::task::spawn_blocking(move || {
        sink.sleep_until_end();
    });
    handle.await?;
    Ok(())
}

pub async fn download_episodes(app: &App) -> Result<(), Error> {
    let mut home_dir = dirs::home_dir().context("Could not find home directory")?;
    home_dir.push(".config");
    home_dir.push("music-for-programming");

    create_dir(&home_dir)
        .await
        .context("Failed to create directory")?;

    let mut file = File::create(home_dir.clone())
        .await
        .context("Failed to create file")?;
    for i in app.episodes.clone() {
        let resp = reqwest::get(i.audio_url)
            .await
            .context("Failed to Get from '{url}'")?;
        let mut stream = resp.bytes_stream();
        while let Ok(Some(chunk)) = stream.try_next().await {
            let mut split_title = i.title.splitn(2, ":");
            let episode_number = split_title.next().unwrap();
            let episode_title = split_title.next().unwrap();
            home_dir.push(format!("{}_{}.mp3", episode_number, episode_title));
            // let audio_chunk = chunk?;
            file.write_all(&chunk)
                .await
                .context("Error while writing to file");
        }
        home_dir.pop();
    }

    Ok(())
}
