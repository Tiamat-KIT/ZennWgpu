use std::{num::NonZeroU64};

pub struct State<'a> {
    pub surface: wgpu::Surface<'a>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub render_pipeline: wgpu::RenderPipeline
}

impl<'a> State<'a> {
    pub async fn new() {
        let instance = wgpu::Instance::new(
            wgpu::InstanceDescriptor { 
                backends: wgpu::Backends::all(), 
                ..Default::default()
            }
        );

        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptionsBase {
                power_preference: wgpu::PowerPreference::default(),
                force_fallback_adapter: false,
                ..Default::default()
            }
        ).await.unwrap();

        let (device,queue) = adapter.request_device(&wgpu::DeviceDescriptor {
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                label: Some("GPU Device"),
                memory_hints: Default::default()
            },
            None
        ).await.unwrap();

        let input_buffer:[u8;3] = [1,3,5];
        let work_buffer  = device.create_buffer(&wgpu::BufferDescriptor{
            label: Some("Compute Shader Calc Input Buffer"),
            size: input_buffer.len() as u64,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false
        });

        let result_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Compute Shadeer Calc Result Buffer"),
            size: input_buffer.len() as u64,
            usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false
        });

        queue.write_buffer(&work_buffer, 0, &input_buffer);
        let bindgroup = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("bindgroup for work buffer"),
            layout: todo!(),
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                        buffer: &work_buffer,
                        offset: 0,
                        size: NonZeroU64::new(input_buffer.len() as u64)
                    })
                }
            ]
        });

        let encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Conpute Shader Encoder"),
        });

        let pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor { 
            label: Some("Compute Pass"),
            timestamp_writes: todo!(), 
            // timestamp_writes: Some(wgpu::ComputePassTimestampWrites{})
        });

        encoder.copy_buffer_to_buffer(&work_buffer, 0, &result_buffer, 0,result_buffer.size());
        let command_buffer = encoder.finish();
        queue.submit([command_buffer]);
        
    }
} 