use std::collections::BTreeSet;

fn main() {
    let mut a = BTreeSet::new();

    a.insert(vec![9, 9, 9, 9]);
    a.insert(vec![1, 1]);
    a.insert(vec![2, 2, 2, 2]);
    a.insert(vec![3]);

    println!("{a:?}")
}

// result: {[1, 1], [2, 2, 2, 2], [3], [9, 9, 9, 9]}
