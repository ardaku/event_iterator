use std::task::Poll;

use event_iterator::EventIterator;

#[async_main::async_main]
async fn main(_spawner: async_main::LocalSpawner) {
    let mut event = Some("event");
    let ei = event_iterator::poll_fn(|_cx| Poll::Ready(event.take()));

    assert_eq!(ei.next_unpinned().await, Some("event"));
    assert_eq!(ei.next_unpinned().await, None);
    assert_eq!(ei.next_unpinned().await, None);
}
