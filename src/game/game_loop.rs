use super::{ThreadsStruct, ChannelStruct};
use std::sync::mpsc::{Receiver, Sender, SendError};
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

#[cfg(feature = "client")]
pub(crate) fn gui_loop(threads: ThreadsStruct, channels: ChannelStruct) {
    use wgpu::{Adapter, CommandEncoder, DeviceDescriptor, Instance, RenderPass, RenderPassDescriptor, Surface, SurfaceTexture, TextureView};
    use winit::event::{Event, KeyEvent, WindowEvent};
    use winit::event_loop::{EventLoop, EventLoopBuilder};
    use winit::window::{Window, WindowBuilder};
    use winit::keyboard::{self, NamedKey};

    #[cfg(target_os = "android")]
    use winit::platform::android::EventLoopBuilderExtAndroid;  // Necessary for with_android_app

    // Grab the handle for sending messages to the server
    // TODO: Relocate and move to function to handle both single/multiplayer
    // http://gameprogrammingpatterns.com/game-loop.html
    // https://zdgeier.com/wgpuintro.html
    // https://sotrh.github.io/learn-wgpu/beginner/tutorial5-textures/#loading-an-image-from-a-file
    #[cfg(feature = "server")]
    let ctrlc_physics_sender: Sender<()> = channels.sender.as_ref().unwrap().clone();

    // Allows handling properly shutting down with SIGINT
    ctrlc::set_handler(move || {
        debug!("SIGINT (Ctrl+C) Was Called! Stopping...");
        let _: Result<(), SendError<()>> = ctrlc_physics_sender.send(());
    })
    .expect("Could not create Interrupt Handler on Gui Loop (e.g. Ctrl+C)...");

    // Create the main loop
    #[cfg(not(target_os = "android"))]
    let event_loop: EventLoop<()> = EventLoopBuilder::new()
        .build()
        .expect("Could not create an event loop!");

    #[cfg(target_os = "android")]
    let event_loop: EventLoop<()> = EventLoopBuilder::new()
        .with_android_app(super::ANDROID_APP.get().unwrap().to_owned())
        .build()
        .expect("Could not create an event loop!");

    // Create a window
    let builder: WindowBuilder = WindowBuilder::new();
    let window: Window = builder
        .build(&event_loop)
        .expect("Could not create window!");

    // Context for all WGPU objects
    // https://docs.rs/wgpu/latest/wgpu/struct.Instance.html
    let instance: Instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());

    // Handle to graphics device (e.g. GPU)
    // https://docs.rs/wgpu/latest/wgpu/struct.Adapter.html
    // https://crates.io/crates/pollster
    let adapter: Adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions::default())).unwrap();

    // Handle to the surface on which to draw on (e.g. a window)
    // TODO: Find replacement for rwh_05 as it is due to be replaced!
    // https://docs.rs/wgpu/latest/wgpu/struct.Surface.html
    let surface: Surface = unsafe { instance.create_surface(&window).unwrap() };

    // Describe's a device
    // For use with adapter's request device
    // https://docs.rs/wgpu/latest/wgpu/type.DeviceDescriptor.html
    let device_descriptor: DeviceDescriptor = wgpu::DeviceDescriptor::default();

    // Opens a connection to the graphics device (e.g. GPU)
    let (device, queue) = pollster::block_on(adapter.request_device(&device_descriptor, None)).unwrap();

    let _ = event_loop.run(move |event, window_target| {
        /* Update Order
         *
         * processInput() - Handles user input
         * update() - Handle game physics
         * render() - Handle graphics
         */

        // ControlFlow::Poll continuously runs the event loop, even if the OS hasn't
        // dispatched any events. This is ideal for games and similar applications.
        window_target.set_control_flow(winit::event_loop::ControlFlow::Poll);

        // TODO: Determine if this should be selected depending on menus and pause state
        // ControlFlow::Wait pauses the event loop if no events are available to process.
        // This is ideal for non-game applications that only update in response to user
        // input, and uses significantly less power/CPU time than ControlFlow::Poll.
        // window_target.set_control_flow(winit::event_loop::ControlFlow::Wait);

        if is_finished(&threads) && !window_target.exiting() {
            info!("Stopping Game...");
            window_target.exit()
        }

        #[cfg(feature = "server")]
        if is_physics_thread_terminated(&channels) {
            debug!("Physics Thread Terminated...");
        }

        match event {
            // The close button was pressed
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                debug!("The Close Button Was Pressed! Stopping...");
                request_exit(&channels);
            }

            // Keyboard keys were pressed
            // TODO: Offload to separate function with key mapping config
            // processInput()
            // update() - Input gets passed to (internal) server, physics gets passed back
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

            // Called every time the engine needs to refresh a frame
            // TODO: Offload to separate function
            // render()
            Event::WindowEvent { event: WindowEvent::RedrawRequested, .. } => {
                // Configure a surface for drawing on
                // Needs to be updated to account for window resizing
                let size = window.inner_size();
                surface.configure(&device, &surface.get_default_config(&adapter, size.width, size.height).unwrap());

                // Get a texture to draw onto the surface
                // https://docs.rs/wgpu/latest/wgpu/struct.SurfaceTexture.html
                // https://stackoverflow.com/a/4262634
                // This segfaults when resizing window if no render commands are executed
                let output: SurfaceTexture = surface.get_current_texture().unwrap();

                // Handle to the TextureView object which describes the texture and related metadata
                // https://docs.rs/wgpu/latest/wgpu/struct.TextureView.html
                let view: TextureView = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

                // Used for encoding instructions to the GPU
                // https://docs.rs/wgpu/latest/wgpu/struct.CommandEncoder.html
                let mut encoder: CommandEncoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());

                // A list of commands to render via the encoder
                // https://docs.rs/wgpu/latest/wgpu/struct.RenderPass.html
                {
                    // Command to render
                    // https://docs.rs/wgpu/latest/wgpu/struct.RenderPassDescriptor.html
                    let render_pass_descriptor: RenderPassDescriptor = wgpu::RenderPassDescriptor {
                        label: Some("Render Pass"),
                        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                            view: &view,
                            resolve_target: None,
                            // Royal Purple - 104, 71, 141
                            ops: wgpu::Operations { load: wgpu::LoadOp::Clear(wgpu::Color { r: 104.0/255.0, g: 71.0/255.0, b: 141.0/255.0, a: 1.0 }), store: wgpu::StoreOp::Store },
                        })],
                        depth_stencil_attachment: None,
                        timestamp_writes: None,
                        occlusion_query_set: None
                    };

                    // Render command
                    let _render_pass: RenderPass = encoder.begin_render_pass(&render_pass_descriptor);
                }

                queue.submit(std::iter::once(encoder.finish()));
                output.present();
                
                // Application update code.
                // let input = egui::RawInput::default();
                // ctx.begin_frame(input);

                // egui::CentralPanel::default().show(&ctx, |ui| {
                //     ui.label("Hello egui!");
                // });

                // let full_output = ctx.end_frame();
                // Context::set_immediate_viewport_renderer(&window_target);
            }

            // Last event to ever be executed on shutdown
            Event::LoopExiting => {
                // Should i use this?
            }

            // All unnamed events
            _ => ()
        }
    });
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