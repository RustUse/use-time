# use-time

Composable primitive time utilities for Rust.

`use-time` is the time-specific RustUse set for small, composable helpers over
`std::time::Duration`. It focuses on unit conversion and simple decomposition
without introducing calendar systems, time zones, or scheduling frameworks.

## Scope

- duration construction
- seconds, minutes, hours, and milliseconds conversion
- safe checked unit multiplication
- simple time decomposition helpers

## Workspace crates

| Crate      | Purpose                                       |
| ---------- | --------------------------------------------- |
| `use-time` | Composable primitive time utilities for Rust. |

## Installation

Crates.io:

```toml
[dependencies]
use-time = "0.1"
```

Local workspace or path dependency:

```toml
[dependencies]
use-time = { path = "crates/use-time" }
```

## Examples

Convert minutes into seconds:

```rust
use use_time::prelude::*;

assert_eq!(minutes_to_seconds(5), Some(300));
```

Build a duration from milliseconds:

```rust
use use_time::prelude::*;

let duration = milliseconds_to_duration(1_250);

assert_eq!(duration.as_secs(), 1);
assert_eq!(duration.subsec_millis(), 250);
```

Split a second count into hours, minutes, and seconds:

```rust
use use_time::prelude::*;

assert_eq!(split_seconds(3_661), (1, 1, 1));
```

## Status

Early v0.1 API.
