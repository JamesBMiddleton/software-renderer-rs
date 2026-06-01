use nalgebra;
use std::io::ErrorKind;

type Vec3 = nalgebra::SVector<f32, 3>;
type Vec4 = nalgebra::SVector<f32, 4>;
type Mat3 = nalgebra::SMatrix<f32, 3, 3>;
type Mat4 = nalgebra::SMatrix<f32, 4, 4>;
type Face3 = nalgebra::SVector<Vec3, 3>;
type Frame = nalgebra::DMatrix<u32>;
type ZBuffer = nalgebra::DMatrix<f32>;

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
    pub zbuffer: ZBuffer,
}

// Perspective projection matrix.
// Right-handed (+z points towards camera), -1,1 NDC range.
// Maps to openGL's gluPerspective(), GLM's perspectiveRH_NO().
fn projection_matrix_get(rad_fovy: f32, aspect: f32, z_near: f32, z_far: f32) -> Mat4 {
    assert!(rad_fovy > 0.0 && rad_fovy < std::f32::consts::PI * 2.0);
    assert!(z_near > 0.01);
    assert!(z_far > z_near);

    let a = (rad_fovy / 2.0).tan();
    let b = 1.0 / (aspect / a);
    let c = 1.0 / a;
    let d = -(z_far + z_near) / (z_far - z_near);
    let e = -(2.0 * z_far * z_near) / (z_far - z_near);

    Mat4::from_row_slice(&[
        b, 0.0, 0.0, 0.0, 0.0, c, 0.0, 0.0, 0.0, 0.0, d, e, 0.0, 0.0, -1.0, 1.0,
    ])
}

fn view_matrix_get(eye: Vec3, center: Vec3, up: Vec3) -> Mat4 {
    let f = (center - eye).normalize();
    let s = f.cross(&up).normalize();
    let u = s.cross(&f);

    Mat4::from_row_slice(&[
        s.x,
        s.y,
        s.z,
        -s.dot(&eye),
        u.x,
        u.y,
        u.z,
        -u.dot(&eye),
        -f.x,
        -f.y,
        -f.z,
        f.dot(&eye),
        0.0,
        0.0,
        0.0,
        0.0,
    ])
}

fn rasterize(framebuffer: &mut Frame, zbuffer: &mut ZBuffer, a_clip: Vec4, b_clip: Vec4, c_clip: Vec4, viewport_width: usize, viewport_height: usize, color: u32) {
    let a_ndc = a_clip.xyz() / a_clip.w;
    let b_ndc = b_clip.xyz() / b_clip.w;
    let c_ndc = c_clip.xyz() / c_clip.w;

    let x_scale = viewport_width as f32 / 2.0;
    let x_translate = viewport_width as f32 / 2.0;
    let y_scale = viewport_height as f32 / 2.0;
    let y_translate = viewport_height as f32 / 2.0;

    let a_screen = Vec3::new((a_ndc.x * x_scale) + x_translate, (a_ndc.y * y_scale) + y_translate, a_ndc.z);
    let b_screen = Vec3::new((b_ndc.x * x_scale) + x_translate, (b_ndc.y * y_scale) + y_translate, b_ndc.z);
    let c_screen = Vec3::new((c_ndc.x * x_scale) + x_translate, (c_ndc.y * y_scale) + y_translate, c_ndc.z);

    let abc = Mat3::from_row_slice(&[
        a_screen.x, a_screen.y, 1.0,
        b_screen.x, b_screen.y, 1.0,
        c_screen.x, c_screen.y, 1.0]);

    if abc.determinant() < 1.0 {
        return;
    }

    let bbox_max_x = a_screen.x.max(b_screen.x).max(c_screen.x) as usize;
    let bbox_max_y = a_screen.y.max(b_screen.y).max(c_screen.y) as usize;
    let bbox_min_x = a_screen.x.min(b_screen.x).min(c_screen.x) as usize;
    let bbox_min_y = a_screen.y.min(b_screen.y).min(c_screen.y) as usize;

    for x in bbox_min_x..bbox_max_x {
        for y in bbox_min_y..bbox_max_y {
            let z_bias = 0.003;
            let p = Vec3::new(x as f32, y as f32, 1.0);
            let bary = abc.try_inverse().unwrap().transpose() * p; // could panic!
            
            if bary.x < 0.0 || bary.y < 0.0 || bary.z < 0.0 {
                continue;
            }

            let z = (a_screen.z * bary.x) + (b_screen.z * bary.y) + (c_screen.z * bary.z); 
            if zbuffer[(x, y)] < (z-z_bias) {
                continue;
            }

            zbuffer[(x, y)] = z;
            framebuffer[(x, y)] = color;
        }
    }
}

impl Renderer {
    pub fn get_frame(
        &mut self,
        view: View,
        faces: Vec<Face3>,
        colors: Vec<u32>,
        options: Options,
    ) -> Result<&Frame, ErrorKind> {
        self.framebuffer =
            Frame::from_element(options.viewport_width, options.viewport_height, 0xFF16110E);
        self.zbuffer = ZBuffer::from_element(options.viewport_width, options.viewport_height, 1.0);

        let ratio = (options.viewport_width as f32) / (options.viewport_height as f32);
        let projection_matrix =
            projection_matrix_get(options.rad_fovy, ratio, options.z_near, options.z_far);
        let eye_view_matrix = view_matrix_get(view.eye, view.center, view.up);

        for (face, color) in faces.iter().zip(colors.iter()) {
            // let world : Face4 = face.iter().map(|vertex| vertex.to_homogeneous()).collect(); why
            // doesn't this work?
            
            let a_world = face[0].to_homogeneous();
            let b_world = face[1].to_homogeneous();
            let c_world = face[2].to_homogeneous();

            let a_eye = eye_view_matrix * a_world;
            let b_eye = eye_view_matrix * b_world;
            let c_eye = eye_view_matrix * c_world;

            let a_clip = projection_matrix * a_eye;
            let b_clip = projection_matrix * b_eye;
            let c_clip = projection_matrix * c_eye;

            rasterize(&mut self.framebuffer, &mut self.zbuffer, a_clip, b_clip, c_clip, options.viewport_width, options.viewport_height, *color);
        }

        return Ok(&self.framebuffer);
    }
}
