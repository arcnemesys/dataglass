use ratatui::widgets::ListState;
use reqwest::get;
use rss::Channel;
use std::{error::Error, io::Read, num::NonZeroUsize, result::Result};
use stream_download::{
    http::{reqwest::Client, HttpStream},
    source::SourceStream,
    storage::{bounded::BoundedStorageProvider, memory::MemoryStorageProvider, StorageProvider},
    Settings, StreamDownload, StreamState,
};
pub type AppResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Episode {
    pub title: String,
    pub audio_url: String,
    pub author: String,
    pub duration: String,
    pub key_words: String,
    pub pub_date: String,
    pub link: String,
}

#[derive(Debug)]
pub struct App {
    pub episodes: Vec<Episode>,
    pub current_track: Option<Episode>,
    pub playback_state: PlaybackState,
    pub list_state: Option<ListState>,
    pub running: bool,
    pub storage: BoundedStorageProvider<MemoryStorageProvider>, // sink:
}

impl App {
    pub fn new() -> Self {
        Self {
            episodes: Vec::new(),
            current_track: None,
            playback_state: PlaybackState::Stopped,
            list_state: None,
            running: true,
            storage: BoundedStorageProvider::new(
                MemoryStorageProvider,
                NonZeroUsize::new(512 * 1024).unwrap(),
            ),
        }
    }
    pub fn quit(&mut self) {
        self.running = false;
    }
}

#[derive(Debug)]
enum PlaybackState {
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

pub async fn music_for_programming() -> Result<Vec<Episode>, Box<dyn Error>> {
    let url = "https://musicforprogramming.net/rss.xml";
    let response = get(url).await.unwrap();
    let mut episodes = Vec::new();

    if response.status().is_success() {
        let content = response.text().await?;
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
