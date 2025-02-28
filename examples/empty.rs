use event_iterator::{Empty, EventIterator};

#[async_main::async_main]
async fn main(_spawner: async_main::LocalSpawner) {
    let mut ei: Empty<u32> = event_iterator::empty();

    assert!(ei.next().await.is_none());
    assert!(ei.next().await.is_none());
}
