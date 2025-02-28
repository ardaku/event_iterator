use event_iterator::EventIterator;

#[async_main::async_main]
async fn main(_spawner: async_main::LocalSpawner) {
    let mut ei = event_iterator::from_iter_ref([1, 2, 3, 4, 5]);

    assert_eq!((5, Some(5)), ei.size_hint());

    let _ = ei.next().await;

    assert_eq!((4, Some(4)), ei.size_hint());

    /////////////////////
    // More complex... //
    /////////////////////

    // The even numbers in the range of zero to nine.
    let iter = event_iterator::from_iter_ref((0..10).filter(|x| x % 2 == 0));

    // We might iterate from zero to ten times. Knowing that it's five
    // exactly wouldn't be possible without executing filter().
    assert_eq!((0, Some(10)), iter.size_hint());

    // Let's add five more numbers with chain()
    let iter = event_iterator::from_iter_ref(
        (0..10).filter(|x| x % 2 == 0).chain(15..20),
    );

    // now both bounds are increased by five
    assert_eq!((5, Some(15)), iter.size_hint());

    ///////////////////////////////////////
    // Returning `None` for upper bound: //
    ///////////////////////////////////////

    // an infinite iterator has no upper bound
    // and the maximum possible lower bound
    let iter = event_iterator::from_iter_ref(0..);

    assert_eq!((usize::MAX, None), iter.size_hint());
}
