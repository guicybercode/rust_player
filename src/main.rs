use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};
use std::{
    io,
    sync::{Arc, Mutex},
    time::Duration,
};

mod audio;
mod cassette;
mod library;
mod metadata;
mod ui;
mod visualizer;

use audio::AudioPlayer;
use library::MusicLibrary;
use ui::{App, AppState};

#[tokio::main]
async fn main() -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Initialize components
    let audio_player = Arc::new(Mutex::new(AudioPlayer::new()?));
    let music_library = Arc::new(Mutex::new(MusicLibrary::new()));
    let app_state = Arc::new(Mutex::new(AppState::new()));

    // Create app
    let mut app = App::new(audio_player, music_library, app_state);

    // Run app
    let res = run_app(&mut terminal, &mut app).await;

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        eprintln!("{err:?}");
    }

    Ok(())
}

async fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> Result<()> {
    loop {
        terminal.draw(|f| app.render(f))?;

        if crossterm::event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => {
                        if key.modifiers.contains(KeyModifiers::CONTROL) {
                            return Ok(());
                        }
                    }
                    KeyCode::Char(' ') => {
                        app.toggle_playback().await?;
                    }
                    KeyCode::Up => {
                        app.navigate_up();
                    }
                    KeyCode::Down => {
                        app.navigate_down();
                    }
                    KeyCode::Left => {
                        app.navigate_left();
                    }
                    KeyCode::Right => {
                        app.navigate_right();
                    }
                    KeyCode::Enter => {
                        app.select_item().await?;
                    }
                    KeyCode::Char('t') => {
                        app.cycle_theme();
                    }
                    KeyCode::Char('r') => {
                        app.toggle_rainbow_mode();
                    }
                    KeyCode::Char('s') => {
                        app.toggle_shortcuts();
                    }
                    KeyCode::Char('d') => {
                        app.toggle_directory_selector();
                    }
                    _ => {}
                }
            }
        }

        app.update().await?;
    }
}