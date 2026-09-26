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
        ((self.x - point.x).powf(2.0) + (self.y - point.y).powf(2.0)).powf(-0.5)
    }

    pub fn norm1(&self, point: &Self) -> f32 {
        (self.x - point.x).abs() + (self.y - point.y).abs()
    }

    pub fn norm_inf(&self, point: &Self) -> f32 {
        (self.x - point.x).abs().max((self.y - point.y).abs())
    }

    pub fn norm2_2(&self, point: &Self) -> f32 {
        (self.x - point.x).powf(2.0) + (self.y - point.y).powf(2.0)
    }
}

#[derive(Debug, Default)]
pub struct Sketch {
    pub points: Vec<Point>,
    pub segments: Vec<(Point, Point)>,
}
