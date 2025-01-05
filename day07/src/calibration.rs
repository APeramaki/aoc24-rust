use itertools::{repeat_n, Itertools};

#[derive(Clone)]
pub enum OPERATOR {
    SUM,
    PRODUCT,
    CONCAT,
}

const OPERATORS: [OPERATOR; 2] = [OPERATOR::SUM, OPERATOR::PRODUCT];
const OPERATORS_EXT: [OPERATOR; 3] = [OPERATOR::SUM, OPERATOR::PRODUCT, OPERATOR::CONCAT];

pub struct Calibration {
    test_value: u64,
    values: Vec<u64>,
    operators: Vec<OPERATOR>,
}

impl Calibration {
    pub fn new(string: &str, use_concat: bool) -> Self {
        let mut split = string.split(':');

        let test_value = split.next().unwrap_or("0").parse::<u64>().unwrap();

        let values: Vec<u64> = split
            .next()
            .unwrap()
            .split(' ')
            .filter_map(|s| s.parse::<u64>().ok())
            .collect();

        Self {
            test_value,
            values,
            operators: if use_concat {
                OPERATORS_EXT.to_vec()
            } else {
                OPERATORS.to_vec()
            },
        }
    }

    pub fn test_all(&self) -> Option<u64> {
        let combinations = repeat_n(&self.operators, self.values.len() - 1)
            .multi_cartesian_product()
            .collect::<Vec<_>>();
        let found = combinations
            .iter()
            .map(|operands| {
                let mut num_iterator = self.values.iter();
                let mut total: u64 = *num_iterator.next().unwrap();
                let mut operand_iter = operands.iter();
                for v in operand_iter.by_ref() {
                    total = match v {
                        OPERATOR::PRODUCT => total * num_iterator.next().unwrap(),
                        OPERATOR::SUM => total + num_iterator.next().unwrap(),
                        OPERATOR::CONCAT => {
                            let num = num_iterator.next().unwrap();
                            let digits = (*num as f64).log10().floor() as u64 + 1;
                            total * 10_u64.pow(digits as u32) + num
                        }
                    };
                }
                total
            })
            .any(|v| v == self.test_value);
        if found {
            return Some(self.test_value);
        }
        None
    }
}
