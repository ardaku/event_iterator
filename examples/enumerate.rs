use event_iterator::EventIterator;

#[async_main::async_main]
async fn main(_spawner: async_main::LocalSpawner) {
    let mut ei = event_iterator::from_iter_ref(['a', 'b', 'c']).enumerate();

    assert_eq!(ei.next().await, Some((0, &'a')));
    assert_eq!(ei.next().await, Some((1, &'b')));
    assert_eq!(ei.next().await, Some((2, &'c')));
    assert_eq!(ei.next().await, None);
}
