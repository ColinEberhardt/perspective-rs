use itertools::Itertools;
use std::iter;

use super::cell_value::{Accumulator, CellValue};
use super::row_aggregator::RowAggregator;
use super::table::Table;

pub struct PivotTable {
    rows: Vec<PivotTableRow>,
    total: Vec<CellValue>,
    columns: Vec<String>,
}

// represents an aggregate over a collection of rows, each sharing the same key
struct PivotTableRow {
    values: Vec<CellValue>,
    key: CellValue,
}

impl PivotTable {
    pub fn new(
        table: &mut Table,
        pivot_index: &usize,
        accumulators: &Vec<Accumulator>,
    ) -> PivotTable {
        // sort by aggregate value (iter-tools group by expects a sorted collection)
        table
            .data
            .sort_by(|a, b| a[*pivot_index].cmp(&b[*pivot_index]));

        let data = &table.data;
        let groups = data.into_iter().group_by(|a| a[*pivot_index].clone());

        // aggregate over each group
        let aggregate_table: Vec<PivotTableRow> = groups
            .into_iter()
            .map(|(key, group)| {
                let materialised = group.collect::<Vec<&Vec<CellValue>>>();
                let agg = materialised.iter().skip(1).fold(
                    RowAggregator::new(&materialised[0], &accumulators),
                    |acc, row| acc.accumulate(&row),
                );
                let values = agg.to_row();
                PivotTableRow { key, values }
            })
            .collect();

        // create the total for this group
        let total_acc: Vec<Accumulator> =
            accumulators.iter().map(|s| s.total_accumulator()).collect();
        let total = aggregate_table.iter().skip(1).fold(
            RowAggregator::new(&aggregate_table[0].values, &total_acc),
            |acc, group| acc.accumulate(&group.values),
        );

        PivotTable {
            total: total.to_row(),
            rows: aggregate_table,
            columns: table.columns.clone(),
        }
    }
    pub fn to_table(&self) -> Table {
        // create a table, appending the total as the first row
        Table {
            data: iter::once(self.total.clone())
                .chain(self.rows.iter().map(|g| g.values.clone()))
                .collect(),
            columns: self.columns.clone(),
        }
    }

    pub fn row_paths(&self) -> Vec<CellValue> {
        iter::once(CellValue::Null)
            .chain(self.rows.iter().map(|g| g.key.clone()))
            .collect()
    }
}
