use std::io::ErrorKind;

mod render;

type Vec3 = nalgebra::SVector<f32, 3>;
type Rot3 = nalgebra::Rotation3<f32>;
type Frame = nalgebra::DMatrix<u32>;
type ZBuffer = nalgebra::DMatrix<f32>;

use nalgebra::vector;

#[derive(Default)]
pub struct Options {
    pub viewport_height: usize,
    pub viewport_width: usize,
}

pub struct Engine {
    framebuffer: Frame,
    zbuffer: ZBuffer,
    theta: f32,
}

impl Engine {
    pub fn new() -> Self {
        Engine {
            framebuffer: Frame::default(),
            zbuffer: ZBuffer::default(),
            theta: 0.0,
        }
    }

    pub fn get_frame(&mut self, options: Options) -> Result<&Frame, ErrorKind> {
        let xrot = Rot3::from_axis_angle(&Vec3::z_axis(), self.theta);
        let yrot = Rot3::from_axis_angle(&Vec3::z_axis(), self.theta);
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

        #[rustfmt::skip]
        let faces = vec![
            vector![ vector![0.0, 0.0, 0.0], vector![0.0, 0.0, 1.0], vector![0.0, 1.0, 1.0] ],
            vector![ vector![0.0, 1.0, 1.0], vector![0.0, 1.0, 0.0], vector![0.0, 0.0, 0.0] ],
            vector![ vector![0.0, 0.0, 0.0], vector![1.0, 0.0, 0.0], vector![1.0, 0.0, 1.0] ],
            vector![ vector![1.0, 0.0, 1.0], vector![0.0, 0.0, 1.0], vector![0.0, 0.0, 0.0] ],
            vector![ vector![0.0, 0.0, 0.0], vector![0.0, 1.0, 0.0], vector![1.0, 1.0, 0.0] ],
            vector![ vector![1.0, 1.0, 0.0], vector![1.0, 0.0, 0.0], vector![0.0, 0.0, 0.0] ],
            vector![ vector![1.0, 1.0, 1.0], vector![1.0, 0.0, 1.0], vector![1.0, 0.0, 0.0] ],
            vector![ vector![1.0, 0.0, 0.0], vector![1.0, 1.0, 0.0], vector![1.0, 1.0, 1.0] ],
            vector![ vector![1.0, 1.0, 1.0], vector![1.0, 1.0, 0.0], vector![0.0, 1.0, 0.0] ],
            vector![ vector![0.0, 1.0, 0.0], vector![0.0, 1.0, 1.0], vector![1.0, 1.0, 1.0] ],
            vector![ vector![1.0, 1.0, 1.0], vector![0.0, 1.0, 1.0], vector![0.0, 0.0, 1.0] ],
            vector![ vector![0.0, 0.0, 1.0], vector![1.0, 0.0, 1.0], vector![1.0, 1.0, 1.0] ],
            vector![ vector![0.0, 0.0, 1.0], vector![1.0, 0.0, 1.0], vector![1.0, 1.0, 1.0] ],
        ];

        let colors = vec![
            0x00FFFFFF, 0x000000FF, 0x00FFFFFF, 0x000000FF, 0x00FFFFFF, 0x000000FF, 0x00FFFFFF,
            0x000000FF, 0x00FFFFFF, 0x000000FF, 0x00FFFFFF, 0x000000FF,
        ];

        render::draw_frame(
            &mut self.framebuffer,
            &mut self.zbuffer,
            view,
            faces,
            colors,
            options,
        );

        Ok(&self.framebuffer)
    }
}
