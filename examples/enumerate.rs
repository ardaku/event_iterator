use event_iterator::EventIterator;

#[async_main::async_main]
async fn main(_spawner: async_main::LocalSpawner) {
    let ei = event_iterator::from_iter(['a', 'b', 'c']);
    let ei = ei.enumerate();

    assert_eq!(ei.next_unpinned().await, Some((0, 'a')));
    assert_eq!(ei.next_unpinned().await, Some((1, 'b')));
    assert_eq!(ei.next_unpinned().await, Some((2, 'c')));
    assert_eq!(ei.next_unpinned().await, None);
}
