use event_iterator::EventIterator;

#[async_main::async_main]
async fn main(_spawner: async_main::LocalSpawner) {
    let mut ei =
        event_iterator::from_iter_ref(["1", "two", "NaN", "four", "5"])
            .filter_map_mut(|&s| s.parse().ok());

    assert_eq!(ei.next().await, Some(&mut 1));
    assert_eq!(ei.next().await, Some(&mut 5));
    assert_eq!(ei.next().await, None);

    // Here’s the same example, but with `filter()` and `map_ref()`:

    let mut ei =
        event_iterator::from_iter_ref(["1", "two", "NaN", "four", "5"])
            .map_mut(|&s| s.parse())
            .filter(|s| s.is_ok())
            .map_mut(|s| s.clone().unwrap());

    assert_eq!(ei.next().await, Some(&mut 1));
    assert_eq!(ei.next().await, Some(&mut 5));
    assert_eq!(ei.next().await, None);
}
