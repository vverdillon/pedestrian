use eframe::egui;

use crate::constraints::ConstraintDistance;
use crate::sketch::Point;
use crate::sketch::Sketch;

mod constraints;
mod constraints_optimiser;
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

                self.sketch.ui(ui);
            });
        });

        // update sketch
        ui.ctx().request_repaint();
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();

    let points = vec![
        Point { x: 100.0, y: 100.0 },
        Point { x: 150.0, y: 150.0 },
        Point { x: 150.0, y: 300.0 },
        Point { x: 300.0, y: 300.0 },
    ];
    let segments = vec![(0, 1), (1, 2), (2, 0), (2, 3)];
    let distance_constraints = vec![
        ConstraintDistance::new(1.0, 100.0, 0, 1),
        ConstraintDistance::new(1.0, 100.0, 1, 2),
        ConstraintDistance::new(1.0, 100.0, 2, 0),
        ConstraintDistance::new(1.0, 100.0, 2, 3),
    ];
    let sketch = Sketch::new(points, segments, distance_constraints);
    let default_app = MyApp { sketch };

    eframe::run_native(
        "Pedestrian optimizer",
        options,
        Box::new(|_cc| Ok(Box::new(default_app))),
    )
}
