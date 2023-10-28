use glow::HasContext;
use minvect::*;
use minimg::*;

#[derive(Debug)]
#[repr(C, packed)]
pub struct XYZRGBAUV {
    pub xyz: Vec3,
    pub rgba: Vec4,
    pub uv: Vec2,
}

pub struct ProgramXYZRGBAUV {
    program: glow::NativeProgram,
    texture: glow::NativeTexture,
}

impl ProgramXYZRGBAUV {
    pub unsafe fn new(gl: &glow::Context, vert: &str, frag: &str, image: &ImageBuffer) -> Self {
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

        gl.use_program(Some(program));

        let texture = gl.create_texture().unwrap();
        gl.bind_texture(glow::TEXTURE_2D, Some(texture));
        gl.tex_image_2d(glow::TEXTURE_2D, 0, glow::RGBA as i32, image.w as i32, image.h as i32, 0, glow::RGBA, glow::UNSIGNED_BYTE, Some(&image.data));
        gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MIN_FILTER, glow::NEAREST as i32);
        gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MAG_FILTER, glow::NEAREST as i32);
        gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_S, glow::CLAMP_TO_EDGE as i32);
        gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_T, glow::CLAMP_TO_EDGE as i32);
        gl.generate_mipmap(glow::TEXTURE_2D);

        ProgramXYZRGBAUV {
            program,
            texture,
        }
    }
    pub unsafe fn default(gl: &glow::Context, image: &ImageBuffer) -> Self {
        Self::new(gl, DEFAULT_VS, DEFAULT_FS, image)
    }
    pub unsafe fn bind(&self, gl: &glow::Context) {
        gl.use_program(Some(self.program));
        gl.bind_texture(glow::TEXTURE_2D, Some(self.texture));
    }
    pub unsafe fn set_proj(&self, proj: &[f32; 16], gl: &glow::Context) {
        gl.uniform_matrix_4_f32_slice(gl.get_uniform_location(self.program, "projection").as_ref(), true, proj);
    }
}

pub unsafe fn upload_xyzrgbauv_mesh(mesh: &[XYZRGBAUV], gl: &glow::Context) -> HandleXYZRGBAUV {
    let vbo = gl.create_buffer().unwrap();
    gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
    let vao = gl.create_vertex_array().unwrap();
    gl.bind_vertex_array(Some(vao));
    let vert_size = std::mem::size_of::<XYZRGBAUV>();

    gl.vertex_attrib_pointer_f32(0, 3, glow::FLOAT, false, vert_size as i32, 0);
    gl.enable_vertex_attrib_array(0);
    gl.vertex_attrib_pointer_f32(1, 4, glow::FLOAT, false, vert_size as i32, 3*4);
    gl.enable_vertex_attrib_array(1);
    gl.vertex_attrib_pointer_f32(2, 2, glow::FLOAT, false, vert_size as i32, 7*4);
    gl.enable_vertex_attrib_array(2);
    
    let vert_bytes: &[u8] = std::slice::from_raw_parts(
        mesh.as_ptr() as *const u8,
        mesh.len() * vert_size,
    );
    gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, vert_bytes, glow::STATIC_DRAW);
    HandleXYZRGBAUV {vao, vbo, num_verts: mesh.len()}
}

pub struct HandleXYZRGBAUV {
    pub vao: glow::NativeVertexArray,
    pub vbo: glow::NativeBuffer,
    pub num_verts: usize,
}

impl HandleXYZRGBAUV {
    pub unsafe fn render(&self, gl: &glow::Context) {
        gl.bind_vertex_array(Some(self.vao));
        gl.draw_arrays(glow::TRIANGLES, 0, self.num_verts as i32);
    }
}

pub const DEFAULT_FS: &str = r#"#version 330 core
in vec4 col;
in vec2 uv;
out vec4 frag_colour;

uniform sampler2D tex;

void main() {
    frag_colour = texture(tex, uv) * col;
}
"#;
pub const DEFAULT_VS: &str = r#"#version 330 core
layout (location = 0) in vec3 in_pos;
layout (location = 1) in vec4 in_col;
layout (location = 2) in vec2 in_uv;

out vec4 col;
out vec2 uv;

uniform mat4 projection;

void main() {
    col = in_col;
    uv = in_uv;

    gl_Position = projection * vec4(in_pos, 1.0);
}
"#;

pub fn put_triangle(buf: &mut Vec<XYZRGBAUV>, a: Vec2, a_uv: Vec2, b: Vec2, b_uv: Vec2, c: Vec2, c_uv: Vec2, col: Vec4, depth: f32) {
    buf.push(XYZRGBAUV {
        xyz: vec3(a.x, a.y, depth),
        rgba: col,
        uv: a_uv,
    });
    buf.push(XYZRGBAUV {
        xyz: vec3(b.x, b.y, depth),
        rgba: col,
        uv: b_uv,
    });
    buf.push(XYZRGBAUV {
        xyz: vec3(c.x, c.y, depth),
        rgba: col,
        uv: c_uv,
    });
}

pub fn put_quad(buf: &mut Vec<XYZRGBAUV>, a: Vec2, b: Vec2, c: Vec2, d: Vec2, col: Vec4, uv_lo: Vec2, uv_hi: Vec2, depth: f32) {
    let a_uv = uv_lo;
    let b_uv = vec2(uv_hi.x, uv_lo.y);
    let c_uv = uv_hi;
    let d_uv = vec2(uv_lo.x, uv_hi.y);

    put_triangle(buf, a, a_uv, b, b_uv, c, c_uv,col, depth);
    put_triangle(buf, a, a_uv, c, c_uv, d, d_uv, col, depth);
}

pub fn put_rect(buf: &mut Vec<XYZRGBAUV>, r: Rect, r_uv: Rect, col: Vec4, depth: f32) {
    let a = r.tl();
    let b = r.tr();
    let c = r.br();
    let d = r.bl();
    put_quad(buf, a, b, c, d, col, r_uv.tl(), r_uv.br(), depth);
}