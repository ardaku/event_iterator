use std::task::Poll;

use event_iterator::EventIterator;

#[async_main::async_main]
async fn main(_spawner: async_main::LocalSpawner) {
    let mut ei = event_iterator::from_iter_ref([1, 2, 3]).tear();

    assert_eq!(futures::poll!(ei.next()), Poll::Ready(Some(&1)));
    assert_eq!(futures::poll!(ei.next()), Poll::Ready(Some(&2)));
    assert_eq!(futures::poll!(ei.next()), Poll::Ready(Some(&3)));
    assert_eq!(futures::poll!(ei.next()), Poll::Ready(None));
    assert_eq!(futures::poll!(ei.next()), Poll::Pending);
    assert_eq!(futures::poll!(ei.next()), Poll::Pending);
}
