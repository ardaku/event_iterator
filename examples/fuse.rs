use std::task::Poll;

use event_iterator::EventIterator;

#[async_main::async_main]
async fn main(_spawner: async_main::LocalSpawner) {
    let ei = event_iterator::from_iter([1, 2, 3]).fuse();

    assert_eq!(futures::poll!(ei.next_unpinned()), Poll::Ready(Some(1)));
    assert_eq!(futures::poll!(ei.next_unpinned()), Poll::Ready(Some(2)));
    assert_eq!(futures::poll!(ei.next_unpinned()), Poll::Ready(Some(3)));
    assert_eq!(futures::poll!(ei.next_unpinned()), Poll::Ready(None));
    assert_eq!(futures::poll!(ei.next_unpinned()), Poll::Ready(None));
}
