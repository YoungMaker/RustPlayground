extern crate glium; 

use glium::{glutin, Surface};

#[derive(Copy, Clone)]
struct Vertex {
  position: [f64; 2],
}

const WINDOW_X_SIZE: f64 = 1024.0;
const WINDOW_Y_SIZE: f64 = 768.0;

// shader programs. Very simple
const VERTEX_SHADER_SRC: &str = r#"
#version 140
in vec2 position;

uniform mat4 matrix;

void main() {
  gl_Position = matrix * vec4(position, 0.0, 1.0);
}
"#;

const FRAGMENT_SHADER_SRC: &str = r#"
#version 140
out vec4 color;
void main() {
    color = vec4(1.0, 1.0, 1.0, 1.0);
}
"#;


fn main() {

  let mut events_loop = glutin::EventsLoop::new();
  let wb = glutin::WindowBuilder::new()
    .with_title("Lines demo Rust")
    .with_dimensions(glutin::dpi::LogicalSize::new(WINDOW_X_SIZE, WINDOW_Y_SIZE))
    .with_resizable( false );
  let cb = glutin::ContextBuilder::new();
  let display = glium::Display::new(wb, cb, &events_loop).unwrap();

  // ad the Vertex trait impl to the Vertex struct
  glium::implement_vertex!(Vertex, position);

  // compile the shader programs into executable GPU code
  let program = glium::Program::from_source(&display, VERTEX_SHADER_SRC, FRAGMENT_SHADER_SRC, None).unwrap();

  // static triangle rn
  let vertex1 = Vertex { position: [-0.5, -0.5] };
  let vertex2 = Vertex { position: [ 0.0,  0.5] };
  let vertex3 = Vertex { position: [ 0.5, -0.25] };
  let vertex4 = Vertex { position: [ 0.5, -0.5] };
  let shape = vec![vertex1, vertex2, vertex3, vertex4];

  let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();

  let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

  let draw_params = glium::draw_parameters::DrawParameters
  {
    line_width : Option::Some(1.0),
    polygon_mode : glium::draw_parameters::PolygonMode::Line,
    .. Default::default()
  };

  //FIXME: Implement real error handling rather than .unwrap()
  let mut window_closed = false;

  // 45 degrees in rad
  let mut angle: f32 = 360.00f32.to_radians();
  while !window_closed {

    let mut target = display.draw();
    target.clear_color(0.0, 0.0, 0.0, 1.0);

    //transformation matrix
    let uniforms = glium::uniform! {
      matrix: [
          [ angle.cos(),  angle.sin(), 0.0,     0.0],
          [ -angle.sin(), angle.cos(), 0.0,     0.0],
          [ 0.0,          0.0,         1.0,     0.0],
          [ shift_x ,     shift_y,     shift_z, scale],
      ]
  };

    // draw as lines
    target.draw(&vertex_buffer, &indices, &program, &uniforms,
      &draw_params).unwrap();
    target.finish().unwrap();

    events_loop.poll_events(|ev| {
      match ev{
        glutin::Event::WindowEvent{ event, .. }  =>  //if event type was WindowEvent
          match event {
            glutin::WindowEvent::CloseRequested => window_closed = true, // if WindowEvent::CloseRequested triggered set window_closed = true
            _ => (), // default do nothing
          }, 
        _ => (), // default do nothing
      }
    });

  }
}
