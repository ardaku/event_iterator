use std::{
    future::Future,
    io::{self, BufRead},
    mem,
    pin::Pin,
    task::{Context, Poll},
    thread,
};

use event_iterator::EventIterator;
use whisk::Channel;

/// An event iterator, for reading from stdin
#[derive(Default)]
pub struct Stdin {
    channel: Channel<Option<String>>,
    buffer: Option<String>,
    send: Option<Pin<Box<dyn Future<Output = ()>>>>,
}

impl EventIterator for Stdin {
    type Event<'me> = &'me str;

    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Self::Event<'_>>> {
        println!("poll_next");

        let this = self.get_mut();

        if let Some(buffer) = this.buffer.take() {
            let channel = this.channel.clone();

            this.send =
                Some(Box::pin(async move { channel.send(Some(buffer)).await }));
        }

        if let Some(future) = &mut this.send {
            if future.as_mut().poll(cx).is_ready() {
                this.send = None;
            } else {
                return dbg!(Poll::Pending);
            }
        }

        match dbg!(Pin::new(&mut this.channel).poll(cx)) {
            Poll::Ready(Some(buffer)) => {
                this.buffer = Some(buffer);
                Poll::Ready(this.buffer.as_ref().map(|x| x.as_str()))
            }
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}

async fn stdin_thread(channel: Channel<Option<String>>) {
    let stdin = io::stdin();
    let mut buffer = String::new();

    while {
        stdin.read_line(&mut buffer);
        true
    } {
        /*if buffer.is_empty() {
            println!("Empty");
            break;
        }*/

        println!("Read {buffer}");
        channel.send(Some(mem::take(&mut buffer))).await;
        println!("Sent");
        buffer = dbg!(channel.recv().await.unwrap_or_default());
        println!("Recvd");
    }

    println!("Finish");
    channel.send(None).await;
}

#[async_main::async_main]
async fn main(_spawner: async_main::LocalSpawner) {
    // Init stdin
    let mut stdin = Stdin::default();
    let channel = stdin.channel.clone();

    thread::spawn(move || {
        pasts::Executor::default().block_on(stdin_thread(channel))
    });

    // Check messages
    while let Some(message) = dbg!(stdin.next_unpinned().await) {
        println!("Echo: {message}");
    }

    println!("Done");
}
