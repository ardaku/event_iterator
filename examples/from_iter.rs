use event_iterator::EventIterator;

#[async_main::async_main]
async fn main(_spawner: async_main::LocalSpawner) {
    let ei = event_iterator::from_iter([1, 2, 3, 4, 5]);

    while let Some(i) = ei.next_unpinned().await {
        println!("{i}");
    }
}
