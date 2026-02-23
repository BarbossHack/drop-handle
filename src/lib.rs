//! `DropHandle` is a handle that aborts the task when dropped.
//!
//! The task will only be aborted when the last `DropHandle` is dropped, so you can clone it to keep the task alive.
//! This is useful for tasks that should be automatically cleaned up when they are no longer needed, without having to manually call `abort()`.
//!
//! Example usage:
//! ```
//! use drop_handle::DropHandle;
//! use tokio::time::{sleep, Duration};
//!
//! #[tokio::main]
//! async fn main() {
//!     let drop_handle: DropHandle = tokio::spawn(async {
//!         loop {
//!             println!("Task is running...");
//!             sleep(Duration::from_secs(1)).await;
//!         }
//!     })
//!     .into();
//!     // The task will be automatically aborted when `drop_handle` goes out of scope.
//! }
//! ```

use std::{ops::Deref, sync::Arc};
use tokio::task::{AbortHandle, JoinHandle};
use tracing::{debug, trace};

#[cfg(test)]
mod tests;

/// A handle that aborts the task when dropped.
///
/// The task will only be aborted when the last `DropHandle` is dropped, so you can clone it to keep the task alive.
/// This is useful for tasks that should be automatically cleaned up when they are no longer needed, without having to manually call `abort()`.
///
/// Example usage:
/// ```
/// use drop_handle::DropHandle;
/// use tokio::time::{sleep, Duration};
///
/// #[tokio::main]
/// async fn main() {
///     let drop_handle: DropHandle = tokio::spawn(async {
///         loop {
///             println!("Task is running...");
///             sleep(Duration::from_secs(1)).await;
///         }
///     })
///     .into();
///     // The task will be automatically aborted when `drop_handle` goes out of scope.
/// }
/// ```
#[derive(Clone, Debug)]
pub struct DropHandle(Arc<AbortHandle>);

impl Deref for DropHandle {
    type Target = AbortHandle;

    fn deref(&self) -> &AbortHandle {
        &self.0
    }
}

impl From<AbortHandle> for DropHandle {
    fn from(value: AbortHandle) -> Self {
        debug!("create DropHandle for task {:?}", value.id());
        Self(Arc::new(value))
    }
}

impl<T> From<JoinHandle<T>> for DropHandle {
    fn from(value: JoinHandle<T>) -> Self {
        value.abort_handle().into()
    }
}

/// When the last `DropHandle` is dropped, the task will be aborted.
impl Drop for DropHandle {
    fn drop(&mut self) {
        let drop_counter = Arc::strong_count(&self.0);
        trace!("DropHandle counter: {}", drop_counter);
        if drop_counter <= 1 {
            debug!("drop DropHandle: abort task {:?}", self.0.id());
            self.abort();
        }
    }
}
