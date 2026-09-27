use crate::sketch::Point;

#[derive(Debug)]
pub struct ConstraintDistance {
    pub weight: f32,
    distance_2: f32,
    point1: usize,
    point2: usize,
}

#[derive(Debug)]
pub struct ConstraintMouse {
    pub weight: f32,
    point: usize,
    mouse: Point,
}

/// Represents a constrain of N dimensions.
pub trait Constraint {
    /// Return the score of the constraint
    fn score(&self, variables: &[f32]) -> f32;
    /// Return the gradient of the score.
    fn gradient(&self, variables: &[f32]) -> Vec<f32>;
}

impl ConstraintDistance {
    pub fn new(weight: f32, distance: f32, point1: usize, point2: usize) -> Self {
        Self {
            weight,
            distance_2: distance.powi(2),
            point1,
            point2,
        }
    }
}

impl Constraint for ConstraintDistance {
    fn score(&self, points: &[f32]) -> f32 {
        let p1_x = points[2 * self.point1];
        let p1_y = points[2 * self.point1 + 1];
        let p2_x = points[2 * self.point2];
        let p2_y = points[2 * self.point2 + 1];

        self.weight * ((p1_x - p2_x).powi(2) + (p1_y - p2_y).powi(2) - self.distance_2).powi(2)
    }

    fn gradient(&self, points: &[f32]) -> Vec<f32> {
        //   d/dx1[ ((x1-x2)^2 + (y1-y2)^2 - d2)^2 ]
        // = 2 * ( (x1-x2)^2 + (y1-y2)^2 - d2 ) * d/dx1[ (x1-x2)^2 + (y1-y2)^2 - d2 ]
        // = 2 * ( (x1-x2)^2 + (y1-y2)^2 - d2 ) * d/dx1[ (x1-x2)^2 ]
        // = 2 * ( (x1-x2)^2 + (y1-y2)^2 - d2 ) * ( 2 * (x1 - x2))
        // = 4 * ( (x1-x2)^2 + (y1-y2)^2 - d2 ) * (x1 - x2)
        // = 4 * ( norm2_2(pi1, p2) - d2 ) * (x1 - x2)

        //   d/dx2[ ((x1-x2)^2 + (y1-y2)^2 - d2)^2 ]
        // = 2 * ( (x1-x2)^2 + (y1-y2)^2 - d2 ) * d/dx2[ (x1-x2)^2 + (y1-y2)^2 - d2 ]
        // = 2 * ( (x1-x2)^2 + (y1-y2)^2 - d2 ) * d/dx2[ (x1-x2)^2 ]
        // = 2 * ( (x1-x2)^2 + (y1-y2)^2 - d2 ) * ( -2 * (x1 - x2))
        // = -4 * ( (x1-x2)^2 + (y1-y2)^2 - d2 ) * (x1 - x2)
        // = -4 * ( norm2_2(pi1, p2) - d2 ) * (x1 - x2)

        let p1_x = points[2 * self.point1];
        let p1_y = points[2 * self.point1 + 1];
        let p2_x = points[2 * self.point2];
        let p2_y = points[2 * self.point2 + 1];

        let dist2 = (p1_x - p2_x).powi(2) + (p1_y - p2_y).powi(2) - self.distance_2;

        let dx1 = self.weight * 4.0 * dist2 * (p1_x - p2_x);
        let dy1 = self.weight * 4.0 * dist2 * (p1_y - p2_y);

        let dx2 = -dx1;
        let dy2 = -dy1;

        let mut gradient = vec![0.0; points.len()];

        gradient[2 * self.point1] = dx1;
        gradient[2 * self.point1 + 1] = dy1;
        gradient[2 * self.point2] = dx2;
        gradient[2 * self.point2 + 1] = dy2;

        gradient
    }
}

impl ConstraintMouse {
    pub fn new(weight: f32, point: usize, mouse: Point) -> Self {
        Self {
            weight,
            point,
            mouse,
        }
    }
}

impl Constraint for ConstraintMouse {
    fn score(&self, points: &[f32]) -> f32 {
        let p_x = points[2 * self.point];
        let p_y = points[2 * self.point + 1];

        self.weight * ((p_x - self.mouse.x).powi(2) + (p_y - self.mouse.y).powi(2))
    }

    fn gradient(&self, points: &[f32]) -> Vec<f32> {
        let p_x = points[2 * self.point];
        let p_y = points[2 * self.point + 1];

        let dx = self.weight * 2.0 * (p_x - self.mouse.x);
        let dy = self.weight * 2.0 * (p_y - self.mouse.y);

        let mut gradient = vec![0.0; points.len()];

        gradient[2 * self.point] = dx;
        gradient[2 * self.point + 1] = dy;

        gradient
    }
}
