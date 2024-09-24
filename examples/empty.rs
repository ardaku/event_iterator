use event_iterator::{Empty, EventIterator};

#[async_main::async_main]
async fn main(_spawner: async_main::LocalSpawner) {
    let ei: Empty<u32> = event_iterator::empty();

    assert!(ei.next_unpinned().await.is_none());
    assert!(ei.next_unpinned().await.is_none());
}
