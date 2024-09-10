use event_iterator::{AsEventIterator, EventIterator};

async fn as_event_iter<'a>(aei: &'a dyn AsEventIterator<'a, Event = i32>) {
    let ei = aei.as_event_iter2(); //AsEventIterator::as_event_iter(aei);

    while let Some(i) = ei.next_unpinned().await {
        println!("{i}");
    }
}

#[async_main::async_main]
async fn main(_spawner: async_main::LocalSpawner) {
    let ei = event_iterator::from_iter([1, 2, 3, 4, 5]);

    as_event_iter(&ei).await;
}
