# use-time

Composable primitive time utilities for Rust.

`use-time` provides small helpers over `std::time::Duration` for common unit
conversion and decomposition tasks.

## Examples

```rust
use use_time::prelude::*;

assert_eq!(minutes_to_seconds(15), Some(900));
assert_eq!(split_seconds(3_661), (1, 1, 1));
```
