use anyhow::Result as AnyResult;
use futures::{StreamExt, TryStream, TryStreamExt};
use ratatui::widgets::ListState;
use ratatui::prelude::Rect;
use reqwest::get;
use rodio::{Decoder, DeviceTrait, OutputStream, Sink, Source};
use rss::Channel;
use std::{
    error::Error,
    io::{Cursor, Read, Write},
    num::NonZeroUsize,
    sync::{Arc, RwLock, mpsc as std_mpsc},
    time::Duration,
};
use stream_download::{
    http::{reqwest::Client, HttpStream},
    source::SourceStream,
    storage::{adaptive::AdaptiveStorageProvider, memory::MemoryStorageProvider, StorageProvider},
    Settings, StreamDownload, StreamState,
};
use image::{DynamicImage, Rgb};
use tokio::fs::{create_dir_all, File};
use tokio::io::AsyncWriteExt;
use tokio::sync::mpsc;
use ratatui_image::{Resize, picker::Picker, protocol::StatefulProtocol, thread::ThreadProtocol};
pub type AppResult<T> = Result<T, Box<dyn Error>>;

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

pub struct App {
    pub episodes: Arc<RwLock<Vec<Episode>>>,
    pub current_track: Option<Episode>,
    pub playback_state: PlaybackState,
    pub episode_list_state: ListState,
    pub menu_list_state: ListState,
    pub selected_episode: usize,
    pub running: bool,
    pub client: Client,
    pub selected_list: SelectedList,
    pub async_state: ThreadProtocol,
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
        let (tx_w, rx_w) = std_mpsc::channel::<(Box<dyn StatefulProtocol>, Resize, Rect)>();
            let mut picker = Picker::from_termios().unwrap();
    picker.guess_protocol();
    picker.background_color = Some(Rgb::<u8>([255, 0, 255]));
let dyn_img = dynamic_image().unwrap();

        Self {
            episodes: Arc::new(RwLock::new(Vec::new())),
            current_track: None,
            selected_episode: 0,
            playback_state: PlaybackState::Stopped,
            episode_list_state,
            menu_list_state,
            running: true,
            client,
            selected_list: SelectedList::Episodes,
            async_state: ThreadProtocol::new(tx_w, picker.new_resize_protocol(dyn_img.expect("Failed to load image"))),
        }
    }
    pub fn quit(&mut self) {
        self.running = false;
    }
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

pub async fn music_for_programming() -> Result<Vec<Episode>, Box<dyn Error>> {
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

pub async fn stream_and_play(url: &str) -> AnyResult<()> {
    let (tx, mut rx) = mpsc::channel(1024);

    let url = url.to_string();
    tokio::spawn(async move {
        let response = reqwest::get(&url).await?;
        let mut stream = response.bytes_stream();

        while let Some(item) = stream.next().await {
            if let Ok(chunk) = item {
                if tx.send(chunk).await.is_err() {
                    break;
                }
            }
        }
        anyhow::Ok(())
    });

    std::thread::spawn(move || -> AnyResult<()> {
        let (_stream, stream_handle) = OutputStream::try_default()?;
        let sink = Sink::try_new(&stream_handle)?;

        let mut buffer = Vec::new();

        let rt = tokio::runtime::Runtime::new()?;

        rt.block_on(async {
            while let Some(chunk) = rx.recv().await {
                buffer.extend_from_slice(&chunk);

                if buffer.len() > 1024 * 256 {
                    if let Ok(source) = Decoder::new(Cursor::new(buffer.clone())) {
                        sink.append(source);
                        buffer.clear();
                    }
                }
            }
        });

        sink.sleep_until_end();
        Ok(())
    });

    Ok(())
}

// tokio::signal::ctrl_c().await?;
//
pub fn dynamic_image() -> Result<DynamicImage, Box<dyn Error>> {
    let dyn_img = image::io::Reader::open("./assets/mfp.jpg")?.decode()?;
    Ok(dyn_img)
}
