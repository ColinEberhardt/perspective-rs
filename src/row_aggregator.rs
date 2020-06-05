use std::cmp::Ordering;

use super::cell_value::{Accumulator, CellValue};

struct CellAccumulator {
    value: CellValue,
    accumulator: Accumulator,
}

impl CellAccumulator {
    fn accumulate(&self, acc: &CellValue) -> CellAccumulator {
        CellAccumulator {
            value: self.value.accumulate(acc, &self.accumulator),
            accumulator: self.accumulator.clone(),
        }
    }
}

pub struct RowAggregator {
    row: Vec<CellAccumulator>,
    pivot_index: usize,
}

impl RowAggregator {
    pub fn new(
        source: &Vec<CellValue>,
        pivot_index: &usize,
        accumulators: &Vec<Accumulator>,
    ) -> RowAggregator {
        let row = source
            .iter()
            .zip(accumulators.iter())
            .map(|(cell, acc)| CellAccumulator {
                value: cell.seed_value(acc),
                accumulator: *acc,
            })
            .collect();
        RowAggregator {
            row,
            pivot_index: *pivot_index,
        }
    }

    pub fn accumulate(&self, values: &Vec<CellValue>) -> RowAggregator {
        let mut row: Vec<CellAccumulator> = vec![];
        // TODO - try zip
        for (i, new_cell) in values.iter().enumerate() {
            row.push(self.row[i].accumulate(&new_cell));
        }
        RowAggregator {
            row,
            pivot_index: self.pivot_index,
        }
    }

    pub fn includes_row(&self, row: &Vec<CellValue>) -> bool {
        self.row[self.pivot_index].value.cmp(&row[self.pivot_index]) == Ordering::Equal
    }

    pub fn to_row(&self) -> Vec<CellValue> {
        self.row.iter().map(|x| x.value.clone()).collect()
    }
}
