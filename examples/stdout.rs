use core::{
    pin::{pin, Pin},
    task::{Context, Poll},
};

use event_iterator::EventIterator;

/// An event iterator, for printing to stdout
#[derive(Default)]
pub struct Stdout {
    buffer: Option<String>,
}

impl EventIterator for Stdout {
    type Event<'me> = &'me mut String;

    fn poll_next(
        self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
    ) -> Poll<Option<Self::Event<'_>>> {
        let this = self.get_mut();

        // Print last buffer contents if set, set if unset
        if let Some(buffer) = &this.buffer {
            // This could be an asynchronous operation
            // Left synchronous for example simplicity
            println!("{buffer}");
        } else {
            this.buffer = Some(String::new());
        }

        Poll::Ready(this.buffer.as_mut())
    }
}

#[async_main::async_main]
async fn main(_spawner: async_main::LocalSpawner) {
    let mut stdout = Stdout::default();

    // Overwrite buffer with text to print
    stdout
        .next_unpinned()
        .await
        .unwrap()
        .replace_range(.., "Hello, world!");
    stdout
        .next_unpinned()
        .await
        .unwrap()
        .replace_range(.., "Hello, again!");

    // Once more, to use the previous buffer contents
    let flush = pin!(stdout);

    flush.next().await.unwrap();
}
