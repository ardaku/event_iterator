use std::{
    cell::Cell,
    future::Future,
    io, mem,
    pin::Pin,
    task::{Context, Poll},
    thread,
};

use event_iterator::EventIterator;
use whisk::Channel;

/// An event iterator, for reading from stdin
#[derive(Default)]
pub struct Stdin {
    sender: Channel<Option<String>>,
    recver: Cell<Option<Channel<Option<String>>>>,
    buffer: Cell<Option<String>>,
    send: Cell<Option<Pin<Box<dyn Future<Output = ()> + Send>>>>,
}

impl EventIterator for Stdin {
    type Event<'me> = Buffer<'me>;

    fn poll_next(
        self: Pin<&Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Self::Event<'_>>> {
        let this = self.get_ref();

        if let Some(buffer) = this.buffer.take() {
            let sender = this.sender.clone();

            this.send.set(Some(Box::pin(async move {
                sender.send(Some(buffer)).await
            })));
        }

        if let Some(mut future) = this.send.take() {
            if future.as_mut().poll(cx).is_pending() {
                this.send.set(Some(future));
                return Poll::Pending;
            }
        }

        let mut recver = this.recver.take().unwrap();
        let poll = match Pin::new(&mut recver).poll(cx) {
            Poll::Ready(Some(buffer)) => {
                this.buffer.set(Some(buffer));
                Poll::Ready(Some(Buffer(&this.buffer)))
            }
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        };

        this.recver.set(Some(recver));
        poll
    }
}

pub struct Buffer<'a>(&'a Cell<Option<String>>);

impl Buffer<'_> {
    pub fn with(&self, f: impl FnOnce(&str)) {
        self.0.set(self.0.take().map(|buf| {
            f(&buf);
            buf
        }));
    }
}

async fn stdin_thread(
    recver: Channel<Option<String>>,
    sender: Channel<Option<String>>,
) {
    let stdin = io::stdin();
    let mut buffer = String::new();

    while stdin.read_line(&mut buffer).is_ok() {
        // Remove trailing newline
        buffer.pop();

        if buffer.is_empty() {
            break;
        }

        sender.send(Some(mem::take(&mut buffer))).await;
        buffer = recver.recv().await.unwrap_or_default();
        buffer.clear();
    }

    sender.send(None).await;
}

#[async_main::async_main]
async fn main(_spawner: async_main::LocalSpawner) {
    // Init stdin
    let stdin = Stdin::default();
    let recver = stdin.sender.clone();
    let sender = Channel::new();

    stdin.recver.set(Some(sender.clone()));
    thread::spawn(move || {
        pasts::Executor::default().block_on(stdin_thread(recver, sender))
    });

    // Check messages
    while let Some(buffer) = stdin.next_unpinned().await {
        buffer.with(|message| {
            println!("Echo: {message}");
        });
    }
}
