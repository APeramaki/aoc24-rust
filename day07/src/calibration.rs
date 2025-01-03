use std::iter;

use itertools::{repeat_n, Itertools};

// use itertools

const OPERATORS: [char; 2] = ['+', '*'];

pub struct Calibration {
    test_value: u64,
    values: Vec<u64>,
}

impl Calibration {
    pub fn new(string: &str) -> Self {
        
        let mut split = string.split(':');
        
        let test_value = split.next().unwrap_or("0").parse::<u64>().unwrap();
        
        let values: Vec<u64> = split
            .next()
            .unwrap()
            .split(' ')
            .filter_map(|s| 
                s.parse::<u64>().ok()
            ).collect();
        
        Self {
            test_value,
            values
        }
    }

    pub fn test_all(&self) -> u64 {
        let combinations = repeat_n(OPERATORS, self
            .values.len()-1)
            .multi_cartesian_product()
            .collect::<Vec<_>>()
            ;
        let found = combinations.iter()
            .map(|operands|{
                let mut num_iterator = self.values.clone().into_iter();
                let mut total: u64 = num_iterator.next().unwrap();
                let mut operand_iter = operands.iter();
                for v in operand_iter.by_ref() {
                    total = match v {
                        '*' => total * num_iterator.next().unwrap(),
                        '+' => total + num_iterator.next().unwrap(),
                        _ => panic!() // should not happen
                    };
                }
                total
            })
            .any(|v| v == self.test_value)
            ;
        if found {
            return self.test_value;
        }
        0
    }

}