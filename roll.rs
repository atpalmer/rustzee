use std::fmt::{Display, Formatter, Result};
#[path = "die.rs"]
mod die;
use self::die::Die;

pub struct Roll {
    dice: [Die; 5],
}

impl From<[i32; 5]> for Roll {
    fn from(value: [i32; 5]) -> Roll {
        let dice: [Die; 5] = [
            value[0].into(),
            value[1].into(),
            value[2].into(),
            value[3].into(),
            value[4].into(),
        ];
        return Roll { dice: dice };
    }
}

impl Roll {
    pub fn of(a: i32, b: i32, c: i32, d: i32, e: i32) -> Roll {
        return Roll::from([a, b, c, d, e]);
    }
}

impl Display for Roll {
    fn fmt(&self, f: &mut Formatter) -> Result {
        return write!(f, "{:?}", self.dice);
    }
}

