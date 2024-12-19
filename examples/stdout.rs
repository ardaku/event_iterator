use core::{
    cell::Cell,
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
    type Event<'me> = Buffer<'me>;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<()> {
        let this = self.get_mut();

        if let Some(ref mut buffer) = this.buffer {
            // This could be an asynchronous operation
            // Left synchronous for example simplicity
            println!("{buffer}");
            buffer.clear();
        } else {
            this.buffer = Some(String::new());
        }

        Poll::Ready(())
    }

    fn event(self: Pin<&mut Self>) -> Option<Self::Event<'_>> {
        let this = self.get_mut();

        Some(Buffer(Cell::from_mut(this.buffer.as_mut().unwrap())))
    }
}

#[derive(Copy, Clone)]
pub struct Buffer<'a>(&'a Cell<String>);

impl Buffer<'_> {
    pub fn write(&self, text: &str) {
        let mut buffer = self.0.take();

        buffer.replace_range(.., text);
        self.0.set(buffer);
    }
}

#[async_main::async_main]
async fn main(_spawner: async_main::LocalSpawner) {
    let mut stdout = Stdout::default();

    // Overwrite buffer with text to print
    stdout.next().await.unwrap().write("Hello, world!");
    stdout.next().await.unwrap().write("Hello, again!");

    // Once more, to flush the previous buffer contents
    pin!(stdout).next_pinned().await.unwrap();
}
