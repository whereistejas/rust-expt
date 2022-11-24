#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let numbers = vec![1, 2, 3, 4, 5];
        let mut even = vec![];

        while let Some(number) = numbers.into_iter().filter(|number| number & 1 == 0).next() {
            even.push(number);
        }

        assert_eq!(even, vec![2, 4]);
        assert_eq!(numbers, vec![1, 3, 5]);
    }
}
