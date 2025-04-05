use std::{
    io::{Write, stdout},
    thread,
    time::Duration,
};

use crossterm::{
    QueueableCommand,
    cursor::{Hide, MoveTo, Show},
    event::{self, Event, KeyCode},
    execute, queue,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};

struct Game {
    width: usize,
    height: usize, // Pixel height (must be even)
    pixels: Vec<Vec<Color>>,
    player_x: usize,
    player_y: usize,
}

impl Game {
    fn new(width: usize, height: usize) -> Self {
        assert!(height % 2 == 0, "Height must be even for square pixels");
        let pixels = vec![vec![Color::Black; width]; height];
        Game {
            width,
            height,
            pixels,
            player_x: width / 2,
            player_y: height / 2,
        }
    }

    fn clear(&mut self) {
        for row in &mut self.pixels {
            for pixel in row {
                *pixel = Color::Black;
            }
        }
    }

    fn draw_player(&mut self) {
        // Draw player as red pixel
        self.pixels[self.player_y][self.player_x] = Color::Rgb { r: 255, g: 0, b: 0 };
    }

    fn render(&self) -> impl Iterator<Item = String> + '_ {
        (0..self.height / 2).map(move |y_term| {
            let top_row = &self.pixels[y_term * 2];
            let bottom_row = &self.pixels[y_term * 2 + 1];

            top_row
                .iter()
                .zip(bottom_row)
                .map(|(top, bottom)| {
                    let (r1, g1, b1) = match top {
                        Color::Rgb { r, g, b } => (*r, *g, *b),
                        _ => (0, 0, 0),
                    };
                    let (r2, g2, b2) = match bottom {
                        Color::Rgb { r, g, b } => (*r, *g, *b),
                        _ => (0, 0, 0),
                    };

                    format!(
                        "\x1b[38;2;{};{};{}m\x1b[48;2;{};{};{}m▀\x1b[0m",
                        r1, g1, b1, r2, g2, b2
                    )
                })
                .collect::<String>()
        })
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut stdout = stdout();
    enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen, Hide)?;

    let (width, height) = (20, 20); // 20x20 pixel grid (10 terminal rows)
    let mut game = Game::new(width, height);
    let frame_duration = Duration::from_millis(50);

    'gameloop: loop {
        // Input handling
        while event::poll(Duration::from_secs(0))? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Char('q') => break 'gameloop,
                    KeyCode::Char('w') if game.player_y > 0 => game.player_y -= 1,
                    KeyCode::Char('s') if game.player_y < game.height - 1 => game.player_y += 1,
                    KeyCode::Char('a') if game.player_x > 0 => game.player_x -= 1,
                    KeyCode::Char('d') if game.player_x < game.width - 1 => game.player_x += 1,
                    _ => {}
                }
            }
        }

        // Update game state
        game.clear();
        game.draw_player();

        // Render frame
        queue!(stdout, MoveTo(0, 0))?;
        for line in game.render() {
            queue!(stdout, Print(line), Print("\r\n"))?;
        }
        stdout.flush()?;

        thread::sleep(frame_duration);
    }

    execute!(stdout, LeaveAlternateScreen, Show)?;
    disable_raw_mode()?;
    Ok(())
}
