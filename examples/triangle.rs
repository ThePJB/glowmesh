use glow::HasContext;
use minvect::*;
extern crate glowmesh;
use glowmesh::xyzrgba::*;
use glowmesh::xyzrgba_build2d::*;
use glutin::event::{Event, WindowEvent};

pub fn main() {

    let mut xres = 512;
    let mut yres = 512;

    unsafe {
        let event_loop = glutin::event_loop::EventLoop::new();
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
        // gl.enable(glow::DEPTH_TEST);
        gl.blend_func(glow::SRC_ALPHA, glow::ONE_MINUS_SRC_ALPHA);
        // gl.depth_func(glow::LEQUAL);
        gl.enable(glow::BLEND);

        let prog = ProgramXYZRGBA::default(&gl);

        let buf = &mut vec![];
        put_triangle(buf, vec2(0.0, 1.0), vec2(1.0, 0.0), vec2(-1.0, 0.0), vec4(1.0, 0.0, 0.0, 1.0), 0.1);
        let h = upload_xyzrgba_mesh(buf, &gl);
        let mat4_ident = [1.0f32, 0., 0., 0., 0., 1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1. ];
        prog.bind(&gl);
        prog.set_proj(&mat4_ident, &gl);

        event_loop.run(move |event, _, _| {
            match event {
                Event::LoopDestroyed |
                Event::WindowEvent {event: WindowEvent::CloseRequested, ..} => {
                    std::process::exit(0);
                },
    
                Event::WindowEvent {event, .. } => {
                    match event {
                        WindowEvent::Resized(size) => {
                            xres = size.width as i32;
                            yres = size.height as i32;
                            gl.viewport(0, 0, size.width as i32, size.height as i32)
                        },
                        _ => {},
                    }
                },
                Event::MainEventsCleared => {
                    gl.clear_color(0.5, 0.5, 0.5, 1.0);
                    gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT); 
                    prog.bind(&gl);
                    h.render(&gl);
                    window.swap_buffers().unwrap();
                },
                _ => {},
            }
        })
    }
}