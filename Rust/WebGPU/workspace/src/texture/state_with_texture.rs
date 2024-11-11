use wgpu::{include_wgsl, util::DeviceExt};

use crate::{
    triangle::state::StateBase,
    colorful_triangle::triangle_vertex::{VertexWithIndex, VertexWithIndexBase},
};

use super::texture;


pub struct VertexIndexWithRenderState<'a> {
    pub window: &'a winit::window::Window,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub surface: wgpu::Surface<'a>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub render_pipeline: wgpu::RenderPipeline,
    pub vertex_buffer: wgpu::Buffer,
    pub num_vertices: u32,
    pub index_buffer: wgpu::Buffer,
    pub num_indices: u32,
    pub diffuse_bind_group: wgpu::BindGroup,
    pub diffuse_texture: texture::Texture, // NEW
}


impl<'a> StateBase<'a> for VertexIndexWithRenderState<'a> {
    async fn new(window: &'a winit::window::Window) -> Self {
        let size = window.inner_size();
        
        let instance = wgpu::Instance::new(
            wgpu::InstanceDescriptor {
                #[cfg(not(target_arch = "wasm32"))]
                backends: wgpu::Backends::PRIMARY,
                #[cfg(target_arch = "wasm32")]
                backends: wgpu::Backends::BROWSER_WEBGPU,
                ..Default::default()
            }
        );
        
        let surface = instance.create_surface(window)
            .expect("Surface Create Error");

        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false
            }
        ).await.unwrap();

        let (device,queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: Some("device and queue"),
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                memory_hints: Default::default(),
            }
            , None
        ).await.unwrap();

        let surface_caps = surface.get_capabilities(&adapter);

        let surface_format = surface_caps.formats.iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        let render_pipeline_layout = device
            .create_pipeline_layout(
                &wgpu::PipelineLayoutDescriptor { 
                    label: Some("Render Pipeline"),
                    bind_group_layouts: &[],
                    push_constant_ranges: &[]
                }
            );
        
        let shader = device.create_shader_module(include_wgsl!("../colorful_triangle/colorful_shader.wgsl"));

        let render_pipeline = device.create_render_pipeline(
            &wgpu::RenderPipelineDescriptor {
                label: Some("Render Pipeline"),
                layout: Some(&render_pipeline_layout),
                vertex: wgpu::VertexState { 
                    module: &shader,
                    entry_point: "vs",
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                    buffers: &[
                        VertexWithIndex::buffer_layout()
                    ]
                },
                fragment: Some(wgpu::FragmentState { 
                    module: &shader,
                    entry_point: "fs",
                    targets: &[Some(wgpu::ColorTargetState {
                        format: config.format,
                        blend: Some(wgpu::BlendState::REPLACE),
                        write_mask: wgpu::ColorWrites::ALL
                    })],
                    compilation_options: wgpu::PipelineCompilationOptions::default()
                }),
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList, // 1.
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw, // 2.
                    cull_mode: Some(wgpu::Face::Back),
                    // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                    polygon_mode: wgpu::PolygonMode::Fill,
                    // Requires Features::DEPTH_CLIP_CONTROL
                    unclipped_depth: false,
                    // Requires Features::CONSERVATIVE_RASTERIZATION
                    conservative: false,
                },
                depth_stencil: None, // 1.
                multisample: wgpu::MultisampleState {
                    count: 1, // 2.
                    mask: !0, // 3.
                    alpha_to_coverage_enabled: false, // 4.
                },
                multiview: None, // 5.
                cache: None, // 6.
            }
        );

        let vertex_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(&VertexWithIndex::VERTICES),
                usage: wgpu::BufferUsages::VERTEX,
            }
        );

        let num_vertices = VertexWithIndex::VERTICES.len() as u32;

        let index_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(VertexWithIndex::INDICES),
                usage: wgpu::BufferUsages::INDEX,
            }
        );
        let num_indices = VertexWithIndex::INDICES.len() as u32;
        
        surface.configure(&device,&config);
        let diffuse_bytes = include_bytes!("../img/utakata.jpeg");
        let diffuse_texture = texture::Texture::from_bytes(&device, &queue, diffuse_bytes, "utakata")
            .expect("Texture Initial Error");
        let texture_bind_group_layout = device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture { 
                            sample_type: wgpu::TextureSampleType::Float { 
                                filterable: true
                            },
                            view_dimension: wgpu::TextureViewDimension::D2,
                            multisampled: false
                        },
                        count: None
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(
                            wgpu::SamplerBindingType::Filtering
                        ),
                        count: None
                    }
                ],
                label: Some("texture_bind_group_layout")
            }
        );

        let diffuse_bind_group = device.create_bind_group(
            &wgpu::BindGroupDescriptor { 
                layout: &texture_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&diffuse_texture.view)
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&diffuse_texture.sampler)
                    }
                ],
                label: Some("Diffuse_Bind_Group")
            }
        );
        

        
        Self {
            window,
            size,
            surface,
            device,
            queue,
            config,
            render_pipeline,
            vertex_buffer,
            num_vertices,
            index_buffer,
            num_indices,
            diffuse_bind_group,
            diffuse_texture
        }
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    fn input(&mut self, _event: &winit::event::WindowEvent) -> bool {
        false// todo!()
    }

    fn update(&mut self) {
        // todo!()
    }

    fn render(&mut self) -> Result<(),wgpu::SurfaceError> {
        let output = self.surface.get_current_texture().unwrap();
        let view = output.texture.create_view(
            &wgpu::TextureViewDescriptor::default()
        );
        let mut encoder = self.device.create_command_encoder(
            &wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder")
            }
        );

        let mut render_fn = || {
            let mut render_pass = encoder.begin_render_pass(
                &wgpu::RenderPassDescriptor {
                    label: Some("Render Pass"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color {
                                r: 0.1,
                                g: 0.2,
                                b: 0.3,
                                a: 1.0,
                            }),
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    depth_stencil_attachment: None,
                    occlusion_query_set: None,
                    timestamp_writes: None
                });
            
            render_pass.set_pipeline(&self.render_pipeline);
            // 追加
            render_pass.set_bind_group(0, &self.diffuse_bind_group, &[]);
            render_pass.set_vertex_buffer(0,self.vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..),wgpu::IndexFormat::Uint16);
            //render_pass.draw(0..self.num_vertices, 0..1);
            render_pass.draw_indexed(0..self.num_indices, 0,0..1);
        };
    
        render_fn();

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
        Ok(())
    }
}

