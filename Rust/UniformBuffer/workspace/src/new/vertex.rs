use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Copy,Clone,Debug,bytemuck::Pod,bytemuck::Zeroable)]
pub struct NewVertex {
    pub position: [f32;3],
    pub color: [f32;4],
    pub scale: [f32;2],
    pub offset: [f32;2]
}

impl NewVertex {
    const ATTRIBS: [wgpu::VertexAttribute;4] = 
        wgpu::vertex_attr_array![
            0 => Float32x3,
            1 => Float32x4,
            2 => Float32x2,
            3 => Float32x2
        ];

    pub fn vertex_buffer_layout() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;
        wgpu::VertexBufferLayout { 
            array_stride: mem::size_of::<Self>() as wgpu::BufferAddress, 
            step_mode: wgpu::VertexStepMode::Vertex, 
            attributes: &Self::ATTRIBS 
        }
    }
}