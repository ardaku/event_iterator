use core::pin::pin;

use event_iterator::{EventIterator, IntoEventIterator};

async fn print<'a, I: IntoEventIterator<'a, Event<'a> = &'static str> + 'a>(
    ei: I,
) where
    <I as IntoEventIterator<'a>>::IntoEventIter: Unpin,
{
    //async fn print<'a, Ei>(ei: impl IntoEventIterator<'a, Event<'a> =
    // &'static str> + 'a) {
    let ei = ei.into_event_iter();

    while let Some(value) = ei.next_unpinned().await {
        println!("{value}");
    }
}

#[async_main::async_main]
async fn main(_spawner: async_main::LocalSpawner) {
    let ei = event_iterator::from_iter(["a", "b", "c"]);

    print(ei).await
}
