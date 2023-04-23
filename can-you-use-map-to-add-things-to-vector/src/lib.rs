fn modify_list(a: Vec<u32>) -> Vec<u32> {
    a.iter().map(|x| x * 2).map(|_| 10u32).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check() {
        let a = vec![1, 2];
        assert_eq!(vec![2, 4, 10], modify_list(a))
    }
}
