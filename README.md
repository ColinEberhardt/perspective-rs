# perspective-rs

A prototype re-implementation of [Perspective](https://perspective.finos.org/) in Rust. The size is approx half the C++ implementation (although I haven't tried optimising yet), and performance is approx x3 faster.

## Progress

- [x] multi-row pivot 
- [x] multi-row sort (although doesn't work fully with pivot + sort)
- [x] filters (although not all criteria are implemented)
- Data types
  - [x] integers
  - [x] string
  - [x] bool
  - [ ] date / time
- [ ] accumulators (sum + count are implemented but not the others)
- [ ] web worker
- [ ] data updates
- [ ] column split
- [ ] unit tests!
- [ ] synthetic columns
- [ ] editing suport