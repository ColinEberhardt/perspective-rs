use serde::de::{self, Deserialize, Deserializer, Visitor};
use serde::ser::{Serialize, Serializer};
use serde_json::Value;
use std::cmp::{max, min, Eq, Ordering};
use std::fmt;

use super::config::Operation;
use super::accumulator::Accumulator;

#[derive(Eq, PartialEq, Clone)]
pub enum CellValue {
    Integer(i64),
    Str(String),
    Bool(bool),
    Null,
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

    pub fn matches(&self, operation: &Operation, value: &CellValue) -> bool {
        use CellValue::*;
        use Operation::*;
        match (operation, value, self) {
            (EqEq, Str(a), Str(b)) => a.eq(b),
            (EqEq, Integer(a), Integer(b)) => a == b,
            (EqEq, Bool(a), Bool(b)) => a == b,
            _ => false,
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

pub struct CellValueVisitor;

impl<'de> Visitor<'de> for CellValueVisitor {
    type Value = CellValue;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("not what we wanted")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(CellValue::Str(value.to_string()))
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(CellValue::Integer(v))
    }

    fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.visit_i64(v as i64)
    }

    fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.visit_i64(v as i64)
    }

    fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.visit_i64(v as i64)
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.visit_i64(v as i64)
    }

    fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.visit_i64(v as i64)
    }

    fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.visit_i64(v as i64)
    }

    fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.visit_i64(v as i64)
    }
}

impl<'de> Deserialize<'de> for CellValue {
    fn deserialize<D>(deserializer: D) -> Result<CellValue, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(CellValueVisitor)
    }
}
