use crate::old::old_vertex_struct::Vertex;

pub struct Triangle<'a> {
    pub vertices: &'a [Vertex]
}

impl Triangle<'static> {
    pub fn new() -> Self {
        Self {
            vertices: &[
                Vertex { 
                    position: [0.0, 0.5, 0.0],
                    color: [1.0, 0.0, 0.0] 
                },
                Vertex { 
                    position: [-0.5, -0.5, 0.0],
                    color: [0.0, 1.0, 0.0] 
                },
                Vertex { 
                    position: [0.5, -0.5, 0.0],
                    color: [0.0, 0.0, 1.0] 
                },
            ]
        }
    }
}