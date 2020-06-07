use cgmath::Point2;

use crate::{drawable::drawable::Drawable, frame::Frame, vertex::Vertex};

#[derive(Clone)]
pub struct Triangle {
    pub vertices: [Vertex; 3],
}
impl Triangle {
    pub fn new(vertices: [Point2<f32>; 3], color: [f32; 3]) -> Self {
        Self {
            vertices: [
                Vertex {
                    color,
                    position: [vertices[0].x, vertices[0].y, 0.0],
                },
                Vertex {
                    color,
                    position: [vertices[1].x, vertices[1].y, 0.0],
                },
                Vertex {
                    color,
                    position: [vertices[2].x, vertices[2].y, 0.0],
                },
            ],
        }
    }
}
impl Drawable for Triangle {
    fn draw<'a>(&mut self, frame: &'a mut Frame) {
        for i in 0..3 {
            frame.vertices.push(self.vertices[i]);
        }
    }
}
