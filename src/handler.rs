use crate::app::{stream_episode, App, AppResult, PlaybackState, SelectedList};
use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Handles the key events and updates the state of [`App`].
pub async fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
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
        KeyCode::Enter => match app.selected_list {
            SelectedList::Episodes => {
                app.playback_state = PlaybackState::Playing;
                let url = app.episodes[app.selected_episode].audio_url.clone();
                stream_episode(app, &url).await?;
            }
        },
        // Counter handlers
        KeyCode::Right => {}
        KeyCode::Left => {}
        // Other handlers you could add here.
        _ => {}
    }
    Ok(())
}
