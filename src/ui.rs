use anyhow::Result;
use crate::audio::AudioPlayer;
use crate::cassette::CassetteWidget;
use crate::library::MusicLibrary;
use ratatui::{
    layout::{
        Alignment, Constraint, Direction, Layout, Rect,
    },
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{
        Block, Borders, List, ListItem, ListState, Paragraph, Wrap,
    },
    Frame,
};
use std::sync::{Arc, Mutex};
use crate::visualizer::Visualizer;

#[derive(Debug, Clone, PartialEq)]
pub enum Theme {
    Dark,
    Light,
    Synthwave,
    Ocean,
    Forest,
    Cyberpunk,
    Neon,
    Retro,
    Sunset,
    Matrix,
    Arctic,
    Fire,
    System,
}

impl Theme {
    pub fn colors(&self) -> ThemeColors {
        match self {
            Theme::Dark => ThemeColors {
                background: Color::Black,
                foreground: Color::White,
                primary: Color::Blue,
                secondary: Color::Cyan,
                accent: Color::Yellow,
                border: Color::Gray,
                text: Color::White,
                highlight: Color::Magenta,
            },
            Theme::Light => ThemeColors {
                background: Color::White,
                foreground: Color::Black,
                primary: Color::Blue,
                secondary: Color::Cyan,
                accent: Color::Yellow,
                border: Color::Gray,
                text: Color::Black,
                highlight: Color::Magenta,
            },
            Theme::Synthwave => ThemeColors {
                background: Color::Rgb(20, 20, 40),
                foreground: Color::Rgb(255, 100, 255),
                primary: Color::Rgb(255, 100, 255),
                secondary: Color::Rgb(100, 255, 255),
                accent: Color::Rgb(255, 255, 100),
                border: Color::Rgb(100, 100, 200),
                text: Color::Rgb(255, 255, 255),
                highlight: Color::Rgb(255, 50, 150),
            },
            Theme::Ocean => ThemeColors {
                background: Color::Rgb(0, 20, 40),
                foreground: Color::Rgb(100, 200, 255),
                primary: Color::Rgb(0, 150, 255),
                secondary: Color::Rgb(100, 255, 255),
                accent: Color::Rgb(255, 255, 100),
                border: Color::Rgb(50, 100, 150),
                text: Color::Rgb(200, 220, 255),
                highlight: Color::Rgb(0, 255, 200),
            },
            Theme::Forest => ThemeColors {
                background: Color::Rgb(20, 40, 20),
                foreground: Color::Rgb(100, 255, 100),
                primary: Color::Rgb(0, 200, 0),
                secondary: Color::Rgb(100, 255, 100),
                accent: Color::Rgb(255, 255, 100),
                border: Color::Rgb(100, 150, 100),
                text: Color::Rgb(200, 255, 200),
                highlight: Color::Rgb(255, 200, 0),
            },
            Theme::Cyberpunk => ThemeColors {
                background: Color::Rgb(10, 5, 20),
                foreground: Color::Rgb(255, 0, 255),
                primary: Color::Rgb(255, 0, 255),
                secondary: Color::Rgb(0, 255, 255),
                accent: Color::Rgb(255, 255, 0),
                border: Color::Rgb(100, 0, 200),
                text: Color::Rgb(255, 200, 255),
                highlight: Color::Rgb(255, 100, 255),
            },
            Theme::Neon => ThemeColors {
                background: Color::Rgb(0, 0, 0),
                foreground: Color::Rgb(0, 255, 255),
                primary: Color::Rgb(0, 255, 255),
                secondary: Color::Rgb(255, 0, 255),
                accent: Color::Rgb(255, 255, 0),
                border: Color::Rgb(50, 50, 50),
                text: Color::Rgb(200, 255, 255),
                highlight: Color::Rgb(0, 255, 200),
            },
            Theme::Retro => ThemeColors {
                background: Color::Rgb(40, 20, 10),
                foreground: Color::Rgb(255, 200, 100),
                primary: Color::Rgb(255, 150, 0),
                secondary: Color::Rgb(255, 200, 100),
                accent: Color::Rgb(255, 100, 0),
                border: Color::Rgb(150, 100, 50),
                text: Color::Rgb(255, 220, 180),
                highlight: Color::Rgb(255, 180, 0),
            },
            Theme::Sunset => ThemeColors {
                background: Color::Rgb(30, 15, 40),
                foreground: Color::Rgb(255, 100, 50),
                primary: Color::Rgb(255, 150, 0),
                secondary: Color::Rgb(255, 100, 150),
                accent: Color::Rgb(255, 200, 0),
                border: Color::Rgb(150, 75, 100),
                text: Color::Rgb(255, 180, 200),
                highlight: Color::Rgb(255, 120, 80),
            },
            Theme::Matrix => ThemeColors {
                background: Color::Rgb(0, 0, 0),
                foreground: Color::Rgb(0, 255, 0),
                primary: Color::Rgb(0, 255, 0),
                secondary: Color::Rgb(0, 200, 0),
                accent: Color::Rgb(0, 255, 100),
                border: Color::Rgb(0, 100, 0),
                text: Color::Rgb(0, 255, 0),
                highlight: Color::Rgb(100, 255, 100),
            },
            Theme::Arctic => ThemeColors {
                background: Color::Rgb(5, 15, 30),
                foreground: Color::Rgb(150, 200, 255),
                primary: Color::Rgb(100, 150, 255),
                secondary: Color::Rgb(150, 200, 255),
                accent: Color::Rgb(200, 220, 255),
                border: Color::Rgb(50, 100, 150),
                text: Color::Rgb(200, 220, 255),
                highlight: Color::Rgb(100, 180, 255),
            },
            Theme::Fire => ThemeColors {
                background: Color::Rgb(20, 5, 0),
                foreground: Color::Rgb(255, 100, 0),
                primary: Color::Rgb(255, 150, 0),
                secondary: Color::Rgb(255, 100, 0),
                accent: Color::Rgb(255, 200, 0),
                border: Color::Rgb(150, 50, 0),
                text: Color::Rgb(255, 180, 150),
                highlight: Color::Rgb(255, 120, 0),
            },
            Theme::System => ThemeColors {
                background: Color::Rgb(0, 15, 20), // Dark teal background
                foreground: Color::Rgb(0, 255, 100), // Bright green
                primary: Color::Rgb(0, 255, 100), // Bright green titles
                secondary: Color::Rgb(255, 100, 255), // Bright pink labels
                accent: Color::Rgb(255, 255, 0), // Bright yellow highlights
                border: Color::Rgb(0, 255, 100), // Bright green borders
                text: Color::Rgb(200, 255, 200), // Light green text
                highlight: Color::Rgb(255, 255, 0), // Bright yellow progress bars
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct ThemeColors {
    pub background: Color,
    pub foreground: Color,
    pub primary: Color,
    pub secondary: Color,
    pub accent: Color,
    pub border: Color,
    pub text: Color,
    pub highlight: Color,
}

#[derive(Debug, Clone)]
pub struct AppState {
    pub current_theme: Theme,
    pub rainbow_mode: bool,
    pub show_albums: bool,
    pub show_tracks: bool,
    pub show_shortcuts: bool,
    pub show_directory_selector: bool,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            current_theme: Theme::System,
            rainbow_mode: false,
            show_albums: true,
            show_tracks: false,
            show_shortcuts: true,
            show_directory_selector: false,
        }
    }

    pub fn cycle_theme(&mut self) {
        self.current_theme = match self.current_theme {
            Theme::Dark => Theme::Light,
            Theme::Light => Theme::Synthwave,
            Theme::Synthwave => Theme::Ocean,
            Theme::Ocean => Theme::Forest,
            Theme::Forest => Theme::Cyberpunk,
            Theme::Cyberpunk => Theme::Neon,
            Theme::Neon => Theme::Retro,
            Theme::Retro => Theme::Sunset,
            Theme::Sunset => Theme::Matrix,
            Theme::Matrix => Theme::Arctic,
            Theme::Arctic => Theme::Fire,
            Theme::Fire => Theme::System,
            Theme::System => Theme::Dark,
        };
    }

    pub fn toggle_rainbow_mode(&mut self) {
        self.rainbow_mode = !self.rainbow_mode;
    }

    pub fn toggle_shortcuts(&mut self) {
        self.show_shortcuts = !self.show_shortcuts;
    }

    pub fn toggle_directory_selector(&mut self) {
        self.show_directory_selector = !self.show_directory_selector;
    }
}

pub struct App {
    audio_player: Arc<Mutex<AudioPlayer>>,
    music_library: Arc<Mutex<MusicLibrary>>,
    app_state: Arc<Mutex<AppState>>,
    visualizer: Visualizer,
    cassette: CassetteWidget,
    album_list_state: ListState,
    track_list_state: ListState,
    music_directory: Option<String>,
}

impl App {
    pub fn new(
        audio_player: Arc<Mutex<AudioPlayer>>,
        music_library: Arc<Mutex<MusicLibrary>>,
        app_state: Arc<Mutex<AppState>>,
    ) -> Self {
        Self {
            audio_player,
            music_library,
            app_state,
            visualizer: Visualizer::new(),
            cassette: CassetteWidget::new(),
            album_list_state: ListState::default(),
            track_list_state: ListState::default(),
            music_directory: None,
        }
    }

    pub async fn update(&mut self) -> Result<()> {
        // Update visualizer with new samples
        let samples = {
            let player = self.audio_player.lock().unwrap();
            player.get_samples()
        };
        self.visualizer.add_samples(&samples);
        self.visualizer.update_spectrum();

        // Update cassette animation
        let is_playing = {
            let player = self.audio_player.lock().unwrap();
            player.is_playing()
        };
        self.cassette.set_playing(is_playing);
        self.cassette.update();

        Ok(())
    }

    pub fn render(&mut self, f: &mut Frame) {
        let app_state = self.app_state.lock().unwrap();
        let colors = app_state.current_theme.colors();
        let rainbow_mode = app_state.rainbow_mode;
        drop(app_state);

        // Aplicar cor de fundo do tema
        f.render_widget(
            Block::default().style(Style::default().bg(colors.background)),
            f.size()
        );

        // Layout do player de música com estética de sistema de monitoramento
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(15), // Top section with cassette and track info
                Constraint::Length(8),  // Visualizer
                Constraint::Min(5),     // Albums and tracks
                Constraint::Length(3),  // Shortcuts bar
            ])
            .split(f.size());

        self.render_top_section(f, chunks[0], &colors, rainbow_mode);
        self.render_visualizer(f, chunks[1], &colors, rainbow_mode);
        self.render_lists(f, chunks[2], &colors, rainbow_mode);
        self.render_shortcuts_bar(f, chunks[3], &colors);
    }

    fn render_top_section(
        &mut self,
        f: &mut Frame,
        area: Rect,
        colors: &ThemeColors,
        rainbow_mode: bool,
    ) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Length(20), Constraint::Min(0)])
            .split(area);

        // Cassette art
        let cassette_area = Rect::new(
            chunks[0].x + 1,
            chunks[0].y + 1,
            chunks[0].width - 2,
            chunks[0].height - 2,
        );
        f.render_widget(self.cassette.clone(), cassette_area);

        // Track info
        self.render_track_info(f, chunks[1], colors, rainbow_mode);
    }

    fn render_track_info(
        &mut self,
        f: &mut Frame,
        area: Rect,
        colors: &ThemeColors,
        _rainbow_mode: bool,
    ) {
        let library = self.music_library.lock().unwrap();
        let current_track = library.get_current_track();
        let audio_player = self.audio_player.lock().unwrap();
        let position = audio_player.get_position();
        let duration = audio_player.get_duration();
        drop(audio_player);
        
        let track_info = current_track.cloned();
        drop(library);

        let mut lines = Vec::new();

        if let Some(track) = track_info {
            let title = track.title.clone();
            let artist = track.artist.clone();
            let album = track.album.clone();
            
            lines.push(Line::from(vec![
                Span::styled("Track: ", Style::default().fg(colors.primary)),
                Span::styled(title, Style::default().fg(colors.text)),
            ]));
            lines.push(Line::from(vec![
                Span::styled("Artist: ", Style::default().fg(colors.primary)),
                Span::styled(artist, Style::default().fg(colors.text)),
            ]));
            lines.push(Line::from(vec![
                Span::styled("Album: ", Style::default().fg(colors.primary)),
                Span::styled(album, Style::default().fg(colors.text)),
            ]));
        } else {
            lines.push(Line::from(Span::styled("No track selected", Style::default().fg(colors.text))));
        }

        // Duration info
        let position_str = format!("{:02}:{:02}", position.as_secs() / 60, position.as_secs() % 60);
        let duration_str = format!("{:02}:{:02}", duration.as_secs() / 60, duration.as_secs() % 60);
        lines.push(Line::from(vec![
            Span::styled("Duration: ", Style::default().fg(colors.primary)),
            Span::styled(&position_str, Style::default().fg(colors.text)),
            Span::styled(" / ", Style::default().fg(colors.border)),
            Span::styled(&duration_str, Style::default().fg(colors.text)),
        ]));

        let paragraph = Paragraph::new(lines)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("NOW PLAYING")
                    .title_style(Style::default().fg(colors.accent).add_modifier(Modifier::BOLD))
                    .border_style(Style::default().fg(colors.border)),
            )
            .wrap(Wrap { trim: true });

        f.render_widget(paragraph, area);
    }

    fn render_visualizer(
        &mut self,
        f: &mut Frame,
        area: Rect,
        colors: &ThemeColors,
        _rainbow_mode: bool,
    ) {
        let spectrum_bars = self.visualizer.get_spectrum_bars();
        let beat_intensity = self.visualizer.get_beat_intensity();
        let rainbow_hue = self.visualizer.get_rainbow_hue();

        let mut lines = Vec::new();
        let bar_height = area.height.saturating_sub(2).max(1) as usize;

        for row in (0..bar_height).rev() {
            let mut spans = Vec::new();
            spans.push(Span::raw("│"));

            for (i, &bar_value) in spectrum_bars.iter().enumerate() {
                let bar_height_f = bar_value * (bar_height - 1) as f32;
                let is_active = row as f32 <= bar_height_f;

                let color = if _rainbow_mode {
                    let hue = (rainbow_hue + (i as f32 * 10.0)) % 360.0;
                    let saturation = 0.8 + beat_intensity * 0.2;
                    let value = if is_active { 0.9 + beat_intensity * 0.1 } else { 0.3 };
                    let (r, g, b) = Visualizer::hsv_to_rgb(hue, saturation, value);
                    Color::Rgb(r, g, b)
                } else if is_active {
                    colors.highlight
                } else {
                    colors.border
                };

                let char = if is_active { "█" } else { " " };
                spans.push(Span::styled(char, Style::default().fg(color)));
            }

            spans.push(Span::raw("│"));
            lines.push(Line::from(spans));
        }

        // Add animated title
        let title = if _rainbow_mode {
            "SPECTRUM ANALYZER [RAINBOW]"
        } else {
            "SPECTRUM ANALYZER"
        };

        let paragraph = Paragraph::new(lines)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(title)
                    .title_style(Style::default().fg(colors.accent).add_modifier(Modifier::BOLD))
                    .border_style(Style::default().fg(colors.border)),
            )
            .alignment(Alignment::Center);

        f.render_widget(paragraph, area);
    }

    fn render_lists(
        &mut self,
        f: &mut Frame,
        area: Rect,
        colors: &ThemeColors,
        rainbow_mode: bool,
    ) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);

        self.render_album_list(f, chunks[0], colors, rainbow_mode);
        self.render_track_list(f, chunks[1], colors, rainbow_mode);
    }

    fn render_album_list(
        &mut self,
        f: &mut Frame,
        area: Rect,
        colors: &ThemeColors,
        _rainbow_mode: bool,
    ) {
        let library = self.music_library.lock().unwrap();
        let albums: Vec<ListItem> = library
            .albums
            .iter()
            .enumerate()
            .map(|(i, album)| {
                let style = if i == library.current_album_index {
                    Style::default().fg(colors.highlight).add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(colors.text)
                };

                let display_name = album.display_name();
                ListItem::new(Line::from(vec![
                    Span::styled("> ", style),
                    Span::styled(display_name, style),
                ]))
            })
            .collect();

        let list = List::new(albums)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("ALBUMS")
                    .title_style(Style::default().fg(colors.accent).add_modifier(Modifier::BOLD))
                    .border_style(Style::default().fg(colors.border)),
            );

        self.album_list_state.select(Some(library.current_album_index));
        f.render_stateful_widget(list, area, &mut self.album_list_state);
        drop(library);
    }

    fn render_track_list(
        &mut self,
        f: &mut Frame,
        area: Rect,
        colors: &ThemeColors,
        _rainbow_mode: bool,
    ) {
        let library = self.music_library.lock().unwrap();
        let tracks: Vec<ListItem> = library
            .get_current_album()
            .map(|album| {
                album
                    .tracks
                    .iter()
                    .enumerate()
                    .map(|(i, track)| {
                        let style = if i == library.current_track_index {
                            Style::default().fg(colors.highlight).add_modifier(Modifier::BOLD)
                        } else {
                            Style::default().fg(colors.text)
                        };

                        let display_title = track.display_title();
                        ListItem::new(Line::from(vec![
                            Span::styled("> ", style),
                            Span::styled(display_title, style),
                        ]))
                    })
                    .collect()
            })
            .unwrap_or_default();

        let list = List::new(tracks)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("TRACKS")
                    .title_style(Style::default().fg(colors.accent).add_modifier(Modifier::BOLD))
                    .border_style(Style::default().fg(colors.border)),
            );

        if let Some(_album) = library.get_current_album() {
            self.track_list_state.select(Some(library.current_track_index));
        }
        f.render_stateful_widget(list, area, &mut self.track_list_state);
        drop(library);
    }

    fn render_shortcuts(&mut self, f: &mut Frame, area: Rect, colors: &ThemeColors) {
        // Barra de atalhos no estilo do sistema de monitoramento
        let shortcuts = vec![
            ("SPACE", "Play/Pause"),
            ("↑↓", "Albums"),
            ("←→", "Tracks"),
            ("ENTER", "Select"),
            ("T", "Theme"),
            ("R", "Rainbow"),
            ("S", "Shortcuts"),
            ("D", "Directory"),
            ("CTRL+Q", "Quit"),
        ];

        let mut spans = Vec::new();
        for (i, (key, action)) in shortcuts.iter().enumerate() {
            if i > 0 {
                spans.push(Span::styled(" | ", Style::default().fg(colors.border)));
            }
            // Chave em destaque (como [Q] no sistema)
            spans.push(Span::styled(
                format!("[{}]", key),
                Style::default().fg(colors.highlight).add_modifier(Modifier::BOLD),
            ));
            spans.push(Span::styled(
                format!(" {}", action),
                Style::default().fg(colors.text),
            ));
        }

        let shortcuts_text = Line::from(spans);
        
        let shortcuts_paragraph = Paragraph::new(shortcuts_text)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("KEYBOARD SHORTCUTS")
                    .title_style(Style::default().fg(colors.accent).add_modifier(Modifier::BOLD))
                    .border_style(Style::default().fg(colors.border)),
            )
            .alignment(Alignment::Center);

        f.render_widget(shortcuts_paragraph, area);
    }

    fn render_system_info(&mut self, f: &mut Frame, area: Rect, colors: &ThemeColors) {
        let system_info = vec![
            "User: eugui@PC-AIIALABS",
            "OS: Windows",
            "Distro: Windows 11 Home",
            "Kernel: 26100",
            "Arch: x86_64",
            "Shell: PowerShell",
            "Processes: 299",
        ];

        let items: Vec<ListItem> = system_info
            .iter()
            .map(|info| {
                let spans = vec![
                    Span::styled("User: ", Style::default().fg(colors.secondary)),
                    Span::styled("eugui@PC-AIIALABS", Style::default().fg(colors.text)),
                ];
                ListItem::new(Line::from(spans))
            })
            .collect();

        let list = List::new(items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("System Information")
                    .title_style(Style::default().fg(colors.primary).add_modifier(Modifier::BOLD))
                    .border_style(Style::default().fg(colors.border)),
            );

        f.render_widget(list, area);
    }

    fn render_hardware_info(&mut self, f: &mut Frame, area: Rect, colors: &ThemeColors) {
        let hardware_info = vec![
            "CPU: 12th Gen Intel(R) Core(TM) i5-1235",
            "Cores: 12 (10 physical)",
            "Frequency: 1.30 GHz",
            "Memory: 15.7 GB",
            "Uptime: 22h 6m",
            "Display: 1920x1080 @ 60Hz",
            "Audio: Intel Smart Sound",
        ];

        let items: Vec<ListItem> = hardware_info
            .iter()
            .map(|info| {
                let parts: Vec<&str> = info.split(": ").collect();
                if parts.len() == 2 {
                    let spans = vec![
                        Span::styled(format!("{}: ", parts[0]), Style::default().fg(colors.secondary)),
                        Span::styled(parts[1], Style::default().fg(colors.text)),
                    ];
                    ListItem::new(Line::from(spans))
                } else {
                    ListItem::new(Line::from(Span::styled(*info, Style::default().fg(colors.text))))
                }
            })
            .collect();

        let list = List::new(items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Hardware Information")
                    .title_style(Style::default().fg(colors.primary).add_modifier(Modifier::BOLD))
                    .border_style(Style::default().fg(colors.border)),
            );

        f.render_widget(list, area);
    }

    fn render_cpu_usage(&mut self, f: &mut Frame, area: Rect, colors: &ThemeColors) {
        let cpu_usage = 21.2;
        let usage_text = format!("CPU Usage: {:.1}%", cpu_usage);
        
        // Barra de progresso
        let bar_width = (area.width.saturating_sub(4)) as usize;
        let filled_width = ((cpu_usage / 100.0) * bar_width as f64) as usize;
        
        let mut bar_chars = vec!['█'; filled_width];
        bar_chars.resize(bar_width, '░');
        let bar_text = bar_chars.iter().collect::<String>();

        let content = vec![
            Line::from(vec![
                Span::styled("CPU Usage: ", Style::default().fg(colors.primary).add_modifier(Modifier::BOLD)),
                Span::styled(format!("{:.1}%", cpu_usage), Style::default().fg(colors.text)),
            ]),
            Line::from(Span::styled(bar_text, Style::default().fg(colors.highlight))),
        ];

        let paragraph = Paragraph::new(content)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("CPU Usage")
                    .title_style(Style::default().fg(colors.primary).add_modifier(Modifier::BOLD))
                    .border_style(Style::default().fg(colors.border)),
            );

        f.render_widget(paragraph, area);
    }

    fn render_disk_usage(&mut self, f: &mut Frame, area: Rect, colors: &ThemeColors) {
        let disk_usage = 20.1; // 188/934GB
        let usage_text = format!("Disk Usage: {:.1}%", disk_usage);
        
        // Barra de progresso
        let bar_width = (area.width.saturating_sub(4)) as usize;
        let filled_width = ((disk_usage / 100.0) * bar_width as f64) as usize;
        
        let mut bar_chars = vec!['█'; filled_width];
        bar_chars.resize(bar_width, '░');
        let bar_text = bar_chars.iter().collect::<String>();

        let content = vec![
            Line::from(vec![
                Span::styled("OS [SSD] ", Style::default().fg(colors.text)),
            ]),
            Line::from(Span::styled(bar_text, Style::default().fg(colors.highlight))),
            Line::from(vec![
                Span::styled("188/934GB", Style::default().fg(colors.text)),
            ]),
        ];

        let paragraph = Paragraph::new(content)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Disk Usage")
                    .title_style(Style::default().fg(colors.primary).add_modifier(Modifier::BOLD))
                    .border_style(Style::default().fg(colors.border)),
            );

        f.render_widget(paragraph, area);
    }

    fn render_memory_info(&mut self, f: &mut Frame, area: Rect, colors: &ThemeColors) {
        let ram_usage = 74.8;
        let swap_usage = 70.1;
        
        // Barras de progresso
        let bar_width = (area.width.saturating_sub(4)) as usize;
        
        let ram_filled = ((ram_usage / 100.0) * bar_width as f64) as usize;
        let swap_filled = ((swap_usage / 100.0) * bar_width as f64) as usize;
        
        let mut ram_bar = vec!['█'; ram_filled];
        ram_bar.resize(bar_width, '░');
        let ram_bar_text = ram_bar.iter().collect::<String>();
        
        let mut swap_bar = vec!['█'; swap_filled];
        swap_bar.resize(bar_width, '░');
        let swap_bar_text = swap_bar.iter().collect::<String>();

        let content = vec![
            Line::from(vec![
                Span::styled("Memory: ", Style::default().fg(colors.primary).add_modifier(Modifier::BOLD)),
                Span::styled(format!("{:.1}%", ram_usage), Style::default().fg(colors.text)),
            ]),
            Line::from(vec![
                Span::styled("RAM: ", Style::default().fg(colors.secondary)),
                Span::styled("11.7 GB used • 4.0 GB free • 74.8%", Style::default().fg(colors.text)),
            ]),
            Line::from(Span::styled(ram_bar_text, Style::default().fg(colors.highlight))),
            Line::from(vec![
                Span::styled("Swap: ", Style::default().fg(colors.secondary)),
                Span::styled("11.6 GB used • 5.0 GB free • 70.1%", Style::default().fg(colors.text)),
            ]),
            Line::from(Span::styled(swap_bar_text, Style::default().fg(colors.highlight))),
        ];

        let paragraph = Paragraph::new(content)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Memory")
                    .title_style(Style::default().fg(colors.primary).add_modifier(Modifier::BOLD))
                    .border_style(Style::default().fg(colors.border)),
            );

        f.render_widget(paragraph, area);
    }

    fn render_network_info(&mut self, f: &mut Frame, area: Rect, colors: &ThemeColors) {
        let content = vec![
            Line::from(vec![
                Span::styled("↓ Download: ", Style::default().fg(colors.highlight)),
                Span::styled("15.0 KB/s", Style::default().fg(colors.text)),
            ]),
            Line::from(vec![
                Span::styled("↑ Upload: ", Style::default().fg(Color::Red)),
                Span::styled("657.2 B/s", Style::default().fg(colors.text)),
            ]),
            Line::from(vec![
                Span::styled("Total RX: ", Style::default().fg(colors.text)),
                Span::styled("0.00 GB", Style::default().fg(colors.text)),
            ]),
            Line::from(vec![
                Span::styled("Total TX: ", Style::default().fg(colors.text)),
                Span::styled("0.00 GB", Style::default().fg(colors.text)),
            ]),
        ];

        let paragraph = Paragraph::new(content)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Network")
                    .title_style(Style::default().fg(colors.primary).add_modifier(Modifier::BOLD))
                    .border_style(Style::default().fg(colors.border)),
            );

        f.render_widget(paragraph, area);
    }

    fn render_shortcuts_bar(&mut self, f: &mut Frame, area: Rect, colors: &ThemeColors) {
        // Garantir que a área seja válida
        if area.height < 1 {
            return;
        }

        let shortcuts = vec![
            ("SPACE", "Play/Pause"),
            ("↑↓", "Albums"),
            ("←→", "Tracks"),
            ("ENTER", "Select"),
            ("T", "Theme"),
            ("R", "Rainbow"),
            ("S", "Shortcuts"),
            ("D", "Directory"),
            ("CTRL+Q", "Quit"),
        ];

        let mut spans = Vec::new();
        for (i, (key, action)) in shortcuts.iter().enumerate() {
            if i > 0 {
                spans.push(Span::styled(" | ", Style::default().fg(colors.border)));
            }
            spans.push(Span::styled(
                format!("[{}]", key),
                Style::default().fg(colors.highlight).add_modifier(Modifier::BOLD),
            ));
            spans.push(Span::styled(
                format!(" {}", action),
                Style::default().fg(colors.text),
            ));
        }

        let shortcuts_text = Line::from(spans);
        let shortcuts_paragraph = Paragraph::new(shortcuts_text)
            .style(Style::default().fg(colors.text))
            .alignment(Alignment::Center);

        // Footer no canto direito
        let footer_text = "made by gui기กีギ";
        let footer = Paragraph::new(footer_text)
            .style(Style::default().fg(colors.border))
            .alignment(Alignment::Right);

        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Min(0), Constraint::Length(footer_text.len() as u16 + 2)])
            .split(area);

        f.render_widget(shortcuts_paragraph, layout[0]);
        f.render_widget(footer, layout[1]);
    }

    pub async fn toggle_playback(&mut self) -> Result<()> {
        let player = self.audio_player.lock().unwrap();
        if player.is_playing() {
            player.pause();
        } else {
            player.play();
        }
        Ok(())
    }

    pub fn navigate_up(&mut self) {
        let mut library = self.music_library.lock().unwrap();
        library.prev_album();
    }

    pub fn navigate_down(&mut self) {
        let mut library = self.music_library.lock().unwrap();
        library.next_album();
    }

    pub fn navigate_left(&mut self) {
        let mut library = self.music_library.lock().unwrap();
        library.prev_track();
    }

    pub fn navigate_right(&mut self) {
        let mut library = self.music_library.lock().unwrap();
        library.next_track();
    }

    pub async fn select_item(&mut self) -> Result<()> {
        let track_path = {
            let library = self.music_library.lock().unwrap();
            library.get_current_track_path()
        };

        if let Some(path) = track_path {
            let mut player = self.audio_player.lock().unwrap();
            player.load_file(&path)?;
            player.play();
        }

        Ok(())
    }

    pub fn cycle_theme(&mut self) {
        let mut app_state = self.app_state.lock().unwrap();
        app_state.cycle_theme();
    }

    pub fn toggle_rainbow_mode(&mut self) {
        let mut app_state = self.app_state.lock().unwrap();
        app_state.toggle_rainbow_mode();
    }

    pub fn toggle_shortcuts(&mut self) {
        let mut app_state = self.app_state.lock().unwrap();
        app_state.toggle_shortcuts();
    }

    pub fn toggle_directory_selector(&mut self) {
        let mut app_state = self.app_state.lock().unwrap();
        app_state.toggle_directory_selector();
    }
}