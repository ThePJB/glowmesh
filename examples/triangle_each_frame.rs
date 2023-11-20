use glow::HasContext;
use minvect::*;
extern crate glow_mesh;
use glow_mesh::xyzrgba::*;
use glutin::event::{Event, WindowEvent};

pub struct TriangleDemo {
    xres: i32,
    yres: i32,
    window: glutin::ContextWrapper<glutin::PossiblyCurrent, glutin::window::Window>,
    gl: glow::Context,

    prog: ProgramXYZRGBA,
}

impl TriangleDemo {
    pub fn new(event_loop: &glutin::event_loop::EventLoop<()>) -> Self {
        let xres = 512;
        let yres = 512;
    
        unsafe {
            let window_builder = glutin::window::WindowBuilder::new()
                .with_title("triangle test")
                .with_inner_size(glutin::dpi::PhysicalSize::new(xres, yres));
            let window = glutin::ContextBuilder::new()
                .with_vsync(true)
                .build_windowed(window_builder, &event_loop)
                .unwrap()
                .make_current()
                .unwrap();
    
            let gl = glow::Context::from_loader_function(|s| window.get_proc_address(s) as *const _);
    
            let prog = ProgramXYZRGBA::default(&gl);
    

            TriangleDemo {
                xres,
                yres,
                window,
                gl,
                prog,
            }
        }
    }

    pub fn handle_event(&mut self, event: glutin::event::Event<()>) {
        unsafe {
            match event {
                Event::LoopDestroyed |
                Event::WindowEvent {event: WindowEvent::CloseRequested, ..} => {
                    std::process::exit(0);
                },

                Event::WindowEvent {event, .. } => {
                    match event {
                        WindowEvent::Resized(size) => {
                            self.xres = size.width as i32;
                            self.yres = size.height as i32;
                            self.window.resize(size);
                            self.gl.viewport(0, 0, size.width as i32, size.height as i32);
                        },
                        _ => {},
                    }
                },
                Event::MainEventsCleared => {
                    self.gl.clear_color(0.5, 0.5, 0.5, 1.0);
                    self.gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT); 

                    let buf = &mut vec![];
                    put_triangle(buf, vec2(0.0, 1.0), vec2(1.0, 0.0), vec2(-1.0, 0.0), vec4(1.0, 0.0, 0.0, 1.0), -0.5);
                    let h = upload_xyzrgba_mesh(buf, &self.gl);
                    self.prog.bind(&self.gl);
                    let mat4_ident = [1.0f32, 0., 0., 0., 0., 1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1. ];
                    self.prog.set_proj(&mat4_ident, &self.gl);

                    h.render(&self.gl);
                    self.window.swap_buffers().unwrap();
                    h.free(&self.gl);
                },
                _ => {},
            }
        }
    }
}

pub fn main() {
        let event_loop = glutin::event_loop::EventLoop::new();
        let mut triangle_demo = TriangleDemo::new(&event_loop);
        event_loop.run(move |event, _, _| triangle_demo.handle_event(event));
}