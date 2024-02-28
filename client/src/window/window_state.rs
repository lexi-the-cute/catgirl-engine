use std::sync::Arc;

use winit::{dpi::PhysicalSize, window::Window};

use wgpu::{Adapter, Device, DeviceDescriptor, Instance, Queue, Surface};

/// Struct used for storing the state of a window
pub struct WindowState<'a> {
    /// Context for WGPU objects
    pub instance: Instance,

    /// Handle to the graphics device (e.g. the gpu)
    pub adapter: Adapter,

    /// The surface on which to draw graphics on
    pub surface: Surface<'a>,

    /// Connection to the graphics device provided by the adapter
    pub device: Device,

    /// Queue in which to send commands to the graphics device
    pub queue: Queue,

    /// Handle to the window which holds the drawable surface
    pub window: Arc<Window>,
}

impl WindowState<'_> {
    /// Used to initialize a new window and setup the graphics
    pub fn new(window: Window) -> Self {
        let window_arc: Arc<Window> = Arc::new(window);

        // Context for all WGPU objects
        // https://docs.rs/wgpu/latest/wgpu/struct.Instance.html
        debug!("Creating wgpu instance...");
        let instance: Instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());

        // Handle to graphics device (e.g. GPU)
        // https://docs.rs/wgpu/latest/wgpu/struct.Adapter.html
        // https://crates.io/crates/futures
        debug!("Grabbing wgpu adapter...");
        let adapter_future = instance.request_adapter(&wgpu::RequestAdapterOptions::default());
        let adapter: Adapter =
            futures::executor::block_on(adapter_future).expect("Could not grab WGPU adapter!");

        // Describe's a device
        // For use with adapter's request device
        // https://docs.rs/wgpu/latest/wgpu/type.DeviceDescriptor.html
        debug!("Describing wgpu device...");
        let mut device_descriptor: DeviceDescriptor = wgpu::DeviceDescriptor::default();

        // Set limits to make this run on more devices
        // TODO: Research how to dynamically set limits for the running device
        debug!("Setting WGPU limits...");
        let limits: wgpu::Limits = wgpu::Limits {
            max_texture_dimension_1d: 4096,
            max_texture_dimension_2d: 4096,
            ..Default::default()
        };

        device_descriptor.required_limits = limits;

        // Opens a connection to the graphics device (e.g. GPU)
        debug!("Opening connection with graphics device (e.g. GPU)...");
        let device_future = adapter.request_device(&device_descriptor, None);
        let (device, queue) = futures::executor::block_on(device_future)
            .expect("Could not open a connection with the graphics device!");

        debug!("Creating wgpu surface...");
        let surface: Surface<'_> = instance
            .create_surface(window_arc.clone())
            .expect("Could not create surface!");

        let size: PhysicalSize<u32> = window_arc.clone().inner_size();
        surface.configure(
            &device,
            &surface
                .get_default_config(&adapter, size.width, size.height)
                .expect("Could not get surface default config!"),
        );

        Self {
            instance,
            adapter,
            surface,
            device,
            queue,
            window: window_arc,
        }
    }

    /// Recreate the surface after it has been destroyed (e.g. used on Android)
    pub fn recreate_surface(&mut self) {
        // Handle to the surface on which to draw on (e.g. a window)
        // https://docs.rs/wgpu/latest/wgpu/struct.Surface.html
        debug!("Creating wgpu surface...");
        let surface: Surface<'_> = self
            .instance
            .create_surface(self.window.clone())
            .expect("Could not create surface!");

        let size: PhysicalSize<u32> = self.window.clone().inner_size();
        surface.configure(
            &self.device,
            &surface
                .get_default_config(&self.adapter, size.width, size.height)
                .expect("Could not get surface default config!"),
        );

        self.surface = surface;
    }
}
