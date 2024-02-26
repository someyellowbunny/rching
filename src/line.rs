use crate::{cast::Cast, yinyang::YinYang};


#[derive(Debug, Clone, Copy)]
pub struct Line(pub u8);

impl Line {
    pub fn new(cast: Cast) -> Self {
        cast.into()
    }

    pub fn yy(self) -> YinYang {
        match self.value() {
            6 | 7 => YinYang::Yin,
            8 | 9 => YinYang::Yang,
            v => panic!("'{v}' is an invalid line value"),
        }
    }

    pub fn pure(self) -> bool {
        match self.value() {
            6 | 9 => false,
            _ => true
        }
    }

    pub fn value(self) -> u8 {
        self.0
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
        Self(val.value())
    }
}

impl PartialEq for Line {
    fn eq(&self, other: &Self) -> bool {
        self.yy() == other.yy()
    }
}
