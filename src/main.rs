use glium::Surface;
use std::fs;
use std::env;
use std::io;
use obj::load_obj;
use obj::Obj;

#[macro_use]
extern crate glium;

mod triangle;
mod glium_teapot;
mod glium_teapot_example;

// Define a 2D vertex here
#[derive(Copy, Clone, Debug)]
struct Vertex {
  position: [f32; 3],
  color: [f32; 3], //Corresponds to vec3 RGB in GLSL,
  normal: [f32; 3],
  tex_coords: [f32; 2]
}
implement_vertex!(Vertex, position, color, normal, tex_coords);

//Converts from obj vertex to Vertex
impl From<obj::Vertex> for Vertex {
    fn from(vertex: obj::Vertex) -> Self {
        Vertex { position: vertex.position, normal: vertex.normal, color: [1.0, 0.0, 0.0], tex_coords: [0.0, 0.0] }
    }
}

// TODO: Can we use generics here to accept other formats such as &String?
fn read_shader(shader_path: &str) -> String {
    return fs::read_to_string(std::path::Path::new(&String::from(shader_path))).unwrap()
}

// TODO: Improve error handling here by removing unwrap() and handling with ? and returning a result
fn load_obj_file(file_path: &str) -> Obj {
    let input = io::BufReader::new(fs::File::open(&file_path).unwrap());
    return load_obj(input).unwrap();
}

fn create_teapot() {
    let event_loop = glium::winit::event_loop::EventLoopBuilder::new().build().unwrap();
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new().build(&event_loop);
    
    let obj_file = load_obj_file("models/obj/teapot.obj");
    
    let shape: Vec<Vertex> = obj_file.vertices.into_iter().map(|v| Vertex::from(v)).collect();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList, &obj_file.indices).unwrap();

    // Create empty texture
    let texture = glium::texture::Texture2d::empty(&display, 200, 200).unwrap();

    // Default shaders
    // let vertex_shader_src = read_shader("shaders/teapot.vert");
    // let fragment_shader_src = read_shader("shaders/teapot.frag");

    // Gouraud shading shaders
    let vertex_shader_src = read_shader("shaders/teapot_gouraud.vert");
    let fragment_shader_src = read_shader("shaders/teapot_gouraud.frag");

    let program = glium::Program::from_source(&display, vertex_shader_src.as_str(), fragment_shader_src.as_str(), None).unwrap();

    let light = [-1.0, 0.4, 0.9f32];

    // TODO: This event loop is faulty
    let _ = event_loop.run(move |event, window_target| {
        match event {
            glium::winit::event::Event::WindowEvent { event, .. } => match event {
                glium::winit::event::WindowEvent::CloseRequested => window_target.exit(),
                glium::winit::event::WindowEvent::Resized(window_size) => {
                    display.resize(window_size.into());
                },
                glium::winit::event::WindowEvent::RedrawRequested => {
                    // Draw code
                    
                    // Set uniform here to be used in the shader code for animating the triangle.
                    // The naiive approach would be to instead handle t in the event loop to update the vertex but that does not make much sense,
                    // We can place the handling and animating of the vertexes in different positions of the animations in the shader code to push that workload to the GPU
                    let x = 0.0;

                    let uniforms = uniform! { 
                        matrix: [
                            [0.05, 0.0, 0.0, 0.0],
                            [0.0, 0.05, 0.0, 0.0],
                            [0.0, 0.0, 0.05, 0.0],
                            [x, 0.0, 0.0, 1.0f32]
                        ],
                        tex: &texture,
                        u_light: light,
                    };
    
                    // Add depth testing here
                    let params = glium::DrawParameters {
                        depth: glium::Depth {
                            test: glium::draw_parameters::DepthTest::IfLess,
                            write: true,
                            ..Default::default()
                        },
                        ..Default::default()
                    };
                    
                    let mut target = display.draw();
                    target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);
                    
                    // We pass t here to the vertex shader using a uniform
                    // A uniform is a global variable whose value is set when we draw by passing its value to the draw function.
                    // The easiest way to do so is by using the uniform! macro
                    target.draw(&vertex_buffer, &indices, &program, &uniforms, &params).unwrap();
                    target.finish().unwrap();
                }
                _ => (),
            },
            glium::winit::event::Event::AboutToWait => {
                window.request_redraw();
            }
            _ => (),
        };
    }); 
}

// Note: Remember that matrices in OpenGL are in column-major order
fn main() {

    // crate::triangle::create_triangle_with_colored_vertices();
    // crate::glium_teapot_example::draw();

    //My own implementation of viewing teapot with reading shaders from file and loading obj from file
    create_teapot();
}

