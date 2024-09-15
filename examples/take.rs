use event_iterator::EventIterator;

#[async_main::async_main]
async fn main(_spawner: async_main::LocalSpawner) {
    let ei = event_iterator::from_iter([1, 2, 3]).take(2);

    assert_eq!(ei.next_unpinned().await, Some(1));
    assert_eq!(ei.next_unpinned().await, Some(2));
    assert_eq!(ei.next_unpinned().await, None);
    assert_eq!(ei.next_unpinned().await, None);

    // `take()` is often used with an infinite iterator, to make it finite:

    let ei = event_iterator::from_iter(0..).take(3);

    assert_eq!(ei.next_unpinned().await, Some(0));
    assert_eq!(ei.next_unpinned().await, Some(1));
    assert_eq!(ei.next_unpinned().await, Some(2));
    assert_eq!(ei.next_unpinned().await, None);
    assert_eq!(ei.next_unpinned().await, None);

    // If less than `n` elements are available, `take()` will limit itself to
    // the size of the underlying iterator:

    let ei = event_iterator::from_iter([1, 2]).take(5);

    assert_eq!(ei.next_unpinned().await, Some(1));
    assert_eq!(ei.next_unpinned().await, Some(2));
    assert_eq!(ei.next_unpinned().await, None);
    assert_eq!(ei.next_unpinned().await, None);
}
