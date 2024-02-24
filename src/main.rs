use std::fmt::Display;
use YinYang::*;
use rand::{prelude::*, distributions::{Distribution, Standard}, thread_rng};

struct Cast(u8);

impl Cast {
    fn cast() -> Self {
        let mut rng = thread_rng();
        let mut throwaway = "".to_owned();
        std::io::stdin().read_line(&mut throwaway).unwrap();
        
        let one: YinYang     = rng.gen();
        let two: YinYang     = rng.gen();
        let three: YinYang   = rng.gen();

        let value = one as u8 + two as u8 + three as u8;

        Self(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum YinYang {
    Yang = 3,
    Yin = 2,
}

impl Display for YinYang {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            YinYang::Yang => write!(f, "———"),
            YinYang::Yin=> write!(f, "— —"),

        }
    }
}

impl Distribution<YinYang> for Standard {
    fn sample<R: rand::prelude::Rng + ?Sized>(&self, rng: &mut R) -> YinYang {
        match rng.gen() {
            true => YinYang::Yang,
            false => YinYang::Yin,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Line(u8);

impl Line {
    fn new(cast: Cast) -> Self {
        cast.into()
    }

    fn yy(self) -> YinYang {
        match self.value() {
            6 | 7 => YinYang::Yin,
            8 | 9 => YinYang::Yang,
            v => panic!("'{v}' is an invalid line value"),
        }
    }

    fn pure(self) -> bool {
        match self.value() {
            6 | 9 => false,
            _ => true
        }
    }

    fn value(self) -> u8 {
        self.0
    }
    
    fn change(self) -> Self {
        !self
    }
}

impl std::ops::Not for Line {
    type Output = Line;

    fn not(self) -> Self::Output {
        match self.value() {
            6 => Self(9),
            9 => Self(6),
            _ => self
        }
    }
}

impl From<Cast> for Line {
    fn from(val: Cast) -> Self {
        Self(val.0)
    }
}

#[derive(Debug, Clone, Copy)]
struct Trigram([Line; 3]);

impl Trigram {
    const CONSTS: [[YinYang; 3]; 8] =
    [
        [Yang, Yang, Yang], // Creative
        [Yin, Yin, Yin],    // Receptive
        [Yin, Yin, Yang],   // Arousing
        [Yin, Yang, Yin],   // Abysmal
        [Yang, Yin, Yin],   // Keeping Still
        [Yang, Yang, Yin],  // Gentle
        [Yang, Yin, Yang],  // Clinging
        [Yin, Yang, Yang],  // Joyous
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

    fn new() -> Self {
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
            if *e == Into::<[YinYang; 3]>::into(self) {
                return i;
            }
        }

        panic!("unable to get id:\t{:?}", self);
    }

    fn change(self) -> Self {
        !self
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

#[derive(Debug, Clone, Copy)]
struct Hexagram {
    upper: Trigram,
    lower: Trigram,
}

impl Hexagram {
    fn new() -> Self {
        let lower = Trigram::new();
        let upper = Trigram::new();

        Self { upper, lower }
    }

    fn change(self) -> Self {
        !self
    }
}

impl std::ops::Not for Hexagram {
    type Output = Hexagram;

    fn not(self) -> Self {
        Self {
            upper: !self.upper,
            lower: !self.lower
        }
    }
}

impl Display for Hexagram {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n{}", self.upper, self.lower)
    }
}

fn main() {
    let h = Hexagram::new();
    let changes = !h;

    println!("{}\n\n{}", h, changes);
}
