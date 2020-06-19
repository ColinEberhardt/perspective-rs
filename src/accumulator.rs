use super::config::Aggregate;

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
            Aggregate::Undefined => Accumulator::Noop,
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
