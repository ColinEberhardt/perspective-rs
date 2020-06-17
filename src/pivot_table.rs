use itertools::Itertools;
use std::cmp::Eq;
use std::cmp::Ordering;
use std::collections::HashMap;

use super::cell_value::{Accumulator, CellValue};
use super::config::{SortDescriptor, SortOrder};
use super::row_aggregator::RowAggregator;
use super::table::Table;

pub struct PivotTable {
    rows: Vec<PivotTableRow>,
    columns: Vec<String>,
}

// represents an aggregate over a collection of rows, each sharing the same key
struct PivotTableRow {
    values: Vec<CellValue>,
    key: RowKey,
}

struct IndexedPivotTableRow {
    row: PivotTableRow,
    index: usize,
}

#[derive(Clone, Serialize)]
struct RowKey {
    values: Vec<CellValue>,
}

// a format which is appropriate for serializing to the client
#[derive(Serialize)]
pub struct SerializablePivotTable<'a> {
    columns: HashMap<String, Vec<&'a CellValue>>,
    row_paths: Vec<RowKey>,
}

impl RowKey {
    fn new(row: &Vec<CellValue>, indices: &Vec<usize>) -> RowKey {
        RowKey {
            values: indices.iter().map(|i| row[*i].clone()).collect(),
        }
    }

    fn empty() -> RowKey {
        RowKey { values: vec![] }
    }

    fn depth(&self) -> usize {
        self.values.len()
    }

    fn eq_depth(&self, other: &Self, depth: &usize) -> bool {
        if self.values.len() != other.values.len() {
            return false;
        }
        if self.values.len() == 0 {
            // empty row keys are always considered unequal
            return false;
        }
        let items: Vec<_> = self
            .values
            .iter()
            .take(*depth)
            .zip(other.values.iter())
            .filter(|(a, b)| a != b)
            .collect();
        return items.len() == 0;
    }

    fn clone_depth(&self, depth: &usize) -> RowKey {
        RowKey {
            values: self.values.iter().take(*depth).map(|x| x.clone()).collect(),
        }
    }
}

impl PartialEq for RowKey {
    fn eq(&self, other: &Self) -> bool {
        self.eq_depth(other, &self.values.len())
    }
}

impl Eq for RowKey {}

struct KeyedRow<'a> {
    values: &'a Vec<CellValue>,
    key: RowKey,
}

struct IndexedSortDescriptor {
    index: usize,
    order: SortOrder,
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

fn aggregate_rows(
    table: &Table,
    row_pivots: &Vec<String>,
    accumulators: &Vec<Accumulator>,
) -> Vec<PivotTableRow> {
    // TODO - if there are not any row pivots, do we short-circuit some of this code

    let data = &table.data;

    // convert row pivot columns into indices
    let row_pivot_indices: Vec<usize> = row_pivots
        .iter()
        .map(|s| table.index_for_column(s))
        .collect();

    // key each columns
    let keyed_rows: Vec<KeyedRow> = data
        .iter()
        .map(|r| KeyedRow {
            key: RowKey::new(r, &row_pivot_indices),
            values: r,
        })
        .collect();

    let groups = keyed_rows.into_iter().group_by(|a| a.key.clone());

    // aggregate over each group
    groups
        .into_iter()
        .map(|(key, group)| {
            // compute the aggregate for this group
            let materialised = group.collect::<Vec<KeyedRow>>();
            let agg = materialised.iter().skip(1).fold(
                RowAggregator::new(materialised[0].values, &accumulators),
                |acc, row| acc.accumulate(&row.values),
            );
            let values = agg.to_row();
            // map to a pivot row
            PivotTableRow { key, values }
        })
        .collect()
}

fn aggregate_totals(
    aggregate_table: &mut Vec<PivotTableRow>,
    depth: &usize,
    accumulators: &Vec<Accumulator>,
) {
    let mut totals: Vec<IndexedPivotTableRow> = vec![];
    let mut current_key = aggregate_table[0].key.clone();
    let mut acc = RowAggregator::new(&aggregate_table[0].values, &accumulators);
    let mut start_index = 0;
    for (i, row) in aggregate_table.iter().skip(1).enumerate() {
        if row.key.depth() - 1 == *depth {
            if row.key.eq_depth(&current_key, depth) {
                acc = acc.accumulate(&row.values);
            } else {
                totals.push(IndexedPivotTableRow {
                    index: start_index,
                    row: PivotTableRow {
                        values: acc.to_row(),
                        key: current_key.clone_depth(&depth),
                    },
                });
                start_index = i;
                acc = RowAggregator::new(&row.values, &accumulators);
                current_key = row.key.clone();
            }
        }
    }
    totals.push(IndexedPivotTableRow {
        index: start_index,
        row: PivotTableRow {
            values: acc.to_row(),
            key: current_key.clone().clone_depth(&depth),
        },
    });

    while totals.len() > 0 {
        let item = totals.remove(totals.len() - 1);
        aggregate_table.insert(item.index, item.row);
    }
}

fn sort_for_pivot(row_pivots: &Vec<String>, sort: &Vec<SortDescriptor>) -> Vec<SortDescriptor> {
    row_pivots
        .iter()
        // create sort descriptors for each pivot
        .map(|column| SortDescriptor {
            column: column.clone(),
            // use the sort order from the sort descriptors if present
            order: match sort.iter().find(|x| x.column.eq(column)) {
                Some(sort_desc) => sort_desc.order,
                None => SortOrder::Asc,
            },
        })
        // combine with the sort descriptors
        .chain(sort.iter().cloned())
        .collect::<Vec<SortDescriptor>>()
}

impl PivotTable {
    pub fn new(
        table: &mut Table,
        row_pivots: &Vec<String>,
        sort: &Vec<SortDescriptor>,
        accumulators: &Vec<Accumulator>,
    ) -> PivotTable {
        if row_pivots.len() > 0 {
            // sort based on pivot information
            let pivot_sort = sort_for_pivot(row_pivots, sort);
            sort_table(table, &pivot_sort);

            // aggregate over the 'raw' rows to create totals
            let mut aggregate_table = aggregate_rows(table, &row_pivots, &accumulators);

            let total_accumulators: Vec<Accumulator> =
                accumulators.iter().map(|s| s.total_accumulator()).collect();

            // add the totals for each level of pivot
            for d in (0..row_pivots.len()).rev() {
                aggregate_totals(&mut aggregate_table, &d, &total_accumulators);
            }

            PivotTable {
                rows: aggregate_table,
                columns: table.columns.clone(),
            }
        } else {
            // when there are no pivots present - we short cut some of the logic above

            // sort
            sort_table(table, &sort);

            // map to the pivot structure
            let data = &table.data;
            let rows = data
                .iter()
                .map(|s| PivotTableRow {
                    key: RowKey::empty(),
                    values: s.clone(),
                })
                .collect();

            PivotTable {
                rows,
                columns: table.columns.clone(),
            }
        }
    }

    pub fn to_serializable(&self, columns: &Vec<String>) -> SerializablePivotTable {
        let mut map: HashMap<String, Vec<&CellValue>> = HashMap::new();
        for (column_index, col) in self.columns.iter().enumerate() {
            if columns.iter().any(|i| i.eq(col)) {
                let mut col_data: Vec<&CellValue> = Vec::new();
                for row_index in 0..self.rows.len() {
                    col_data.push(&self.rows[row_index].values[column_index]);
                }
                map.insert(col.to_string(), col_data);
            }
        }

        let row_paths = self.rows.iter().map(|s| s.key.clone()).collect();

        SerializablePivotTable {
            columns: map,
            row_paths,
        }
    }
}
