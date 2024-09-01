use glium::Surface;
use std::fs;
use std::env;

#[macro_use]
extern crate glium;

//Define a 2D vertex here
#[derive(Copy, Clone)]
struct Vertex {
  position: [f32; 2],
  color: [f32; 3], //Corresponds to vec3 RGB in GLSL
  tex_coords: [f32; 2]
}
implement_vertex!(Vertex, position, color);

//OpenGL refresher
//OpenGL's coordinate system for the viewport space (aka NDC space) is a square centered at coordinate vec3(0.0, 0.0, 0.0)
//Camera is placed at z = 0, x-y plane.
//Top-right-back of the cube is vec3(1.0, 1.0, 1.0). Bottom-left-back of the cube is (-1.0, -1.0, 0)
//Now include color into each vertex as well, note that OpenGL interpolates colours between vertexes automatically
fn construct_triangle_vectors() -> Vec<Vertex> {
    return vec![
        Vertex { position: [-0.5, -0.5], color: [1.0, 0.0, 0.0], tex_coords: [0.0, 0.0] },
        Vertex { position: [0.0, 0.5], color: [0.0, 1.0, 0.0], tex_coords: [0.0, 0.0] },
        Vertex { position: [0.5, -0.25], color: [0.0, 0.0, 1.0], tex_coords: [0.0, 0.0] }
    ]
}

fn create_triangle_with_colored_vertices() {
    //Create Event Loop with winit crate and window with glium glutin re-export crate
    let event_loop = glium::winit::event_loop::EventLoopBuilder::new().build().unwrap();
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new().build(&event_loop);
    
    // //Start drawing within the window
    let mut frame = display.draw();
    frame.clear_color(0.0, 0.0, 1.0, 1.0);
    frame.finish().unwrap();
    
    let shape = construct_triangle_vectors();

    // Send vertexes to vertex buffer for faster access by GPU
    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();

    // Set rendering type for vertices
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    // Create empty texture
    let texture = glium::texture::Texture2d::empty(&display, 200, 200).unwrap();
    

    // Set Vertex Shader, ideally should be located in it's own file
    // Send matrices to vertex shader via uniforms
    // Execution is vertex shader -> fragment shader
    // Vertex shader outputs fragment color and other attributes to the fragment shader -> whatever we need in the fragment shader needs to be passed to the vertex shader
    // The passing of attributes from vertex shader to fragment shader is 
    let vertex_shader_src = r#"
        #version 140

        in vec2 position;
        in vec3 color;
        in vec2 tex_coords;
        out vec2 v_tex_coords;
        out vec3 vertex_color;

        uniform mat4 matrix;

        void main() {
            vec2 pos = position;
            vertex_color = color;
            v_tex_coords = tex_coords;
            gl_Position = matrix * vec4(pos, 0.0, 1.0);
        }
    "#;

    //Set Fragment Shader, ideally should be located in it's own file
    let fragment_shader_src = r#"
        #version 140

        in vec3 vertex_color;
        in vec2 v_tex_coords;

        uniform sampler2D tex;

        out vec4 color;

        void main() {
            color = vec4(vertex_color, 1.0);            
        }
    "#;

    //Send shaders to GLIUM wrappers for OpenGL
    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();
  
    // Set t
    let mut t: f32 = 0.0;

    //Set some callbacks for the Event Loop, this code basically handles the event loop for the window 
    let _ = event_loop.run(move |event, window_target| {
        match event {
            glium::winit::event::Event::WindowEvent { event, .. } => match event {
                glium::winit::event::WindowEvent::CloseRequested => window_target.exit(),
                glium::winit::event::WindowEvent::Resized(window_size) => {
                    display.resize(window_size.into());
                },
                glium::winit::event::WindowEvent::RedrawRequested => {
                    // Draw code
                    t += 0.02;
                    
                    // Set uniform here to be used in the shader code for animating the triangle.
                    // The naiive approach would be to instead handle t in the event loop to update the vertex but that does not make much sense,
                    // We can place the handling and animating of the vertexes in different positions of the animations in the shader code to push that workload to the GPU
                    let x = t.sin() * 0.5;

                    let uniforms = uniform! { 
                        matrix: [
                            [1.0, 0.0, 0.0, 0.0],
                            [0.0, 1.0, 0.0, 0.0],
                            [0.0, 0.0, 1.0, 0.0],
                            [x, 0.0, 0.0, 1.0f32]
                        ],
                        tex: &texture
                    };

                    let mut target = display.draw();
                    target.clear_color(0.0, 0.0, 1.0, 1.0);
                    
                    // We pass t here to the vertex shader using a uniform
                    // A uniform is a global variable whose value is set when we draw by passing its value to the draw function.
                    // The easiest way to do so is by using the uniform! macro
                    target.draw(&vertex_buffer, &indices, &program, &uniforms, &Default::default()).unwrap();
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
    create_triangle_with_colored_vertices()
}

