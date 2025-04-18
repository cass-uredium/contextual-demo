use std::sync::{Arc, mpsc};
use std::thread;
use std::time::Duration;

use anyhow::Context;
use wgpu::InstanceDescriptor;
use winit::application::ApplicationHandler;
use winit::dpi::{LogicalSize, PhysicalSize};
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::platform::macos::WindowAttributesExtMacOS;
use winit::window::{Window, WindowAttributes, WindowLevel};

use crate::controller::Controller;
use crate::event::Event;

const WINDOW_SIZE: (u32, u32) = (320, 36);

const FONT_SIZE: f32 = 32.0;
const LINE_HEIGHT: f32 = 36.0;

const TEXT_COLOR: glyphon::Color = glyphon::Color::rgb(0, 0, 0);

#[derive(Default)]
pub struct App {
    state: Option<State>,
    event_rx: Option<mpsc::Receiver<Event>>,
    controller: Option<Controller>,
}

impl App {
    pub fn run(&mut self) -> anyhow::Result<()> {
        let event_loop = EventLoop::new().unwrap();
        event_loop.run_app(self)?;
        Ok(())
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, _event_loop: &dyn ActiveEventLoop) {
        if let Some(state) = &mut self.state {
            state.window.request_redraw();
        }
    }

    fn can_create_surfaces(&mut self, event_loop: &dyn winit::event_loop::ActiveEventLoop) {
        let window = create_window(event_loop).unwrap();
        self.state = Some(pollster::block_on(State::new(window)));

        let (event_tx, event_rx) = mpsc::channel();
        self.event_rx = Some(event_rx);

        // self.controller = Some(Controller::new(event_tx));

        thread::spawn(move || {
            let controller = Controller::new();
            loop {
                let focused_app = controller.focused_app().unwrap();
                let focused_element = controller.focused_element(Some(&focused_app));

                if let Ok(focused_element) = focused_element {
                    let selected_text = controller.selected_text(&focused_element);
                    let selected_text_bounds = controller.selected_text_bounds(&focused_element);
                    // TODO
                }
                thread::sleep(Duration::from_millis(1500));
            }
        });
    }

    fn window_event(
        &mut self,
        event_loop: &dyn winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        let Some(state) = &mut self.state else {
            return;
        };

        match event {
            WindowEvent::SurfaceResized(size) => {
                state.resize(size);
                state.window.request_redraw();
            }
            WindowEvent::RedrawRequested => {
                state.render();
            }
            WindowEvent::CloseRequested => event_loop.exit(),
            _ => {}
        }
    }
}

struct State {
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface: wgpu::Surface<'static>,
    surface_config: wgpu::SurfaceConfiguration,

    font_system: glyphon::FontSystem,
    swash_cache: glyphon::SwashCache,
    viewport: glyphon::Viewport,
    atlas: glyphon::TextAtlas,
    text_renderer: glyphon::TextRenderer,
    text_buffer: glyphon::Buffer,

    window: Arc<dyn Window>,
}

impl State {
    async fn new(window: Arc<dyn Window>) -> Self {
        let physical_size = window.surface_size();
        let logical_size = physical_size.to_logical::<f64>(window.scale_factor());

        // initialize wgpu
        let instance = wgpu::Instance::new(&InstanceDescriptor::default());
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions::default())
            .await
            .unwrap();
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor::default())
            .await
            .unwrap();

        // configure surface
        let surface = instance
            .create_surface(window.clone())
            .expect("create surface");
        let swapchain_format = wgpu::TextureFormat::Bgra8UnormSrgb;
        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: swapchain_format,
            width: physical_size.width,
            height: physical_size.height,
            present_mode: wgpu::PresentMode::AutoVsync,
            alpha_mode: wgpu::CompositeAlphaMode::Opaque,
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &surface_config);

        // text renderer
        let mut font_system = glyphon::FontSystem::new();
        let swash_cache = glyphon::SwashCache::new();
        let cache = glyphon::Cache::new(&device);
        let viewport = glyphon::Viewport::new(&device, &cache);
        let mut atlas = glyphon::TextAtlas::new(&device, &queue, &cache, swapchain_format);
        let text_renderer = glyphon::TextRenderer::new(
            &mut atlas,
            &device,
            wgpu::MultisampleState::default(),
            None,
        );
        let mut text_buffer = glyphon::Buffer::new(
            &mut font_system,
            glyphon::Metrics::new(FONT_SIZE, LINE_HEIGHT),
        );
        text_buffer.set_size(
            &mut font_system,
            Some(logical_size.width as f32),
            Some(logical_size.height as f32),
        );

        Self {
            device,
            queue,
            surface,
            surface_config,
            font_system,
            swash_cache,
            viewport,
            atlas,
            text_renderer,
            text_buffer,
            window,
        }
    }

    fn resize(&mut self, size: PhysicalSize<u32>) {
        self.surface_config.width = size.width;
        self.surface_config.height = size.height;
        self.surface.configure(&self.device, &self.surface_config);
    }

    fn render(&mut self) {
        self.viewport.update(
            &self.queue,
            glyphon::Resolution {
                width: self.surface_config.width,
                height: self.surface_config.height,
            },
        );
        self.text_renderer
            .prepare(
                &self.device,
                &self.queue,
                &mut self.font_system,
                &mut self.atlas,
                &self.viewport,
                [glyphon::TextArea {
                    buffer: &self.text_buffer,
                    left: 0.0,
                    top: 0.0,
                    scale: self.window.scale_factor() as f32,
                    bounds: glyphon::TextBounds {
                        left: 0,
                        top: 0,
                        right: self.surface_config.width.try_into().unwrap(),
                        bottom: self.surface_config.height.try_into().unwrap(),
                    },
                    default_color: TEXT_COLOR,
                    custom_glyphs: &[],
                }],
                &mut self.swash_cache,
            )
            .unwrap();

        let frame = self.surface.get_current_texture().unwrap();
        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self.device.create_command_encoder(&Default::default());
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::WHITE),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            self.text_renderer
                .render(&self.atlas, &self.viewport, &mut pass)
                .unwrap();
        }

        self.queue.submit(Some(encoder.finish()));
        self.window.pre_present_notify();
        frame.present();

        self.atlas.trim();
    }

    fn set_text(&mut self, text: &str) {
        self.text_buffer.set_text(
            &mut self.font_system,
            text,
            &glyphon::Attrs::new().family(glyphon::Family::SansSerif),
            glyphon::Shaping::Advanced,
        );
        self.text_buffer
            .shape_until_scroll(&mut self.font_system, false);
    }
}

fn create_window(event_loop: &dyn ActiveEventLoop) -> anyhow::Result<Arc<dyn Window>> {
    let attrs = WindowAttributes::default()
        .with_surface_size(LogicalSize::<u32>::from(WINDOW_SIZE))
        // .with_resizable(false)
        .with_title(env!("CARGO_BIN_NAME"))
        // .with_visible(false)
        // .with_decorations(false)
        .with_window_level(WindowLevel::AlwaysOnTop)
        .with_active(false)
        .with_panel(true);
    Ok(Arc::from(
        event_loop
            .create_window(attrs)
            .context("failed to create window")?,
    ))
}
