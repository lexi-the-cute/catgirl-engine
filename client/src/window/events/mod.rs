use wgpu::{
    Adapter, CommandEncoder, Device, Queue, RenderPass, RenderPassDescriptor, Surface,
    SurfaceTexture, TextureView,
};
use winit::{
    dpi::PhysicalSize,
    event::{ElementState, Event, KeyEvent, MouseButton, Touch, TouchPhase},
    event_loop::EventLoopWindowTarget,
    keyboard::NamedKey,
    window::{Window, WindowBuilder},
};

use crate::window::window_state::WindowState;

/// The close button was pressed. Usually on the top right corner
pub(crate) fn close_requested(window_target: &EventLoopWindowTarget<()>) {
    debug!("The close button was pressed! Stopping...");
    window_target.exit();
}

/// This is technically not an event, but is called by the Resume event
pub(crate) fn create_window(window_target: &EventLoopWindowTarget<()>) -> WindowState<'static> {
    debug!("Creating window...");

    #[cfg(not(target_family = "wasm"))]
    let window: Window = WindowBuilder::new()
        .with_title("Catgirl Engine")
        .with_window_icon(Some(crate::get_icon()))
        .build(window_target)
        .expect("Could not create window!");

    #[cfg(target_family = "wasm")]
    {
        use winit::platform::web::WindowBuilderExtWebSys;

        let window: Window = WindowBuilder::new()
            .with_canvas(crate::window::web::get_canvas())
            .with_title("Catgirl Engine")
            .with_window_icon(Some(crate::get_icon()))
            .build(window_target)
            .expect("Could not create window!");
    }

    WindowState::new(window)
}

/// Resumed window after first resume call (e.g Android)
pub(crate) fn resumed_window(window_state: &mut WindowState) {
    debug!("Resuming application...");

    window_state.recreate_surface();
}

/// Suspended window
pub(crate) fn suspended_window() {
    debug!("Suspending application...");
}

// TODO: Offload to separate function with key mapping config
// processInput()
// update() - Input gets passed to (internal) server, physics gets passed back
/// Key was pressed on keyboard
pub(crate) fn pressed_key(event: KeyEvent, window_target: &EventLoopWindowTarget<()>) {
    match event.logical_key {
        winit::keyboard::Key::Named(NamedKey::Escape) => {
            debug!("The escape key Was pressed! Stopping...");
            window_target.exit();
        }
        _ => {
            trace!("Event: {:#?}", event);
        }
    }
}

/// Mouse was clicked
pub(crate) fn clicked_mouse(
    state: ElementState,
    button: MouseButton,
    _window_target: &EventLoopWindowTarget<()>,
) {
    if state.is_pressed() {
        trace!("Mouse {:?} was pressed...", button);
    } else {
        trace!("Mouse {:?} was released...", button);
    }
}

/// Screen was touched
pub(crate) fn touched_screen(touch: Touch) {
    match touch.phase {
        TouchPhase::Started => {
            trace!(
                "Finger {:?} touched screen at ({:?}, {:?})",
                touch.id,
                touch.location.x,
                touch.location.y
            );
        }
        TouchPhase::Ended => {
            trace!(
                "Finger {:?} released screen at ({:?}, {:?})",
                touch.id,
                touch.location.x,
                touch.location.y
            );
        }
        TouchPhase::Moved => {
            trace!(
                "Finger {:?} moved across the screen to ({:?}, {:?})",
                touch.id,
                touch.location.x,
                touch.location.y
            );
        }
        TouchPhase::Cancelled => {
            trace!(
                "Finger {:?} touch at ({:?}, {:?}) was cancelled",
                touch.id,
                touch.location.x,
                touch.location.y
            );
        }
    }
}

/// The window was resized
pub(crate) fn resized_window(window_state: &WindowState, _size: PhysicalSize<u32>) {
    if window_state.device.is_none() || window_state.adapter.is_none() {
        warn!(
            "Device: {:?} or adapter: {:?} is none",
            window_state.device.is_none(),
            window_state.adapter.is_none()
        )
    }

    let window: &Window = &window_state.window;
    let surface: &Surface = &window_state.surface;
    let device: &Device = &window_state.device.as_ref().unwrap();
    let adapter: &Adapter = &window_state.adapter.as_ref().unwrap();

    let size: PhysicalSize<u32> = window.inner_size();
    surface.configure(
        device,
        &surface
            .get_default_config(adapter, size.width, size.height)
            .expect("Could not get surface default config!"),
    );
}

/// The window was either just focused or lost focus
pub(crate) fn changed_focus(focused: bool) {
    trace!("Window focused: {focused}");
}

/// Redraw surface
pub(crate) fn requested_redraw(window_state: &WindowState) {
    if window_state.device.is_none() || window_state.queue.is_none() {
        warn!(
            "Device: {:?} or queue: {:?} is none",
            window_state.device.is_none(),
            window_state.queue.is_none()
        )
    }

    // TODO: https://sotrh.github.io/learn-wgpu/beginner/tutorial3-pipeline/#what-s-a-pipeline
    // Configure a surface for drawing on
    let surface: &Surface = &window_state.surface;
    let device: &Device = &window_state.device.as_ref().unwrap();
    let queue: &Queue = &window_state.queue.as_ref().unwrap();

    // Get a texture to draw onto the surface
    // https://docs.rs/wgpu/latest/wgpu/struct.SurfaceTexture.html
    // https://stackoverflow.com/a/4262634
    // This segfaults when resizing window if no render commands are executed
    let output: SurfaceTexture = surface
        .get_current_texture()
        .expect("Could not get a texture to draw on!");

    // Handle to the TextureView object which describes the texture and related metadata
    // https://docs.rs/wgpu/latest/wgpu/struct.TextureView.html
    let view: TextureView = output
        .texture
        .create_view(&wgpu::TextureViewDescriptor::default());

    // Used for encoding instructions to the GPU
    // https://docs.rs/wgpu/latest/wgpu/struct.CommandEncoder.html
    let mut encoder: CommandEncoder =
        device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());

    // Color to render
    // Royal Purple - 104, 71, 141
    let (r, g, b) = crate::render::srgb_to_linear_srgb(104, 71, 141);

    // Command to render
    // https://docs.rs/wgpu/latest/wgpu/struct.RenderPassDescriptor.html
    let render_pass_descriptor: RenderPassDescriptor = wgpu::RenderPassDescriptor {
        label: Some("Render Pass"),
        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
            view: &view,
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(wgpu::Color { r, g, b, a: 1.0 }),
                store: wgpu::StoreOp::Store,
            },
        })],
        depth_stencil_attachment: None,
        timestamp_writes: None,
        occlusion_query_set: None,
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

/// Exiting loop
pub(crate) fn exiting_loop() {
    trace!("Winit loop is exiting...");
}

/// Low memory warning
pub(crate) fn low_memory_warning() {
    trace!("Low memory warning was called...");
}

/// New events have arrived
pub(crate) fn new_events() {
    // Currently only exists to remove from unhandled_event logging
}

/// About to wait for new events to arrive
pub(crate) fn about_to_wait_event() {
    // Currently only exists to remove from unhandled_event logging
}

/// Catches previously unhandled events
pub(crate) fn unhandled_event(event: Event<()>) {
    trace!("Unhandled event: {:?}", event);
}
