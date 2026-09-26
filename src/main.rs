use eframe::egui;

use crate::sketch::Point;

mod constrains;
mod paint;
mod sketch;

#[derive(Default)]
/// This struct holds the data (state) for our application.
pub struct MyApp {
    sketch: sketch::Sketch,
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            egui::Frame::canvas(ui.style()).show(ui, |ui| {
                ui.set_width(ui.available_width());
                ui.set_height(ui.available_height());

                let sketch_response = self.sketch.ui(ui);
                if sketch_response.clicked() {
                    println!("{}", sketch_response.hover_pos().unwrap());
                }
            });
        });

        // update sketch
        ui.ctx().request_repaint();
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();

    let mut default_app = MyApp::default();

    default_app.sketch.points.push(Point { x: 100.0, y: 100.0 });
    default_app.sketch.points.push(Point { x: 150.0, y: 150.0 });
    default_app
        .sketch
        .segments
        .push((default_app.sketch.points[0], default_app.sketch.points[1]));

    eframe::run_native(
        "Pedestrian optimizer",
        options,
        Box::new(|_cc| Ok(Box::new(default_app))),
    )
}
