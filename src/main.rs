mod frame;
mod renderer;
mod shapes;
mod vertex;

use futures::executor::block_on;
use wgpu::*;
use winit::{
    dpi::PhysicalSize,
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use renderer::Renderer;
use shapes::{drawable::Drawable, rectangle::Rectangle, triangle::Triangle};
use vertex::Vertex;

const VERTICES: [Vertex; 3] = [
    Vertex {
        position: [0.0, 0.5, 0.0],
        color: [1.0, 0.0, 0.0],
    },
    Vertex {
        position: [-0.5, -0.5, 0.0],
        color: [0.0, 1.0, 0.0],
    },
    Vertex {
        position: [0.5, -0.5, 0.0],
        color: [0.0, 0.0, 1.0],
    },
];

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_inner_size(PhysicalSize::new(500.0, 500.0))
        .with_resizable(true)
        .build(&event_loop)
        .unwrap();
    let mut renderer = block_on(Renderer::new(&window));
    let mut rectangle = Rectangle::new(
        [(0.5, 0.5), (-0.5, 0.5), (-0.5, -0.5), (0.5, -0.5)],
        [0.5, 0.5, 0.5],
    );
    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == window.id() => match event {
            WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
            WindowEvent::KeyboardInput { input, .. } => match input {
                KeyboardInput {
                    state: ElementState::Pressed,
                    virtual_keycode: Some(VirtualKeyCode::Escape),
                    ..
                } => *control_flow = ControlFlow::Exit,
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
            let mut frame = renderer.next_frame();
            rectangle.draw(&mut frame);
            block_on(frame.render());
        }
        Event::MainEventsCleared => {
            window.request_redraw();
        }
        _ => {}
    });
}
