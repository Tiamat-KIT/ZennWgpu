use bytemuck::{Pod,Zeroable};
use wgpu::{util::DeviceExt, Buffer};

/**
 * Uniform Buffer用の構造体を宣言
 */

#[repr(C)]
#[derive(Clone,Copy,Pod,Zeroable)]

pub struct UniformBufferStruct {
    pub color: [f32;4],
    pub scale: [f32;2],
    pub offset: [f32;2]
}

impl UniformBufferStruct {
    /**
     * 構造体値のデータの新規作成
     */
    pub fn new(
        color: [f32;4],
        scale: [f32;2],
        offset: [f32;2]
    ) -> Self {
        Self {
            color,
            scale,
            offset
        }
    }

    pub fn get_uniform_buffer_size(self) -> u64 {
        let uniform_buffer_size = 
            4 * 4 + // color size
            2 * 4 + // scale size
            2 * 4; // offset size
        uniform_buffer_size
    }
    

    pub fn set_color(&mut self,new_color:[f32;4]) {
        self.color = new_color
    }

    pub fn set_scale(&mut self,new_scale:[f32;2]) {
        self.scale = new_scale
    }

    pub fn set_offset(&mut self,new_offset:[f32;2]) {
        self.offset = new_offset
    }

    pub fn get_uniform_buffer(self,device: &wgpu::Device) -> Buffer {
        let uniform_buffer = device.create_buffer(
            &wgpu::BufferDescriptor {
                label: Some("Triangle Uniform Buffer"),
                size: self.get_uniform_buffer_size(),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false
            }
        );
        uniform_buffer
    }

    const ATTRIBS: [wgpu::VertexAttribute;3] = 
        wgpu::vertex_attr_array![0 => Float32x4,1 => Float32x2,2 => Float32x2];

    pub fn desc() -> wgpu::VertexBufferLayout<'static>{
        use std::mem;

        wgpu::VertexBufferLayout { 
            array_stride: mem::size_of::<Self>() as wgpu::BufferAddress, 
            step_mode: wgpu::VertexStepMode::Vertex, 
            attributes:  &Self::ATTRIBS
        }
    }
}



pub struct UniformAndIndexBuffers {
    pub buffers: Vec<UniformAndIndexBuffers>,
    pub indexes: Vec<u16>
}


