use crate::DropHandle;
use std::{sync::Arc, time::Duration};
use tracing::Level;

#[tokio::test]
async fn test_drop_handle() {
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .with_line_number(true)
        .finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();

    // Create an `Arc` to track the number of references to the task
    let arc_counter = Arc::new(String::from("counter"));
    assert_eq!(Arc::strong_count(&arc_counter), 1);

    let arc_counter_clone = arc_counter.clone();
    let drop_handle: DropHandle = tokio::spawn(async move {
        loop {
            // Just use the counter to ensure the task stays alive and isn't optimized away by the compiler
            tokio::time::sleep(Duration::from_millis(
                Arc::strong_count(&arc_counter_clone) as u64 * 100,
            ))
            .await;
        }
    })
    .into();

    assert!(!drop_handle.is_finished());
    assert_eq!(Arc::strong_count(&arc_counter), 2);

    // Drop the handle, but clone it first
    drop(drop_handle.clone());
    tokio::time::sleep(Duration::from_millis(200)).await;

    // Should not be `finished` because there are still references to this `drop_handle` since we cloned it
    assert!(!drop_handle.is_finished());
    // ... so there should still be 2 counters left
    assert_eq!(Arc::strong_count(&arc_counter), 2);

    // Now drop the last handle tracked by the `Arc`; the task should then be aborted.
    drop(drop_handle);
    tokio::time::sleep(Duration::from_millis(200)).await;
    // ... so there should be only 1 counter left (this one). This attests that the task was successfully terminated.
    assert_eq!(Arc::strong_count(&arc_counter), 1);
}
