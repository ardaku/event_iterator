use event_iterator::EventIterator;

#[async_main::async_main]
async fn main(_spawner: async_main::LocalSpawner) {
    let ei = event_iterator::from_iter([3, 1, 2]);

    assert_eq!(Some(3), ei.next_unpinned().await);
    assert_eq!(Some(1), ei.next_unpinned().await);
    assert_eq!(Some(2), ei.next_unpinned().await);
    assert_eq!(None, ei.next_unpinned().await);
}
