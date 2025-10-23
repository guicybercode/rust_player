use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::Widget,
};

#[derive(Clone)]
pub struct CassetteWidget {
    frame: u8,
    is_playing: bool,
}

impl CassetteWidget {
    pub fn new() -> Self {
        Self {
            frame: 0,
            is_playing: false,
        }
    }

    pub fn set_playing(&mut self, playing: bool) {
        self.is_playing = playing;
    }

    pub fn update(&mut self) {
        if self.is_playing {
            self.frame = (self.frame + 1) % 4;
        }
    }

    fn get_wheel_char(&self) -> char {
        match self.frame {
            0 => '|',
            1 => '/',
            2 => '-',
            3 => '\\',
            _ => '|',
        }
    }

    fn get_cassette_art(&self) -> Vec<&'static str> {
        vec![
            "┌─────────────────┐",
            "│  ████████████   │",
            "│ ██            ██│",
            "│██              ██│",
            "│██    ┌────┐    ██│",
            "│██    │    │    ██│",
            "│██    │    │    ██│",
            "│██    └────┘    ██│",
            "│██              ██│",
            "│ ██            ██│",
            "│  ████████████   │",
            "└─────────────────┘",
        ]
    }

    fn get_wheel_positions(&self) -> Vec<(usize, usize)> {
        vec![(4, 6), (4, 11), (7, 6), (7, 11)]
    }
}

impl Widget for CassetteWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let art = self.get_cassette_art();
        let wheel_positions = self.get_wheel_positions();
        let wheel_char = self.get_wheel_char();

        for (i, line) in art.iter().enumerate() {
            if area.y + (i as u16) < area.height {
                let mut spans = Vec::new();

                for (j, ch) in line.chars().enumerate() {
                    let is_wheel = wheel_positions.contains(&(i, j));
                    let display_char = if is_wheel && self.is_playing {
                        wheel_char
                    } else {
                        ch
                    };

                    let color = if is_wheel && self.is_playing {
                        Color::Yellow
                    } else {
                        Color::White
                    };

                    spans.push(Span::styled(display_char.to_string(), Style::default().fg(color)));
                }

                let line = Line::from(spans);
                buf.set_line(area.x, area.y + (i as u16), &line, area.width);
            }
        }
    }
}