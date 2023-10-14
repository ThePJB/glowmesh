use glow::HasContext;
use minvect::*;

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
    let vao = gl.create_vertex_array().unwrap();
    let vbo = gl.create_buffer().unwrap();
    let vert_size = std::mem::size_of::<XYZRGBA>();

    let vert_bytes: &[u8] = std::slice::from_raw_parts(
        mesh.as_ptr() as *const u8,
        mesh.len() * vert_size,
    );

    gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
    gl.bind_vertex_array(Some(vao));
    gl.vertex_attrib_pointer_f32(0, 3, glow::FLOAT, false, vert_size as i32, 0);
    gl.enable_vertex_attrib_array(0);
    gl.vertex_attrib_pointer_f32(1, 4, glow::FLOAT, false, vert_size as i32, 3*4);
    gl.enable_vertex_attrib_array(1);

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
        dbg!("rendering", self.num_verts);
        gl.bind_vertex_array(Some(self.vao));
        gl.bind_buffer(glow::ARRAY_BUFFER, Some(self.vbo));
        gl.draw_arrays(glow::TRIANGLES, 0, self.num_verts as i32);
    }
}

//#todo
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