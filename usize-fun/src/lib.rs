#![feature(unchecked_math)]
#![feature(mixed_integer_ops)]

#[cfg(test)]
mod tests {
    use std::ops::{AddAssign, SubAssign};

    #[test]
    fn subtract_usize_within_bounds() {
        let a = 12usize;
        let b = 10usize;

        assert_eq!(a.checked_sub(b), Some(2));
    }

    #[test]
    fn unchecked_sub_usize_not_within_bounds() {
        let a = 8usize;
        let b = 10usize;

        // NOTE: The result of `unchecked_sub` is Undefined Behaviour and this test can fail at any
        // time.
        assert_eq!(unsafe { a.unchecked_sub(b) }, usize::MAX - 1);
    }

    #[test]
    fn subtract_usize_not_within_bounds() {
        let a = 8usize;
        let b = 10usize;

        // NOTE: The result of `unchecked_sub` is Undefined Behaviour and this test can fail at any
        // time.
        assert_eq!(a - b, 2usize);
    }

    #[test]
    fn add_isize_zero_sum() {
        let a = 10isize;
        let b = -10isize;

        assert_eq!(a.checked_add(b), Some(0));
    }

    #[test]
    fn add_isize_negative_sum() {
        let a = 10isize;
        let b = -12isize;

        assert_eq!(a.checked_add(b), Some(-2));
    }

    #[test]
    fn increment_negative_isize() {
        let mut a = -10isize;
        a.add_assign(1isize);

        let mut b = -10isize;
        b.add_assign(-1isize);

        assert_eq!(a, -9isize);
        assert_eq!(b, -11isize);
    }

    #[test]
    fn increment_positive_isize() {
        let mut a = 10isize;
        a.add_assign(1isize);

        let mut b = 10isize;
        b.add_assign(-1isize);

        assert_eq!(a, 11isize);
        assert_eq!(b, 9isize);
    }

    #[test]
    fn decrement_negative_isize() {
        let mut a = -10isize;
        a.sub_assign(1isize);

        let mut b = -10isize;
        b.sub_assign(-1isize);

        assert_eq!(a, -11isize);
        assert_eq!(b, -9isize);
    }

    #[test]
    fn decrement_positive_isize() {
        let mut a = -10isize;
        a.sub_assign(1isize);

        let mut b = -10isize;
        b.sub_assign(-1isize);

        assert_eq!(a, -11isize);
        assert_eq!(b, -9isize);
    }

    #[test]
    fn add_isize_unsigned_negative_sum() {
        let a = -10isize;
        let b = 8usize;

        assert_eq!(a.checked_add_unsigned(b), Some(-2isize));
    }

    #[test]
    fn add_isize_unsigned_positive_sum() {
        let a = -10isize;
        let b = 12usize;

        assert_eq!(a.checked_add_unsigned(b), Some(2isize));
    }

    #[test]
    fn try_from_negative_isize_to_usize_errs() {
        let a = -10isize;

        assert!(usize::try_from(a).is_err());
    }
    #[test]
    fn try_from_negative_isize_to_usize_ok() {
        let a = -10isize;

        assert_eq!(usize::try_from(a.abs()), Ok(10));
    }

    #[test]
    fn try_from_positive_isize_to_usize_ok() {
        let a = 10isize;

        assert_eq!(usize::try_from(a), Ok(10));
    }
}
