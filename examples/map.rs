use event_iterator::EventIterator;

#[async_main::async_main]
async fn main(_spawner: async_main::LocalSpawner) {
    let mut ei =
        event_iterator::from_iter([1, 2, 3, 4, 5]).map(|&x| "uwu".repeat(x));

    while let Some(i) = ei.next().await {
        println!("{i}");
    }
}
