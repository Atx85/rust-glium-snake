
use crate::sprite::Sprite;
use glium::{Surface};
use glium::{glutin::event::{ElementState, VirtualKeyCode}, texture::SrgbTexture2d};
use std::{collections::HashMap, io::Cursor};
use crate::glutin::event::KeyboardInput;
use crate::snake::Snake;

pub struct Game {
    texture: SrgbTexture2d,
    sprites: HashMap<String, Sprite>,
    snake: Snake
}
impl  Game {
    pub fn new (display: & glium::Display) -> Game {
        let image =  image::load(Cursor::new(&include_bytes!("snake-graphics.png")),
        image::ImageFormat::Png).unwrap().to_rgba16();
        let image_dimensions = image.dimensions();
        let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
        let texture = glium::texture::SrgbTexture2d::new(display, image).unwrap();
        let mut game = Game {
            sprites: HashMap::new(),
            texture: texture,
            snake: Snake::new(0.0, 0.0)
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
            "head_right",
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

        game
    }

    pub fn add_sprites (&mut self, sprite: Sprite , sprite_name: String) {
        self.sprites.insert(sprite_name, sprite);
    }

    pub fn handle_input (&mut self, input: KeyboardInput) {
        if let ElementState::Pressed = input.state {
            match input.virtual_keycode.unwrap() {
                VirtualKeyCode::Up => {
                    self.snake.move_up();
                },
                VirtualKeyCode::Down => {
                    self.snake.move_down();
                },
                VirtualKeyCode::Left => {
                    self.snake.move_left();
                },
                VirtualKeyCode::Right => {
                    self.snake.move_right();
                },
                _=> return,
            }
        }
    }

    pub fn draw(&mut self, display :&glium::Display) {
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        self.snake.update();

        // drawing head
        if let Some(s) = self.sprites.get_mut(&self.snake.head.sprite) {
            s.translate.x = self.snake.head.x;
            s.translate.y = self.snake.head.y;
            s.draw(display, &mut target, &self.texture);
        }

        // drawing body
        for i in 0..self.snake.length {
            if let Some(s) = self.sprites.get_mut(&"body_vertical".to_string()) {
                s.translate.x = self.snake.head.x + 0.05;
                s.translate.y = self.snake.head.y + 0.0;
                s.draw(display, &mut target, &self.texture);
            }
        }
        
        // for (_name, sprite) in &self.sprites {
        //     sprite.draw(&display, &mut target, &self.texture);
        // }
        target.finish().unwrap();
    }
}