use event_iterator::{EventIterator, Pending};

#[async_main::async_main]
async fn main(_spawner: async_main::LocalSpawner) {
    let mut ei: Pending<u32> = event_iterator::pending();

    assert!(futures::poll!(ei.next()).is_pending());
    assert!(futures::poll!(ei.next()).is_pending());
}
