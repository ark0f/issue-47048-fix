# [Issue #47048](https://github.com/rust-lang/rust/issues/47048) fix
![License](https://img.shields.io/crates/l/issue_47048_fix.svg)
[![crates.io](https://img.shields.io/crates/v/issue-47048-fix.svg)](https://crates.io/crates/issue-47048-fix)
[![API docs](https://docs.rs/issue-47048-fix/badge.svg?version=0.1.2)](https://docs.rs/issue-47048-fix/0.1.2)

Thanks to [Trevor Spiteri](https://github.com/tspiteri)

# Quick start
`Cargo.toml`:
```toml
[build-dependencies]
issue-47048-fix = "0.1"
```

`build.rs`:
```rust
use issue_47048_fix::issue_47048_fix;

fn main() {
    /* your build code */

    issue_47048_fix();
}
```

# License
`issue-47048-fix` under either of:

* [Apache License 2.0](https://github.com/ark0f/issue-47048-fix/blob/master/LICENSE-APACHE.md)
* [MIT](https://github.com/ark0f/issue-47048-fix/blob/master/LICENSE-MIT.md)

at your option.
