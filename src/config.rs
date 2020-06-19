use std::collections::HashMap;

use super::cell_value::CellValue;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub sort: Vec<SortDescriptor>,
    pub columns: Vec<String>,
    pub filter: Vec<FilterDescriptor>,
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

#[derive(Serialize, Deserialize)]
pub struct FilterDescriptor {
    pub column: String,
    pub operation: Operation,
    pub value: CellValue,
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
#[serde(rename_all = "lowercase")]
pub enum Operation {
    EqEq,
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
