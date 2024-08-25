use glium::Surface;

#[macro_use]
extern crate glium;

#[derive(Copy, Clone)]
struct Vertex {
  position: [f32; 2]
}
implement_vertex!(Vertex, position);

fn main() {

    //Create Event Loop with winit crate and window with glium glutin re-export crate
    let event_loop = glium::winit::event_loop::EventLoopBuilder::new().build().unwrap();
    let (_window, display) = glium::backend::glutin::SimpleWindowBuilder::new().build(&event_loop);
    
    // //Start drawing within the window
    let mut frame = display.draw();
    frame.clear_color(0.0, 0.0, 1.0, 1.0);
    frame.finish().unwrap();
    
    //OpenGL refresher
    //OpenGL's coordinate system for the viewport space (aka NDC space) is a square centered at coordinate 0.0,0.0,0.0
    //Camera is placed at z = 0, x-y plane.
    //Top-right-back of the cube is  (1,1,1). Bottom-left-back of the cube is (-1,-1,0)
    let vertex1 = Vertex { position: [-0.5, -0.5] };
    let vertex2 = Vertex { position: [0.0, 0.5] };
    let vertex3 = Vertex { position: [0.5, -0.25] };
    let shape = vec![vertex1, vertex2, vertex3];

    //Send vertexes to vertex buffer for faster access by GPU
    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();

    //Set rendering type for vertices
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    //Set Vertex Shader, ideally should be located in it's own file
    let vertex_shader_src = r#"
        #version 140

        in vec2 position;

        void main() {
            gl_Position = vec4(position, 0.0, 1.0);
        }
    "#;

    //Set Fragment Shader, ideally should be located in it's own file
    let fragment_shader_src = r#"
        #version 140

        out vec4 color;

        void main() {
            color = vec4(1.0, 0.0, 0.0, 1.0);            
        }
    "#;

    //Send shaders to GLIUM wrappers for OpenGL
    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    //Draw triangles
    let mut target = display.draw();
    target.clear_color(0.0, 0.0, 1.0, 1.0);
    target.draw(&vertex_buffer, &indices, &program, &glium::uniforms::EmptyUniforms,
            &Default::default()).unwrap();
    target.finish().unwrap();
    
    //Set some callbacks for the Event Loop 
    let _ = event_loop.run(move |event, window_target| {
        match event {
                glium::winit::event::Event::WindowEvent { event, .. } => match event {
                glium::winit::event::WindowEvent::CloseRequested => window_target.exit(),
                _ => (),
                },
                _ => (),
            };
    });
}

