use nalgebra;

type Vec3 = nalgebra::SVector<f32, 3>;
type Vec4 = nalgebra::SVector<f32, 4>;
type Mat3 = nalgebra::SMatrix<f32, 3, 3>;
type Mat4 = nalgebra::SMatrix<f32, 4, 4>;
type Face3 = nalgebra::SVector<Vec3, 3>;
type Face4 = nalgebra::SVector<Vec4, 3>;
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

fn rasterize(framebuffer: &mut Frame, zbuffer: &mut ZBuffer, clip: Face4, viewport_width: usize, viewport_height: usize, color: u32) {

    let ndc = Face3::from_iterator(clip.iter().map(|vertex| vertex.xyz() / vertex.w ));

    let x_scale = viewport_width as f32 / 2.0;
    let x_translate = viewport_width as f32 / 2.0;
    let y_scale = viewport_height as f32 / 2.0;
    let y_translate = viewport_height as f32 / 2.0;

    let screen = Face3::from_iterator(ndc.iter().map(|vertex| Vec3::new((vertex.x * x_scale) + x_translate, (vertex.y * y_scale) + y_translate, vertex.z)));

    let abc = Mat3::from_row_slice(&[
        screen[0].x, screen[0].y, 1.0,
        screen[1].x, screen[1].y, 1.0,
        screen[2].x, screen[2].y, 1.0]);

    if abc.determinant() < 1.0 {
        return;
    }

    let bbox_max_x = screen[0].x.max(screen[1].x).max(screen[2].x) as usize;
    let bbox_max_y = screen[0].y.max(screen[1].y).max(screen[2].y) as usize;
    let bbox_min_x = screen[0].x.min(screen[1].x).min(screen[2].x) as usize;
    let bbox_min_y = screen[0].y.min(screen[1].y).min(screen[2].y) as usize;

    for x in bbox_min_x..bbox_max_x {
        for y in bbox_min_y..bbox_max_y {
            if let Some(abc_inv) = abc.try_inverse() {
                let p = Vec3::new(x as f32, y as f32, 1.0);
                let bary = abc_inv.transpose() * p;

                if bary.x < 0.0 || bary.y < 0.0 || bary.z < 0.0 {
                    continue;
                }

                let z = (screen[0].z * bary.x) + (screen[1].z * bary.y) + (screen[2].z * bary.z); 
                let z_bias = 0.003;
                if zbuffer[(x, y)] < (z-z_bias) {
                    continue;
                }

                zbuffer[(x, y)] = z;
                framebuffer[(x, y)] = color;
            }
        }
    }
}

pub fn draw_frame(
        framebuffer: &mut Frame,
        zbuffer: &mut ZBuffer,
        view: View,
        faces: Vec<Face3>,
        colors: Vec<u32>,
        options: Options,
    ) {
        *framebuffer =
            Frame::from_element(options.viewport_width, options.viewport_height, 0xFF16110E);
        *zbuffer = ZBuffer::from_element(options.viewport_width, options.viewport_height, 2.0);

        let ratio = (options.viewport_width as f32) / (options.viewport_height as f32);
        let projection_matrix =
            projection_matrix_get(options.rad_fovy, ratio, options.z_near, options.z_far);
        let view_matrix = view_matrix_get(view.eye, view.center, view.up);

        for (face, color) in faces.iter().zip(colors.iter()) {
            let world = Face4::from_iterator(face.iter().map(|vertex| vertex.push(1.0)));
            let eye = Face4::from_iterator(world.iter().map(|vertex| view_matrix * *vertex));
            let clip = Face4::from_iterator(eye.iter().map(|vertex| projection_matrix * *vertex));
            rasterize(framebuffer, zbuffer, clip, options.viewport_width, options.viewport_height, *color);
        }
}
