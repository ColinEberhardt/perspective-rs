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
}

impl RowAggregator {
    pub fn new(source: &Vec<CellValue>, accumulators: &Vec<Accumulator>) -> RowAggregator {
        let row = source
            .iter()
            .zip(accumulators.iter())
            .map(|(cell, acc)| CellAccumulator {
                value: cell.seed_value(acc),
                accumulator: *acc,
            })
            .collect();
        RowAggregator { row }
    }

    pub fn accumulate(&self, values: &Vec<CellValue>) -> RowAggregator {
        let mut row: Vec<CellAccumulator> = vec![];
        // TODO - try zip
        for (i, new_cell) in values.iter().enumerate() {
            row.push(self.row[i].accumulate(&new_cell));
        }
        RowAggregator { row }
    }

    pub fn to_row(&self) -> Vec<CellValue> {
        self.row.iter().map(|x| x.value.clone()).collect()
    }
}
