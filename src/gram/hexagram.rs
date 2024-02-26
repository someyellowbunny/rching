use std::fmt::Display;

use super::Trigram;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Hexagram {
    upper: Trigram,
    lower: Trigram,
}

impl Hexagram {
    const CONSTS: [(Trigram, Trigram); 64] = 
    [
        (Trigram::CREATIVE, Trigram::CREATIVE),
        (Trigram::RECEPTIVE, Trigram::RECEPTIVE),
        (Trigram::ABYSMAL, Trigram::AROUSING),
        (Trigram::KEEPING_STILL, Trigram::ABYSMAL),
        (Trigram::ABYSMAL, Trigram::CREATIVE),
        (Trigram::CREATIVE, Trigram::ABYSMAL),
        (Trigram::RECEPTIVE, Trigram::ABYSMAL),
        (Trigram::ABYSMAL, Trigram::RECEPTIVE),
        (Trigram::GENTLE, Trigram::CREATIVE),
        (Trigram::CREATIVE, Trigram::JOYOUS),
        (Trigram::RECEPTIVE, Trigram::CREATIVE),
        (Trigram::CREATIVE, Trigram::RECEPTIVE),
        (Trigram::CREATIVE, Trigram::CLINGING),
        (Trigram::CLINGING, Trigram::CREATIVE),
        (Trigram::RECEPTIVE, Trigram::KEEPING_STILL),
        (Trigram::AROUSING, Trigram::RECEPTIVE),
        (Trigram::JOYOUS, Trigram::AROUSING),
        (Trigram::KEEPING_STILL, Trigram::GENTLE),
        (Trigram::RECEPTIVE, Trigram::JOYOUS),
        (Trigram::GENTLE, Trigram::RECEPTIVE),
        (Trigram::CLINGING, Trigram::AROUSING),
        (Trigram::KEEPING_STILL, Trigram::CLINGING),
        (Trigram::KEEPING_STILL, Trigram::RECEPTIVE),
        (Trigram::RECEPTIVE, Trigram::AROUSING),
        (Trigram::CREATIVE, Trigram::AROUSING),
        (Trigram::KEEPING_STILL, Trigram::CREATIVE),
        (Trigram::KEEPING_STILL, Trigram::AROUSING),
        (Trigram::JOYOUS, Trigram::GENTLE),
        (Trigram::ABYSMAL, Trigram::ABYSMAL),
        (Trigram::CLINGING, Trigram::CLINGING),
        (Trigram::JOYOUS, Trigram::KEEPING_STILL),
        (Trigram::AROUSING, Trigram::GENTLE),
        (Trigram::CREATIVE, Trigram::KEEPING_STILL),
        (Trigram::AROUSING, Trigram::CREATIVE),
        (Trigram::CLINGING, Trigram::RECEPTIVE),
        (Trigram::RECEPTIVE, Trigram::CLINGING),
        (Trigram::GENTLE, Trigram::CLINGING),
        (Trigram::CLINGING, Trigram::JOYOUS),
        (Trigram::ABYSMAL, Trigram::KEEPING_STILL),
        (Trigram::AROUSING, Trigram::ABYSMAL),
        (Trigram::KEEPING_STILL, Trigram::JOYOUS),
        (Trigram::GENTLE, Trigram::AROUSING),
        (Trigram::JOYOUS, Trigram::CREATIVE),
        (Trigram::CREATIVE, Trigram::GENTLE),
        (Trigram::JOYOUS, Trigram::RECEPTIVE),
        (Trigram::RECEPTIVE, Trigram::GENTLE),
        (Trigram::JOYOUS, Trigram::ABYSMAL),
        (Trigram::ABYSMAL, Trigram::GENTLE),
        (Trigram::JOYOUS, Trigram::CLINGING),
        (Trigram::CLINGING, Trigram::GENTLE),
        (Trigram::AROUSING, Trigram::AROUSING),
        (Trigram::KEEPING_STILL, Trigram::KEEPING_STILL),
        (Trigram::GENTLE, Trigram::KEEPING_STILL),
        (Trigram::AROUSING, Trigram::JOYOUS),
        (Trigram::AROUSING, Trigram::CLINGING),
        (Trigram::CLINGING, Trigram::KEEPING_STILL),
        (Trigram::GENTLE, Trigram::GENTLE),
        (Trigram::JOYOUS, Trigram::JOYOUS),
        (Trigram::GENTLE, Trigram::ABYSMAL),
        (Trigram::ABYSMAL, Trigram::JOYOUS),
        (Trigram::GENTLE, Trigram::JOYOUS),
        (Trigram::AROUSING, Trigram::KEEPING_STILL),
        (Trigram::ABYSMAL, Trigram::CLINGING),
        (Trigram::CLINGING, Trigram::ABYSMAL)
    ];

    const EN_NAMES: [&'static str; 64] =
    [
        "The Creative",
        "The Receptive",
        "Difficulty at the Beginning",
        "Youthful Folly",
        "Waiting",
        "Conflict",
        "The Army",
        "Holding Together",
        "The Taming Power of the Small",
        "Treading",
        "Peace",
        "Standstill",
        "Fellowship with Men",
        "Posession in Great Measure",
        "Modesty",
        "Enthusiasm",
        "Following",
        "Work on What Has Been Spoiled",
        "Approach",
        "Contemplation",
        "Biting Through",
        "Grace",
        "Splitting Apart",
        "Return",
        "Innocence",
        "The Taming Power of the Great",
        "The Corners of the Mouth",
        "Preponderance of the Great",
        "The Abysmal",
        "The Clinging",
        "Influence",
        "Duration",
        "Retreat",
        "The Power of the Great",
        "Progress",
        "Darkening of the Light",
        "The Family",
        "Opposition",
        "Obstruction",
        "Deliverance",
        "Decrease",
        "Increase",
        "Break-through",
        "Coming to Meet",
        "Gathering Together",
        "Pushing Upward",
        "Oppression",
        "The Well",
        "Revolution",
        "The Caldron",
        "The Arousing",
        "Keeping Still",
        "Development",
        "The Marrying Maiden",
        "Abundance",
        "The Wanderer",
        "The Gentle",
        "The Joyous",
        "Dispersion",
        "Limitation",
        "Inner Truth",
        "Preponderance of the Small",
        "After Completion",
        "Before Completion",
    ];

    pub fn new() -> Self {
        let lower = Trigram::new();
        let upper = Trigram::new();

        Self { upper, lower }
    }

    fn get_id(self) -> usize {
        for (a, b) in Self::CONSTS.iter().enumerate() {
            if *b == self.into() {
                return a;
            }
        }

        panic!("unable to get id:\t{:?}", self);
    }

    pub fn pure(self) -> bool {
        self == !self
    }

    pub fn name(self) -> &'static str {
        Self::EN_NAMES[self.get_id()]
    }
}

impl From<(Trigram, Trigram)> for Hexagram {
    fn from(value: (Trigram, Trigram)) -> Self {
        Self {
            upper: value.0,
            lower: value.1,
        }
    }
}

impl Into<(Trigram, Trigram)> for Hexagram {
    fn into(self) -> (Trigram, Trigram) {
        (self.upper, self.lower)
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
