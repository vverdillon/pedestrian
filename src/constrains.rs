use crate::sketch::Point;



#[derive(Debug)]
pub struct ConstrainDistance {
    weight: f32,
    distance_2: f32,
    point1: usize,
    point2: usize,
}

#[derive(Debug)]
pub struct ConstrainMouse {
    weight: f32,
    point: usize,
    mouse: Point,
}

/// Represents a constrain of N dimensions.
trait Constrain<T, const N: usize> {
    /// Return the score of the constraint
    fn score(&self, variables: [f32; N]) -> f32;
    /// Return the gradient of the score.
    fn gradient(&self, variables: [f32; N]) -> [f32; N];
    /// Update the constrain parameter.
    fn update(&mut self, new_parameters: T) -> ();
}

impl<const N: usize> Constrain<(f32, f32, usize, usize), N> for ConstrainDistance {
    fn score(&self, points: [f32; N]) -> f32 {
        let p1_x = points[2 * self.point1];
        let p1_y = points[2 * self.point1 + 1];
        let p2_x = points[2 * self.point2];
        let p2_y = points[2 * self.point2 + 1];

        self.weight
            * ((p1_x - p2_x).powf(2.0) + (p1_y - p2_y).powf(2.0) - self.distance_2).powf(2.0)
    }

    fn gradient(&self, points: [f32; N]) -> [f32; N] {
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

        let dist2 = (p1_x - p2_x).powf(2.0) + (p1_y - p2_y).powf(2.0) - self.distance_2;

        let dx1 = self.weight * 4.0 * dist2 * (p1_x - p2_x);
        let dy1 = self.weight * 4.0 * dist2 * (p1_y - p2_y);

        let dx2 = -dx1;
        let dy2 = -dy1;

        let mut gradient = [0.0; N];

        gradient[2 * self.point1] = dx1;
        gradient[2 * self.point1 + 1] = dy1;
        gradient[2 * self.point2] = dx2;
        gradient[2 * self.point2 + 1] = dy2;

        gradient
    }

    /// Update the distance constrain by providing:
    /// - The constrain weight
    /// - The wanted distance
    /// - The first Point
    /// - The second Point
    fn update(&mut self, new_parameters: (f32, f32, usize, usize)) {
        self.weight = new_parameters.0;
        self.distance_2 = new_parameters.1.powf(2.0);
        self.point1 = new_parameters.2;
        self.point2 = new_parameters.3;
    }
}

impl<const N: usize> Constrain<(f32, usize, Point), N> for ConstrainMouse {
    fn score(&self, points: [f32; N]) -> f32 {
        let p_x = points[2 * self.point];
        let p_y = points[2 * self.point + 1];

        self.weight * (p_x - self.mouse.x).powf(2.0) + (p_y - self.mouse.y).powf(2.0)
    }

    fn gradient(&self, points: [f32; N]) -> [f32; N] {
        let p_x = points[2 * self.point];
        let p_y = points[2 * self.point + 1];

        let dx = self.weight * 2.0 * (p_x - self.mouse.x);
        let dy = self.weight * 2.0 * (p_y - self.mouse.y);

        let mut gradient = [0.0; N];

        gradient[2 * self.point] = dx;
        gradient[2 * self.point + 1] = dy;

        gradient
    }

    /// Update the distance constrain by providing:
    /// - The constrain weight
    /// - The Point
    /// - The mouse position
    fn update(&mut self, new_parameters: (f32, usize, Point)) {
        self.weight = new_parameters.0;
        self.point = new_parameters.1;
        self.mouse = new_parameters.2;
    }
}
