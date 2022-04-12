use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize, PartialEq, Eq, Hash)]
struct A {
    field: String,
}

#[derive(Serialize)]
struct B {
    items: String,
}

fn main() {
    let mut map: HashMap<A, Vec<B>> = HashMap::new();

    map.insert(
        A {
            field: "tom".to_string(),
        },
        vec![
            B {
                items: "jerry".to_string(),
            },
            B {
                items: "mouse".to_string(),
            },
            B {
                items: "cat".to_string(),
            },
        ],
    );

    map.insert(
        A {
            field: "batman".to_string(),
        },
        vec![
            B {
                items: "joker".to_string(),
            },
            B {
                items: "superman".to_string(),
            },
            B {
                items: "nam nam".to_string(),
            },
        ],
    );

    std::fs::write("some.yaml", &serde_yaml::to_string(&map).unwrap()).unwrap();
}
