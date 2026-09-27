use crate::constraints::{Constraint, ConstraintDistance, ConstraintMouse};
use crate::constraints_optimiser::GradientDescent;
use eframe::egui::Pos2;

#[derive(Debug, Copy, Clone)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl From<&Point> for Pos2 {
    fn from(point: &Point) -> Self {
        Self::new(point.x, point.y)
    }
}

impl Point {
    pub fn norm2(&self, point: &Self) -> f32 {
        ((self.x - point.x).powf(2.0) + (self.y - point.y).powf(2.0)).powf(0.5)
    }

    pub fn norm2_2(&self, point: &Self) -> f32 {
        (self.x - point.x).powf(2.0) + (self.y - point.y).powf(2.0)
    }

    pub fn norm1(&self, point: &Self) -> f32 {
        (self.x - point.x).abs() + (self.y - point.y).abs()
    }

    pub fn norm_inf(&self, point: &Self) -> f32 {
        (self.x - point.x).abs().max((self.y - point.y).abs())
    }
}

#[derive(Debug, Default)]
pub struct Sketch {
    pub points: Vec<Point>,
    pub segments: Vec<(usize, usize)>,
    pub dragged_point_index: Option<usize>,
    distance_constraints: Vec<ConstraintDistance>,
}

impl Sketch {
    pub fn new(
        points: Vec<Point>,
        segments: Vec<(usize, usize)>,
        distance_constraints: Vec<ConstraintDistance>,
    ) -> Self {
        let mut sketch = Self {
            points,
            segments,
            dragged_point_index: None,
            distance_constraints,
        };
        sketch.solve_constraints(None);
        sketch
    }

    pub fn solve(&mut self, mouse_position: Point) {
        self.solve_constraints(Some(mouse_position));
    }

    fn solve_constraints(&mut self, mouse_position: Option<Point>) {
        let mouse_constraint = mouse_position.map(|position| {
            let dragged_point = self
                .dragged_point_index
                .expect("mouse constraint requires a dragged point");
            ConstraintMouse::new(100.0, dragged_point, position)
        });
        let mut constraints: Vec<&dyn Constraint> = self
            .distance_constraints
            .iter()
            .map(|constraint| constraint as &dyn Constraint)
            .collect();
        if let Some(mouse_constraint) = mouse_constraint.as_ref() {
            constraints.push(mouse_constraint);
        }

        let variables = self
            .points
            .iter()
            .flat_map(|point| [point.x, point.y])
            .collect::<Vec<_>>();
        let solved = GradientDescent::default().solve(&variables, &constraints);

        for (point, coordinates) in self.points.iter_mut().zip(solved.variables.chunks_exact(2)) {
            point.x = coordinates[0];
            point.y = coordinates[1];
        }
    }
}
