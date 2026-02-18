use gl::types::{GLchar, GLenum, GLfloat, GLint, GLsizeiptr, GLuint};
use std::{ffi::CString, mem, ptr};

#[repr(C)]
#[derive(Copy, Clone)]
struct Vertex {
    pos: [f32; 3],
    color: [f32; 3],
}

pub struct Renderer {
    program: GLuint,
    vbo: GLuint,
    a_pos: GLuint,
    a_color: GLuint,
    u_mvp: GLint,
    u_viewport: GLint,
    mvp_2d: [f32; 16],
    vp_3d: [f32; 16],
    width: f32,
    height: f32,
}

impl Renderer {
    pub fn new(window: &crate::engine::window::WindowContext) -> Result<Self, String> {
        gl::load_with(|s| window.window.subsystem().gl_get_proc_address(s) as *const _);

        let vert_src = CString::new(
            "#version 110\n\
             attribute vec3 a_pos;\n\
             attribute vec3 a_color;\n\
             uniform mat4 u_mvp;\n\
             varying vec3 v_color;\n\
             void main() {\n\
               gl_Position = u_mvp * vec4(a_pos, 1.0);\n\
               v_color = a_color;\n\
             }\n",
        )
        .map_err(|e| e.to_string())?;

        let frag_src = CString::new(
            "#version 110\n\
             varying vec3 v_color;\n\
             void main() {\n\
               gl_FragColor = vec4(v_color, 1.0);\n\
             }\n",
        )
        .map_err(|e| e.to_string())?;

        let program = unsafe { link_program(&vert_src, &frag_src)? };

        let mut vbo = 0;
        unsafe {
            gl::GenBuffers(1, &mut vbo);
            gl::Enable(gl::DEPTH_TEST);
            gl::DepthFunc(gl::LEQUAL);
            gl::Enable(gl::CULL_FACE);
            gl::CullFace(gl::BACK);
            gl::ClearColor(0.6, 0.8, 1.0, 1.0);
        }

        let a_pos =
            unsafe { gl::GetAttribLocation(program, CString::new("a_pos").unwrap().as_ptr()) };
        let a_color =
            unsafe { gl::GetAttribLocation(program, CString::new("a_color").unwrap().as_ptr()) };
        let u_mvp =
            unsafe { gl::GetUniformLocation(program, CString::new("u_mvp").unwrap().as_ptr()) };
        let u_viewport = unsafe {
            gl::GetUniformLocation(program, CString::new("u_viewport").unwrap().as_ptr())
        };

        Ok(Self {
            program,
            vbo,
            a_pos: a_pos as GLuint,
            a_color: a_color as GLuint,
            u_mvp,
            u_viewport,
            mvp_2d: identity(),
            vp_3d: identity(),
            width: 1.0,
            height: 1.0,
        })
    }

    pub fn begin_frame(&mut self, width: i32, height: i32) {
        self.width = width as f32;
        self.height = height as f32;
        unsafe {
            gl::Viewport(0, 0, width, height);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::UseProgram(self.program);
            gl::Uniform2f(self.u_viewport, self.width.max(1.0), self.height.max(1.0));
        }
    }

    pub fn end_frame(&self) {}

    pub fn setup_2d(&mut self, width: f32, height: f32) {
        self.mvp_2d = ortho(0.0, width, height, 0.0, -1.0, 1.0);
        unsafe {
            gl::Disable(gl::DEPTH_TEST);
        }
    }

    pub fn setup_3d(&mut self, aspect: f32, fov_deg: f32, near: f32, far: f32, view: [f32; 16]) {
        let proj = perspective(fov_deg, aspect, near, far);
        self.vp_3d = mul_mat4(proj, view);
        unsafe {
            gl::Enable(gl::DEPTH_TEST);
        }
    }

    pub fn rect(&self, x: f32, y: f32, w: f32, h: f32, color: [f32; 3]) {
        let vertices = [
            Vertex {
                pos: [x, y, 0.0],
                color,
            },
            Vertex {
                pos: [x + w, y, 0.0],
                color,
            },
            Vertex {
                pos: [x + w, y + h, 0.0],
                color,
            },
            Vertex {
                pos: [x, y, 0.0],
                color,
            },
            Vertex {
                pos: [x + w, y + h, 0.0],
                color,
            },
            Vertex {
                pos: [x, y + h, 0.0],
                color,
            },
        ];
        self.draw_vertices(&vertices, self.mvp_2d);
    }

    pub fn cube(&self, x: f32, y: f32, z: f32, color: [f32; 3]) {
        let s = 0.5;
        let p = |px: f32, py: f32, pz: f32| [x + px, y + py, z + pz];
        let mut v = Vec::with_capacity(36);

        // front
        v.extend_from_slice(&tri_quad(
            p(-s, -s, s),
            p(s, -s, s),
            p(s, s, s),
            p(-s, s, s),
            color,
        ));
        // back
        v.extend_from_slice(&tri_quad(
            p(s, -s, -s),
            p(-s, -s, -s),
            p(-s, s, -s),
            p(s, s, -s),
            color,
        ));
        // left
        v.extend_from_slice(&tri_quad(
            p(-s, -s, -s),
            p(-s, -s, s),
            p(-s, s, s),
            p(-s, s, -s),
            color,
        ));
        // right
        v.extend_from_slice(&tri_quad(
            p(s, -s, s),
            p(s, -s, -s),
            p(s, s, -s),
            p(s, s, s),
            color,
        ));
        // top
        v.extend_from_slice(&tri_quad(
            p(-s, s, s),
            p(s, s, s),
            p(s, s, -s),
            p(-s, s, -s),
            color,
        ));
        // bottom
        v.extend_from_slice(&tri_quad(
            p(-s, -s, -s),
            p(s, -s, -s),
            p(s, -s, s),
            p(-s, -s, s),
            color,
        ));

        self.draw_vertices(&v, self.vp_3d);
    }

    fn draw_vertices(&self, vertices: &[Vertex], mvp: [f32; 16]) {
        unsafe {
            gl::UseProgram(self.program);
            gl::UniformMatrix4fv(self.u_mvp, 1, gl::FALSE, mvp.as_ptr());

            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * mem::size_of::<Vertex>()) as GLsizeiptr,
                vertices.as_ptr() as *const _,
                gl::STREAM_DRAW,
            );

            gl::EnableVertexAttribArray(self.a_pos);
            gl::EnableVertexAttribArray(self.a_color);
            gl::VertexAttribPointer(
                self.a_pos,
                3,
                gl::FLOAT,
                gl::FALSE,
                mem::size_of::<Vertex>() as i32,
                ptr::null(),
            );
            gl::VertexAttribPointer(
                self.a_color,
                3,
                gl::FLOAT,
                gl::FALSE,
                mem::size_of::<Vertex>() as i32,
                (3 * mem::size_of::<f32>()) as *const _,
            );

            gl::DrawArrays(gl::TRIANGLES, 0, vertices.len() as i32);
            gl::DisableVertexAttribArray(self.a_pos);
            gl::DisableVertexAttribArray(self.a_color);
        }
    }
}

fn tri_quad(a: [f32; 3], b: [f32; 3], c: [f32; 3], d: [f32; 3], color: [f32; 3]) -> [Vertex; 6] {
    [
        Vertex { pos: a, color },
        Vertex { pos: b, color },
        Vertex { pos: c, color },
        Vertex { pos: a, color },
        Vertex { pos: c, color },
        Vertex { pos: d, color },
    ]
}

unsafe fn link_program(vert: &CString, frag: &CString) -> Result<GLuint, String> {
    let vs = compile_shader(vert, gl::VERTEX_SHADER)?;
    let fs = compile_shader(frag, gl::FRAGMENT_SHADER)?;
    let program = gl::CreateProgram();
    gl::AttachShader(program, vs);
    gl::AttachShader(program, fs);
    gl::LinkProgram(program);

    let mut success = 0;
    gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);
    if success == 0 {
        let mut len = 0;
        gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
        let mut buf = vec![0 as GLchar; len as usize];
        gl::GetProgramInfoLog(program, len, ptr::null_mut(), buf.as_mut_ptr());
        return Err(
            String::from_utf8_lossy(&buf.iter().map(|c| *c as u8).collect::<Vec<_>>()).to_string(),
        );
    }

    gl::DeleteShader(vs);
    gl::DeleteShader(fs);
    Ok(program)
}

unsafe fn compile_shader(src: &CString, shader_type: GLenum) -> Result<GLuint, String> {
    let shader = gl::CreateShader(shader_type);
    gl::ShaderSource(shader, 1, &src.as_ptr(), ptr::null());
    gl::CompileShader(shader);

    let mut success = 0;
    gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
    if success == 0 {
        let mut len = 0;
        gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
        let mut buf = vec![0 as GLchar; len as usize];
        gl::GetShaderInfoLog(shader, len, ptr::null_mut(), buf.as_mut_ptr());
        return Err(
            String::from_utf8_lossy(&buf.iter().map(|c| *c as u8).collect::<Vec<_>>()).to_string(),
        );
    }
    Ok(shader)
}

pub fn look_at(eye: [f32; 3], center: [f32; 3], up: [f32; 3]) -> [f32; 16] {
    let f = normalize([center[0] - eye[0], center[1] - eye[1], center[2] - eye[2]]);
    let s = normalize(cross(f, up));
    let u = cross(s, f);

    [
        s[0],
        u[0],
        -f[0],
        0.0,
        s[1],
        u[1],
        -f[1],
        0.0,
        s[2],
        u[2],
        -f[2],
        0.0,
        -dot(s, eye),
        -dot(u, eye),
        dot(f, eye),
        1.0,
    ]
}

fn perspective(fov_deg: f32, aspect: f32, near: f32, far: f32) -> [f32; 16] {
    let f = 1.0 / (fov_deg.to_radians() / 2.0).tan();
    [
        f / aspect,
        0.0,
        0.0,
        0.0,
        0.0,
        f,
        0.0,
        0.0,
        0.0,
        0.0,
        (far + near) / (near - far),
        -1.0,
        0.0,
        0.0,
        (2.0 * far * near) / (near - far),
        0.0,
    ]
}

fn ortho(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> [f32; 16] {
    [
        2.0 / (right - left),
        0.0,
        0.0,
        0.0,
        0.0,
        2.0 / (top - bottom),
        0.0,
        0.0,
        0.0,
        0.0,
        -2.0 / (far - near),
        0.0,
        -(right + left) / (right - left),
        -(top + bottom) / (top - bottom),
        -(far + near) / (far - near),
        1.0,
    ]
}

fn identity() -> [f32; 16] {
    [
        1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
    ]
}

fn mul_mat4(a: [f32; 16], b: [f32; 16]) -> [f32; 16] {
    let mut r = [0.0; 16];
    for row in 0..4 {
        for col in 0..4 {
            r[col * 4 + row] = a[row] * b[col * 4]
                + a[4 + row] * b[col * 4 + 1]
                + a[8 + row] * b[col * 4 + 2]
                + a[12 + row] * b[col * 4 + 3];
        }
    }
    r
}

fn dot(a: [f32; 3], b: [f32; 3]) -> f32 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

fn cross(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

fn normalize(v: [f32; 3]) -> [f32; 3] {
    let len = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt().max(1e-6);
    [v[0] / len, v[1] / len, v[2] / len]
}
