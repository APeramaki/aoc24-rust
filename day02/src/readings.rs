pub struct Readings {
   values: Vec<i32>
}

impl Readings {
    pub fn new(vec: Vec<i32>) -> Readings {
        Readings { values: vec }
    }
    pub fn is_safe(&self) -> Option<()>{
        let is_rising = self.values[0] < self.values[1];
        if self.values.windows(2)
            .any(|pair|
                (pair[0] < pair[1]) != is_rising ||
                (pair[0] == pair[1] ) ||
                (pair[0] - pair[1]).abs() > 3) {
                return None;
            }
        return Some(());
    }
    pub fn safety_filter(&self) -> Self {
        let is_rising = self.values
            .windows(2)
            .filter(|pair| pair[0] < pair[1])
            .count() > 2;
            
    }
}
