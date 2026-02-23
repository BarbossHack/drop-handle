# **DropHandle**

**A handle that will abort a Tokio task when dropped.**

The task will only be aborted when the last `DropHandle` is dropped, so you can clone it to keep the task alive.

This is useful for tasks that should be automatically cleaned up when they are no longer needed, without having to manually call `abort()`.

[![Crates.io][crates-badge]][crates-url]
[![MIT licensed][mit-badge]][mit-url]
[![Build Status][actions-badge]][actions-url]

[crates-badge]: https://img.shields.io/crates/v/drop-handle.svg
[crates-url]: https://crates.io/crates/drop-handle
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/BarbossHack/drop-handle/blob/master/LICENSE
[actions-badge]: https://github.com/BarbossHack/drop-handle/workflows/CI/badge.svg
[actions-url]: https://github.com/BarbossHack/drop-handle/actions?query=workflow%3ACI+branch%3Amaster

## Example

```rust
use drop_handle::DropHandle;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let drop_handle: DropHandle = tokio::spawn(async {
        loop {
            println!("Task is running...");
            sleep(Duration::from_secs(1)).await;
        }
    })
    .into();
    // The task will be automatically aborted when `drop_handle` goes out of scope.
}
```
