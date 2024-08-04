use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::palette::{material, tailwind::{STONE, ROSE, FUCHSIA }},
    style::{Color, Style},
    text::Text,
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

const BACKGROUND: Color = STONE.c400;
const TEXT_COLOR: Color = ROSE.c800;
use crate::app::App;

pub fn render(app: &mut App, frame: &mut Frame) {
    let outer_layout = Layout::default()
        .direction(ratatui::layout::Direction::Horizontal)
        .constraints([
            Constraint::Percentage(30),
            Constraint::Percentage(40),
            Constraint::Percentage(30),
        ])
        .split(frame.size());

    let left_layout = Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints([
            Constraint::Percentage(10),
            Constraint::Percentage(10),
            Constraint::Percentage(10),
            Constraint::Percentage(10),
            Constraint::Percentage(10),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
        ])
        .split(outer_layout[0]);

    let menu_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::new().bg(BACKGROUND));

    let menu =
        Paragraph::new(Text::styled("MENU", Style::default().fg(TEXT_COLOR))).block(menu_block);

    let open_playlist_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::new().bg(BACKGROUND));

    let open_playlist = Paragraph::new(Text::styled(
        "OPEN PLAYLIST",
        Style::default().fg(TEXT_COLOR),
    ))
    .block(open_playlist_block);

    let save_to_playlist_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::new().bg(BACKGROUND));

    let save_to_playlist = Paragraph::new(Text::styled(
        "SAVE TO PLAYLIST",
        Style::default().fg(TEXT_COLOR),
    ))
    .block(save_to_playlist_block);

    let remove_from_playlist_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::new().bg(BACKGROUND));

    let remove_from_playlist = Paragraph::new(Text::styled(
        "REMOVE FROM PLAYLIST",
        Style::default().fg(TEXT_COLOR),
    ))
    .block(remove_from_playlist_block);

    let change_theme_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::new().bg(BACKGROUND));

    let change_theme = Paragraph::new(Text::styled(
        "Change Theme",
        Style::default().fg(TEXT_COLOR),
    ))
    .block(change_theme_block);

    let about_mfp_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::new().bg(BACKGROUND));

    let about_mfp = Paragraph::new(Text::styled(
        "musicForProgramming About",
        Style::default().fg(TEXT_COLOR),
    ))
    .block(about_mfp_block);

    let mfp_credits_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::new().bg(BACKGROUND));

    let mfp_credits = Paragraph::new(Text::styled(
        "musicForProgramming Credits",
        Style::default().fg(TEXT_COLOR),
    ))
    .block(mfp_credits_block);

    let middle_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(10),
            Constraint::Percentage(50),
            Constraint::Percentage(40),
        ])
        .split(outer_layout[1]);

    let ep_title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::new().bg(BACKGROUND));

    let ep_title = Paragraph::new(Text::styled(
        "Episode Title",
        Style::default().fg(TEXT_COLOR),
    ))
    .block(ep_title_block);

    let ep_info_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::new().bg(BACKGROUND));

    let ep_info = Paragraph::new(Text::styled(
        "Episode Information",
        Style::default().fg(TEXT_COLOR),
    ))
    .block(ep_info_block);

    let play_status_bar_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::new().bg(BACKGROUND));

    let play_status_bar = Paragraph::new(Text::styled(
        "Play Status Bar",
        Style::default().fg(TEXT_COLOR),
    ))
    .block(play_status_bar_block);

    let right_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(10), Constraint::Percentage(90)])
        .split(outer_layout[2]);

    let search_bar_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::new().bg(BACKGROUND));

    let search_bar = Paragraph::new(Text::styled(
        "Search Bar",
        Style::default().fg(TEXT_COLOR),
    ))
    .block(search_bar_block);

    let ep_list_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::new().bg(BACKGROUND));

    let ep_list = Paragraph::new(Text::styled(
        "Episode List",
        Style::default().fg(TEXT_COLOR),
    ))
    .block(ep_list_block);

    frame.render_widget(menu, left_layout[0]);
    frame.render_widget(open_playlist, left_layout[1]);
    frame.render_widget(save_to_playlist, left_layout[2]);
    frame.render_widget(remove_from_playlist, left_layout[3]);
    frame.render_widget(change_theme, left_layout[4]);
    frame.render_widget(about_mfp, left_layout[5]);
    frame.render_widget(mfp_credits, left_layout[6]);
    frame.render_widget(ep_title, middle_layout[0]);
    frame.render_widget(ep_info, middle_layout[1]);
    frame.render_widget(play_status_bar, middle_layout[2]);
    frame.render_widget(search_bar, right_layout[0]);
    frame.render_widget(ep_list, right_layout[1]);

    // frame.render_widget(composed_commit, inner_right_layout[0]);
    // frame.render_widget(interaction_panel, inner_right_layout[1]);
}
