use std::future;

use event_iterator::EventIterator;

#[async_main::async_main]
async fn main(_spawner: async_main::LocalSpawner) {
    let ei = event_iterator::once(future::ready(()));

    assert!(ei.next_unpinned().await.is_some());
    assert!(ei.next_unpinned().await.is_none());
    assert!(ei.next_unpinned().await.is_none());
}
