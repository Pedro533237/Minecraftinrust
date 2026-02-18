use gl::types::GLfloat;

pub struct Renderer;

impl Renderer {
    pub fn new(window: &crate::engine::window::WindowContext) -> Result<Self, String> {
        gl::load_with(|s| window.window.subsystem().gl_get_proc_address(s) as *const _);
        unsafe {
            gl::Enable(gl::DEPTH_TEST);
            gl::DepthFunc(gl::LEQUAL);
            gl::Enable(gl::CULL_FACE);
            gl::CullFace(gl::BACK);
            gl::ClearColor(0.6, 0.8, 1.0, 1.0);
        }
        Ok(Self)
    }

    pub fn begin_frame(&mut self, width: i32, height: i32) {
        unsafe {
            gl::Viewport(0, 0, width, height);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
    }

    pub fn end_frame(&self) {}

    pub fn setup_2d(&self, width: f32, height: f32) {
        unsafe {
            gl::Disable(gl::DEPTH_TEST);
            gl::MatrixMode(gl::PROJECTION);
            gl::LoadIdentity();
            gl::Ortho(0.0, width as f64, height as f64, 0.0, -1.0, 1.0);
            gl::MatrixMode(gl::MODELVIEW);
            gl::LoadIdentity();
        }
    }

    pub fn setup_3d(&self, aspect: f32, fov_deg: f32, near: f32, far: f32) {
        let f = 1.0 / (fov_deg.to_radians() / 2.0).tan();
        let mut m = [0.0f32; 16];
        m[0] = f / aspect;
        m[5] = f;
        m[10] = (far + near) / (near - far);
        m[11] = -1.0;
        m[14] = (2.0 * far * near) / (near - far);

        unsafe {
            gl::Enable(gl::DEPTH_TEST);
            gl::MatrixMode(gl::PROJECTION);
            gl::LoadMatrixf(m.as_ptr() as *const GLfloat);
            gl::MatrixMode(gl::MODELVIEW);
            gl::LoadIdentity();
        }
    }

    pub fn rect(&self, x: f32, y: f32, w: f32, h: f32, color: [f32; 3]) {
        unsafe {
            gl::Color3f(color[0], color[1], color[2]);
            gl::Begin(gl::QUADS);
            gl::Vertex2f(x, y);
            gl::Vertex2f(x + w, y);
            gl::Vertex2f(x + w, y + h);
            gl::Vertex2f(x, y + h);
            gl::End();
        }
    }

    pub fn cube(&self, x: f32, y: f32, z: f32, color: [f32; 3]) {
        let s = 0.5;
        unsafe {
            gl::Color3f(color[0], color[1], color[2]);
            gl::PushMatrix();
            gl::Translatef(x, y, z);
            gl::Begin(gl::QUADS);
            // front
            gl::Vertex3f(-s, -s, s);
            gl::Vertex3f(s, -s, s);
            gl::Vertex3f(s, s, s);
            gl::Vertex3f(-s, s, s);
            // back
            gl::Vertex3f(-s, -s, -s);
            gl::Vertex3f(-s, s, -s);
            gl::Vertex3f(s, s, -s);
            gl::Vertex3f(s, -s, -s);
            // left
            gl::Vertex3f(-s, -s, -s);
            gl::Vertex3f(-s, -s, s);
            gl::Vertex3f(-s, s, s);
            gl::Vertex3f(-s, s, -s);
            // right
            gl::Vertex3f(s, -s, -s);
            gl::Vertex3f(s, s, -s);
            gl::Vertex3f(s, s, s);
            gl::Vertex3f(s, -s, s);
            // top
            gl::Vertex3f(-s, s, -s);
            gl::Vertex3f(-s, s, s);
            gl::Vertex3f(s, s, s);
            gl::Vertex3f(s, s, -s);
            // bottom
            gl::Vertex3f(-s, -s, -s);
            gl::Vertex3f(s, -s, -s);
            gl::Vertex3f(s, -s, s);
            gl::Vertex3f(-s, -s, s);
            gl::End();
            gl::PopMatrix();
        }
    }
}
