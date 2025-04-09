use crate::color::PaletteColor;
use std::time::Instant;

pub struct Game {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<Vec<PaletteColor>>,
    pub player_x: usize,
    pub player_y: usize,
    pub player_color: PaletteColor,
    pub border_color: PaletteColor,
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
            player_color: PaletteColor::Red,
            border_color: PaletteColor::Green,
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

        let border_escape = self.border_color.to_ansi_escape();

        // Top border
        let top_border = format!("{}{}{}{}\x1b[0m", border_escape, '╔', "═".repeat(self.width), '╗');
        output.push((0, 0, top_border));

        // Game area with side borders
        for (y, row) in self.pixels.iter().enumerate() {
            let mut line = String::new();
            line.push_str(&format!("{}{}\x1b[0m", border_escape, '║')); // Left border, then reset
            for pixel in row {
                if *pixel == PaletteColor::Transparent {
                    line.push(' ');
                } else {
                    let pixel_escape = pixel.to_ansi_escape();
                    line.push_str(&format!("{}{}\x1b[0m", pixel_escape, '█'));
                }
            }
            line.push_str(&format!("{}{}\x1b[0m", border_escape, '║')); // Right border, then reset
            output.push((0, (y + 1) as u16, line));
        }

        // Bottom border
        let bottom_border = format!("{}{}{}{}\x1b[0m", border_escape, '╚', "═".repeat(self.width), '╝');
        output.push((0, (self.height + 1) as u16, bottom_border));

        output
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: PaletteColor) {
        if x < self.width && y < self.height {
            self.pixels[y][x] = color;
        }
    }
}
