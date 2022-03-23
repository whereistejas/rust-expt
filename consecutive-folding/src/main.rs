use itertools::Itertools;

#[derive(Debug)]
enum Type {
    A(String),
    B(String),
}

fn main() {
    let one = "1".to_string();
    let two = "2".to_string();

    let list = vec![
        Type::A(one.clone()),
        Type::A(one.clone()),
        Type::A(one.clone()),
        Type::B(two.clone()),
        Type::B(two.clone()),
        Type::B(two.clone()),
        Type::A(one),
        Type::B(two),
    ];

    let list = list
        .into_iter()
        .coalesce(|x, y| match (x, y) {
            (Type::A(a1), Type::A(a2)) => Ok(Type::A(format!("{a1}{a2}"))),
            (Type::B(b1), Type::B(b2)) => Ok(Type::B(format!("{b1}{b2}"))),
            (a @ Type::A(..), b @ Type::B(..)) => Err((a, b)),
            (b @ Type::B(..), a @ Type::A(..)) => Err((b, a)),
        })
        .collect::<Vec<Type>>();

    println!("{list:?}")
}

// without iterators
fn main_ver1() {
    let one = "1".to_string();
    let two = "2".to_string();

    let list = vec![
        Type::A(one.clone()),
        Type::A(one.clone()),
        Type::A(one.clone()),
        Type::B(two.clone()),
        Type::B(two.clone()),
        Type::B(two.clone()),
        Type::A(one),
        Type::B(two),
    ];

    let mut prev_item = match list.first().unwrap() {
        Type::A(_) => "a",
        Type::B(_) => "b",
    };

    let mut agg = "".to_string();
    let mut agg_list: Vec<Type> = Vec::new();

    for item in list.iter() {
        match item {
            Type::A(a) => {
                if prev_item == "a" {
                    agg = format!("{agg}{a}");
                } else if prev_item == "b" {
                    prev_item = "a";
                    agg_list.push(Type::B(agg.clone()));
                    agg.clear();
                    agg = a.to_string();
                }
            }
            Type::B(b) => {
                if prev_item == "b" {
                    agg = format!("{agg}{b}");
                } else if prev_item == "a" {
                    prev_item = "b";
                    agg_list.push(Type::A(agg.clone()));
                    agg.clear();
                    agg = b.to_string();
                }
            }
        }
    }

    println!("{agg_list:?}")
    // result: [A("111"), B("222"), A("1")]
}
