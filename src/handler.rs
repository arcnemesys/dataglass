use crate::app::{stream_episode, App, AppResult, PlaybackState, SelectedList};
use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use reqwest::blocking::get;
use rodio::{source::Source, Decoder, OutputStream};
use std::io::Cursor;
use tokio;
/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        // Exit application on `ESC` or `q`
        KeyCode::Esc | KeyCode::Char('q') => {
            app.quit();
        }
        // Exit application on `Ctrl-C`
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit();
            }
        }
        KeyCode::Up => {
            if app.selected_episode > 0 {
                app.selected_episode -= 1;
                app.episode_list_state.select(Some(app.selected_episode));
            }
        }
        KeyCode::Down => {
            if app.selected_episode < app.episodes.len() - 1 {
                app.selected_episode += 1;
                app.episode_list_state.select(Some(app.selected_episode));
            }
        }
        KeyCode::Enter => {}
        // Counter handlers
        KeyCode::Right => {}
        KeyCode::Left => {}
        // Other handlers you could add here.
        _ => {}
    }
    Ok(())
}
