use eframe::egui;

use engine;

const VIEWPORT_WIDTH: usize = 800;
const VIEWPORT_HEIGHT: usize = 800;

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([VIEWPORT_WIDTH as f32, VIEWPORT_HEIGHT as f32]),
        ..Default::default()
    };

    let mut engine = engine::Engine {
        ..Default::default()
    };

    eframe::run_ui_native("Demo", options, move |ui, _frame| {
        egui::CentralPanel::default().show_inside(ui, |ui| {

            let frame = engine
                .get_frame(engine::Options {
                    viewport_width: VIEWPORT_WIDTH,
                    viewport_height: VIEWPORT_HEIGHT,
                })
                .unwrap();

            let pixeldata = frame
                .as_slice()
                .to_vec()
                .into_iter()
                .map(|rgba| {
                    egui::Color32::from_rgb(rgba as u8, (rgba >> 8) as u8, (rgba >> 16) as u8)
                })
                .collect();

            let image = egui::ColorImage::new([frame.nrows(), frame.ncols()], pixeldata);

            let texture = ui.ctx().load_texture("frame", image, Default::default());
            ui.image((texture.id(), texture.size_vec2()));

            println!("frame");
        });
    })
}
