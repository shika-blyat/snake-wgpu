use crate::{drawable::drawable::Drawable, frame::Frame, vertex::Vertex};

use cgmath::Point2;

#[derive(Clone)]
pub struct Rectangle {
    pub vertices: [Vertex; 6],
}

impl Rectangle {
    ///     1       0
    ///   
    ///     2       3
    pub fn new(points: [Point2<f32>; 4], color: [f32; 3]) -> Self {
        let vertices = [
            Vertex {
                position: [points[0].x, points[0].y, 0.0],
                color,
            },
            Vertex {
                position: [points[1].x, points[1].y, 0.0],
                color,
            },
            Vertex {
                position: [points[3].x, points[3].y, 0.0],
                color,
            },
            Vertex {
                position: [points[1].x, points[1].y, 0.0],
                color,
            },
            Vertex {
                position: [points[2].x, points[2].y, 0.0],
                color,
            },
            Vertex {
                position: [points[3].x, points[3].y, 0.0],
                color,
            },
        ];
        Self { vertices }
    }

    pub fn map(mut self, f: impl Fn(Rectangle) -> Rectangle) -> Self {
        f(self)
    }

    pub fn map_x(mut self, f: impl Fn(f32) -> f32) -> Self {
        for v in self.vertices.iter_mut() {
            v.position[0] = f(v.position[0]);
        }
        self
    }
    pub fn map_y(mut self, f: impl Fn(f32) -> f32) -> Self {
        for v in self.vertices.iter_mut() {
            v.position[1] = f(v.position[1]);
        }
        self
    }
}
impl Drawable for Rectangle {
    fn draw<'a>(&mut self, frame: &'a mut Frame) {
        for i in 0..6 {
            frame.vertices.push(self.vertices[i]);
        }
    }
}
