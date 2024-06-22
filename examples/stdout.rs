use core::{
    cell::Cell,
    pin::{pin, Pin},
    task::{Context, Poll},
};

use event_iterator::EventIterator;

/// An event iterator, for printing to stdout
#[derive(Default)]
pub struct Stdout {
    buffer: Cell<Option<String>>,
}

impl EventIterator for Stdout {
    type Event<'me> = Buffer<'me>;

    fn poll_next(
        self: Pin<&Self>,
        _cx: &mut Context<'_>,
    ) -> Poll<Option<Self::Event<'_>>> {
        let this = self.get_ref();

        // Print last buffer contents if set, set if unset
        this.buffer.set(if let Some(buffer) = this.buffer.take() {
            // This could be an asynchronous operation
            // Left synchronous for example simplicity
            println!("{buffer}");
            // Reuse buffer
            Some(buffer)
        } else {
            Some(String::new())
        });
        Poll::Ready(Some(Buffer(&this.buffer)))
    }
}

pub struct Buffer<'a>(&'a Cell<Option<String>>);

impl Buffer<'_> {
    pub fn write(&self, text: &str) {
        self.0.set(self.0.take().map(|mut buf| {
            buf.replace_range(.., text);
            buf
        }));
    }
}

#[async_main::async_main]
async fn main(_spawner: async_main::LocalSpawner) {
    let stdout = Stdout::default();

    // Overwrite buffer with text to print
    stdout.next_unpinned().await.unwrap().write("Hello, world!");
    stdout.next_unpinned().await.unwrap().write("Hello, again!");

    // Once more, to use the previous buffer contents
    let flush = pin!(stdout);

    flush.as_ref().next().await.unwrap();
}
