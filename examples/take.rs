use event_iterator::EventIterator;

#[async_main::async_main]
async fn main(_spawner: async_main::LocalSpawner) {
    let mut ei = event_iterator::from_iter([1, 2, 3]).take(2);

    assert_eq!(ei.next().await, Some(&1));
    assert_eq!(ei.next().await, Some(&2));
    assert_eq!(ei.next().await, None);
    assert_eq!(ei.next().await, None);

    // `take()` is often used with an infinite event iterator, to make it
    // finite:

    let mut ei = event_iterator::from_iter(0..).take(3);

    assert_eq!(ei.next().await, Some(&0));
    assert_eq!(ei.next().await, Some(&1));
    assert_eq!(ei.next().await, Some(&2));
    assert_eq!(ei.next().await, None);
    assert_eq!(ei.next().await, None);

    // If less than `n` elements are available, `take()` will limit itself to
    // the size of the underlying event iterator:

    let mut ei = event_iterator::from_iter([1, 2]).take(5);

    assert_eq!(ei.next().await, Some(&1));
    assert_eq!(ei.next().await, Some(&2));
    assert_eq!(ei.next().await, None);
    assert_eq!(ei.next().await, None);
}
