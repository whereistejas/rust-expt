#![feature(derive_default_enum)]

use std::fmt::Debug;
use strum::EnumIter;
use strum::IntoEnumIterator;

#[derive(Debug, Default, EnumIter)]
enum SomeEnum {
    A(SomeA),
    #[default]
    B,
    C,
}

#[derive(Debug, Default, EnumIter)]
enum SomeA {
    #[default]
    A,
    B,
    C,
}

impl Iterator for SomeEnum {
    type Item = SomeEnum;

    fn next(&mut self) -> Option<Self::Item> {
        SomeEnum::iter().next()
    }
}

fn main() {
    let my_var: SomeEnum;
    my_var.iter().map(|item| print_me(item));
}

fn print_me<T: IntoEnumIterator + Debug>(item: T) {
    T::iter().map(|item| println!("{item:?}"));
}
