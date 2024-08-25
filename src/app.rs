// use anyhow::{Context, Error, Result};
// use color_eyre::{eyre::Context, Result as EyResult, Section};
use futures::{StreamExt, TryStream, TryStreamExt};
use ratatui::widgets::ListState;
use reqwest::get;
use rodio::{DeviceTrait, Decoder, OutputStream, Sink, Source};
use rss::Channel;
use std::{
    error::Error,
    num::NonZeroUsize,
    sync::{Arc, RwLock},
    time::Duration,
    io::{Write, Read, Cursor}
};
use stream_download::{
    http::{reqwest::Client, HttpStream},
    source::SourceStream,
    storage::{adaptive::AdaptiveStorageProvider, memory::MemoryStorageProvider, StorageProvider},
    Settings, StreamDownload, StreamState,
};
use anyhow::Result as AnyResult;
use tokio::sync::mpsc;
use tokio::fs::{create_dir_all, File};
use tokio::io::AsyncWriteExt;
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

#[derive(Debug, Clone)]
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
            episodes: Arc::new(RwLock::new(Vec::new())),
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

// see: https://stackoverflow.com/questions/63463503/playing-audio-from-url-in-rust

    pub async fn stream_episode(&self, url: &String) -> Result<(), Box<dyn Error>> {
        let (_stream, handle) = OutputStream::try_default()?;
        let sink = Sink::try_new(&handle)?;
        let prefetch_bytes = 192 / 8 * 1024 * 10;
        let settings = Settings::default().prefetch_bytes(prefetch_bytes);
        let adaptive_storage = AdaptiveStorageProvider::new(
            MemoryStorageProvider,
            NonZeroUsize::new((settings.get_prefetch_bytes() * 2) as usize).unwrap(),
        );
        let stream = HttpStream::new(self.client.clone(), url.parse()?).await?;

        let reader = StreamDownload::from_stream(stream, adaptive_storage, settings).await?;

        sink.append(rodio::Decoder::new(reader)?);
        sink.play();
        Ok(())
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

pub async fn stream_episode(app: &mut App, url: &String) -> Result<(), Box<dyn Error>> {
    let (_stream, handle) = OutputStream::try_default()?;
    let sink = Sink::try_new(&handle)?;
    let prefetch_bytes = 192 / 8 * 1024 * 10;
    let settings = Settings::default().prefetch_bytes(prefetch_bytes);
    let adaptive_storage = AdaptiveStorageProvider::new(
        MemoryStorageProvider,
        NonZeroUsize::new((settings.get_prefetch_bytes() * 2) as usize).unwrap(),
    );
    let stream = HttpStream::new(app.client.clone(), url.parse()?).await?;

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

// pub async fn download_episodes(app: &App) -> Result<(), Error> {
//     let mut home_dir = dirs::home_dir().context("Could not find home directory")?;
//     home_dir.push(".config");
//     home_dir.push("music-for-programming");

//     create_dir(&home_dir)
//         .await
//         .context("Failed to create directory")?;

//     let mut file = File::create(home_dir.clone())
//         .await
//         .context("Failed to create file")?;
//     for i in app.episodes.clone() {
//         let resp = reqwest::get(i.audio_url)
//             .await
//             .context("Failed to Get from '{url}'")?;
//         let mut stream = resp.bytes_stream();
//         while let Ok(Some(chunk)) = stream.try_next().await {
//             let mut split_title = i.title.splitn(2, ":");
//             let episode_number = split_title.next().unwrap();
//             let episode_title = split_title.next().unwrap();
//             home_dir.push(format!("{}_{}.mp3", episode_number, episode_title));
//             // let audio_chunk = chunk?;
//             file.write_all(&chunk)
//                 .await
//                 .context("Error while writing to file");
//         }
//         home_dir.pop();
//     }

//     Ok(())
// }

#[derive(Debug, Default)]
struct EpisodeListState {
    episodes: Vec<Episode>,
    loading_state: LoadingState,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
enum LoadingState {
    #[default]
    Idle,
    Loading,
    Loaded,
    Error(String),
}
#[derive(Debug, Clone, Default)]
struct EpisodeListWidget {
    state: Arc<RwLock<EpisodeListState>>,
}

pub fn cursor_stream(url: String) {
    let response = reqwest::blocking::get(url).unwrap();
    let mut cursor = Cursor::new(response.bytes().unwrap());
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let source = rodio::Decoder::new(cursor).unwrap();
    stream_handle.play_raw(source.convert_samples());
    std::thread::sleep(std::time::Duration::from_secs(600));
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

                if buffer.len() > 1024 * 64 {
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

