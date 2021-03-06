use std::thread;

use mio_more;

#[cfg(unix)]
use unix::eventloop::Message;

#[cfg(unix)]
pub use unix::eventloop::EventLoop;

#[cfg(windows)]
use windows::eventloop::Message;

#[cfg(windows)]
pub use windows::eventloop::EventLoop;

pub struct EventLoopHandle {
    handle: thread::JoinHandle<()>,
    tx_msg_channel: mio_more::channel::Sender<Message>,
}

impl EventLoopHandle {
    pub fn new(
        handle: thread::JoinHandle<()>,
        tx_msg_channel: mio_more::channel::Sender<Message>) -> EventLoopHandle {
        EventLoopHandle {
            handle: handle,
            tx_msg_channel: tx_msg_channel,
        }
    }

    pub fn shutdown(self) {
        if self.tx_msg_channel.send(Message::ShutDown).is_ok() {
            let _ = self.handle.join();
        }
    }
}
