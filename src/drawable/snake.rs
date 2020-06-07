use std::collections::VecDeque;

use cgmath::Point2;

use super::{drawable::Drawable, rectangle::Rectangle};
use crate::frame::Frame;
pub enum Orientation {
    Top,
    Left,
    Right,
    Bot,
}
pub struct Snake {
    positions: VecDeque<Rectangle>,
    orientation: Orientation,
}

impl Snake {
    pub fn new() -> Self {
        let mut positions = VecDeque::new();
        positions.push_front(Rectangle::new(
            [
                Point2::new(0.0, 0.2),
                Point2::new(-0.1, 0.2),
                Point2::new(-0.1, 0.1),
                Point2::new(0.0, 0.1),
            ],
            [1.0, 0.0, 0.0],
        ));
        positions.push_front(Rectangle::new(
            [
                Point2::new(0.0, 0.1),
                Point2::new(-0.1, 0.1),
                Point2::new(-0.1, 0.0),
                Point2::new(0.0, 0.0),
            ],
            [1.0, 0.0, 0.0],
        ));
        Self {
            positions,
            orientation: Orientation::Top,
        }
    }
    pub fn go_to(&mut self, orientation: Orientation) {
        self.orientation = orientation;
    }
    pub fn move_(&mut self) {
        self.positions.pop_back();
        let new_head = self.positions[0]
            .clone()
            .map(|rect| match self.orientation {
                Orientation::Top => {
                    if rect.vertices[0].position[1] >= 1.0 {
                        rect.map_y(|y| y - 1.9)
                    } else {
                        rect.map_y(|y| y + 0.1)
                    }
                }
                Orientation::Bot => {
                    if rect.vertices[0].position[1] <= -1.0 {
                        rect.map_y(|y| y + 1.9)
                    } else {
                        rect.map_y(|y| y - 0.1)
                    }
                }
                Orientation::Right => {
                    if rect.vertices[0].position[0] >= 1.0 {
                        rect.map_x(|x| x - 1.9)
                    } else {
                        rect.map_x(|x| x + 0.1)
                    }
                }
                Orientation::Left => {
                    if rect.vertices[0].position[0] <= -1.0 {
                        rect.map_x(|x| x + 1.9)
                    } else {
                        rect.map_x(|x| x - 0.1)
                    }
                }
            });
        self.positions.push_front(new_head);
    }
}

impl Drawable for Snake {
    fn draw<'a>(&mut self, frame: &'a mut Frame) {
        for rect in self.positions.iter_mut() {
            rect.draw(frame)
        }
    }
}
