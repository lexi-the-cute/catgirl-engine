use crate::game::{ThreadsStruct, ChannelStruct};

use std::sync::Arc;
use std::sync::mpsc::{Receiver, Sender, SendError};
use std::thread::JoinHandle;

#[cfg(feature = "client")]
pub(crate) fn gui_loop(threads: ThreadsStruct, channels: ChannelStruct) {
    use wgpu::{Adapter, CommandEncoder, Device, DeviceDescriptor, Instance, Queue, RenderPass, RenderPassDescriptor, Surface, SurfaceTexture, TextureView};
    use winit::dpi::PhysicalSize;
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
    #[cfg(not(target_family = "wasm"))]
    let ctrlc_physics_sender: Sender<()> = channels.sender.as_ref().unwrap().clone();

    // Allows handling properly shutting down with SIGINT
    #[cfg(not(target_family = "wasm"))]
    ctrlc::set_handler(move || {
        debug!("SIGINT (Ctrl+C) Was Called! Stopping...");
        let _: Result<(), SendError<()>> = ctrlc_physics_sender.send(());
    })
    .expect("Could not create Interrupt Handler on Gui Loop (e.g. Ctrl+C)...");

    // Create the main loop
    debug!("Creating event loop...");
    #[cfg(not(target_os = "android"))]
    let event_loop: EventLoop<()> = EventLoopBuilder::new()
        .build()
        .expect("Could not create an event loop!");

    #[cfg(target_os = "android")]
    let event_loop: EventLoop<()> = EventLoopBuilder::new()
        .with_android_app(crate::game::ANDROID_APP.get().unwrap().to_owned())
        .build()
        .expect("Could not create an event loop!");

    let mut window_arc: Option<Arc<Window>> = None;
    let mut adapter: Option<Adapter> = None;
    let mut surface: Option<Surface> = None;
    let mut device: Option<Device> = None;
    let mut queue: Option<Queue> = None;
    debug!("Starting event loop...");
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

            Event::Resumed => {
                debug!("Resuming application...");

                // Create window in an Atomically Reference Counted object
                // This is to allow retaining a handle to the window after passing it to create_surface
                // https://github.com/gfx-rs/wgpu/discussions/5213
                // https://doc.rust-lang.org/std/sync/struct.Arc.html
                debug!("Creating window...");
                window_arc = Some(Arc::new(WindowBuilder::new()
                    .with_title(crate::game::NAME)
                    .build(&window_target)
                    .expect("Could not create window!")));

                // Context for all WGPU objects
                // https://docs.rs/wgpu/latest/wgpu/struct.Instance.html
                debug!("Creating wgpu instance...");
                let instance: Instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());

                // Handle to graphics device (e.g. GPU)
                // https://docs.rs/wgpu/latest/wgpu/struct.Adapter.html
                // https://crates.io/crates/pollster
                debug!("Grabbing wgpu adapter...");
                adapter = Some(pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions::default()))
                    .expect("Could not grab WGPU adapter!"));

                // Handle to the surface on which to draw on (e.g. a window)
                // https://docs.rs/wgpu/latest/wgpu/struct.Surface.html
                debug!("Creating wgpu surface...");
                surface = Some(instance.create_surface(window_arc.as_ref().unwrap().clone())
                    .expect("Could not create surface!"));

                // Describe's a device
                // For use with adapter's request device
                // https://docs.rs/wgpu/latest/wgpu/type.DeviceDescriptor.html
                debug!("Describing wgpu device...");
                let mut device_descriptor: DeviceDescriptor = wgpu::DeviceDescriptor::default();

                // Set limits to make this run on more devices
                // TODO: Research how to dynamically set limits for the running device
                debug!("Setting WGPU limits...");
                let mut limits: wgpu::Limits = wgpu::Limits::default();
                limits.max_texture_dimension_1d = 4096;
                limits.max_texture_dimension_2d = 4096;

                device_descriptor.required_limits = limits;

                // Opens a connection to the graphics device (e.g. GPU)
                debug!("Opening connection with graphics device (e.g. GPU)...");
                let (_device, _queue) = pollster::block_on(adapter.as_ref().unwrap().request_device(&device_descriptor, None))
                    .expect("Could not open a connection with the graphics device!");
                device = Some(_device);
                queue = Some(_queue);
            }

            Event::Suspended => {
                debug!("Suspending application...");
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
                let window: &Window = window_arc.as_ref().unwrap().as_ref();
                let device: &Device = device.as_ref().unwrap();
                let surface: &Surface = surface.as_ref().unwrap();
                let adapter: &Adapter = adapter.as_ref().unwrap();
                let queue: &Queue = queue.as_ref().unwrap();

                let size: PhysicalSize<u32> = window.inner_size();
                surface.configure(&device, &surface.get_default_config(&adapter, size.width, size.height)
                    .expect("Could not get surface default config!"));

                // Get a texture to draw onto the surface
                // https://docs.rs/wgpu/latest/wgpu/struct.SurfaceTexture.html
                // https://stackoverflow.com/a/4262634
                // This segfaults when resizing window if no render commands are executed
                let output: SurfaceTexture = surface.get_current_texture()
                    .expect("Could not get a texture to draw on!");

                // Handle to the TextureView object which describes the texture and related metadata
                // https://docs.rs/wgpu/latest/wgpu/struct.TextureView.html
                let view: TextureView = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

                // Used for encoding instructions to the GPU
                // https://docs.rs/wgpu/latest/wgpu/struct.CommandEncoder.html
                let mut encoder: CommandEncoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());

                // Command to render
                // https://docs.rs/wgpu/latest/wgpu/struct.RenderPassDescriptor.html
                let render_pass_descriptor: RenderPassDescriptor = wgpu::RenderPassDescriptor {
                    label: Some("Render Pass"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        // Royal Purple - 104, 71, 141
                        // TODO: Fix so color is shown accurately
                        ops: wgpu::Operations { load: wgpu::LoadOp::Clear(wgpu::Color { r: 104.0/255.0, g: 71.0/255.0, b: 141.0/255.0, a: 1.0 }), store: wgpu::StoreOp::Store },
                    })],
                    depth_stencil_attachment: None,
                    timestamp_writes: None,
                    occlusion_query_set: None
                };

                // Block expression for ending the borrow of encoder
                // https://doc.rust-lang.org/reference/expressions/block-expr.html
                // https://doc.rust-lang.org/beta/rust-by-example/scope/borrow.html
                {
                    // Render command
                    // https://docs.rs/wgpu/latest/wgpu/struct.RenderPass.html
                    let _render_pass: RenderPass = encoder.begin_render_pass(&render_pass_descriptor);
                }

                queue.submit(core::iter::once(encoder.finish()));
                output.present();
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