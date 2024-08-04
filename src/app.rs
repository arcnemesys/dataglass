use ratatui::widgets::ListState;
use reqwest::get;
use rss::Channel;
use std::error::Error;

pub type AppResult<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Debug)]
struct Episode {
    title: String,
    audio_url: String,
    author: String,
    duration: String,
    key_words: String,
    pub_date: String,
    link: String,
}

#[derive(Debug)]
pub struct App {
    pub episodes: Vec<Episode>,
    pub current_track: Option<Episode>,
    pub playback_state: PlaybackState,
    pub list_state: Option<ListState>,
    pub running: bool,
    // sink:
}

impl App {
    pub fn new() -> Self {
        Self {
            episodes: Vec::new(),
            current_track: None,
            playback_state: PlaybackState::Stopped,
            list_state: None,
            running: true,
        }
    }
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub async fn music_for_programming(&mut self) -> Result<(), Box<dyn Error>> {
        let url = "https://musicforprogramming.net/rss.xml";
        let response = get(url).await.unwrap();
    
        if response.status().is_success() {
            let content = response.text().await?;
            let channel = Channel::read_from(content.as_bytes())?;
    
            println!("Title: {}", channel.title());
            println!("Description: {}", channel.description());
    
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
                    link
                };
                self.episodes.push(episode);
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

pub async fn music_for_programming() -> Result<(), Box<dyn Error>> {
    let url = "https://musicforprogramming.net/rss.xml";
    let response = get(url).await.unwrap();

    if response.status().is_success() {
        let content = response.text().await?;
        let channel = Channel::read_from(content.as_bytes())?;

        println!("Title: {}", channel.title());
        println!("Description: {}", channel.description());

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
                link
            };
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