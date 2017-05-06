#[macro_use]
extern crate glium;
extern crate regex;
#[macro_use]
extern crate lazy_static;

mod shader;

use glium::DisplayBuild;
use glium::Surface;
use glium::Texture2d;

use std::time::Instant;

#[derive(Clone, Copy)]
struct Vertex {
    position: [f32; 3],
}
implement_vertex!(Vertex, position);


#[derive(Debug)]
struct State {
    size: (f32, f32),
    cursor: (f32, f32),
    start_time: Instant,
}

impl State {
    pub fn new(size: (u32, u32)) -> State {
        State {
            start_time: Instant::now(),
            cursor: (0., 0.),
            size: (size.0 as f32, size.1 as f32),
        }
    }

    pub fn handle_event(&mut self, event: glium::glutin::Event) {
            match event {
                glium::glutin::Event::MouseMoved(x,y) => {
                    self.cursor = (x as f32, y as f32);
                },
                glium::glutin::Event::Resized(x,y) => {
                    self.size = (x as f32, y as f32);
                },
                _=> ()
            }
    }
}

impl glium::uniforms::Uniforms for State {
    fn visit_values<'a, F: FnMut(&str, glium::uniforms::UniformValue<'a>)>(&'a self, mut output: F) {
        use glium::uniforms::AsUniformValue;

        output("iResolution", self.size.as_uniform_value());

        let cursor = [self.cursor.0, self.size.1 - self.cursor.1];
        output("iMouse", glium::uniforms::UniformValue::Vec2(cursor));

        let elapsed = self.start_time.elapsed();
        let time = elapsed.as_secs() as f32 + elapsed.subsec_nanos() as f32/ 1000000000.0;
        output("iGlobalTime", glium::uniforms::UniformValue::Float(time as f32));
    }
}

struct Pass<'a> {
    channels: Vec<&'a Texture2d>,
}

impl<'a> Pass<'a> {
    //TODO new
    //TODO add_channel
}

impl<'b> glium::uniforms::Uniforms for Pass<'b> {
    fn visit_values<'a, F: FnMut(&str, glium::uniforms::UniformValue<'a>)>(&'a self, mut output: F) {
        use glium::uniforms::AsUniformValue;

        for(index, channel) in self.channels.iter().enumerate() {
            output(format!("iChannel{}",index).as_str(), channel.as_uniform_value());
        }
    }
}

struct CombinedState<'a, 'b> {
    state: &'a State,
    pass: &'b Pass<'b>,
}

impl<'a, 'b> CombinedState<'a, 'b> {
    fn combine(state: &'a State, pass: &'b Pass<'b>) -> CombinedState<'a, 'b> {
        CombinedState {
            state: state,
            pass: pass,
        }
    }
}

impl<'c, 'd> glium::uniforms::Uniforms for CombinedState<'c, 'd> {
    fn visit_values<'a, F: FnMut(&str, glium::uniforms::UniformValue<'a>)>(&'a self, mut output: F) {
        self.state.visit_values(|name, value| { output(name, value) });
        self.pass.visit_values(|name, value| { output(name, value) });
    }
}

fn main() {
    let mut r = shader::Resolver::new();
    //TODO glob
    r.push("toy.frag");
    r.push("shader.vert");
    r.push("shader.frag");

    let display = glium::glutin::WindowBuilder::new().build_glium().unwrap();


    let texture1 = glium::texture::Texture2d::empty_with_format(&display,
                                                                glium::texture::UncompressedFloatFormat::F32F32F32F32,
                                                                glium::texture::MipmapsOption::NoMipmap,
                                                                800,
                                                                500
                                                                ).unwrap();
    let mut tex_buffer = glium::framebuffer::SimpleFrameBuffer::new(&display, &texture1).unwrap();
    let pass = Pass {
        channels: vec![&texture1],
    };

    // Fullscreen quad
    let shape = vec![
        Vertex { position: [-1.0, -1.0, 0.0] },
        Vertex { position: [1.0, 1.0, 0.0] },
        Vertex { position: [1.0, -1.0, 0.0] },

        Vertex { position: [-1.0, -1.0, 0.0] },
        Vertex { position: [1.0, 1.0, 0.0] },
        Vertex { position: [-1.0, 1.0, 0.0] },
    ];

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vertex_shader_src= r.resolve("shader.vert").unwrap();
    let fragment_shader_src = r.resolve("shader.frag").unwrap();

    let program = glium::Program::from_source(&display, &vertex_shader_src, &fragment_shader_src, None).unwrap();

    let mut state = State::new(display.get_framebuffer_dimensions());

    loop {
        tex_buffer.clear_color(1.0, 1.0, 0.0, 1.0);
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        target.draw(&vertex_buffer, &indices, &program, &CombinedState::combine(&state, &pass), &Default::default()).unwrap();
        target.finish().unwrap();

        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return,
                glium::glutin::Event::MouseMoved(_,_) => state.handle_event(ev),
                glium::glutin::Event::Resized(_,_) => state.handle_event(ev),
                _=> ()
            }
        }
    }
}
