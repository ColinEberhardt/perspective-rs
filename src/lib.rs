mod accumulator;
mod cell_value;
mod config;
mod macros;
mod pivot_table;
mod row_aggregator;
mod table;
mod utils;
mod view;

#[macro_use]
extern crate serde_derive;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
