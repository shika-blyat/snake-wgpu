use crate::{frame::Frame, shapes::drawable::Drawable, vertex::Vertex};

pub struct Rectangle {
    pub vertices: [Vertex; 6],
}

impl Rectangle {
    ///     1       0
    ///   
    ///     2       3
    pub fn new(points: [(f32, f32); 4], color: [f32; 3]) -> Self {
        let vertices = [
            Vertex {
                position: [points[0].0, points[0].1, 0.0],
                color,
            },
            Vertex {
                position: [points[1].0, points[1].1, 0.0],
                color,
            },
            Vertex {
                position: [points[3].0, points[3].1, 0.0],
                color,
            },
            Vertex {
                position: [points[1].0, points[1].1, 0.0],
                color,
            },
            Vertex {
                position: [points[2].0, points[2].1, 0.0],
                color,
            },
            Vertex {
                position: [points[3].0, points[3].1, 0.0],
                color,
            },
        ];
        println!("{:#?}", vertices);
        Self { vertices }
    }
}
impl Drawable for Rectangle {
    fn draw<'a>(&mut self, frame: &'a mut Frame) {
        for i in 0..6 {
            frame.vertices.push(self.vertices[i]);
        }
    }
}
