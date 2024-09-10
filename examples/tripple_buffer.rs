use core::{
    cell::Cell,
    pin::Pin,
    task::{Context, Poll},
};

use event_iterator::{AsEventIter, AsEventIterator, EventIterator};

struct TrippleBuffer {
    buffer_a: [u8; 2],
    buffer_b: [u8; 3],
    buffer_c: [u8; 1],
    which: Cell<u8>,
}

impl TrippleBuffer {
    pub fn new() -> Self {
        Self {
            buffer_a: [0, 0],
            buffer_b: [0, 1, 1],
            buffer_c: [1],
            which: Cell::new(0),
        }
    }
}

impl EventIterator for TrippleBuffer {
    type Event<'me> = &'me [u8];

    fn poll_next<'a>(
        self: Pin<&'a Self>,
        _cx: &mut Context<'_>,
    ) -> Poll<Option<Self::Event<'a>>> {
        let which = self.which.get();
        let this = self.get_ref();

        if which >= 9 {
            return Poll::Ready(None);
        }

        // cycle through them
        self.which.set(which + 1);
        Poll::Ready(Some(match which % 3 {
            0 => this.buffer_a.as_ref(),
            1 => this.buffer_b.as_ref(),
            2 => this.buffer_c.as_ref(),
            _ => unreachable!(),
        }))
    }
}

struct MyBuffer {
    buffer_a: TrippleBuffer,
    name: String,
}

impl MyBuffer {
    pub fn new(name: impl ToString) -> Self {
        Self {
            buffer_a: TrippleBuffer::new(),
            name: name.to_string(),
        }
    }
}

impl<'b> AsEventIterator<'b> for MyBuffer {
    type Event = &'b [u8];

    fn as_event_iter(&'b self) -> AsEventIter<'b, Self::Event> {
        AsEventIter::new(&self.buffer_a)
    }
}

#[async_main::async_main]
async fn main(_spawner: async_main::LocalSpawner) {
    let my_buffer = MyBuffer::new("example");
    let ei = my_buffer.as_event_iter();

    while let Some(i) = ei.next_unpinned().await {
        println!("{}: {i:?}", my_buffer.name);
    }
}
