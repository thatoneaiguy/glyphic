use winit::{
    event_loop::{EventLoop, ControlFlow},
    window::{Window, WindowBuilder},
};
use wgpu;

async fn create_window() -> (EventLoop<()>, Window, wgpu::Device, wgpu::Queue, wgpu::Surface, wgpu::Adapter) {
    // 1. Create event loop and window
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Glyphic Animation Studio")
        .with_inner_size(winit::dpi::LogicalSize::new(1280, 720))
        .with_visible(false)
        .build(&event_loop)
        .unwrap();

    // 2. Initialize GPU
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::PRIMARY,
        dx12_shader_compiler: Default::default(),
    });

    let surface = unsafe { instance.create_surface(&window) }.unwrap();

    let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::HighPerformance,
        compatible_surface: Some(&surface),
        force_fallback_adapter: false,
    }).await.unwrap();

    let (device, queue) = adapter.request_device(
        &wgpu::DeviceDescriptor {
            features: wgpu::Features::empty(),
            limits: wgpu::Limits::default(),
            label: None,
        },
        None
    ).await.unwrap();

    // 3. Configure surface
    let surface_caps = surface.get_capabilities(&adapter);
    let surface_format = surface_caps.formats[0];

    let config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: surface_format,
        width: window.inner_size().width,
        height: window.inner_size().height,
        present_mode: wgpu::PresentMode::Mailbox,
        alpha_mode: surface_caps.alpha_modes[0],
        view_formats: vec![],
    };
    surface.configure(&device, &config);

    window.set_visible(true);
    (event_loop, window, device, queue, surface, adapter)
}

pub struct Renderer {
    pipeline: wgpu::RenderPipeline,
    config: wgpu::SurfaceConfiguration, // Store config for resizing
}

impl Renderer {
    pub fn new(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration) -> Self {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Glyphic Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Glyphic Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });

        Self {
            pipeline,
            config: config.clone(),
        }
    }

    pub fn resize(&mut self, _device: &wgpu::Device, width: u32, height: u32) {
        self.config.width = width;
        self.config.height = height;
    }

    pub fn render(&mut self, view: &wgpu::TextureView, device: &wgpu::Device, queue: &wgpu::Queue) {
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });

            render_pass.set_pipeline(&self.pipeline);
            render_pass.draw(0..3, 0..1);
        }

        queue.submit(std::iter::once(encoder.finish()));
    }
}

fn main() {
    let (event_loop, window, device, queue, surface, adapter) =
        futures::executor::block_on(create_window());

    let surface_caps = surface.get_capabilities(&adapter);
    let mut config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: surface_caps.formats[0],
        width: window.inner_size().width,
        height: window.inner_size().height,
        present_mode: wgpu::PresentMode::Mailbox,
        alpha_mode: surface_caps.alpha_modes[0],
        view_formats: vec![],
    };
    surface.configure(&device, &config);

    let mut renderer = Renderer::new(&device, &config);
    let mut last_frame_time = std::time::Instant::now();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            winit::event::Event::RedrawRequested(window_id) if window_id == window.id() => {
                let now = std::time::Instant::now();
                let _delta_time = now - last_frame_time;
                last_frame_time = now;

                match surface.get_current_texture() {
                    Ok(frame) => {
                        let view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());
                        renderer.render(&view, &device, &queue);
                        frame.present();
                    }
                    Err(wgpu::SurfaceError::Lost) => {
                        let size = window.inner_size();
                        config.width = size.width;
                        config.height = size.height;
                        surface.configure(&device, &config);
                        renderer.resize(&device, size.width, size.height);
                    }
                    Err(wgpu::SurfaceError::OutOfMemory) => {
                        eprintln!("Out of memory, exiting");
                        *control_flow = ControlFlow::Exit;
                    }
                    Err(e) => eprintln!("Dropped frame: {:?}", e),
                }
            }
            winit::event::Event::WindowEvent { event, window_id } if window_id == window.id() => {
                match event {
                    winit::event::WindowEvent::Resized(size) => {
                        config.width = size.width;
                        config.height = size.height;
                        surface.configure(&device, &config);
                        renderer.resize(&device, size.width, size.height);
                        window.request_redraw();
                    }
                    winit::event::WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    _ => (),
                }
            }
            _ => (),
        }
    });
}