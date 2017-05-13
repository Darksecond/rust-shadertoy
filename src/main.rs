#[macro_use]
extern crate glium;
extern crate regex;
#[macro_use]
extern crate lazy_static;
extern crate rust_rocket;

mod shader;

use glium::DisplayBuild;
use glium::Surface;
use glium::Texture2d;
use glium::uniforms::Uniforms;
use glium::Program;

use std::time::Instant;

use rust_rocket::{Rocket, Event};

#[derive(Clone, Copy)]
struct Vertex {
    position: [f32; 3],
}
implement_vertex!(Vertex, position);


struct Sync {
    start_time: Instant,
    rocket: Rocket,
    paused: bool,
    row: u32,
}

impl Sync {
    pub fn new() -> Sync {
        Sync {
            start_time: Instant::now(),
            rocket: Rocket::new().unwrap(),
            paused: true,
            row: 0,
        }
    }

    pub fn track(&mut self, name: &str) {
        self.rocket.get_track_mut(name);
    }

    pub fn update(&mut self) {
        while let Some(event) = self.rocket.poll_events() {
            match event {
                Event::Pause(flag) => {
                    self.paused = flag;
                },
                Event::SetRow(row) => self.row = row,
                _ => (),
            }
        }

        let elapsed = self.start_time.elapsed();
        let time = elapsed.as_secs() as f32 + elapsed.subsec_nanos() as f32/ 1000000000.0;
        let bpm = 150.0;
        let rpb = 8.;
        let row_rate = (bpm / 60.) * rpb;
        let row_count = time * row_rate;

        if(row_count > 1.) {
            self.start_time = Instant::now();
            if(!self.paused) {
                self.row = self.row + (row_count.floor() as u32);
                self.rocket.set_row(self.row);
            }
        }
    }

    pub fn val(&self, name: &str) -> f32 {
        self.rocket.get_track(name).get_value(self.row as f32)
    }
}

impl glium::uniforms::Uniforms for Sync {
    fn visit_values<'a, F: FnMut(&str, glium::uniforms::UniformValue<'a>)>(&'a self, mut output: F) {
        use glium::uniforms::AsUniformValue;

        //TODO iGlobalTime
        output("color_r", glium::uniforms::UniformValue::Float(self.val("color:r")));
        output("color_g", glium::uniforms::UniformValue::Float(self.val("color:g")));
        output("color_b", glium::uniforms::UniformValue::Float(self.val("color:b")));
    }
}

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
    program: &'a Program,
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

struct CombinedUniforms2<'a, 'b, A: 'a + Uniforms, B: 'b + Uniforms> {
    first: &'a A,
    second: &'b B,
}
impl<'c, 'd, C: 'c + Uniforms, D: 'd + Uniforms> glium::uniforms::Uniforms for CombinedUniforms2<'c, 'd, C, D> {
    fn visit_values<'a, F: FnMut(&str, glium::uniforms::UniformValue<'a>)>(&'a self, mut output: F) {
        self.first.visit_values(|name, value| { output(name, value) });
        self.second.visit_values(|name, value| { output(name, value) });
    }
}

fn combine_uniforms<'a, 'b, A: 'a + Uniforms, B: 'b + Uniforms>(first: &'a A, second: &'b B) -> CombinedUniforms2<'a, 'b, A, B> {
    CombinedUniforms2 {
        first: first,
        second: second,
    }
}

enum Command<'a, T: 'a + Surface> {
    Draw(&'a Pass<'a>),
    DrawFrameBuffer(&'a Pass<'a>, &'a std::cell::RefCell<T>),
    Clear(f32, f32, f32, f32),
    ClearFrameBuffer(f32, f32, f32, f32, &'a std::cell::RefCell<T>),
}

fn main() {
    let mut r = shader::Resolver::new();
    //TODO glob
    r.push("toy.frag");
    r.push("shader.vert");
    r.push("shader.frag");
    r.push("shader2.vert");
    r.push("shader2.frag");

    let display = glium::glutin::WindowBuilder::new().build_glium().unwrap();


    let texture1 = glium::texture::Texture2d::empty_with_format(&display,
                                                                glium::texture::UncompressedFloatFormat::F32F32F32F32,
                                                                glium::texture::MipmapsOption::NoMipmap,
                                                                800,
                                                                500
                                                                ).unwrap();
    let tex_buffer = std::cell::RefCell::new(glium::framebuffer::SimpleFrameBuffer::new(&display, &texture1).unwrap());

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

    let program1 = glium::Program::from_source(&display, &r.resolve("shader.vert").unwrap(), &r.resolve("shader.frag").unwrap(), None).unwrap();
    let program2 = glium::Program::from_source(&display, &r.resolve("shader2.vert").unwrap(), &r.resolve("shader2.frag").unwrap(), None).unwrap();

    let pass1 = Pass {
        channels: vec![],
        program: &program1,
    };
    let pass2 = Pass {
        channels: vec![&texture1],
        program: &program2,
    };

    let mut commands = vec![
        Command::Clear(0.0, 0.0, 0.0, 1.0),
        Command::ClearFrameBuffer(0., 0., 0., 1., &tex_buffer),
        Command::DrawFrameBuffer(&pass1, &tex_buffer),
        Command::Draw(&pass2),
    ];

    let mut state = State::new(display.get_framebuffer_dimensions());

    let mut sync = Sync::new();
    sync.track("color:r");
    sync.track("color:g");
    sync.track("color:b");

    loop {
        sync.update();

        let mut target = display.draw();
        for command in &mut commands {
            match command {
                &mut Command::Draw(pass) => {
                    target.draw(&vertex_buffer, &indices, &pass.program,
                                &combine_uniforms(&combine_uniforms(&state, pass), &sync),
                                &Default::default()).unwrap();
                },
                &mut Command::DrawFrameBuffer(pass, ref frame_buffer_cell) => {
                    let mut frame_buffer = frame_buffer_cell.borrow_mut();
                    frame_buffer.draw(&vertex_buffer, &indices, &pass.program,
                                      &combine_uniforms(&combine_uniforms(&state, pass), &sync),
                                      &Default::default()).unwrap();
                },
                &mut Command::Clear(r,g,b,a) => {
                    target.clear_color(r,g,b,a);
                },
                &mut Command::ClearFrameBuffer(r,g,b,a, ref frame_buffer_cell) => {
                    let mut frame_buffer = frame_buffer_cell.borrow_mut();
                    frame_buffer.clear_color(r,g,b,a);
                },
            }
        }
        target.finish().unwrap();

        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return,
                _=> state.handle_event(ev),
            }
        }
    }
}
