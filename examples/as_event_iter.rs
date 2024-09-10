use event_iterator::{AsEventIterator, EventIterator};

async fn as_event_iter<'a>(ei: &'a dyn AsEventIterator<'a, Event = i32>) {
    let ei = ei.as_event_iter();

    while let Some(i) = ei.next_unpinned().await {
        println!("{i}");
    }
}

#[async_main::async_main]
async fn main(_spawner: async_main::LocalSpawner) {
    let ei = event_iterator::from_iter([1, 2, 3, 4, 5]);

    as_event_iter(&ei).await;
}
