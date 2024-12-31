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

// Equivalent to OpenGL gluLookAt
fn view_matrix(position: &[f32; 3], direction: &[f32;3], up: &[f32; 3]) -> [[f32;4]; 4] {
    let f = {
        let f = direction;
        let len = f[0]*f[0] + f[1]*f[1] + f[2]*f[2];
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
        [p[0], p[1], p[2], 1.0]
    ]
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

fn distance(p1: &[f32; 3], p2: &[f32; 3]) -> f32 {
    let term1 = p1[0] - p2[0];
    let term2 = p1[1] - p2[1];
    let term3 = p1[2] - p2[2];
    return f32::sqrt(term1 * term1 + term2 * term2 + term3 * term3);
}

fn magnitude(p: &[f32; 3]) -> f32 {
    return f32::sqrt(p[0] * p[0] + p[1] * p[1] + p[2] * p[2]);
} 

fn update_lookat_direction(camera_position: &[f32; 3], lookat_position: &[f32; 3]) -> [f32; 3] {
    let term1 = lookat_position[0] - camera_position[0];
    let term2 = lookat_position[1] - camera_position[1];
    let term3 = lookat_position[2] - camera_position[2];
    let vector = [term1, term2, term3];
    return [term1 / magnitude(&vector), term2 / magnitude(&vector), term3 / magnitude(&vector)];
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

    // TODO: Dynamically edit this view matrix to allow for camera movement with WASD
    // TODO: Need some way to record keystrokes and update the view matrix accordingly
    // Note: Teapot bottom is located at approximately [0.0, -0.5,  2.0]

    // To rotate around the lookAt point, need to change the up vector and the position of the camera.
    // For example, we "rotate" by theta angles around the x axis in a quarternion
    // Then we need to shift the z and y coordinates using simple trigonometry
    
    let teapot_position = [0.0, -0.5, 2.0];

    let mut camera_position = [0.0, 0.5, 0.0];
    let mut camera_direction = [teapot_position[0] - camera_position[0], teapot_position[1] - camera_position[1], teapot_position[2] - camera_position[2]];
    let camera_up = [0.0, 1.0, 0.0];
    let mut view = view_matrix(&camera_position, &camera_direction, &camera_up);
    // let view = view_matrix(&[0.0, 3.0, 2.0], &[0.0, -1.0, 0.0], &[0.0, 0.0, 1.0]);
    
    let radius = distance(&camera_position, &teapot_position);
    //simple rotation around the y axis
    let mut x_angle = 0.0;
    let mut z_angle = 0.0;

    let _ = event_loop.run(move |event, window_target| {
        match event {
            glium::winit::event::Event::WindowEvent { event, .. } => match event {
                glium::winit::event::WindowEvent::KeyboardInput { device_id, event, is_synthetic } => match event {
                    // println!("Keyboard input event: {:?}", event);
                    glium::winit::event::KeyEvent { physical_key, .. } => match physical_key {
                        glium::winit::keyboard::PhysicalKey::Code(key_code) => match key_code {
                            glium::winit::keyboard::KeyCode::KeyW => println!("Key: {:?}", key_code),
                            glium::winit::keyboard::KeyCode::KeyA => {
                                x_angle -= 0.05;
                                z_angle -= 0.05;
                                camera_position = [teapot_position[0] + f32::sin(x_angle) * radius, 0.5, teapot_position[2] + f32::cos(z_angle) * radius];
                                camera_direction = update_lookat_direction(&camera_position, &teapot_position);
                                println!("Camera position: {:?}", camera_position);
                                view = view_matrix(&camera_position, &camera_direction, &camera_up); 
                            }
                            glium::winit::keyboard::KeyCode::KeyS => println!("Key: {:?}", key_code),
                            glium::winit::keyboard::KeyCode::KeyD => {
                                x_angle -= 0.05;
                                z_angle -= 0.05;
                                camera_position = [teapot_position[0] + f32::cos(x_angle) * radius, 0.5, teapot_position[2] + f32::sin(z_angle) * radius];
                                camera_direction = update_lookat_direction(&camera_position, &teapot_position);
                                println!("Camera position: {:?}", camera_position);
                                view = view_matrix(&camera_position, &camera_direction, &camera_up); 
                            },
                            _ => ()
                        }, 
                        _ => ()
                    },
                    _ => ()
                },
                glium::winit::event::WindowEvent::CloseRequested => window_target.exit(),
                glium::winit::event::WindowEvent::Resized(window_size) => {
                    display.resize(window_size.into());
                },
                glium::winit::event::WindowEvent::RedrawRequested => {
                    // Draw code
                    let mut target = display.draw();
                    target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);
                    
                    // Perspective Matrix and Aspect Ratio
                    let perspective = {
                        let (width, height) = target.get_dimensions();
                        let aspect_ratio = height as f32 / width as f32;
                        let fov: f32 = std::f32::consts::PI / 3.0;
                        let zfar = 1024.0;
                        let znear = 0.1;
                        let f = 1.0 / (fov / 2.0).tan();

                        [
                            [f * aspect_ratio, 0.0, 0.0, 0.0],
                            [0.0, f, 0.0, 0.0],
                            [0.0, 0.0, (zfar + znear) / (zfar - znear), 1.0],
                            [0.0, 0.0, -(2.0 * zfar * znear) / (zfar - znear), 0.0],
                        ]
                    };

                    // Set uniform here to be used in the shader code for animating the triangle.
                    // The naiive approach would be to instead handle t in the event loop to update the vertex but that does not make much sense,
                    // We can place the handling and animating of the vertexes in different positions of the animations in the shader code to push that workload to the GPU
                    let x = 0.0;

                    // In column major order, order of transformations is inverse that of multiplication
                    // So for transform: scale, rotate then translate, the order of multiplication is translate * rotate * scale * vector
                    // In row major order, the order of multiplication is scale * rotate * translate * vector
                    let uniforms = uniform! { 
                        // Remember that in CG most matrices are in column-major order
                        // So matrix is actually 
                        // 0.05 0.0 0.0 x
                        // 0.0 0.05 0.0 0.0
                        // 0.0 0.0 0.05 0.0
                        // 0.0 0.0 2.0 1.0
                        // in row major order

                        // This model matrix is just a scale
                        model: [
                            [0.05, 0.0, 0.0, 0.0],
                            [0.0, 0.05, 0.0, 0.0],
                            [0.0, 0.0, 0.05, 0.0],
                            [x, 0.0, 2.0, 1.0f32]
                        ],
                        tex: &texture,
                        u_light: light,
                        perspective : perspective,
                        view: view
                    };
    
                    // Add depth testing here
                    let params = glium::DrawParameters {
                        depth: glium::Depth {
                            test: glium::draw_parameters::DepthTest::IfLess,
                            write: true,
                            ..Default::default()
                        },
                        // backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
                        ..Default::default()
                    };
                    
                    
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


