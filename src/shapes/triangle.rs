use crate::{frame::Frame, shapes::drawable::Drawable, vertex::Vertex};

pub struct Triangle {
    pub vertices: [Vertex; 3],
}

impl Drawable for Triangle {
    fn draw<'a>(&mut self, frame: &'a mut Frame) {
        for i in 0..3 {
            frame.vertices.push(self.vertices[i]);
        }
    }
}
