use eframe::egui;
use eframe::egui::{Color32, Pos2, Sense, Stroke, Ui, vec2};

use crate::sketch::{Point, Sketch};

impl Sketch {
    pub fn ui(&mut self, ui: &mut Ui) -> egui::Response {
        // Dimensions du canvas
        let canvas_width = ui.available_width();
        let canvas_height = ui.available_height();

        let canvas_size = vec2(canvas_width, canvas_height);
        let (response, painter) = ui.allocate_painter(canvas_size, Sense::click_and_drag());

        // Colors
        let red = Color32::from_rgb(255, 0, 0);
        let green = Color32::from_rgb(0, 255, 0);
        let blue = Color32::from_rgb(0, 0, 255);
        let white = Color32::from_rgb(255, 255, 255);

        let stroke_red = Stroke::new(2.0, red);
        let _stroke_green = Stroke::new(2.0, green);
        let _stroke_blue = Stroke::new(2.0, blue);
        let stroke_white = Stroke::new(2.0, white);

        // drag and drop parameters
        let point_radius_sq = 11.0 * 11.0;

        // check if we are dragging a point
        if (response.drag_started() || response.dragged()) && self.dragged_point_index.is_none() {
            if let Some(interaction_pos) = response.interact_pointer_pos() {
                if let Some((index, _)) = self.points.iter().enumerate().find(|(_, p)| {
                    p.norm2_2(&Point {
                        x: interaction_pos.x,
                        y: interaction_pos.y,
                    }) <= point_radius_sq
                }) {
                    self.dragged_point_index = Some(index);
                }
            }
        }

        // Move the selected point through the mouse constraint and the solver.
        if self.dragged_point_index.is_some() && response.dragged() {
            if let Some(interaction_pos) = response.interact_pointer_pos() {
                self.solve(Point {
                    x: interaction_pos.x,
                    y: interaction_pos.y,
                });
            }
        }

        // release the previously selected point
        if response.drag_stopped() || response.clicked() {
            self.dragged_point_index = None;
        }

        // draw segments
        self.segments.iter().for_each(|(p1, p2)| {
            let p1_pos: Pos2 = (&self.points[*p1]).into();
            let p2_pos: Pos2 = (&self.points[*p2]).into();
            painter.line_segment([p1_pos, p2_pos], stroke_white);
        });

        // draw points
        self.points.iter().enumerate().for_each(|(index, p)| {
            let p_pos: Pos2 = p.into();

            if self.dragged_point_index == Some(index) {
                painter.circle(p_pos, 7.0, blue, Stroke::new(3.0, blue));
            } else {
                painter.circle(p_pos, 4.0, red, stroke_red);
            }
        });

        response
    }
}
