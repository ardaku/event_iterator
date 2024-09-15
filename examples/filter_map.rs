use event_iterator::EventIterator;

#[async_main::async_main]
async fn main(_spawner: async_main::LocalSpawner) {
    let ei = event_iterator::from_iter(["1", "two", "NaN", "four", "5"]);
    let ei = ei.filter_map(|s| s.parse().ok());

    assert_eq!(ei.next_unpinned().await, Some(1));
    assert_eq!(ei.next_unpinned().await, Some(5));
    assert_eq!(ei.next_unpinned().await, None);

    // Here’s the same example, but with `filter()` and `map()`:

    let ei = event_iterator::from_iter(["1", "two", "NaN", "four", "5"]);
    let ei = ei
        .map(|s| s.parse())
        .filter(|s| s.is_ok())
        .map(|s| s.unwrap());

    assert_eq!(ei.next_unpinned().await, Some(1));
    assert_eq!(ei.next_unpinned().await, Some(5));
    assert_eq!(ei.next_unpinned().await, None);
}
