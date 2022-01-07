mod types {
    use schemars::schema::SchemaObject;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct ConnectorSpecification {
        pub documentation_url: Option<String>,
        pub changelog_url: Option<String>,
        pub connection_specification: ConnectionSpecification,
        pub supports_incremental: Option<bool>,
        #[serde(default)]
        pub supports_normalization: bool,
        #[serde(default)]
        #[serde(rename = "supportsDBT")]
        pub supports_dbt: bool,
        #[serde(rename = "supported_destination_sync_modes")]
        pub supported_destination_sync_modes: Option<Vec<DestinationSyncMode>>,
        pub auth_specification: Option<AuthSpecification>,
    }

    #[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
    #[serde(transparent)]
    pub struct ConnectionSpecification(pub SchemaObject);

    impl Eq for ConnectionSpecification {}

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "snake_case")]
    pub enum DestinationSyncMode {
        Append,
        Overwrite,
        // UpsertDedup, (watch for this as it's due to be added)
        AppendDedup,
    }

    impl Default for DestinationSyncMode {
        fn default() -> Self {
            DestinationSyncMode::Append
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct AuthSpecification {
        pub auth_type: Option<AuthType>,
        #[serde(rename = "oauth2Specification")]
        pub oauth2_specification: OAuth2Specification,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum AuthType {
        #[serde(rename = "oauth2.0")]
        OAuth2,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct OAuth2Specification {
        pub oauth_flow_init_parameters: Option<Vec<Vec<String>>>,
    }
}

fn main() {
    let file = std::fs::File::open("spec.json").unwrap();
    let json: serde_json::Value = serde_json::from_reader(file).unwrap();
    let schema = serde_json::from_value::<types::ConnectorSpecification>(json)
        .unwrap()
        .connection_specification
        .0;

    println!("{:#?}", schema.instance_type);
}
