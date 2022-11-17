#![feature(test)]

extern crate test;

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    use std::fs::{remove_file, OpenOptions};
    use std::io::Write;
    use std::path::Path;

    #[bench]
    fn single_write(b: &mut Bencher) {
        let path = Path::new("/tmp/random_test_file");

        let mut file = OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .open(path)
            .unwrap();

        let array: [u64; 8] = rand::random();

        b.iter(|| {
            for _ in 0..10_000 {
                file.write_all(format!("{:?}\n", array).as_bytes()).unwrap();
            }
        });

        remove_file(path).unwrap();
    }
}
