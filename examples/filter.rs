use event_iterator::EventIterator;

#[async_main::async_main]
async fn main(_spawner: async_main::LocalSpawner) {
    let mut ei =
        event_iterator::from_iter_ref([1, 2, 3, 4, 5]).filter(|&x| x > 1);
    let mut events = Vec::new();

    while let Some(event) = ei.next().await {
        events.push(event.clone());
    }

    println!("{events:?}");
    assert_eq!(events, [2, 3, 4, 5]);
}
