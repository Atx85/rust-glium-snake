
use glium::index::PrimitiveType;
use glium::Frame;
use glium::texture::SrgbTexture2d;  
use crate::vertex::ImageVertex;
use  glium::uniform;
use glium::Surface;

pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

pub struct Sprite {
    vertex_shader_src: &'static str,
    fragment_shader_src: &'static str,
    shape: Vec<ImageVertex>,
    pub translate: Vector3,
}

impl  Sprite  {
    pub fn new(_x: f32, _y: f32, _w:f32, _h:f32, _world_width:f32, _world_height: f32, sprite_left_offset: f32, sprite_top_offset: f32) -> Self {
        let sprite_w: f32 =  _w / 320.0;
        let sprite_h: f32 = _h / 256.0;
        let _world_width = _world_width ;
        let _world_height = _world_height;
        let left_offset: f32 = sprite_left_offset * (sprite_w);
        let top_offset= sprite_top_offset * sprite_h;
        Self {
            translate: Vector3 {x: _x / 640.0, y: _y / 480.0, z: 2.0},
        shape: vec![
            ImageVertex { position: [0.0, 0.0],                                tex_coords: [left_offset           , top_offset] },
            ImageVertex { position: [0.0, _world_height],                      tex_coords: [left_offset           , sprite_h +  top_offset] },
            ImageVertex { position: [0.0 + _world_width, 0.0],                 tex_coords: [left_offset + sprite_w, top_offset] },
            ImageVertex { position: [0.0 + _world_width, 0.0 + _world_height], tex_coords: [left_offset + sprite_w, sprite_h +  top_offset] }
        ],
        
        vertex_shader_src: r#"
            #version 140
    
            in vec2 position;
            in vec2 tex_coords;
            out vec2 my_attr;      // our new attribute
            out vec2 v_tex_coords;
    
            // uniform mat4 matrix;
            uniform mat4 perspective;
            uniform mat4 view;
            uniform mat4 model;
    
            void main() {
                my_attr = position;     // we need to set the value of each `out` variable.
                v_tex_coords = tex_coords;
                // gl_Position = perspective * matrix * vec4(position, 0.0, 1.0);
                mat4 modelview = view * model;
                // v_normal = transpose(inverse(mat3(modelview))) * normal;
                gl_Position = perspective * modelview * vec4(position, 0.0 , 1.0);
            }
        "#,
    
        fragment_shader_src: r#"
            #version 140
    
            in vec2 my_attr;
            in vec2 v_tex_coords;
            out vec4 color;

            uniform sampler2D tex;
    
            void main() {
                vec4 texColor = texture(tex, v_tex_coords);
                color = texture(tex, v_tex_coords); //vec4(1.0, 1.0, 1.0, 1.0);   // we build a vec4 from a vec2 and two floats
            }
        "#
        }
    }

    pub fn draw<'a>(&self, display: &glium::Display,target: &'a mut Frame,  image: &SrgbTexture2d /*std::rc::Rc<SrgbTexture2d>*/) {
        let vertex_buffer = glium::VertexBuffer::new(display, &self.shape).unwrap();
        let index_buffer = glium::IndexBuffer::new(display, PrimitiveType::TrianglesList, &[0u16, 1, 2, 1, 3, 2]).unwrap();
        let program = glium::Program::from_source(display, self.vertex_shader_src, self.fragment_shader_src, None).unwrap();

        let model = [
            [0.01, 0.0, 0.0, 0.0],
            [0.0, 0.01, 0.0, 0.0],
            [0.0, 0.0, 0.01, 0.0],
            [self.translate.x, self.translate.y, self.translate.z, 1.0f32]
        ];

        let view = view_matrix(&[0.0, 0.0 , 1.0], &[0.0, 0.0, 1.0], &[0.0, 1.0, 0.0]);

        let perspective = {
            let (width, height) = target.get_dimensions();
            let aspect_ratio = height as f32 / width as f32;
        
            let fov: f32 = 3.141592 / 3.0;
            let zfar = 1024.0;
            let znear = 0.1;
        
            let f = 1.0 / (fov / 2.0).tan();
        
            [
                [f *   aspect_ratio   ,    0.0,              0.0              ,   0.0],
                [         0.0         ,     f ,              0.0              ,   0.0],
                [         0.0         ,    0.0,  (zfar+znear)/(zfar-znear)    ,   1.0],
                [         0.0         ,    0.0, -(2.0*zfar*znear)/(zfar-znear),   0.0],
            ]
        };

        let uniforms = uniform! {
            model: model,
            view: view,
            perspective: perspective,
            tex: image
        };

        let draw_parameters = glium::DrawParameters {
            //depth_function: glium::DepthFunction::IfLessOrEqual,
            // blend: glium::Blend {
            //     color: glium::BlendingFunction::Addition {
            //         source: glium::LinearBlendingFactor::SourceAlpha,
            //         destination: glium::LinearBlendingFactor::OneMinusSourceAlpha
            //     },
            //     alpha: glium::BlendingFunction::Addition {
            //         source: glium::LinearBlendingFactor::One,
            //         destination: glium::LinearBlendingFactor::OneMinusSourceAlpha
            //     },
            //     constant_value: (0.0, 0.0, 0.0, 0.0)
            // },
            blend: glium::Blend::alpha_blending(),
            .. Default::default()
        };

        target.draw(&vertex_buffer, &index_buffer, &program, &uniforms,
            &draw_parameters).unwrap();
    }
}

fn view_matrix(position: &[f32; 3], direction: &[f32; 3], up: &[f32; 3]) -> [[f32; 4]; 4] {
    let f = {
        let f = direction;
        let len = f[0] * f[0] + f[1] * f[1] + f[2] * f[2];
        let len = len.sqrt();
        [f[0] / len, f[1] / len, f[2] / len]
    };

    let s = [up[1] * f[2] - up[2] * f[1],
             up[2] * f[0] - up[0] * f[2],
             up[0] * f[1] - up[1] * f[0]];

    let s_norm = {
        let len = s[0] * s[0] + s[1] * s[1] + s[2] * s[2];
        let len = len.sqrt();
        [s[0] / len, s[1] / len, s[2] / len]
    };

    let u = [f[1] * s_norm[2] - f[2] * s_norm[1],
             f[2] * s_norm[0] - f[0] * s_norm[2],
             f[0] * s_norm[1] - f[1] * s_norm[0]];

    let p = [-position[0] * s_norm[0] - position[1] * s_norm[1] - position[2] * s_norm[2],
             -position[0] * u[0] - position[1] * u[1] - position[2] * u[2],
             -position[0] * f[0] - position[1] * f[1] - position[2] * f[2]];

    [
        [s_norm[0], u[0], f[0], 0.0],
        [s_norm[1], u[1], f[1], 0.0],
        [s_norm[2], u[2], f[2], 0.0],
        [p[0], p[1], p[2], 1.0],
    ]
}