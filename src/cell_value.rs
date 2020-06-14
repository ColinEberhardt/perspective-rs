use serde::ser::{Serialize, Serializer};
use serde_json::Value;
use std::cmp::{max, min, Eq, Ordering};

use super::config::Aggregate;

#[derive(Eq, PartialEq, Clone)]
pub enum CellValue {
    Integer(i64),
    Str(String),
    Bool(bool),
    Null,
}

#[derive(Clone, Copy)]
pub enum Accumulator {
    Sum,
    Noop,
    Count,
    Low,
    High,
}

impl Accumulator {
    pub fn from_aggregate(agg: &Aggregate) -> Accumulator {
        match agg {
            Aggregate::Sum => Accumulator::Sum,
            Aggregate::Count => Accumulator::Count,
            Aggregate::Low => Accumulator::Low,
            Aggregate::High => Accumulator::High,
        }
    }

    pub fn total_accumulator(&self) -> Accumulator {
        match self {
            Accumulator::Sum => Accumulator::Sum,
            Accumulator::Count => Accumulator::Sum,
            Accumulator::Low => Accumulator::Low,
            Accumulator::High => Accumulator::High,
            Accumulator::Noop => Accumulator::Noop,
        }
    }
}

impl CellValue {
    pub fn new(serde_value: &Value) -> CellValue {
        use CellValue::*;
        match serde_value {
            Value::Bool(value) => Bool(*value),
            Value::Number(value) => Integer(value.as_i64().unwrap()),
            Value::String(value) => Str(value.clone()),
            _ => Null,
        }
    }

    pub fn seed_value(&self, accumulator: &Accumulator) -> CellValue {
        use Accumulator::*;
        use CellValue::*;
        match (accumulator, self) {
            (High, Integer(value)) => Integer(*value),
            (High, _) => Null,
            (Low, Integer(value)) => Integer(*value),
            (Low, _) => Null,
            (Count, _) => Integer(1),
            (Sum, Integer(value)) => Integer(*value),
            (Sum, _) => Null,
            (Noop, Bool(value)) => Bool(*value),
            (Noop, Integer(value)) => Integer(*value),
            (Noop, Str(value)) => Str(value.clone()),
            (Noop, Null) => Null,
            _ => Null,
        }
    }

    pub fn accumulate(&self, other: &Self, operation: &Accumulator) -> CellValue {
        use Accumulator::*;
        use CellValue::*;
        match (operation, self, other) {
            (High, Integer(a), Integer(b)) => Integer(max(*a, *b)),
            (High, Integer(a), _) => Integer(*a),
            (High, _, _) => Null,
            (Low, Integer(a), Integer(b)) => Integer(min(*a, *b)),
            (Low, Integer(a), _) => Integer(*a),
            (Low, _, _) => Null,
            (Count, Integer(a), Null) => Integer(*a),
            (Count, Integer(a), _) => Integer(a + 1),
            (Sum, Integer(a), Integer(b)) => Integer(a + b),
            (Sum, _, _) => Null,
            (Noop, _, _) => self.clone(),
            _ => Null,
        }
    }
}

impl PartialOrd for CellValue {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for CellValue {
    fn cmp(&self, other: &Self) -> Ordering {
        use CellValue::*;
        match (self, other) {
            (Integer(a), Integer(b)) => a.cmp(b),
            (Str(a), Str(b)) => a.cmp(b),
            (Bool(a), Bool(b)) => a.cmp(b),
            _ => Ordering::Greater,
        }
    }
}

impl Serialize for CellValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            CellValue::Integer(value) => serializer.serialize_i64(*value),
            CellValue::Str(value) => serializer.serialize_str(value.as_str()),
            CellValue::Bool(value) => serializer.serialize_bool(*value),
            CellValue::Null => serializer.serialize_none(),
        }
    }
}
