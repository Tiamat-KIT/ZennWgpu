use wgpu::include_wgsl;

use super::{triangle::{self, NewTriangle}, vertex::NewVertex};

pub struct State<'a>{ 
    pub surface: wgpu::Surface<'a>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub window: &'a winit::window::Window,
    pub render_pipeline: wgpu::RenderPipeline,
    pub vertex_buffer: wgpu::Buffer,
    pub num_vertices: u32
}

impl<'a> State<'a> {
    pub async fn new(window: &'a winit::window::Window) -> State<'a> {
        let size = window.inner_size();
        let instance_descriptor = wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        };

        let instance = wgpu::Instance::new(
            instance_descriptor
        );
        
        let surface = instance.create_surface(
            window
        ).expect("Surface Create Error");

        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptionsBase {
                power_preference: wgpu::PowerPreference::default(),
                force_fallback_adapter: false,
                compatible_surface: Some(&surface)
            }
        ).await.expect("Request Adapter Error");

        let (device,queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                label: Some("Device And Queue"),
                memory_hints: Default::default()
            },
            None
        ).await.expect("Device And Queue SetUp Error");

        let surface_caps  = surface.get_capabilities(&adapter);

        let surface_format = surface_caps.formats.iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);

        let shader = device.create_shader_module(
            wgpu::ShaderModuleDescriptor { 
                label: Some("Shader Module"), 
                source: wgpu::ShaderSource::Wgsl(include_str!("../uniform-shader.wgsl").into())
            }
        );

        let triangle = NewTriangle::new(&device);
        let vertices = triangle.vertices;
        let vertex_buffer = triangle.vertex_buffer(&device);
        let num_vertices = vertices.len() as u32;

        let render_pipeline_layout = device.create_pipeline_layout(
            &wgpu::PipelineLayoutDescriptor { 
                label: Some("Render Pipeline Layout"), 
                bind_group_layouts: &[], 
                push_constant_ranges: &[] 
            }
        );

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2
        };

        let render_pipeline = device.create_render_pipeline(
            &wgpu::RenderPipelineDescriptor {
                label: Some("RenderPipeline"),
                layout: Some(&render_pipeline_layout),
                vertex: wgpu::VertexState { 
                    module: &shader, 
                    entry_point: "vs", 
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                    buffers: &[NewVertex::vertex_buffer_layout()] 
                },
                primitive: wgpu::PrimitiveState { 
                    topology: wgpu::PrimitiveTopology::TriangleList, 
                    strip_index_format: None, 
                    front_face: wgpu::FrontFace::Ccw, 
                    cull_mode: Some(wgpu::Face::Back), 
                    unclipped_depth: false, 
                    polygon_mode: wgpu::PolygonMode::Fill, 
                    conservative: false 
                },
                depth_stencil: None,
                multisample: wgpu::MultisampleState { 
                    count: 1, 
                    mask: 0, 
                    alpha_to_coverage_enabled: false 
                },
                fragment: Some(
                    wgpu::FragmentState { 
                        module: &shader,
                        entry_point: "fs", 
                        compilation_options: wgpu::PipelineCompilationOptions::default(), 
                        targets: &[Some(wgpu::ColorTargetState { 
                            format: config.format, 
                            blend: Some(wgpu::BlendState::REPLACE), 
                            write_mask: wgpu::ColorWrites::ALL
                        })] 
                    }
                ),
                multiview: None,
                cache: None,
            }
        );
        
        Self {
            surface,
            device,
            queue,
            config,
            size,
            window,
            render_pipeline,
            vertex_buffer,
            num_vertices
        }
    }
    pub fn render(&mut self) -> Result<(),wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()
            .expect("CurrentText Get Error");
        let view = output.texture.create_view(
            &wgpu::TextureViewDescriptor {
                label: Some("TextureView"),
                ..Default::default()
            }
        );

        let mut encoder = self.device.create_command_encoder(
            &wgpu::CommandEncoderDescriptor {
                label: Some("Command Encoder")
            }
        );
        {
            let mut render_pass = encoder.begin_render_pass(
                &wgpu::RenderPassDescriptor {
                    label: Some("RenderPass"),
                    color_attachments: &[
                        Some(wgpu::RenderPassColorAttachment { 
                            view: &view, 
                            resolve_target: None, 
                            ops: wgpu::Operations { 
                                load: wgpu::LoadOp::Clear(
                                    wgpu::Color { 
                                        r: 0.3,
                                        g: 0.3,
                                        b: 0.3,
                                        a: 1.0
                                    }
                                ), 
                                store: wgpu::StoreOp::Store
                            } 
                        })
                    ],
                    depth_stencil_attachment: None,
                    timestamp_writes: None,
                    occlusion_query_set: None,
                }
            );
            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_vertex_buffer(0,self.vertex_buffer.slice(..));
            render_pass.draw(0..self.num_vertices, 0..1);

            
        }
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
        Ok(())
    }
}
