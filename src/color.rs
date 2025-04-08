use crossterm::style::Color;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PaletteColor {
    Black,
    DarkBlue,
    DarkPurple,
    DarkGreen,
    Brown,
    DarkGray,
    LightGray,
    White,
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    Indigo,
    Pink,
    Peach,
    Transparent,
}

impl PaletteColor {
    pub fn to_rgb(self) -> Color {
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
