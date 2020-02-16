use crate::roll::Roll;

mod value_counts {
    use crate::roll::Roll;
    use crate::die::Die;

    pub struct ValueCounts {
        counts: [i32; 6],
    }

    impl ValueCounts {
        fn count(&self, die: &Die) -> i32{
            let die_value = die.value() as usize;
            let index = die_value - 1;
            return self.counts[index];
        }

        pub fn has_exact(&self, count: i32) -> bool {
            for value in (1..7).rev() {
                let die = Die::from(i32::from(value));
                if self.count(&die) == count {
                    return true;
                };
            }
            return false;
        }

        pub fn has_kind(&self, kind: i32) -> bool {
            for value in (1..7).rev() {
                let die = Die::from(i32::from(value));
                if self.count(&die) >= kind {
                    return true;
                };
            }
            return false;
        }
    }

    impl From<&Roll> for ValueCounts {
        fn from(roll: &Roll) -> ValueCounts {
            let mut counts = [0; 6];
            for die in roll {
                let die_value = die.value() as usize;
                let index = die_value - 1;
                counts[index] += 1;
            }
            return ValueCounts { counts: counts };
        }
    }
}

use value_counts::ValueCounts;

pub fn total(roll: &Roll) -> i32 {
    let mut result: i32 = 0;
    for die in roll.into_iter() {
        result += die.value();
    }
    return result;
}

pub fn score_as(roll: &Roll, value: i32) -> i32 {
    let count = roll.count_values(value);
    return count * value;
}

pub fn four_of_a_kind(roll: &Roll) -> i32 {
    let counts = ValueCounts::from(roll);
    return match counts.has_kind(4) {
        true => total(roll),
        false => 0,
    };
}

pub fn three_of_a_kind(roll: &Roll) -> i32 {
    let counts = ValueCounts::from(roll);
    return match counts.has_kind(3) {
        true => total(roll),
        false => 0,
    };
}

pub fn full_house(roll: &Roll) -> i32 {
    let counts = ValueCounts::from(roll);
    let is_fullhouse = counts.has_exact(3) && counts.has_exact(2);
    return match is_fullhouse {
        true => 25,
        false => 0,
    };
}

pub fn rustzee(roll: &Roll) -> i32 {
    let counts = ValueCounts::from(roll);
    return match counts.has_kind(5) {
        true => 50,
        false => 0,
    };
}
