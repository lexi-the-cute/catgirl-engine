use crate::game::{ThreadsStruct, ChannelStruct};

use std::sync::mpsc::{SendError, Sender, Receiver};
use std::thread::JoinHandle;

#[cfg(feature = "server")]
pub(crate) fn headless_loop(threads: ThreadsStruct, channels: ChannelStruct) {
    let ctrlc_sender: Sender<()> = channels.sender.as_ref().unwrap().clone();
    ctrlc::set_handler(move || {
        debug!("SIGINT (Ctrl+C) Was Called! Stopping...");
        let _: Result<(), SendError<()>> = ctrlc_sender.send(());
    })
    .expect("Could not create Interrupt Handler on Headless Loop (e.g. Ctrl+C)...");

    loop {
        if is_finished(&threads) {
            info!("Stopping Headless Server...");
            break;
        }

        if is_physics_thread_terminated(&channels) {
            debug!("Physics Thread Terminated...");
            request_exit(&channels);
        }
    }
}

fn request_exit(_channels: &ChannelStruct) {
    // Yes, it's supposed to be _client_channels under the server tag and vice versa

    // Send Exit to Server (Physics) Thread
    #[cfg(feature = "server")]
    let _: Result<(), SendError<()>> = _channels.sender.as_ref().unwrap().send(());
}

fn is_finished(threads: &ThreadsStruct) -> bool {
    #[cfg(feature = "server")]
    let server_thread: &JoinHandle<()> = &threads.server;

    server_thread.is_finished()
}

#[cfg(feature = "server")]
fn is_physics_thread_terminated(channels: &ChannelStruct) -> bool {
    let receiver: &Receiver<()> = &channels.receiver.as_ref().unwrap();

    #[cfg(feature = "client")]
    let sender: &Sender<()> = &channels.sender.as_ref().unwrap();

    match receiver.try_recv() {
        Ok(_) => {
            #[cfg(feature = "client")]
            sender.send(()).ok();

            true
        }
        Err(_) => {
            false
        }
    }
}