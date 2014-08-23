#![feature(globs)]

extern crate piston;

extern crate hgl;
extern crate gl;
extern crate sdl2_game_window;

use sdl2_game_window::GameWindowSDL2 as Window;
use piston::{
    GameIterator,
    GameIteratorSettings,
    GameWindowSettings, 
    Render,
    RenderArgs
};

use std::mem::size_of;
use hgl::{Shader, Program, Triangles, Vbo, Vao};

#[allow(dead_code)]
pub struct App {
    program: Program,
    vao: Vao,
    vbo: Vbo
}

static VERTEX_SHADER: &'static str = r"
#version 330
in vec2 position;
    
void main() {
    gl_Position = vec4(position, 0.0, 1.0);
}
";

static FRAGMENT_SHADER: &'static str = r"
#version 330
out vec4 out_color;

void main() {
    out_color = vec4(1.0, 0.0, 0.0, 1.0);
}
";

impl App {
    /// Creates a new application.
    pub fn new() -> App {
        let vao = Vao::new();
        vao.bind();

        let program = Program::link([Shader::compile(VERTEX_SHADER, hgl::VertexShader),
        Shader::compile(FRAGMENT_SHADER, hgl::FragmentShader)]).unwrap();
        program.bind_frag(0, "out_color");
        program.bind();

        let vbo = Vbo::from_data([
            0.0f32, 0.5, 1.0, 0.0, 0.0,
            0.5,   -0.5, 0.0, 1.0, 0.0,
            -0.5,  -0.5, 0.0, 0.0, 1.0
        ], hgl::StaticDraw);

        vao.enable_attrib(&program, "position", gl::FLOAT, 2, 5*size_of::<f32>() as i32, 0);
        vao.enable_attrib(&program, "color", gl::FLOAT, 3, 5*size_of::<f32>() as i32, 2*size_of::<f32>());
        vbo.bind();

        App {
            program: program,
            vao: vao,
            vbo: vbo
        }
    }
    
    fn render(&mut self, args: &RenderArgs) {
        gl::Viewport(0, 0, args.width as i32, args.height as i32);
        gl::ClearColor(0.0, 0.0, 0.0, 0.1);
        gl::Clear(gl::COLOR_BUFFER_BIT);
        self.vao.draw_array(Triangles, 0, 3);
    }
}

fn main() {
    let mut window = Window::new(
        piston::shader_version::opengl::OpenGL_3_2,
        GameWindowSettings {
            title: "Test".to_string(),
            size: [800, 600],
            fullscreen: false,
            exit_on_esc: true
        }
    );

    let mut app = App::new();
    let game_iter_settings = GameIteratorSettings {
            updates_per_second: 120,
            max_frames_per_second: 60,
        };
    for e in GameIterator::new(&mut window, &game_iter_settings) {
        match e {
            Render(args) => app.render(&args),
            _ => {}
        }
    }
}

