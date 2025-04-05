use std::{
    collections::HashMap,
    io::{Write, stdout},
    thread,
    time::{Duration, Instant},
};

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{self, Event, KeyCode},
    execute,
    queue,
    style::{Color, Print}, // Removed unused imports
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};

// =====================
// Color System
// =====================
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum PaletteColor {
    Black,       // 0x000000
    DarkBlue,    // 0x1D2B53
    DarkPurple,  // 0x7E2553
    DarkGreen,   // 0x008751
    Brown,       // 0xAB5236
    DarkGray,    // 0x5F574F
    LightGray,   // 0xC2C3C7
    White,       // 0xFFF1E8
    Red,         // 0xFF004D
    Orange,      // 0xFFA300
    Yellow,      // 0xFFEC27
    Green,       // 0x00E436
    Blue,        // 0x29ADFF
    Indigo,      // 0x83769C
    Pink,        // 0xFF77A8
    Peach,       // 0xFFCCAA
    Transparent, // Transparent Color
}

impl PaletteColor {
    fn to_rgb(self) -> Color {
        match self {
            Self::Black => Color::Rgb {
                r: 0x00,
                g: 0x00,
                b: 0x00,
            },
            Self::DarkBlue => Color::Rgb {
                r: 0x1D,
                g: 0x2B,
                b: 0x53,
            },
            Self::DarkPurple => Color::Rgb {
                r: 0x7E,
                g: 0x25,
                b: 0x53,
            },
            Self::DarkGreen => Color::Rgb {
                r: 0x00,
                g: 0x87,
                b: 0x51,
            },
            Self::Brown => Color::Rgb {
                r: 0xAB,
                g: 0x52,
                b: 0x36,
            },
            Self::DarkGray => Color::Rgb {
                r: 0x5F,
                g: 0x57,
                b: 0x4F,
            },
            Self::LightGray => Color::Rgb {
                r: 0xC2,
                g: 0xC3,
                b: 0xC7,
            },
            Self::White => Color::Rgb {
                r: 0xFF,
                g: 0xF1,
                b: 0xE8,
            },
            Self::Red => Color::Rgb {
                r: 0xFF,
                g: 0x00,
                b: 0x4D,
            },
            Self::Orange => Color::Rgb {
                r: 0xFF,
                g: 0xA3,
                b: 0x00,
            },
            Self::Yellow => Color::Rgb {
                r: 0xFF,
                g: 0xEC,
                b: 0x27,
            },
            Self::Green => Color::Rgb {
                r: 0x00,
                g: 0xE4,
                b: 0x36,
            },
            Self::Blue => Color::Rgb {
                r: 0x29,
                g: 0xAD,
                b: 0xFF,
            },
            Self::Indigo => Color::Rgb {
                r: 0x83,
                g: 0x76,
                b: 0x9C,
            },
            Self::Pink => Color::Rgb {
                r: 0xFF,
                g: 0x77,
                b: 0xA8,
            },
            Self::Peach => Color::Rgb {
                r: 0xFF,
                g: 0xCC,
                b: 0xAA,
            },
            Self::Transparent => Color::Rgb {
                r: 0x00,
                g: 0x00,
                b: 0x00,
            },
        }
    }
}

// =====================
// Sprite System
// =====================
#[derive(Clone)]
struct Sprite {
    width: usize,
    height: usize,
    pixels: Vec<Vec<Option<PaletteColor>>>,
}

impl Sprite {
    fn new(data: &[&str], palette_map: &HashMap<char, PaletteColor>) -> Self {
        let height = data.len();
        let width = data[0].chars().count();
        let mut pixels = vec![vec![None; width]; height];

        for (y, row) in data.iter().enumerate() {
            for (x, c) in row.chars().enumerate() {
                pixels[y][x] = palette_map.get(&c).copied();
            }
        }

        Sprite {
            width,
            height,
            pixels,
        }
    }
}

// =====================
// Animation System
// =====================
struct Animation {
    frames: Vec<Sprite>,
    current_frame: usize,
    frame_duration: Duration,
    accumulated_time: Duration,
}

impl Animation {
    fn new(frames: Vec<Sprite>, fps: u32) -> Self {
        Animation {
            frames,
            current_frame: 0,
            frame_duration: Duration::from_secs_f32(1.0 / fps as f32),
            accumulated_time: Duration::ZERO,
        }
    }

    fn update(&mut self, delta_time: Duration) {
        self.accumulated_time += delta_time;
        while self.accumulated_time >= self.frame_duration {
            self.accumulated_time -= self.frame_duration;
            self.current_frame = (self.current_frame + 1) % self.frames.len();
        }
    }

    fn current_sprite(&self) -> &Sprite {
        &self.frames[self.current_frame]
    }
}

// =====================
// Game State
// =====================
struct Game {
    width: usize,
    height: usize,
    pixels: Vec<Vec<PaletteColor>>,
    animations: HashMap<String, Animation>,
    player_x: usize,
    player_y: usize,
    last_frame_time: Instant,
    is_running: bool,
}

impl Game {
    fn new(width: usize, height: usize) -> Self {
        assert!(height % 2 == 0, "Height must be even for square pixels");
        let pixels = vec![vec![PaletteColor::Black; width]; height];
        Game {
            width,
            height,
            pixels,
            animations: HashMap::new(),
            player_x: width / 2,
            player_y: height / 2,
            last_frame_time: Instant::now(),
            is_running: true,
        }
    }

    fn clear(&mut self) {
        for row in &mut self.pixels {
            for pixel in row {
                *pixel = PaletteColor::Black;
            }
        }
    }

    fn draw_sprite(&mut self, x: usize, y: usize, sprite: &Sprite) {
        for (dy, row) in sprite.pixels.iter().enumerate() {
            for (dx, color) in row.iter().enumerate() {
                let px = x + dx;
                let py = y + dy;

                if px < self.width && py < self.height {
                    if let Some(color) = color {
                        self.pixels[py][px] = *color;
                    }
                }
            }
        }
    }

    fn render(&self) -> impl Iterator<Item = String> + '_ {
        (0..self.height / 2).map(move |y_term| {
            let top_row = &self.pixels[y_term * 2];
            let bottom_row = &self.pixels[y_term * 2 + 1];

            top_row
                .iter()
                .zip(bottom_row)
                .map(|(top, bottom)| {
                    let top_rgb = top.to_rgb();
                    let bottom_rgb = bottom.to_rgb();

                    // Match the RGB values from our PaletteColor
                    let (top_r, top_g, top_b) = match top_rgb {
                        Color::Rgb { r, g, b } => (r, g, b),
                        _ => (0, 0, 0), // Fallback to black
                    };

                    let (bottom_r, bottom_g, bottom_b) = match bottom_rgb {
                        Color::Rgb { r, g, b } => (r, g, b),
                        _ => (0, 0, 0), // Fallback to black
                    };

                    format!(
                        "\x1b[38;2;{};{};{}m\x1b[48;2;{};{};{}m▀\x1b[0m",
                        top_r, top_g, top_b, bottom_r, bottom_g, bottom_b
                    )
                })
                .collect::<String>()
        })
    }
}

// =====================
// Main Game Loop
// =====================
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Terminal setup
    let mut stdout = stdout();
    enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen, Hide)?;

    // Create color mapping
    let mut palette = HashMap::new();
    palette.insert('0', PaletteColor::Black);
    palette.insert('1', PaletteColor::White);
    palette.insert('2', PaletteColor::Red);
    palette.insert('3', PaletteColor::Green);
    palette.insert('4', PaletteColor::Blue);
    palette.insert('t', PaletteColor::Transparent); // Transparent

    // Create game state
    let mut game = Game::new(40, 40);

    // Player animation frames
    let player_idle = Sprite::new(&[" t2t ", "22222", "2t2t2", " t2t "], &palette);

    let player_run1 = Sprite::new(&[" t2t ", "22222", "2ttt2", " t2t "], &palette);

    let player_run2 = Sprite::new(&[" t2t ", "22222", "2t2t2", " t2t "], &palette);

    game.animations.insert(
        "player".to_string(),
        Animation::new(vec![player_idle, player_run1, player_run2], 5),
    );

    // Game loop timing
    let target_fps = 60;
    let frame_duration = Duration::from_secs_f32(1.0 / target_fps as f32);
    let mut last_update = Instant::now();
    let mut accumulator = Duration::ZERO;

    // =====================
    // Main Game Loop - Fixed Section
    // =====================
    'gameloop: loop {
        // Handle input
        while event::poll(Duration::from_secs(0))? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Char('q') => break 'gameloop,
                    KeyCode::Char('a') => game.player_x = game.player_x.saturating_sub(1),
                    KeyCode::Char('d') => game.player_x = (game.player_x + 1).min(game.width - 5),
                    KeyCode::Char('w') => game.player_y = game.player_y.saturating_sub(1),
                    KeyCode::Char('s') => game.player_y = (game.player_y + 1).min(game.height - 4),
                    _ => {}
                }
            }
        }

        // Update timing
        let delta_time = last_update.elapsed();
        last_update = Instant::now();
        accumulator += delta_time;

        // Fixed timestep updates
        while accumulator >= frame_duration {
            // Update animations
            for animation in game.animations.values_mut() {
                animation.update(frame_duration);
            }
            accumulator -= frame_duration;
        }

        // Extract values
        let player_x = game.player_x;
        let player_y = game.player_y;

        // Extract sprite before any mutable borrow
        let sprite_opt = {
            let anim = game.animations.get("player");
            anim.map(|a| a.current_sprite().clone()) // Clone to break borrow
        };

        // MUTABLE operations first
        game.clear();

        // Draw environment
        for x in 0..game.width {
            game.pixels[game.height - 1][x] = PaletteColor::DarkGreen;
        }

        // Draw player sprite
        if let Some(sprite) = sprite_opt {
            game.draw_sprite(player_x, player_y, &sprite); // Note the reference
        }

        // Render frame
        queue!(stdout, MoveTo(0, 0))?;
        for line in game.render() {
            queue!(stdout, Print(line), Print("\r\n"))?;
        }
        stdout.flush()?;

        // Frame rate maintenance
        thread::sleep(frame_duration.saturating_sub(last_update.elapsed()));
    }

    // Cleanup
    execute!(stdout, LeaveAlternateScreen, Show)?;
    disable_raw_mode()?;
    Ok(())
}
