use std::io::ErrorKind;
use eframe::egui;

use engine;

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
                        //
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

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };

    eframe::run_ui_native("Demo", options, move |ui, _frame| {
        egui::CentralPanel::default().show_inside(ui, |_ui| {});
    })
}
