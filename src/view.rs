use wasm_bindgen::prelude::*;

use super::cell_value::Accumulator;
use super::config::Config;
use super::pivot_table::{PivotTable, SerializablePivotTable};
use super::table::Table;

#[wasm_bindgen]
pub struct View {
    columns: SerializablePivotTable,
    config: Config,

    pub num_rows: usize,
    pub num_columns: usize,
}

#[wasm_bindgen]
impl View {
    #[wasm_bindgen(skip)]
    pub fn new(table: &mut Table, config: &str) -> View {
        let config = Config::new(config.to_string());

        // determine how to accumulate each column
        let mut accumulators: Vec<Accumulator> = vec![];
        for (_, col) in table.columns.iter().enumerate() {
            match config.aggregates.get(col) {
                Some(agg) => accumulators.push(Accumulator::from_aggregate(agg)),
                None => accumulators.push(Accumulator::Noop),
            }
        }

        let ag_table = PivotTable::new(table, &config.row_pivots, &config.sort, &accumulators);

        return View {
            columns: ag_table.to_serializable(&config.columns),
            num_rows: table.size(),
            num_columns: table.columns.len(),
            config,
        };
    }

    pub fn to_columns(&self) -> JsValue {
        JsValue::from_serde(&self.columns).unwrap()
    }

    pub fn columns(&self) -> String {
        // wasm-bindgen cannot return vectors!
        // https://github.com/rustwasm/wasm-bindgen/issues/111
        let mut foo = self.config.columns.join(",");
        if self.config.row_pivots.len() > 0 {
            foo.insert_str(0, "__ROW_PATH__,");
        }
        return foo;
    }
}
