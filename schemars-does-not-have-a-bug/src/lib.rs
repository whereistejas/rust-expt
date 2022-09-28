#[cfg(test)]
mod tests {
    use jsonschema::JSONSchema;
    use serde_json::json;

    #[test]
    fn test_different_types() {
        let schema = json!({
            "$schema": "http://json-schema.org/draft-07/schema#",
            "title": "Illegal schema",
            "type": "object",
            "properties" : {
                "start_time": {
                    "title": "Start Time",
                    "type": [
                      "integer",
                      "string"
                    ],
                },
            }
        });

        let value_integer = json!({
            "start_time": 123,
        });
        let value_string = json!({
            "start_time": "123",
        });

        let compiled_schema = JSONSchema::compile(&schema).unwrap();

        let _ = compiled_schema
            .validate(&value_integer)
            .map_err(|err| {
                err.map(|err| err.to_string())
                    .collect::<Vec<_>>()
                    .join("\n")
            })
            .unwrap();

        let _ = compiled_schema
            .validate(&value_string)
            .map_err(|err| {
                err.map(|err| err.to_string())
                    .collect::<Vec<_>>()
                    .join("\n")
            })
            .unwrap();
    }
}
