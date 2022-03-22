use std::cmp::Ordering;

fn main() {}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Number {
    decimal: String,
    fraction: Option<String>,
}
impl PartialOrd for Number {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Number {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // compare the decimal parts of the numbers
        self.decimal
            .parse::<u64>()
            .unwrap()
            .cmp(&other.decimal.parse::<u64>().unwrap())
            // compare the fraction part of the numbers after getting rid of the trailling zeroes.
            .then_with(|| {
                (self
                    .fraction
                    .as_ref()
                    .map(|x| x.trim_end_matches('0').parse::<u64>().unwrap()))
                .cmp(
                    &other
                        .fraction
                        .as_ref()
                        .map(|x| x.trim_end_matches('0').parse::<u64>().unwrap()),
                )
            })
            // compare the decimal part of numbers as strings
            .then_with(|| self.decimal.cmp(&other.decimal))
            // compare the fraction part of numbers as strings
            .then_with(|| self.fraction.cmp(&other.fraction))
    }
}

#[cfg(test)]
mod tests {
    use super::Number;
    use std::cmp::Ordering;

    #[test]
    fn equal_int_with_diff_leading_zeroes_equal_frac() {
        let num1 = Number {
            decimal: "00000".to_string(),
            fraction: Some("00000001".to_string()),
        };

        let num2 = Number {
            decimal: "0000000000".to_string(),
            fraction: Some("00000001".to_string()),
        };

        assert_eq!(Ordering::Greater, num1.cmp(&num2));
    }
    #[test]
    fn equal_int_with_diff_leading_zeroes_equal_frac2() {
        let num1 = Number {
            decimal: "000001".to_string(),
            fraction: Some("00000001".to_string()),
        };

        let num2 = Number {
            decimal: "00000000001".to_string(),
            fraction: Some("00000001".to_string()),
        };

        assert_eq!(Ordering::Less, num1.cmp(&num2));
    }
    #[test]
    fn greater_int_with_diff_leading_zeroes_equal_frac2() {
        let num1 = Number {
            decimal: "000002".to_string(),
            fraction: Some("00000001".to_string()),
        };

        let num2 = Number {
            decimal: "00000000001".to_string(),
            fraction: Some("00000001".to_string()),
        };

        assert_eq!(Ordering::Greater, num1.cmp(&num2));
    }
    #[test]
    fn greater_int_with_diff_leading_zeroes_equal_frac_with_diff_trailing_zeroes() {
        let num1 = Number {
            decimal: "000002".to_string(),
            fraction: Some("000000010000000".to_string()),
        };

        let num2 = Number {
            decimal: "00000000001".to_string(),
            fraction: Some("00000001".to_string()),
        };

        assert_eq!(Ordering::Greater, num1.cmp(&num2));
    }
    #[test]
    fn equal_int_with_diff_leading_zeroes_equal_frac_with_diff_trailing_zeroes() {
        let num1 = Number {
            decimal: "000001".to_string(),
            fraction: Some("000000010000000".to_string()),
        };

        let num2 = Number {
            decimal: "00000000001".to_string(),
            fraction: Some("00000001".to_string()),
        };

        assert_eq!(Ordering::Less, num1.cmp(&num2));
    }
    #[test]
    fn equal_int_with_diff_leading_zeroes_lesser_frac_with_diff_leading_trailing_zeroes() {
        let num1 = Number {
            decimal: "000001".to_string(),
            fraction: Some("000000000000010000000".to_string()),
        };

        let num2 = Number {
            decimal: "00000000001".to_string(),
            fraction: Some("00000001".to_string()),
        };

        assert_eq!(Ordering::Less, num1.cmp(&num2));
    }
    #[test]
    fn equal_int_with_diff_leading_zeroes_no_frac_in_one_number() {
        let num1 = Number {
            decimal: "000001".to_string(),
            fraction: Some("000000000000010000000".to_string()),
        };

        let num2 = Number {
            decimal: "00000000001".to_string(),
            fraction: None,
        };

        assert_eq!(Ordering::Greater, num1.cmp(&num2));
    }
    #[test]
    fn no_int_lesser_frac_trailing_zeroes() {
        let num1 = Number {
            decimal: "".to_string(),
            fraction: Some("000000000000010000000".to_string()),
        };

        let num2 = Number {
            decimal: "".to_string(),
            fraction: Some("000000000000000000111110000000".to_string()),
        };

        assert_eq!(Ordering::Greater, num1.cmp(&num2));
    }
}
