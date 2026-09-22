use eframe::egui;
use eframe::egui::{Color32, Pos2, Sense, Stroke, Ui, vec2};

use crate::sketch;

impl sketch::Sketch {
    pub fn ui(&mut self, ui: &mut Ui) -> egui::Response {
        // dimensions
        let canvas_width = ui.available_width();
        let canvas_height = ui.available_height();

        let canvas_size = vec2(canvas_width, canvas_height);
        let (response, painter) = ui.allocate_painter(canvas_size, Sense::click());
        let rect = response.rect;
        let canvas_center = rect.center();

        // colors
        let red = Color32::from_rgb(255, 0, 0);
        let green = Color32::from_rgb(0, 255, 0);
        let blue = Color32::from_rgb(0, 0, 255);
        let white = Color32::from_rgb(255, 255, 255);

        let stroke = Stroke::new(2.0, white);

        // draw segments between points
        self.segments.iter().for_each(|(p1, p2)| {
            let p1_pos: Pos2 = p1.into();
            let p2_pos: Pos2 = p2.into();
            painter.line_segment([p1_pos, p2_pos], stroke);
        });

        // draw all points
        self.points.iter().for_each(|p| {
            let p_pos: Pos2 = p.into();
            painter.circle(p_pos, 5.0, red, stroke);
        });

        // let masse_coord = c + stick_length * vec2(alpha.sin(), alpha.cos());
        // let masse_coord_goal = c + stick_length * vec2(alpha_goal.sin(), alpha_goal.cos());
        //
        // // cross
        // painter.line_segment([c - vec2(0.0, 25.0), c + vec2(0.0, 25.0)], stroke);
        // painter.line_segment([c - vec2(25.0, 0.0), c + vec2(25.0, 0.0)], stroke);
        // painter.circle(c, 2.0, green, Stroke::new(2.0, green));
        //
        // // actual stick and masse
        // painter.line_segment([c, masse_coord], stroke);
        // painter.circle(masse_coord, 10.0, red, Stroke::new(2.0, red));
        //
        // // goal stick and masse
        // painter.line_segment([c, masse_coord_goal], stroke_alpha);
        // painter.circle(
        //     masse_coord_goal,
        //     10.0,
        //     green_alpha,
        //     Stroke::new(2.0, green_alpha),
        // );
        //
        // let text_pos = rect.min + vec2(10.0, 10.0);
        // let mut text: &str = "Pendulum - Simulation";
        // if pause {
        //     text = "Pendulum - Simulation (paused)";
        // }
        //
        // painter.text(
        //     text_pos,
        //     Align2::LEFT_TOP,
        //     text,
        //     FontId::proportional(14.0),
        //     Color32::WHITE,
        // );

        response
    }
}
