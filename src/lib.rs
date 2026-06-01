use std::io::ErrorKind;

mod chunk;
mod render;

type Vec3 = nalgebra::SVector<f32, 3>;
type Face = nalgebra::SVector<Vec3, 3>;
type Frame = nalgebra::DMatrix<u32>;

#[derive(Default)]
pub struct Options {
    pub viewport_height: usize,
    pub viewport_width: usize,
}

#[derive(Default)]
pub struct Engine {
    pub renderer: render::Renderer,
}

impl Engine {
    pub fn get_frame(&mut self, options: Options) -> Result<&Frame, ErrorKind> {
        let view = render::View {
            eye: Vec3::new(0.0, 1.0, 3.0),
            center: Vec3::new(0.0, 1.0, 3.0),
            up: Vec3::new(0.0, 1.0, 3.0),
        };
        let options = render::Options {
            viewport_height: options.viewport_height,
            viewport_width: options.viewport_width,
            rad_fovy: 90.0_f32.to_radians(),
            z_near: 1.0,
            z_far: 100.0,
        };

        let faces = vec![Face::new(
            Vec3::new(0.0, 1.0, 3.0),
            Vec3::new(0.0, 1.0, 3.0),
            Vec3::new(0.0, 1.0, 3.0),
        )];

        let colors = vec![1];

        self.renderer.get_frame(view, faces, colors, options)
    }
}
