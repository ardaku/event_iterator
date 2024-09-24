use std::task::Poll;

use event_iterator::{EventIterator, Pending};

#[async_main::async_main]
async fn main(_spawner: async_main::LocalSpawner) {
    let ei: Pending<u32> = event_iterator::pending();

    assert_eq!(futures::poll!(ei.next_unpinned()), Poll::Pending);
    assert_eq!(futures::poll!(ei.next_unpinned()), Poll::Pending);
}
