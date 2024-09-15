use event_iterator::EventIterator;

#[async_main::async_main]
async fn main(_spawner: async_main::LocalSpawner) {
    let ei = event_iterator::from_iter([-2i32, -1, 0, 1, -2])
        .take_while(|x| x.is_negative());

    assert_eq!(ei.next_unpinned().await, Some(-2));
    assert_eq!(ei.next_unpinned().await, Some(-1));
    assert_eq!(ei.next_unpinned().await, None);
    assert_eq!(ei.next_unpinned().await, None);
}
