use ratatui::widgets::ListState;
use std::error::Error;

pub type AppResult<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Debug)]
struct Episode {
    title: String,
    audio_file: String,
    author: String,
    image: String,
    duration: String,
    summary: String,
    key_words: Vec<String>,
    pub_date: String,
}

#[derive(Debug)]
pub struct App {
    pub playlist: Option<Vec<Episode>>,
    pub current_track: Option<Episode>,
    pub playback_state: PlaybackState,
    pub list_state: Option<ListState>,
    pub running: bool,
    // sink:
}

impl App {
    pub fn new() -> Self {
        Self {
            playlist: None,
            current_track: None,
            playback_state: PlaybackState::Stopped,
            list_state: None,
            running: true,
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
