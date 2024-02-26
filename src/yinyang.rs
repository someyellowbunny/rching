use rand::{distributions::Standard, prelude::*};
use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum YinYang {
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
