use std::{
    future::Future,
    io,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
    thread::{self, JoinHandle},
};

use event_iterator::EventIterator;
use whisk::Channel;

/// An event iterator, for scanning from stdin
#[derive(Default)]
pub struct Stdin {
    channel: Channel<Option<Arc<String>>>,
    buffer: Option<Arc<String>>,
    join: Option<JoinHandle<()>>,
}

impl Drop for Stdin {
    fn drop(&mut self) {
        self.join.take().unwrap().join().unwrap();
    }
}

impl Stdin {
    pub fn new() -> Self {
        let channel = Channel::new();
        let sender = channel.clone();
        let join = thread::spawn(move || {
            pasts::Executor::default().block_on(async move {
                let stdin = io::stdin();
                let mut buffer = String::new();
                let mut sending = Arc::new(String::new());

                while stdin.read_line(&mut buffer).is_ok() {
                    let buf = if let Some(s) = Arc::get_mut(&mut sending) {
                        s
                    } else {
                        sending = Arc::new(String::new());
                        Arc::get_mut(&mut sending).unwrap()
                    };

                    // Remove trailing newline
                    buffer.pop();

                    if buffer.is_empty() {
                        break;
                    }

                    buf.replace_range(.., buffer.as_str());
                    sender.send(Some(sending.clone())).await;
                    buffer.clear();
                }

                sender.send(None).await;
            })
        });

        Self {
            buffer: None,
            join: Some(join),
            channel,
        }
    }
}

impl EventIterator for Stdin {
    type Event<'me> = Buffer<'me>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        let this = self.get_mut();

        this.buffer = None;

        match Pin::new(&mut this.channel).poll(cx) {
            Poll::Ready(Some(buffer)) => this.buffer = Some(buffer),
            Poll::Ready(None) => {}
            Poll::Pending => return Poll::Pending,
        };

        Poll::Ready(())
    }

    fn event(self: Pin<&mut Self>) -> Option<Self::Event<'_>> {
        let this = self.get_mut();

        Some(Buffer(this.buffer.as_ref()?))
    }
}

pub struct Buffer<'a>(&'a String);

impl Buffer<'_> {
    pub fn with(&self, f: impl FnOnce(&str)) {
        f(self.0)
    }
}

#[async_main::async_main]
async fn main(_spawner: async_main::LocalSpawner) {
    println!("Echo example - enter empty line to quit");

    let mut stdin = Stdin::new();

    // Check messages
    while let Some(buffer) = stdin.next().await {
        buffer.with(|message| println!("Echo: {message}"));
    }
}
