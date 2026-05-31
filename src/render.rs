use nalgebra;
use std::io::ErrorKind;

type Vec3 = nalgebra::SVector<f32, 3>;
type Face = nalgebra::SVector<Vec3, 3>;
type Frame = nalgebra::DMatrix<u32>;

#[derive(Default)]
pub struct Options {
    pub viewport_height: usize,
    pub viewport_width: usize,
    pub rad_fovy: f32,
    pub z_near: f32,
    pub z_far: f32,
}

pub struct View {
    pub eye: Vec3,
    pub center: Vec3,
    pub up: Vec3,
}

#[derive(Default)]
pub struct Renderer {
    pub framebuffer: Frame,
}

impl Renderer {
    pub fn get_frame(&mut self, view: View, faces: Vec<Face>, colors: Vec<u32>, options: Options) -> Result<&Frame, ErrorKind> {
        self.framebuffer = Frame::zeros(options.viewport_width, options.viewport_height);
        return Ok(&self.framebuffer); 
    }
}
