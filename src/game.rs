use std::sync::mpsc::{self, Receiver, Sender, SendError};
use std::thread::{Builder, JoinHandle};

#[cfg(feature = "client")]
use winit::event::{Event, KeyEvent, WindowEvent};

#[cfg(feature = "client")]
use winit::event_loop::{EventLoop, EventLoopBuilder};

#[cfg(feature = "client")]
use winit::window::{Window, WindowBuilder};

#[cfg(feature = "server")]
use crate::server;

#[cfg(target_os = "android")]
use std::sync::OnceLock;

#[cfg(target_os = "android")]
use winit::platform::android::activity::AndroidApp;

#[cfg(target_os = "android")]
use winit::platform::android::EventLoopBuilderExtAndroid;

#[cfg(target_os = "android")]
static ANDROID_APP: OnceLock<AndroidApp> = OnceLock::new();

struct ThreadsStruct {
    #[cfg(feature = "server")]
    server: JoinHandle<()>,
}

struct ChannelStruct {
    sender: Option<Sender<()>>,
    receiver: Option<Receiver<()>>,
}

#[cfg(not(target_os = "android"))]
pub fn launch() -> isize {
    // Handle Command Line Arguments Here
    let args: Vec<String> = std::env::args().collect();
    debug!("Args: {:?}", args);

    match start() {
        Ok(_) => {
            return 0;
        }
        Err(_error) => {
            return -1;
        }
    }
}

#[allow(dead_code)]
#[cfg(all(target_os = "android", feature = "client"))]
pub fn start_android(app: AndroidApp) -> Result<(), String> {
    let _app: &AndroidApp = ANDROID_APP.get_or_init(|| app);

    return start();
}

pub fn start() -> Result<(), String> {
    // let (tx, rx) = mpsc::channel();
    info!("Starting Game...");

    /* This is a server/client model
     *
     * The server will be solely loaded on a standalone server.
     *
     * The client can either run standalone (multiplayer)
     *   or run both at the same time (singleplayer).
     */
    #[cfg(feature = "server")]
    let (sptx, sprx) = mpsc::channel::<()>(); // Physics Messages Send

    #[cfg(feature = "server")]
    let (rptx, rprx) = mpsc::channel::<()>(); // Physics Messages Receive

    // Treat As If Physical Server (Player Movement)
    #[cfg(feature = "server")]
    let physics_thread: JoinHandle<()> = Builder::new()
        .name("physics".to_string())
        .spawn(|| server::start(rptx, sprx))
        .unwrap(); // Physics

    debug!("Starting Main Loop...");

    let threads: ThreadsStruct = ThreadsStruct {
        #[cfg(feature = "server")]
        server: physics_thread,
    };

    let channels: ChannelStruct = ChannelStruct {
        sender: Some(sptx),
        receiver: Some(rprx),
    };

    #[cfg(not(feature = "client"))]
    headless_loop(threads, channels);

    #[cfg(feature = "client")]
    gui_loop(threads, channels);

    Ok(())
}

#[cfg(any(feature = "server", feature = "client"))]
fn is_finished(threads: &ThreadsStruct) -> bool {
    #[cfg(feature = "server")]
    let server_thread: &JoinHandle<()> = &threads.server;

    return server_thread.is_finished();
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

            return true;
        }
        Err(_) => {
            return false;
        }
    }
}

#[allow(dead_code)]
#[cfg(feature = "server")]
fn headless_loop(
    threads: ThreadsStruct,
    channels: ChannelStruct
) {
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

#[cfg(any(feature = "server", feature = "client"))]
fn request_exit(_channels: &ChannelStruct) {
    // Yes, it's supposed to be _client_channels under the server tag and vice versa

    // Send Exit to Server (Physics) Thread
    #[cfg(feature = "server")]
    let _: Result<(), mpsc::SendError<()>> = _channels.sender.as_ref().unwrap().send(());
}

#[cfg(feature = "client")]
fn gui_loop(
    threads: ThreadsStruct,
    channels: ChannelStruct
) {
    use winit::keyboard::{self, NamedKey};

    #[cfg(feature = "server")]
    let ctrlc_physics_sender: Sender<()> = channels.sender.as_ref().unwrap().clone();

    ctrlc::set_handler(move || {
        debug!("SIGINT (Ctrl+C) Was Called! Stopping...");
        let _: Result<(), SendError<()>> = ctrlc_physics_sender.send(());
    })
    .expect("Could not create Interrupt Handler on Gui Loop (e.g. Ctrl+C)...");

    #[cfg(not(target_os = "android"))]
    let event_loop: EventLoop<()> = EventLoopBuilder::new()
        .build()
        .expect("Could not create an event loop!");

    #[cfg(target_os = "android")]
    let event_loop: EventLoop<()> = EventLoopBuilder::new()
        .with_android_app(ANDROID_APP.get().unwrap().to_owned())
        .build()
        .expect("Could not create an event loop!");

    let builder: WindowBuilder = WindowBuilder::new();
    let window: Window = builder
        .build(&event_loop)
        .expect("Could not create window!");

    let _ = event_loop.run(move |event, window_target| {
        // ControlFlow::Poll continuously runs the event loop, even if the OS hasn't
        // dispatched any events. This is ideal for games and similar applications.
        window_target.set_control_flow(winit::event_loop::ControlFlow::Poll);

        // TODO: Determine if this should be selected depending on menus and pause state
        // ControlFlow::Wait pauses the event loop if no events are available to process.
        // This is ideal for non-game applications that only update in response to user
        // input, and uses significantly less power/CPU time than ControlFlow::Poll.
        // window_target.set_control_flow(winit::event_loop::ControlFlow::Wait);

        #[cfg(any(feature = "server", feature = "client"))]
        if is_finished(&threads) && !window_target.exiting() {
            info!("Stopping Game...");
            window_target.exit()
        }

        #[cfg(feature = "server")]
        if is_physics_thread_terminated(&channels) {
            debug!("Physics Thread Terminated...");
        }

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                debug!("The Close Button Was Pressed! Stopping...");
                request_exit(&channels);
            }
            Event::WindowEvent {
                event:
                    WindowEvent::KeyboardInput {
                        event:
                            KeyEvent {
                                logical_key: keyboard::Key::Named(NamedKey::Escape),
                                ..
                            },
                        ..
                    },
                ..
            } => {
                debug!("The Escape Key Was Pressed! Stopping...");
                request_exit(&channels);
            }
            Event::AboutToWait => {
                // Application update code.

                // Queue a RedrawRequested event.
                //
                // You only need to call this if you've determined that you need to redraw, in
                // applications which do not always need to. Applications that redraw continuously
                // can just render here instead.
                window.request_redraw();
            }
            _ => (),
        }
    });
}

pub fn setup_logger() {
    if cfg!(target_os = "android") {
        // Limited Filter: trace,android_activity=debug,winit=debug
        // Stronger Filter: trace,android_activity=off,winit=off

        #[cfg(target_os = "android")]
        android_logger::init_once(
            android_logger::Config::default()
                .with_max_level(log::LevelFilter::Trace)
                .with_tag("CatgirlEngine")
                .with_filter(
                    android_logger::FilterBuilder::new()
                        .parse("trace,android_activity=off,winit=off")
                        .build(),
                ),
        );
    } else {
        // windows, unix (which includes Linux, BSD, and OSX), or target_os = "macos"
        pretty_env_logger::init();
    }
}