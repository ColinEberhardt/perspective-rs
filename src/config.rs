use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub sort: Vec<SortDescriptor>,
    pub columns: Vec<String>,
    pub aggregates: HashMap<String, Aggregate>,
    pub row_pivots: Vec<String>,
}

impl Config {
    pub fn new(config_string: String) -> Config {
        let config: Config = serde_json::from_str(config_string.as_str()).unwrap();
        return config;
    }
}

#[derive(Serialize, Deserialize)]
pub struct SortDescriptor {
    pub column: String,
    pub order: SortOrder,
}

impl Clone for SortDescriptor {
    fn clone(&self) -> SortDescriptor {
        SortDescriptor {
            column: self.column.clone(),
            order: self.order,
        }
    }
}

#[derive(Serialize, Deserialize, Copy, Clone)]
#[serde(rename_all = "camelCase")]
pub enum SortOrder {
    Desc,
    Asc,
    None,
}

#[derive(Serialize, Deserialize, Copy, Clone)]
#[serde(rename_all = "camelCase")]
pub enum Aggregate {
    Sum,
    Count,
    Low,
    High,
    Undefined,
}
