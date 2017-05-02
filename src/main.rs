#[macro_use]
extern crate glium;
extern crate regex;
#[macro_use]
extern crate lazy_static;

mod shader;

use glium::DisplayBuild;
use glium::Surface;

use std::time::Instant;

#[derive(Clone, Copy)]
struct Vertex {
    position: [f32; 3],
}
implement_vertex!(Vertex, position);


fn main() {
    let mut r = shader::Resolver::new();
    //TODO glob
    r.push("toy.frag");
    r.push("shader.vert");
    r.push("shader.frag");

    let display = glium::glutin::WindowBuilder::new().build_glium().unwrap();

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

    let start_time =  Instant::now();
    let mut cursor = (0 as f32,0 as f32);

    loop {
        let mut target = display.draw();
        let (w,h) = display.get_framebuffer_dimensions();
        let elapsed = start_time.elapsed();
        let time = elapsed.as_secs() as f32 + elapsed.subsec_nanos() as f32/ 1000000000.0;
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        target.draw(&vertex_buffer, &indices, &program, &uniform! {
            iResolution: [w as f32, h as f32, 1.0 as f32],
            iGlobalTime: time as f32,
            iMouse: [cursor.0, h as f32 - cursor.1]
        }, &Default::default()).unwrap();
        target.finish().unwrap();

        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return,
                glium::glutin::Event::MouseMoved(x,y) => {
                    cursor = (x as f32,y as f32);
                },
                _=> ()
            }
        }
    }
}
