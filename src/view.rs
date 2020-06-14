use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::iter;
use wasm_bindgen::prelude::*;

use super::cell_value::{Accumulator, CellValue};
use super::config::{Config, SortDescriptor, SortOrder};
use super::row_aggregator::RowAggregator;
use super::table::Table;

#[wasm_bindgen]
#[derive(Serialize)]
pub struct View {
    #[wasm_bindgen(skip)]
    pub columns: HashMap<String, Vec<CellValue>>,
    #[wasm_bindgen(skip)]
    pub config: Config,

    pub num_rows: usize,
    pub num_columns: usize,
}

struct IndexedSortDescriptor {
    pub index: usize,
    pub order: SortOrder,
}

fn compare(a: &Vec<CellValue>, b: &Vec<CellValue>, order: &Vec<IndexedSortDescriptor>) -> Ordering {
    for (_, desc) in order.iter().enumerate() {
        let col_index = desc.index;
        let order = match desc.order {
            SortOrder::Asc => a[col_index].cmp(&b[col_index]),
            SortOrder::Desc => b[col_index].cmp(&a[col_index]),
            SortOrder::None => Ordering::Equal,
        };

        match order {
            Ordering::Greater | Ordering::Less => return order,
            _ => {}
        }
    }
    return Ordering::Equal;
}

fn table_to_columns(include_cols: &Vec<String>, table: &Table) -> HashMap<String, Vec<CellValue>> {
    let mut map: HashMap<String, Vec<CellValue>> = HashMap::new();
    for (column_index, col) in table.columns.iter().enumerate() {
        if include_cols.iter().any(|i| i.eq(col)) {
            let mut col_data: Vec<CellValue> = Vec::new();
            for row_index in 0..table.data.len() {
                col_data.push(table.data[row_index][column_index].clone());
            }
            map.insert(col.to_string(), col_data);
        }
    }
    return map;
}

fn sort_table(table: &mut Table, sort: &Vec<SortDescriptor>) {
    // create sort descriptors with column indices
    let indexed_sort_descriptors: Vec<IndexedSortDescriptor> = sort
        .iter()
        .map(|x| IndexedSortDescriptor {
            order: x.order,
            // look up the index of this column
            index: table.index_for_column(&x.column),
        })
        .collect();

    // sort the table (a bit yuck, shouldn't be mutating)
    table
        .data
        .sort_by(|a, b| compare(&a, &b, &indexed_sort_descriptors));
}

// represents an aggregate over a collection of rows, each sharing the same key
struct TableGroupRow {
    values: Vec<CellValue>,
    key: CellValue,
}

struct TableGroup {
    groups: Vec<TableGroupRow>,
    total: Vec<CellValue>,
    columns: Vec<String>,
}

impl TableGroup {
    fn to_table(&self) -> Table {
        // create a table, appending the total as the first row
        Table {
            data: iter::once(self.total.clone())
                .chain(self.groups.iter().map(|g| g.values.clone()))
                .collect(),
            columns: self.columns.clone(),
        }
    }

    fn row_paths(&self) -> Vec<CellValue> {
        iter::once(CellValue::Null)
            .chain(self.groups.iter().map(|g| g.key.clone()))
            .collect()
    }
}

fn pivot_table(
    table: &mut Table,
    pivot_index: &usize,
    accumulators: &Vec<Accumulator>,
) -> TableGroup {
    // sort by aggregate value (iter-tools group by expects a sorted collection)
    table
        .data
        .sort_by(|a, b| a[*pivot_index].cmp(&b[*pivot_index]));

    let data = &table.data;
    let groups = data.into_iter().group_by(|a| a[*pivot_index].clone());

    // aggregate over each group
    let aggregate_table: Vec<TableGroupRow> = groups
        .into_iter()
        .map(|(key, group)| {
            let materialised = group.collect::<Vec<&Vec<CellValue>>>();
            let agg = materialised.iter().skip(1).fold(
                RowAggregator::new(&materialised[0], &accumulators),
                |acc, row| acc.accumulate(&row),
            );
            let values = agg.to_row();
            TableGroupRow { key, values }
        })
        .collect();

    // create the total for this group
    let total_acc: Vec<Accumulator> = accumulators.iter().map(|s| s.total_accumulator()).collect();
    let total = aggregate_table.iter().skip(1).fold(
        RowAggregator::new(&aggregate_table[0].values, &total_acc),
        |acc, group| acc.accumulate(&group.values),
    );

    TableGroup {
        total: total.to_row(),
        groups: aggregate_table,
        columns: table.columns.clone(),
    }
}

#[wasm_bindgen]
impl View {
    #[wasm_bindgen(skip)]
    pub fn new(table: &mut Table, config: &str) -> View {
        let config = Config::new(config.to_string());

        if config.row_pivots.len() > 0 {
            // TODO support multiple row pivots
            let pivot_column = &config.row_pivots[0];
            let pivot_index = table.index_for_column(&pivot_column);

            // determine how to accumulate each column
            let mut accumulators: Vec<Accumulator> = vec![];
            for (_, col) in table.columns.iter().enumerate() {
                match config.aggregates.get(col) {
                    Some(agg) => accumulators.push(Accumulator::from_aggregate(agg)),
                    None => accumulators.push(Accumulator::Noop),
                }
            }

            let ag_table = pivot_table(table, &pivot_index, &accumulators);

            // sort_table(&mut ag_table, &config.sort);

            let mut columns = table_to_columns(&config.columns, &ag_table.to_table());
            columns.insert("__ROW_PATH__".to_string(), ag_table.row_paths());

            return View {
                columns,
                num_rows: table.size(),
                num_columns: table.columns.len(),
                config,
            };
        }

        sort_table(table, &config.sort);
        let columns = table_to_columns(&config.columns, &table);

        return View {
            columns,
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
