use glow::HasContext;
use minvect::*;
use std::f32::consts::PI;

#[derive(Debug, Clone)]
#[repr(C, packed)]
pub struct XYZRGBA {
    pub xyz: Vec3,
    pub rgba: Vec4,
}

pub struct ProgramXYZRGBA {
    program: glow::NativeProgram,
}

impl ProgramXYZRGBA {
    pub unsafe fn new(gl: &glow::Context, vert: &str, frag: &str) -> Self {
        let program = gl.create_program().expect("Cannot create program");
    
        let vs = gl.create_shader(glow::VERTEX_SHADER).expect("cannot create vertex shader");
        gl.shader_source(vs, vert);
        gl.compile_shader(vs);
        if !gl.get_shader_compile_status(vs) {
            panic!("{}", gl.get_shader_info_log(vs));
        }
        gl.attach_shader(program, vs);

        let fs = gl.create_shader(glow::FRAGMENT_SHADER).expect("cannot create fragment shader");
        gl.shader_source(fs, frag);
        gl.compile_shader(fs);
        if !gl.get_shader_compile_status(fs) {
            panic!("{}", gl.get_shader_info_log(fs));
        }
        gl.attach_shader(program, fs);

        gl.link_program(program);
        if !gl.get_program_link_status(program) {
            panic!("{}", gl.get_program_info_log(program));
        }
        gl.detach_shader(program, fs);
        gl.delete_shader(fs);
        gl.detach_shader(program, vs);
        gl.delete_shader(vs);

        ProgramXYZRGBA {
            program
        }
    }
    pub unsafe fn default(gl: &glow::Context) -> Self {
        Self::new(gl, DEFAULT_VS, DEFAULT_FS)
    }
    pub unsafe fn bind(&self, gl: &glow::Context) {
        gl.use_program(Some(self.program))
    }
    pub unsafe fn set_proj(&self, proj: &[f32; 16], gl: &glow::Context) {
        gl.uniform_matrix_4_f32_slice(gl.get_uniform_location(self.program, "projection").as_ref(), true, proj);
    }
}

pub unsafe fn upload_xyzrgba_mesh(mesh: &[XYZRGBA], gl: &glow::Context) -> HandleXYZRGBA {
    let vbo = gl.create_buffer().unwrap();
    gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
    let vao = gl.create_vertex_array().unwrap();
    gl.bind_vertex_array(Some(vao));
    let vert_size = std::mem::size_of::<XYZRGBA>();

    gl.vertex_attrib_pointer_f32(0, 3, glow::FLOAT, false, vert_size as i32, 0);
    gl.enable_vertex_attrib_array(0);
    gl.vertex_attrib_pointer_f32(1, 4, glow::FLOAT, false, vert_size as i32, 3*4);
    gl.enable_vertex_attrib_array(1);
    
    let vert_bytes: &[u8] = std::slice::from_raw_parts(
        mesh.as_ptr() as *const u8,
        mesh.len() * vert_size,
    );
    gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, vert_bytes, glow::STATIC_DRAW);
    HandleXYZRGBA {vao, vbo, num_verts: mesh.len()}
}

pub struct HandleXYZRGBA {
    pub vao: glow::NativeVertexArray,
    pub vbo: glow::NativeBuffer,
    pub num_verts: usize,
}

impl HandleXYZRGBA {
    pub unsafe fn render(&self, gl: &glow::Context) {
        gl.bind_vertex_array(Some(self.vao));
        gl.draw_arrays(glow::TRIANGLES, 0, self.num_verts as i32);
    }

    /// definitely want to call this when dropping the buffer. not impling drop because how to reference opengl context hey?
    pub fn free(&self, gl: &glow::Context) {
        unsafe {
            gl.delete_vertex_array(self.vao);
            gl.delete_buffer(self.vbo);
        }
    }
}

pub const DEFAULT_FS: &str = r#"#version 330 core
in vec4 col;
out vec4 frag_colour;

void main() {
    frag_colour = col;
}
"#;
pub const DEFAULT_VS: &str = r#"#version 330 core
layout (location = 0) in vec3 in_pos;
layout (location = 1) in vec4 in_col;

out vec4 col;

uniform mat4 projection;

void main() {
    col = in_col;
    gl_Position = projection * vec4(in_pos, 1.0);
}
"#;

pub fn put_triangle(buf: &mut Vec<XYZRGBA>, a: Vec2, b: Vec2, c: Vec2, col: Vec4, depth: f32) {
    buf.push(XYZRGBA {
        xyz: vec3(a.x, a.y, depth),
        rgba: col,
    });
    buf.push(XYZRGBA {
        xyz: vec3(b.x, b.y, depth),
        rgba: col,
    });
    buf.push(XYZRGBA {
        xyz: vec3(c.x, c.y, depth),
        rgba: col,
    });
}

pub fn put_quad(buf: &mut Vec<XYZRGBA>, a: Vec2, b: Vec2, c: Vec2, d: Vec2, col: Vec4, depth: f32) {
    put_triangle(buf, a, b, c, col, depth);
    put_triangle(buf, a, c, d, col, depth);
}

pub fn put_rect(buf: &mut Vec<XYZRGBA>, r: Rect, col: Vec4, depth: f32) {
    let a = r.tl();
    let b = r.tr();
    let c = r.br();
    let d = r.bl();
    put_quad(buf, a, b, c, d, col, depth);
}

pub fn put_line(buf: &mut Vec<XYZRGBA>, p1: Vec2, p2: Vec2, thickness: f32, col: Vec4, depth: f32) {
    let v = (p2-p1).normalize();
    let u = thickness*vec2(-v.y, v.x);
    let a = p1 + u;
    let b = p2 + u;
    let c = p2 - u;
    let d = p1 - u;
    put_quad(buf, a, b, c, d, col, depth)
}

pub fn put_poly(buf: &mut Vec<XYZRGBA>, c: Vec2, r: f32, n: usize, phase: f32, col: Vec4, depth: f32) {
    let dtheta = (2.0 * PI) / n as f32;
    for i in 0..n {
        let theta = phase + dtheta * i as f32;
        let p1 = c + r*vec2(theta.cos(), theta.sin());
        let theta = theta - dtheta;
        let p2 = c + r*vec2(theta.cos(), theta.sin());
        put_triangle(buf, c, p1, p2, col, depth);
    }
}

pub fn transform_mesh(v: &mut Vec<XYZRGBA>, mat: &[f32; 16]) {
    for i in 0..v.len() {
        v[i].xyz = mat4_trans_homog(v[i].xyz, mat);
    }
}