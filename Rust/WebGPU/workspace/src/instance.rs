use wgpu::{util::DeviceExt, BufferAddress, VertexAttribute};
use cgmath::Zero;

pub struct Instance {
    pub position: cgmath::Vector3<f32>,
    pub rotation: cgmath::Quaternion<f32>
}

#[repr(C)]
#[derive(Debug,Clone, Copy,bytemuck::Pod,bytemuck::Zeroable)]
pub struct InstanceRaw {
    model: [[f32;4];4]
}

impl Instance {
    fn to_raw(&self) -> InstanceRaw {
        InstanceRaw {
            model: (cgmath::Matrix4::from_translation(self.position) * cgmath::Matrix4::from(self.rotation)).into()
        }
    }

    fn get_instance(device: &wgpu::Device) -> (Vec<Instance>,Vec<InstanceRaw>,wgpu::Buffer){
        let instances = (0 .. Self::NUM_INSTANCES_PER_ROW).flat_map(|z| {
            (0 .. Self::NUM_INSTANCES_PER_ROW).map(move |x| {
                let position = cgmath::Vector3 {
                    x: x as f32,
                    y: 0.0,
                    z: z as f32
                } - Self::INSTANCE_DISPLACEMENT;

                let rotation = if position.is_zero() {
                    <cgmath::Quaternion<f32> as cgmath::Rotation3>::from_axis_angle(cgmath::Vector3::unit_z(), cgmath::Deg(0.0))
                } else {
                    <cgmath::Quaternion<f32> as cgmath::Rotation3>::from_axis_angle(cgmath::InnerSpace::normalize(position), cgmath::Deg(45.0))
                };

                Instance {
                    position,rotation
                }
            })
        }).collect::<Vec<_>>();

        let instance_data = instances.iter().map(Instance::to_raw).collect::<Vec<_>>();

        let instance_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Instance Buffer"),
                contents: bytemuck::cast_slice(&instance_data),
                usage: wgpu::BufferUsages::VERTEX
            }
        );
        return (instances,instance_data,instance_buffer)
    }
    pub const VERTEX_BUFFER_LAYOUT: wgpu::VertexBufferLayout<'_> = wgpu::VertexBufferLayout { 
        array_stride: std::mem::size_of::<InstanceRaw>() as BufferAddress,
        step_mode: wgpu::VertexStepMode::Instance,
        attributes: &wgpu::vertex_attr_array![
            0 => Float32x4,
            1 => Float32x4,
            2 => Float32x4,
            3 => Float32x4
        ]
    };
    pub const NUM_INSTANCES_PER_ROW: u32 = 10;
    pub const INSTANCE_DISPLACEMENT: cgmath::Vector3<f32> = cgmath::Vector3::new(Self::NUM_INSTANCES_PER_ROW as f32 * 0.5,0.0,Self::NUM_INSTANCES_PER_ROW as f32 * 0.5);
}
