use std::cmp::Ordering;

// pub fn natual_cmp(a: &str, b: &str) -> Ordering {
//     let pieces_a = str_to_pieces(a);
//     let pieces_b = str_to_pieces(b);

//     for (a, b) in pieces_a.iter().zip(pieces_b.iter()) {
//         return a.partial_cmp(b).unwrap();
//     }
// }

// pub fn str_to_pieces(value: &str) -> Vec<Pieces> {
//     let value = value.as_bytes();

//     let mut pieces = Vec::new();
//     let mut start: usize = 0;
//     let mut end: usize;

//     let mut piece_type = 0;

//     for (idx, c) in value.iter().enumerate() {
//         let prev_type = piece_type;

//         // check what kind of piece it is.
//         // it can either be:
//         // 1 - Digits
//         // 2 - Chars
//         if c.is_ascii_digit() {
//             // the number is a plain digit
//             piece_type = 1;
//         } else if c == &b'.' {
//             // looks like we have run into a decimal point
//             // check if the character before and after the decimal point is a digit
//             if value[idx - 1].is_ascii_digit() && value[idx + 1].is_ascii_digit() {
//                 piece_type = 1;
//             } else {
//                 piece_type = 2;
//             }
//         } else {
//             // its a plain character
//             piece_type = 2;
//         }

//         // the first and last character of the string are special cases
//         if (piece_type != prev_type && prev_type != 0) || (idx + 1) == value.len() {
//             // handle the special case of being at the end of the string
//             end = if (idx + 1) == value.len() {
//                 idx + 1
//             } else {
//                 idx
//             };

//             if prev_type == 1 {
//                 let temp = &value[start..end];
//                 pieces.push(Pieces::Digits(
//                     std::str::from_utf8(temp).unwrap().parse::<f64>().unwrap(),
//                 ))
//             }

//             if prev_type == 2 {
//                 let temp = &value[start..end];
//                 pieces.push(Pieces::Chars(std::str::from_utf8(temp).unwrap()))
//             }

//             // set the next piece's start value to the end of the current piece.
//             start = idx;
//         }
//     }

//     pieces
// }

// // The idea is to force each parse into a standard type, so that we can leave the comparison to the std library.
// #[derive(Debug, PartialEq)]
// pub enum Pieces<'a> {
//     Chars(&'a str),
//     Digits(f64),
// }

// impl<'a> PartialOrd for Pieces<'a> {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         fn compare<T: PartialOrd>(a: &T, b: &T) -> Option<Ordering> {
//             if a.gt(b) {
//                 Some(Ordering::Greater)
//             } else if a.lt(b) {
//                 Some(Ordering::Less)
//             } else {
//                 Some(Ordering::Equal)
//             }
//         }

//         match (self, other) {
//             (Pieces::Chars(me), Pieces::Chars(other)) => compare(me, other),
//             (Pieces::Digits(me), Pieces::Digits(other)) => compare(me, other),
//             // numbers always come before alphabets
//             (Pieces::Chars(_), Pieces::Digits(_)) => Some(Ordering::Greater),
//             // alphabets always come after numbers
//             (Pieces::Digits(_), Pieces::Chars(_)) => Some(Ordering::Less),
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use super::{str_to_pieces, Pieces};

    #[test]
    fn test_splitting_pieces() {
        let value = "z22";
        let result = str_to_pieces(value);

        assert_eq!(result, vec![Pieces::Chars("z"), Pieces::Digits(22.)])
    }

    #[test]
    fn test_splitting_pieces_decimals() {
        let value = "z22.676";
        let result = str_to_pieces(value);

        assert_eq!(result, vec![Pieces::Chars("z"), Pieces::Digits(22.676)])
    }

    #[test]
    fn test_numbers_decimals() {
        let mut list = vec![
            Pieces::Digits(1.1),
            Pieces::Digits(4.2),
            Pieces::Digits(4.1),
            Pieces::Digits(4.2),
            Pieces::Digits(0.2),
        ];

        list.sort_by(|a, b| a.partial_cmp(b).unwrap());
        assert_eq!(
            list,
            vec![
                Pieces::Digits(0.2),
                Pieces::Digits(1.1),
                Pieces::Digits(4.1),
                Pieces::Digits(4.2),
                Pieces::Digits(4.2),
            ]
        )
    }

    #[test]
    fn test_numbers() {
        let mut list = vec![
            Pieces::Digits(1.),
            Pieces::Digits(2.),
            Pieces::Digits(8.),
            Pieces::Digits(4.),
            Pieces::Digits(0.),
        ];

        list.sort_by(|a, b| a.partial_cmp(b).unwrap());
        assert_eq!(
            list,
            vec![
                Pieces::Digits(0.),
                Pieces::Digits(1.),
                Pieces::Digits(2.),
                Pieces::Digits(4.),
                Pieces::Digits(8.),
            ]
        )
    }

    #[test]
    fn test_strings() {
        let mut list = vec![
            Pieces::Chars("1"),
            Pieces::Chars("2"),
            Pieces::Chars("8"),
            Pieces::Chars("4"),
            Pieces::Chars("0"),
        ];

        list.sort_by(|a, b| a.partial_cmp(b).unwrap());
        assert_eq!(
            list,
            vec![
                Pieces::Chars("0"),
                Pieces::Chars("1"),
                Pieces::Chars("2"),
                Pieces::Chars("4"),
                Pieces::Chars("8"),
            ]
        )
    }
}
