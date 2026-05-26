use std::sync::atomic::{AtomicUsize, Ordering};

// Initialize a static counter.
// 'static' ensures it lives for the entire program duration.
static UNIQUE_ID_COUNTER: AtomicUsize = AtomicUsize::new(1);

pub fn get_next_id() -> usize {
    // fetch_add returns the previous value, then increments.
    // Ordering::Relaxed is sufficient for simple counters where
    // you don't need to synchronize other memory operations.
    UNIQUE_ID_COUNTER.fetch_add(1, Ordering::Relaxed)
}
