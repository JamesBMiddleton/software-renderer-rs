use eframe::egui;
use std::io::ErrorKind;

use engine;

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };

    eframe::run_ui_native("Demo", options, move |ui, _frame| {
        egui::CentralPanel::default().show_inside(ui, |ui| {
            // each frame do...

            // Output of engine needs to be of type in type "ImageData" to replace egui::ColorImage::example()
            // actual data will be in ImageData->Arc<ColorImage>->   Vec<Color32>
            // the options don't need changing, just match the window size to size of pixel array
            let frame = engine::get_frame(engine::Config {
                viewport_width: 1,
                viewport_height: 1,
            });
            match frame {
                Ok(f) => println!("{}", f.x),
                Err(e) => match e {
                    ErrorKind::NotFound => println!("wuh woh"),
                    _ => panic!("crash n burn"),
                },
            }
            let texture =
                ui.ctx()
                    .load_texture("frame", egui::ColorImage::example(), Default::default());
            ui.image((texture.id(), texture.size_vec2()));
        });
    })
}
