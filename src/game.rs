use crate::color::PaletteColor;
use std::time::Instant;

pub struct Game {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<Vec<PaletteColor>>,
    pub player_x: usize,
    pub player_y: usize,
}

impl Game {
    pub fn new(width: usize, height: usize) -> Self {
        let pixels = vec![vec![PaletteColor::Transparent; width]; height];
        Game {
            width,
            height,
            pixels,
            player_x: width / 2,
            player_y: height / 2,
        }
    }

    pub fn clear(&mut self) {
        for row in &mut self.pixels {
            for pixel in row {
                *pixel = PaletteColor::Transparent;
            }
        }
    }

    pub fn render(&self) -> Vec<(u16, u16, String)> {
        let mut output = Vec::with_capacity(self.height + 2);

        // Top border
        let mut top_border = String::new();
        top_border.push_str("\x1b[32m╔"); // Green
        top_border.push_str(&"═".repeat(self.width));
        top_border.push_str("╗\x1b[0m"); // Reset color
        output.push((0, 0, top_border));

        // Game area with side borders
        for (y, row) in self.pixels.iter().enumerate() {
            let mut line = String::new();
            line.push_str("\x1b[32m║"); // Green
            for pixel in row {
                match pixel {
                    PaletteColor::Transparent => line.push(' '),
                    PaletteColor::Red => line.push_str("\x1b[31m█\x1b[0m"), // Red
                    _ => line.push(' '),
                }
            }
            line.push_str("\x1b[32m║\x1b[0m"); // Green and reset color
            output.push((0, (y + 1) as u16, line));
        }

        // Bottom border
        let mut bottom_border = String::new();
        bottom_border.push_str("\x1b[32m╚"); // Green
        bottom_border.push_str(&"═".repeat(self.width));
        bottom_border.push_str("╝\x1b[0m"); // Reset color
        output.push((0, (self.height + 1) as u16, bottom_border));

        output
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: PaletteColor) {
        if x < self.width && y < self.height {
            self.pixels[y][x] = color;
        }
    }
}
