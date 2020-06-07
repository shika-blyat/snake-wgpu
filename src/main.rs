mod drawable;
mod frame;
mod game;
mod renderer;
mod vertex;

use futures::executor::block_on;
use winit::{
    dpi::PhysicalSize,
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use drawable::{
    drawable::Drawable,
    snake::{Orientation, Snake},
};
use game::Game;
use renderer::Renderer;

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_inner_size(PhysicalSize::new(500.0, 500.0))
        .with_resizable(true)
        .build(&event_loop)
        .unwrap();
    let mut renderer = block_on(Renderer::new(&window));
    let mut game = Game::new();

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == window.id() => match event {
            WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
            WindowEvent::KeyboardInput { input, .. } => match input {
                KeyboardInput {
                    state: ElementState::Pressed,
                    virtual_keycode: Some(key_code),
                    ..
                } => match key_code {
                    VirtualKeyCode::Escape => *control_flow = ControlFlow::Exit,
                    VirtualKeyCode::Up => game.snake_go_to(Orientation::Top),
                    VirtualKeyCode::Right => game.snake_go_to(Orientation::Right),
                    VirtualKeyCode::Left => game.snake_go_to(Orientation::Left),
                    VirtualKeyCode::Down => game.snake_go_to(Orientation::Bot),
                    _ => (),
                },
                _ => {}
            },
            WindowEvent::Resized(physical_size) => {
                renderer.resize(*physical_size);
            }
            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                renderer.resize(**new_inner_size);
            }
            _ => {}
        },
        Event::RedrawRequested(_) => {
            game.update();
            let mut frame = renderer.next_frame();
            game.draw(&mut frame);
            block_on(frame.render());
        }
        Event::MainEventsCleared => {
            window.request_redraw();
        }
        _ => {}
    });
}
