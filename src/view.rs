use wasm_bindgen::prelude::*;

use super::accumulator::Accumulator;
use super::config::{Config, FilterDescriptor};
use super::pivot_table::PivotTable;
use super::table::Table;

#[wasm_bindgen]
pub struct View {
    pivot_table: PivotTable,
    config: Config,

    pub num_rows: usize,
    pub num_columns: usize,
}

#[derive(Serialize, Deserialize)]
pub struct ViewOptions {
    pub end_col: usize,
    pub end_row: usize,
    pub start_col: usize,
    pub start_row: usize,
}

impl ViewOptions {
    pub fn new(options_string: String) -> ViewOptions {
        let options: ViewOptions = serde_json::from_str(options_string.as_str()).unwrap();
        return options;
    }
}

#[wasm_bindgen]
impl View {
    #[wasm_bindgen(skip)]
    pub fn new(table: &Table, config: &str) -> View {
        let config = Config::new(config.to_string());

        // create tuples with column indices alongside filters
        let keyed_filters = config
            .filter
            .iter()
            .map(|s| table.index_for_column(&s.column))
            .zip(config.filter.iter())
            .collect::<Vec<(usize, &FilterDescriptor)>>();

        let mut filtered_table = Table {
            data: table
                .data
                .iter()
                .filter(|row| {
                    keyed_filters.iter().all(|(col_index, filter)| {
                        row[*col_index].matches(&filter.operation, &filter.value)
                    })
                })
                .cloned()
                .collect(),
            // TODO - do we need to keep copying these values
            columns: table.columns.iter().cloned().collect(),
        };

        // determine how to accumulate each column
        let mut accumulators: Vec<Accumulator> = vec![];
        for (_, col) in table.columns.iter().enumerate() {
            match config.aggregates.get(col) {
                Some(agg) => accumulators.push(Accumulator::from_aggregate(agg)),
                None => accumulators.push(Accumulator::Noop),
            }
        }

        let pivot_table = PivotTable::new(
            &mut filtered_table,
            &config.row_pivots,
            &config.sort,
            &accumulators,
        );

        return View {
            pivot_table,
            num_rows: table.size(),
            num_columns: table.columns.len(),
            config,
        };
    }

    pub fn to_columns(&self, options: &str) -> JsValue {
        let options = ViewOptions::new(options.to_string());
        JsValue::from_serde(
            &self
                .pivot_table
                .to_serializable_columns(&self.config.columns, &options),
        )
        .unwrap()
    }

    pub fn to_json(&self) -> JsValue {
        JsValue::from_serde(&self.pivot_table.to_serializable_rows()).unwrap()
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
