use std::sync::mpsc::{self, Receiver, Sender, SendError};
use std::thread::{Builder, JoinHandle};

#[cfg(feature = "client")]
use winit::event::{Event, KeyEvent, WindowEvent};

#[cfg(feature = "client")]
use winit::event_loop::{EventLoop, EventLoopBuilder};

#[cfg(feature = "client")]
use winit::window::{Window, WindowBuilder};

#[cfg(feature = "client")]
use crate::client;

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
    #[cfg(feature = "client")]
    client: JoinHandle<()>,

    #[cfg(feature = "server")]
    server: JoinHandle<()>,
}

struct ChannelStruct {
    sender: Option<Sender<()>>,
    receiver: Option<Receiver<()>>,
}

#[cfg(not(target_os = "android"))]
pub fn launch(_argc: isize, _argv: *const *const u8) -> isize {
    // Handle Command Line Arguments Here
    // ...

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
     * The server will only be loaded on a standalone server.
     *
     * The client can either run standalone (multiplayer)
     *   or run both at the same time (singleplayer).
     */
    #[cfg(feature = "server")]
    let (sptx, sprx) = mpsc::channel::<()>(); // Physics Messages Send

    #[cfg(feature = "client")]
    let (srtx, srrx) = mpsc::channel::<()>(); // Render Messages Send

    #[cfg(feature = "server")]
    let (rptx, rprx) = mpsc::channel::<()>(); // Physics Messages Receive

    #[cfg(feature = "client")]
    let (rrtx, rrrx) = mpsc::channel::<()>(); // Render Messages Receive

    // Treat As If Physical Server (Player Movement)
    #[cfg(feature = "server")]
    let physics_thread: JoinHandle<()> = Builder::new()
        .name("physics".to_string())
        .spawn(|| server::start(rptx, sprx))
        .unwrap(); // Physics

    // Treat As If Physical Client (User Input)
    #[cfg(feature = "client")]
    let render_thread: JoinHandle<()> = Builder::new()
        .name("render".to_string())
        .spawn(|| client::start(rrtx, srrx))
        .unwrap(); // Render

    debug!("Starting Main Loop...");

    let threads: ThreadsStruct = ThreadsStruct {
        #[cfg(feature = "client")]
        client: render_thread,

        #[cfg(feature = "server")]
        server: physics_thread,
    };

    let server_channels: ChannelStruct = ChannelStruct {
        #[cfg(feature = "client")]
        sender: Some(srtx),

        #[cfg(not(feature = "client"))]
        sender: None,

        #[cfg(feature = "server")]
        receiver: Some(rprx),

        #[cfg(not(feature = "server"))]
        receiver: None,
    };

    let client_channels: ChannelStruct = ChannelStruct {
        #[cfg(feature = "server")]
        sender: Some(sptx),

        #[cfg(not(feature = "server"))]
        sender: None,

        #[cfg(feature = "client")]
        receiver: Some(rrrx),

        #[cfg(not(feature = "client"))]
        receiver: None,
    };

    #[cfg(not(feature = "client"))]
    headless_loop(threads, server_channels, client_channels);

    #[cfg(feature = "client")]
    gui_loop(threads, server_channels, client_channels);

    Ok(())
}

#[cfg(any(feature = "server", feature = "client"))]
fn is_finished(threads: &ThreadsStruct) -> bool {
    #[cfg(feature = "server")]
    let server_thread: &JoinHandle<()> = &threads.server;

    #[cfg(feature = "client")]
    let client_thread: &JoinHandle<()> = &threads.client;

    #[cfg(all(feature = "server", feature = "client"))]
    return server_thread.is_finished() && client_thread.is_finished();

    #[cfg(all(not(feature = "client"), feature = "server"))]
    return server_thread.is_finished();

    #[cfg(all(not(feature = "server"), feature = "client"))]
    return client_thread.is_finished();
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

#[cfg(feature = "client")]
fn is_render_thread_terminated(channels: &ChannelStruct) -> bool {
    let receiver: &Receiver<()> = &channels.receiver.as_ref().unwrap();

    #[cfg(feature = "server")]
    let sender: &Sender<()> = &channels.sender.as_ref().unwrap();

    match receiver.try_recv() {
        Ok(_) => {
            #[cfg(feature = "server")]
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
    server_channels: ChannelStruct,
    client_channels: ChannelStruct,
) {
    let ctrlc_sender: Sender<()> = client_channels.sender.as_ref().unwrap().clone();
    ctrlc::set_handler(move || {
        let _: Result<(), SendError<()>> = ctrlc_sender.send(());
    })
    .expect("Could not create Interrupt Handler on Headless Loop (e.g. Ctrl+C)...");

    loop {
        #[cfg(any(feature = "server", feature = "client"))]
        if is_finished(&threads) {
            info!("Stopping Headess Server...");
            break;
        }

        #[cfg(feature = "server")]
        if is_physics_thread_terminated(&server_channels) {
            debug!("Physics Thread Terminated...");
            request_exit(&server_channels, &client_channels);
        }
    }
}

#[cfg(any(feature = "server", feature = "client"))]
fn request_exit(_server_channels: &ChannelStruct, _client_channels: &ChannelStruct) {
    // Yes, it's supposed to be _client_channels under the server tag and vice versa

    // Send Exit to Server (Physics) Thread
    #[cfg(feature = "server")]
    let _: Result<(), mpsc::SendError<()>> = _client_channels.sender.as_ref().unwrap().send(());

    // Send Exit to Client (Render) Thread
    #[cfg(feature = "client")]
    let _: Result<(), mpsc::SendError<()>> = _server_channels.sender.as_ref().unwrap().send(());
}

#[cfg(feature = "client")]
fn gui_loop(
    threads: ThreadsStruct,
    server_channels: ChannelStruct,
    client_channels: ChannelStruct,
) {
    use winit::keyboard::{self, NamedKey};

    #[cfg(feature = "server")]
    let ctrlc_physics_sender: Sender<()> = client_channels.sender.as_ref().unwrap().clone();

    #[cfg(feature = "client")]
    let ctrlc_render_sender: Sender<()> = server_channels.sender.as_ref().unwrap().clone();

    ctrlc::set_handler(move || {
        #[cfg(feature = "server")]
        let _: Result<(), SendError<()>> = ctrlc_physics_sender.send(());

        #[cfg(feature = "client")]
        let _: Result<(), SendError<()>> = ctrlc_render_sender.send(());
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
        // control_flow.set_wait();

        #[cfg(any(feature = "server", feature = "client"))]
        if is_finished(&threads) {
            info!("Stopping Game...");
            window_target.exit()
        }

        #[cfg(feature = "server")]
        if is_physics_thread_terminated(&server_channels) {
            debug!("Physics Thread Terminated...");
        }

        #[cfg(feature = "client")]
        if is_render_thread_terminated(&client_channels) {
            debug!("Render Thread Terminated...");
        }

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                debug!("The Close Button Was Pressed! Stopping...");
                request_exit(&server_channels, &client_channels);
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
                request_exit(&server_channels, &client_channels);
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