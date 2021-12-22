use jsonschema::JSONSchema;
use serde_json::json;

fn main() {
    // What schema should the JSON blob obey?
    let schema = json!({
        "properties": {
            "fruits": {
                "type": "string",
                "minLength": 5,
                "default": "apples"
            }
        },
        "required": ["fruits"]
    });
    // The JSON blob itself.
    let json_blob = json!({"fruits": "abc"});

    // Compile the JSON Schema. (this is a little like regex, where we compile _schema the regex rule, beforehand).
    let compiled_schema = JSONSchema::compile(&schema).expect("The JSON schema is invalid.");

    // Check the JSON blob against the JSON Schema.
    let result = compiled_schema.validate(&json_blob);
    if let Err(errors) = result {
        for e in errors {
            println!("{:?}", e.kind);
        }
    } else {
        println!("The JSON blob was okay");
    }
}
