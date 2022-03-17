use std::cmp::Ordering;

#[derive(Clone, PartialEq, Eq, Debug)]
struct Number {
    integer: String,
    fraction: Option<String>,
}
impl PartialOrd for Number {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {}

impl Ord for Number {
    fn cmp(&self, other: &Self) -> Ordering {
        // TODO: think this thru and check it's correct
        let a_int = self.integer.trim_start_matches('0').to_string();
        let b_int = other.integer.trim_start_matches('0').to_string();

        let a_dec = self
            .fraction
            .clone()
            .unwrap_or_else(|| "".to_string())
            .trim_end_matches('0')
            .to_string();

        let b_dec = other
            .fraction
            .clone()
            .unwrap_or_else(|| "".to_string())
            .trim_end_matches('0')
            .to_string();

        (a_int.len())
            .cmp(&b_int.len())
            .then_with(|| a_int.cmp(&b_int))
            .then_with(|| a_dec.cmp(&b_dec))
    }
}

#[cfg(test)]
mod tests {
    use super::Number;
    use std::cmp::Ordering;

    #[test]
    fn equal_int_with_diff_leading_zeroes_equal_frac() {
        let num1 = Number {
            integer: "00000".to_string(),
            fraction: Some("00000001".to_string()),
        };

        let num2 = Number {
            integer: "0000000000".to_string(),
            fraction: Some("00000001".to_string()),
        };

        assert_eq!(Ordering::Equal, num1.cmp(&num2));
    }
    #[test]
    fn equal_int_with_diff_leading_zeroes_equal_frac2() {
        let num1 = Number {
            integer: "000001".to_string(),
            fraction: Some("00000001".to_string()),
        };

        let num2 = Number {
            integer: "00000000001".to_string(),
            fraction: Some("00000001".to_string()),
        };

        assert_eq!(Ordering::Equal, num1.cmp(&num2));
    }
    #[test]
    fn greater_int_with_diff_leading_zeroes_equal_frac2() {
        let num1 = Number {
            integer: "000002".to_string(),
            fraction: Some("00000001".to_string()),
        };

        let num2 = Number {
            integer: "00000000001".to_string(),
            fraction: Some("00000001".to_string()),
        };

        assert_eq!(Ordering::Greater, num1.cmp(&num2));
    }
    #[test]
    fn greater_int_with_diff_leading_zeroes_equal_frac_with_diff_trailing_zeroes() {
        let num1 = Number {
            integer: "000002".to_string(),
            fraction: Some("000000010000000".to_string()),
        };

        let num2 = Number {
            integer: "00000000001".to_string(),
            fraction: Some("00000001".to_string()),
        };

        assert_eq!(Ordering::Greater, num1.cmp(&num2));
    }
    #[test]
    fn equal_int_with_diff_leading_zeroes_equal_frac_with_diff_trailing_zeroes() {
        let num1 = Number {
            integer: "000001".to_string(),
            fraction: Some("000000010000000".to_string()),
        };

        let num2 = Number {
            integer: "00000000001".to_string(),
            fraction: Some("00000001".to_string()),
        };

        assert_eq!(Ordering::Equal, num1.cmp(&num2));
    }
    #[test]
    fn equal_int_with_diff_leading_zeroes_lesser_frac_with_diff_leading_trailing_zeroes() {
        let num1 = Number {
            integer: "000001".to_string(),
            fraction: Some("000000000000010000000".to_string()),
        };

        let num2 = Number {
            integer: "00000000001".to_string(),
            fraction: Some("00000001".to_string()),
        };

        assert_eq!(Ordering::Less, num1.cmp(&num2));
    }
    #[test]
    fn equal_int_with_diff_leading_zeroes_no_frac_in_one_number() {
        let num1 = Number {
            integer: "000001".to_string(),
            fraction: Some("000000000000010000000".to_string()),
        };

        let num2 = Number {
            integer: "00000000001".to_string(),
            fraction: None,
        };

        assert_eq!(Ordering::Greater, num1.cmp(&num2));
    }
    #[test]
    fn no_int_lesser_frac_trailing_zeroes() {
        let num1 = Number {
            integer: "".to_string(),
            fraction: Some("000000000000010000000".to_string()),
        };

        let num2 = Number {
            integer: "".to_string(),
            fraction: Some("000000000000000000111110000000".to_string()),
        };

        assert_eq!(Ordering::Greater, num1.cmp(&num2));
    }
}
