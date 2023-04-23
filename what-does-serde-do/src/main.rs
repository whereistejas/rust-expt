use std::collections::HashMap;

fn main() {
    let mut object = HashMap::new();

    object.insert("value1", vec!["a", "b", "c"]);
    object.insert("value2", vec!["d", "e", "f"]);
    object.insert("value3", vec!["g", "h", "i"]);

    let value = serde_json::to_value(object).unwrap();

    println!("Value: {:?}", value.to_string())
}
