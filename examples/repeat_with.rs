use std::future;

use event_iterator::EventIterator;

#[async_main::async_main]
async fn main(_spawner: async_main::LocalSpawner) {
    let mut count = 0;
    let ei = event_iterator::repeat_with(move || {
        // Increment our count. This is why we started at zero.
        count += 1;
        future::ready(count)
    });
    let ei = ei.take(5);

    assert_eq!(ei.next_unpinned().await, Some(1));
    assert_eq!(ei.next_unpinned().await, Some(2));
    assert_eq!(ei.next_unpinned().await, Some(3));
    assert_eq!(ei.next_unpinned().await, Some(4));
    assert_eq!(ei.next_unpinned().await, Some(5));
    assert_eq!(ei.next_unpinned().await, None);
    assert_eq!(ei.next_unpinned().await, None);
}
