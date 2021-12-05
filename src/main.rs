extern crate glium;
extern crate image;
mod vertex;
mod sprite;
mod snake;
mod game;

use glium::glutin;
use glium::{glutin::event::WindowEvent};
use crate::game::Game;

fn main() {
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new()
    .with_title("Rust-Glium-Snake")
    .with_inner_size(glutin::dpi::LogicalSize::new(640.0, 480.0));
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let mut game = Game::new(&display);
    event_loop.run(move |event, _, control_flow| {
            match event {
                glutin::event::Event::WindowEvent { event, .. } => match event {
                    glutin::event::WindowEvent::CloseRequested => {
                        *control_flow = glutin::event_loop::ControlFlow::Exit;
                        return;
                    },
                    WindowEvent::KeyboardInput { input, .. } => {
                        game.handle_input(input);
                    },
                    _ => return,
                },
                glutin::event::Event::NewEvents(cause) => match cause {
                    glutin::event::StartCause::ResumeTimeReached { .. } => (),
                    glutin::event::StartCause::Init => (),
                    _ => return,
                },
                glutin::event::Event::MainEventsCleared =>{
                    game.draw(&display);
                }
                _ => return,
            }
    let next_frame_time = std::time::Instant::now() +
    std::time::Duration::from_nanos(16_666_667);
    *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
    });
}