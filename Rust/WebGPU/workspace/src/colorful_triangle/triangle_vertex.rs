#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32;3],
    pub color: [f32;3]
}

pub struct VertexWithIndex {
    pub position: [f32;3],
    pub color: [f32;3]
}

pub trait VertexBase<'a> {
    const ATTRIBS: &'a [wgpu::VertexAttribute;2];
    const VERTICES: [Vertex;3];
    fn buffer_layout() -> wgpu::VertexBufferLayout<'static>;
}

pub trait VertexWithIndexBase<'a> {
    const ATTRIBS: &'a [wgpu::VertexAttribute;2];
    const VERTICES: [Vertex;5];
    const INDICES: &'a [u16];
    fn buffer_layout() -> wgpu::VertexBufferLayout<'static>;
}



impl<'a> VertexBase<'a> for Vertex {
    const ATTRIBS: &'a [wgpu::VertexAttribute;2] =  &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3];
    const VERTICES: [Vertex;3] = [
        Vertex { position: [0.0, 0.5, 0.0], color: [1.0, 0.0, 0.0] },
        Vertex { position: [-0.5, -0.5, 0.0], color: [0.0, 1.0, 0.0] },
        Vertex { position: [0.5, -0.5, 0.0], color: [0.0, 0.0, 1.0] },
    ];
    fn buffer_layout() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout { 
            array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: Self::ATTRIBS
        }
    }
}

impl<'a> VertexWithIndexBase<'a> for VertexWithIndex {
    const ATTRIBS: &'a [wgpu::VertexAttribute;2] =  &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3];
    const VERTICES: [Vertex;5] = [
        Vertex { position: [-0.0868241, 0.49240386, 0.0], color: [0.5, 0.0, 0.5] }, // A
        Vertex { position: [-0.49513406, 0.06958647, 0.0], color: [0.5, 0.0, 0.5] }, // B
        Vertex { position: [-0.21918549, -0.44939706, 0.0], color: [0.5, 0.0, 0.5] }, // C
        Vertex { position: [0.35966998, -0.3473291, 0.0], color: [0.5, 0.0, 0.5] }, // D
        Vertex { position: [0.44147372, 0.2347359, 0.0], color: [0.5, 0.0, 0.5] }, // E
    ];
    const INDICES: &'a [u16] = &[
        0, 1, 4,
        1, 2, 4,
        2, 3, 4,
    ];
    fn buffer_layout() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout { 
            array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: Self::ATTRIBS
        }
    }
}