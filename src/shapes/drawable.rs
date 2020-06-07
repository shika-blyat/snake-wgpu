use crate::frame::Frame;
pub trait Drawable {
    fn draw<'a>(&mut self, frame: &'a mut Frame);
}
