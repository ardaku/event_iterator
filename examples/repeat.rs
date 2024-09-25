use event_iterator::EventIterator;

#[async_main::async_main]
async fn main(_spawner: async_main::LocalSpawner) {
    let ei = event_iterator::repeat("event").take(3);

    assert_eq!(ei.next_unpinned().await, Some("event"));
    assert_eq!(ei.next_unpinned().await, Some("event"));
    assert_eq!(ei.next_unpinned().await, Some("event"));
    assert!(ei.next_unpinned().await.is_none());
    assert!(ei.next_unpinned().await.is_none());
}
