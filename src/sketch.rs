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

#[derive(Debug, Default)]
pub struct Sketch {
    pub points: Vec<Point>,
    pub segments: Vec<(Point, Point)>,
}
