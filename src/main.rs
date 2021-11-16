extern crate glium;
extern crate image;
mod vertex;
mod sprite;

use crate::sprite::Sprite;
use glium::{glutin, Surface};
use glium::{glutin::event::{ElementState, WindowEvent, VirtualKeyCode}, texture::SrgbTexture2d};
use std::{collections::HashMap, io::Cursor};
use crate::glutin::event::KeyboardInput;

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
    // let next_frame_time = std::time::Instant::now() +
    // std::time::Duration::from_nanos(16_666_667);
    // *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
    
    
    });
}

struct Game {
    texture: SrgbTexture2d,//std::rc::Rc<SrgbTexture2d>,
    sprites: HashMap<String, Sprite>
}
impl  Game {
    fn new (display: & glium::Display) -> Game {
        let image =  image::load(Cursor::new(&include_bytes!("snake-graphics.png")),
        image::ImageFormat::Png).unwrap().to_rgba16();
        let image_dimensions = image.dimensions();
        let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
        let texture = glium::texture::SrgbTexture2d::new(display, image).unwrap();
        let mut game = Game {
            sprites: HashMap::new(),
            texture: texture//std::rc::Rc::new(texture),
        };
        let names = vec![
            "apple",
            "empty",
            "empty",
            "tail_left",
            "tail_down",
            "empty",
            "empty",
            "body_turn_lower_right",
            "tail_up",
            "tail_right",
            "body_turn_lower_left",
            "empty",
            "body_vertical",
            "head_left",
            "head_down",
            "body_turn_upper_left",
            "body_horizontal",
            "body_turn_upper_right",
            "head_up",
            "head_head_right",
        ];

        for (pos, el) in names.iter().enumerate() {
            if (*el) != "empty" {       
                let left_offset =(pos as f32) % 5.0;  
                let top_offset = ((pos as f32) - ((pos as f32) % 5.0)) / 5.0 ;      
                
                game.add_sprites(Sprite::new( left_offset * 70.0, 
                    -100.0 + top_offset * 70.0, 
                    64.0, 
                    64.0,
                    6.0,
                    6.0,
                    left_offset,
                     top_offset,
                    ), (*el).to_string());
            }
        }


        let s2 = Sprite::new( 120.0, 
            -80.0, 
            64.0, 
            64.0,
            6.0,
            6.0,
             4.0,
             3.0
            );
        game.add_sprites(s2, "head".to_string());

        game
    }

    fn add_sprites (&mut self, sprite: Sprite , sprite_name: String) {
        self.sprites.insert(sprite_name, sprite);
    }

    fn handle_input (&mut self, input: KeyboardInput) {
        if let ElementState::Pressed = input.state {
            match input.virtual_keycode.unwrap() {
                VirtualKeyCode::Up => {
                    if let Some(head) = self.sprites.get_mut(&"head".to_string()) {
                        head.translate.y += 0.01;
                    }
                },
                VirtualKeyCode::Down => {
                    if let Some(head) = self.sprites.get_mut(&"head".to_string()) {
                        head.translate.y -= 0.01;
                    }
                },
                VirtualKeyCode::Left => {
                    if let Some(head) = self.sprites.get_mut(&"head".to_string()) {
                        head.translate.x -= 0.01;
                    }
                },
                VirtualKeyCode::Right => {
                    if let Some(head) = self.sprites.get_mut(&"head".to_string()) {
                        head.translate.x += 0.01;
                    }
                },
                _=> return,
            }
        }
    }

    fn draw(&mut self, display :&glium::Display) {
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        
        for (_name, sprite) in &self.sprites {
            sprite.draw(&display, &mut target, &self.texture);
        }
        target.finish().unwrap();
    }
}