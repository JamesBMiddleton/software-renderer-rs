use std::io::ErrorKind;

mod chunk;
mod render;

type Vec3 = nalgebra::SVector<f32, 3>;
type Rot3 = nalgebra::Rotation3<f32>;
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
    pub theta: f32,
}

impl Engine {
    pub fn get_frame(&mut self, options: Options) -> Result<&Frame, ErrorKind> {

        let xrot = Rot3::from_axis_angle( &Vec3::z_axis(), self.theta);
        let yrot = Rot3::from_axis_angle( &Vec3::z_axis(), self.theta);
        self.theta += 0.01;

        let view = render::View {
            eye: xrot * (yrot * Vec3::new(0.0, 5.0, 10.0)),
            center: Vec3::new(0.0, 0.0, 0.0),
            up: Vec3::new(0.0, 1.0, 0.0),
        };
        let options = render::Options {
            viewport_height: options.viewport_height,
            viewport_width: options.viewport_width,
            rad_fovy: 90.0_f32.to_radians(),
            z_near: 1.0,
            z_far: 100.0,
        };

        let faces = vec![Face::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0), Vec3::new(0.0, 1.0, 1.0)),
                        Face::new(Vec3::new(0.0, 1.0, 1.0), Vec3::new(0.0, 1.0, 0.0), Vec3::new(0.0, 0.0, 0.0)),
                        Face::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 1.0)),
                        Face::new(Vec3::new(1.0, 0.0, 1.0), Vec3::new(0.0, 0.0, 1.0), Vec3::new(0.0, 0.0, 0.0)),
                        Face::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0), Vec3::new(1.0, 1.0, 0.0)),
                        Face::new(Vec3::new(1.0, 1.0, 0.0), Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0)),
                        Face::new(Vec3::new(1.0, 1.0, 1.0), Vec3::new(1.0, 0.0, 1.0), Vec3::new(1.0, 0.0, 0.0)),
                        Face::new(Vec3::new(1.0, 0.0, 0.0), Vec3::new(1.0, 1.0, 0.0), Vec3::new(1.0, 1.0, 1.0)),
                        Face::new(Vec3::new(1.0, 1.0, 1.0), Vec3::new(1.0, 1.0, 0.0), Vec3::new(0.0, 1.0, 0.0)),
                        Face::new(Vec3::new(0.0, 1.0, 0.0), Vec3::new(0.0, 1.0, 1.0), Vec3::new(1.0, 1.0, 1.0)),
                        Face::new(Vec3::new(1.0, 1.0, 1.0), Vec3::new(0.0, 1.0, 1.0), Vec3::new(0.0, 0.0, 1.0)),
                        Face::new(Vec3::new(0.0, 0.0, 1.0), Vec3::new(1.0, 0.0, 1.0), Vec3::new(1.0, 1.0, 1.0)),
                        Face::new(Vec3::new(0.0, 0.0, 1.0), Vec3::new(1.0, 0.0, 1.0), Vec3::new(1.0, 1.0, 1.0)),
        ];

        let colors = vec![
            0xFFFFFFFF, 
            0xFF0000FF,
            0xFFFFFFFF,
            0xFF0000FF,
            0xFFFFFFFF,
            0xFF0000FF,
            0xFFFFFFFF,
            0xFF0000FF,
            0xFFFFFFFF,
            0xFF0000FF,
            0xFFFFFFFF,
            0xFF0000FF,
        ];

        self.renderer.get_frame(view, faces, colors, options)
    }
}
