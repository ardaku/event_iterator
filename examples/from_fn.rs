use event_iterator::EventIterator;

#[async_main::async_main]
async fn main(_spawner: async_main::LocalSpawner) {
    let mut count = 0;
    let ei = event_iterator::from_fn(move || {
        // Increment our count. This is why we started at zero.
        count += 1;
        Box::pin(async move {
            // Check to see if we've finished counting or not.
            if count < 6 {
                Some(count)
            } else {
                None
            }
        })
    });

    assert_eq!(ei.next().await, Some(1));
    assert_eq!(ei.next().await, Some(2));
    assert_eq!(ei.next().await, Some(3));
    assert_eq!(ei.next().await, Some(4));
    assert_eq!(ei.next().await, Some(5));
    assert_eq!(ei.next().await, None);
    assert_eq!(ei.next().await, None);
}
