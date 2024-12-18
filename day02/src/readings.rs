use itertools::Itertools;

pub struct Readings {
    values: Vec<i32>,
}

impl Readings {
    pub fn new(vec: Vec<i32>) -> Readings {
        Readings { values: vec }
    }
    pub fn is_safe(&self) -> Option<()> {
        let is_safe = self
            .values
            .iter()
            .combinations(self.values.len() -1)
            .any(|combination| {
                let is_rising = combination[0] < combination[1];
                combination.windows(2).all(|pair| {
                    (pair[0] < pair[1]) == is_rising
                        && (pair[0] != pair[1])
                        && (pair[0] - pair[1]).abs() <= 3
                })
                
            });
        if is_safe {
            return Some(());
        }
        return None;
    }
}
