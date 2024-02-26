use std::fmt::Display;

use crate::{cast::Cast, line::Line, yinyang::YinYang};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Trigram([Line; 3]);

impl Trigram {
    pub const CREATIVE: Trigram = Trigram([Line(8), Line(8), Line(8)]);
    pub const RECEPTIVE: Trigram = Trigram([Line(7), Line(7), Line(7)]);
    pub const AROUSING: Trigram = Trigram([Line(7), Line(7), Line(8)]);
    pub const KEEPING_STILL: Trigram = Trigram([Line(8), Line(7), Line(7)]);
    pub const ABYSMAL: Trigram = Trigram([Line(7), Line(8), Line(7)]);
    pub const CLINGING: Trigram = Trigram([Line(8), Line(7), Line(8)]);
    pub const JOYOUS: Trigram = Trigram([Line(7), Line(8), Line(8)]);
    pub const GENTLE: Trigram = Trigram([Line(8), Line(8), Line(7)]);

    const CONSTS: [Self; 8] =
    [
        Self::CREATIVE,
        Self::RECEPTIVE,
        Self::AROUSING,
        Self::KEEPING_STILL,
        Self::ABYSMAL,
        Self::CLINGING,
        Self::JOYOUS,
        Self::GENTLE,
    ];

    const EN_NAMES: [&'static str; 8] =
    [
        "the Creative",
        "the Receptive",
        "the Arousing",
        "the Abysmal",
        "Keeping Still",
        "the Gentle",
        "the Clinging",
        "the Joyous",
    ];

    const IMAGE: [&'static str; 8] =
    [
        "Heaven",
        "Earth",
        "Thunder",
        "Water",
        "Mountain",
        "Wind/Wood",
        "Fire",
        "Lake",
    ];

    pub fn new() -> Self {
        let mut inner = [
            Line::new(Cast::cast()),
            Line::new(Cast::cast()),
            Line::new(Cast::cast())
        ];

        inner.reverse();

        Self(inner)
    }

    fn get_id(self) -> usize {
        for (i, e) in Self::CONSTS.iter().enumerate() {
            if *e == self {
                return i;
            }
        }

        panic!("unable to get id:\t{:?}", self);
    }

    pub fn name(self) -> &'static str {
        Self::EN_NAMES[self.get_id()]
    }

    pub fn image(self) -> &'static str {
        Self::IMAGE[self.get_id()]
    }
}

impl Into<[YinYang; 3]> for Trigram {
    fn into(self) -> [YinYang; 3] {
        [
            self.0[0].yy(),
            self.0[1].yy(),
            self.0[2].yy(),
        ]
    }
}

impl std::ops::Not for Trigram {
    type Output = Trigram;

    fn not(self) -> Self::Output {
        Self(
            [
                !self.0[0],
                !self.0[1],
                !self.0[2],
            ]
        )
    }
}

impl Display for Trigram {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, 
            "{} {}\n{} {}\n{} {}",
            self.0[0].value(), self.0[0].yy(),
            self.0[1].value(), self.0[1].yy(),
            self.0[2].value(), self.0[2].yy()
        )
    }
}
