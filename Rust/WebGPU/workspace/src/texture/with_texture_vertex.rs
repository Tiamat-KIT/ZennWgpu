#[repr(C)]
#[derive(Debug,Clone, Copy,bytemuck::Pod,bytemuck::Zeroable)]
pub struct TextureWithVertex {
    pub position: [f32;3],
    pub tex_coords: [f32;2]
}

impl TextureWithVertex {
    // Changed
    const VERTICES: &[TextureWithVertex] = &[
        TextureWithVertex { position: [-0.0868241, 0.49240386, 0.0], tex_coords: [0.4131759, 0.99240386], }, // A
        TextureWithVertex { position: [-0.49513406, 0.06958647, 0.0], tex_coords: [0.0048659444, 0.56958647], }, // B
        TextureWithVertex { position: [-0.21918549, -0.44939706, 0.0], tex_coords: [0.28081453, 0.05060294], }, // C
        TextureWithVertex { position: [0.35966998, -0.3473291, 0.0], tex_coords: [0.85967, 0.1526709], }, // D
        TextureWithVertex { position: [0.44147372, 0.2347359, 0.0], tex_coords: [0.9414737, 0.7347359], }, // E
    ];

    const VERTX_BUFFER_LAYOUT: wgpu::VertexBufferLayout<'static> = wgpu::VertexBufferLayout {
        array_stride: std::mem::size_of::<TextureWithVertex>() as wgpu::BufferAddress,
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x2]
    };
    
}