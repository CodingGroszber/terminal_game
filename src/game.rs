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

    fn get_braille_char_and_color(&self, bx: usize, by: usize) -> (char, Option<PaletteColor>) {
        let x = bx * 2;
        let y = by * 4;
        let mut n = 0u8;
        let mut color = None;
        for dy in 0..4 {
            for dx in 0..2 {
                let px = x + dx;
                let py = y + dy;
                if px < self.width
                    && py < self.height
                    && self.pixels[py][px] != PaletteColor::Transparent
                {
                    let bit = match (dx, dy) {
                        (0, 0) => 0,
                        (0, 1) => 1,
                        (0, 2) => 2,
                        (0, 3) => 6,
                        (1, 0) => 3,
                        (1, 1) => 4,
                        (1, 2) => 5,
                        (1, 3) => 7,
                        _ => unreachable!(),
                    };
                    n |= 1 << bit;
                    if color.is_none() {
                        color = Some(self.pixels[py][px]);
                    }
                }
            }
        }
        if n == 0 {
            (' ', None)
        } else {
            let braille_char = char::from_u32(0x2800 + n as u32).unwrap();
            (braille_char, color)
        }
    }

    pub fn render(&self) -> Vec<(u16, u16, String)> {
        let braille_width = self.width / 2;
        let braille_height = self.height / 4;
        let mut output = Vec::with_capacity(braille_height + 2);
        let border_escape = self.border_color.to_ansi_escape();

        let top_border = format!(
            "{}{}{}{}\x1b[0m",
            border_escape,
            '╔',
            "═".repeat(braille_width),
            '╗'
        );
        output.push((0, 0, top_border));

        for by in 0..braille_height {
            let mut line = String::new();
            line.push_str(&format!("{}{}\x1b[0m", border_escape, '║'));
            for bx in 0..braille_width {
                let (ch, color_opt) = self.get_braille_char_and_color(bx, by);
                if let Some(color) = color_opt {
                    let color_escape = color.to_ansi_escape();
                    line.push_str(&format!("{}{}\x1b[0m", color_escape, ch));
                } else {
                    line.push(' ');
                }
            }
            line.push_str(&format!("{}{}\x1b[0m", border_escape, '║'));
            output.push((0, (by + 1) as u16, line));
        }

        let bottom_border = format!(
            "{}{}{}{}\x1b[0m",
            border_escape,
            '╚',
            "═".repeat(braille_width),
            '╝'
        );
        output.push((0, (braille_height + 1) as u16, bottom_border));

        output
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: PaletteColor) {
        if x < self.width && y < self.height {
            self.pixels[y][x] = color;
        }
    }
}
