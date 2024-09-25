use event_iterator::EventIterator;

#[async_main::async_main]
async fn main(_spawner: async_main::LocalSpawner) {
    let mut counter = 0;
    let ei = event_iterator::ready(|| {
        counter += 1;
        "event".repeat(counter)
    })
    .take(3);

    assert_eq!(ei.next_unpinned().await.as_deref(), Some("event"));
    assert_eq!(ei.next_unpinned().await.as_deref(), Some("eventevent"));
    assert_eq!(ei.next_unpinned().await.as_deref(), Some("eventeventevent"));
    assert!(ei.next_unpinned().await.is_none());
    assert!(ei.next_unpinned().await.is_none());
}
