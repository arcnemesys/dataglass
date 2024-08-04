use ratatui::widgets::ListState;

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
struct App {
    playlist: Vec<Episode>,
    current_track: Option<Episode>,
    playback_state: PlaybackState,
    list_state: ListState,
    // sink:
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
