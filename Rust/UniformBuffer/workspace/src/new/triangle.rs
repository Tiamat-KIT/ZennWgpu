
use wgpu::util::DeviceExt;

use super::vertex::NewVertex;


pub struct NewTriangle<'a> {
    pub vertices: &'a [NewVertex;3],
    // pub indexes: &'a [u16;3]
}

impl NewTriangle<'static> {
    pub fn new(device: &wgpu::Device) -> Self {
        Self {
            vertices: &[
                NewVertex {
                    position: [0.0, 0.5, 0.0],
                    color: [1.0, 0.0, 0.0,1.0],
                    scale: [1.0,1.0],
                    offset: [1.0,1.0]
                },
                NewVertex {
                    position: [-0.5, -0.5, 0.0],
                    color: [0.0, 1.0, 0.0,1.0], 
                    scale: [1.0,1.0],
                    offset: [1.0,1.0]
                },
                NewVertex {
                    position: [0.5, -0.5, 0.0],
                    color: [0.0, 0.0, 1.0,1.0], 
                    scale: [1.0,1.0],
                    offset: [1.0,1.0]
                }
            ],
            // indexes: &[0,1,4]
        }
    }
    pub fn vertex_buffer(self,device: &wgpu::Device) -> wgpu::Buffer{
        let vertex_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Triangle Vertex Buffer"),
                contents: bytemuck::cast_slice(self.vertices),
                usage: wgpu::BufferUsages::VERTEX
            }
        );
        vertex_buffer
    }
    /* pub fn index_buffer(self,device: &wgpu::Device) -> wgpu::Buffer {
        let index_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Triangle Index Buffer"),
                contents: bytemuck::cast_slice(self.indexes),
                usage: wgpu::BufferUsages::INDEX
            }
        );
        index_buffer
    }
    pub fn index_length(self) -> u32  {
        self.indexes.len() as u32
    } */

}

pub struct Triangles<'a> {
    pub triangles: &'a [NewTriangle<'a>],
}

impl Triangles<'static> {
    pub fn new() -> Self {
        Self {
            triangles: &[
                NewTriangle {
                    vertices: &[
                        NewVertex {
                            position: [0.0, 0.5, 0.0],
                            color: [1.0, 0.0, 0.0,1.0],
                            scale: [1.0,1.0],
                            offset: [1.0,1.0]
                        },
                        NewVertex {
                            position: [-0.5, -0.5, 0.0],
                            color: [0.0, 1.0, 0.0,1.0], 
                            scale: [1.0,1.0],
                            offset: [1.0,1.0]
                        },
                        NewVertex {
                            position: [0.5, -0.5, 0.0],
                            color: [0.0, 0.0, 1.0,1.0], 
                            scale: [1.0,1.0],
                            offset: [1.0,1.0]
                        }
                    ],
                    // indexes: &[0,1,4]
                }
            ]
        }
    }
}