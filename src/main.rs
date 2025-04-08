mod color;
mod game;

use crate::{color::PaletteColor, game::Game};
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind}, // Import KeyEventKind
    execute,
    queue,
    style::Print,
    terminal::{
        self, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
    },
};
use std::{
    io::{Write, stdout},
    thread,
    time::{Duration, Instant},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Terminal setup
    let mut stdout = stdout();
    enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen, Hide)?;

    // Create game state with fixed size
    let mut game = Game::new(40, 20);

    // Game loop timing
    let target_fps = 60;
    let frame_duration = Duration::from_secs_f32(1.0 / target_fps as f32);
    let mut last_update = Instant::now();

    // Main game loop
    'gameloop: loop {
        // Handle input
        while event::poll(Duration::ZERO)? {
            match event::read()? {
                Event::Key(KeyEvent {
                    code,
                    kind: KeyEventKind::Press,
                    ..
                }) => match code {
                    KeyCode::Char('q') | KeyCode::Esc => break 'gameloop,
                    KeyCode::Left => game.player_x = game.player_x.saturating_sub(1),
                    KeyCode::Right => game.player_x = (game.player_x + 1).min(game.width - 1),
                    KeyCode::Up => game.player_y = game.player_y.saturating_sub(1),
                    KeyCode::Down => game.player_y = (game.player_y + 1).min(game.height - 1),
                    _ => {}
                },
                _ => {}
            }
        }

        // Update game state
        game.clear();
        game.set_pixel(game.player_x, game.player_y, PaletteColor::Red);

        // Render game
        let rendered = game.render();
        for (_, y, line) in rendered.into_iter() {
            queue!(stdout, MoveTo(0, y), Print(line))?;
        }
        stdout.flush()?;

        // Frame timing
        let frame_time = last_update.elapsed();
        if frame_time < frame_duration {
            thread::sleep(frame_duration - frame_time);
        }
        last_update = Instant::now();
    }

    // Cleanup
    execute!(stdout, LeaveAlternateScreen, Show)?;
    disable_raw_mode()?;
    Ok(())
}
