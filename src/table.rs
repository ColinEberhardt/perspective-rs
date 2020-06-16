use serde_json::Value;
use wasm_bindgen::prelude::*;

use super::cell_value::CellValue;
use super::utils::*;
use super::view::View;

// describes a tabular structure with columns (described by strings)
// and a two-dimensional array of data
#[wasm_bindgen]
pub struct Table {
    #[wasm_bindgen(skip)]
    pub data: Vec<Vec<CellValue>>,
    #[wasm_bindgen(skip)]
    pub columns: Vec<String>,
}

impl Table {
    pub fn index_for_column(&self, col_name: &String) -> usize {
        self.columns.iter().position(|y| y.eq(col_name)).unwrap()
    }
}

#[wasm_bindgen]
impl Table {
    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn columns(&self) -> String {
        // wasm-bindgen cannot return vectors!
        // https://github.com/rustwasm/wasm-bindgen/issues/111
        self.columns.join(",")
    }

    pub fn to_view(&mut self, config: &str) -> View {
        View::new(self, config)
    }

    #[wasm_bindgen(constructor)]
    pub fn new(json: JsValue) -> Result<Table, JsValue> {
        set_panic_hook();

        let json_value: Value = json
            .into_serde()
            .map_err(|_| JsValue::from("JSON parse error"))?;

        let array = json_value
            .as_array()
            .ok_or(JsValue::from("Data should be an array"))?;
        let rows = array.len();

        let first_row = array[0]
            .as_object()
            .ok_or(JsValue::from("The elements of the array should be objects"))?;

        let columns: Vec<String> = first_row.keys().map(|s| s.clone()).collect();

        let mut data: Vec<Vec<CellValue>> = vec![];
        for row_index in 0..rows {
            let row = columns
                .iter()
                .map(|col| &array[row_index][col])
                .map(|x| CellValue::new(x))
                .collect();
            data.push(row);
        }

        return Ok(Table { columns, data });
    }
}
