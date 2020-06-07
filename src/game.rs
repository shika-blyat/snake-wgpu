use super::{
    drawable::{
        drawable::Drawable,
        snake::{Orientation, Snake},
    },
    frame::Frame,
};

use std::time::{Duration, Instant};

pub struct Game {
    elasped_time: Instant,
    snake: Snake,
    speed: u64,
}
impl Game {
    pub fn new() -> Self {
        Self {
            elasped_time: Instant::now(),
            snake: Snake::new(),
            speed: 1,
        }
    }
    pub fn update(&mut self) {
        while self.elasped_time.elapsed() >= Duration::from_millis(500 / self.speed) {
            self.snake.move_();
            self.elasped_time = self
                .elasped_time
                .checked_add(Duration::from_millis(500 / self.speed))
                .unwrap();
        }
    }
    pub fn speed_up(&mut self) {
        if self.speed <= 20 {
            self.speed += 1;
        }
    }
    pub fn draw<'a>(&mut self, frame: &'a mut Frame) {
        self.snake.draw(frame);
    }
    pub fn snake_go_to(&mut self, go_to: Orientation) {
        self.snake.go_to(go_to)
    }
}
