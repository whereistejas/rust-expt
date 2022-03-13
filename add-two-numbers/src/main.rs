use std::io::stdin;

fn main() {
    let mut some_string = String::new();

    println!("Type some numbers seperated by spaces: ");
    stdin().read_line(&mut some_string).unwrap();

    let numbers = some_string
        .trim()
        .split(' ')
        .map(|number| number.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    let sum: u32 = numbers.iter().sum();

    println!("Their sum is: {sum}");
}
