use rand::prelude::*;

use crate::yinyang::YinYang;
pub struct Cast(u8);

impl Cast {
    pub fn cast() -> Self {
        let mut rng = thread_rng();
        let mut throwaway = "".to_owned();
        std::io::stdin().read_line(&mut throwaway).unwrap();
        
        let one: YinYang     = rng.gen();
        let two: YinYang     = rng.gen();
        let three: YinYang   = rng.gen();

        let value = one as u8 + two as u8 + three as u8;

        Self(value)
    }

    pub fn value(self) -> u8 {
        self.0
    }
}
