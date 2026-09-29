use crate::Shape;

impl Shape {
    #[must_use]
    pub fn points(&self) -> Vec<(f32, f32)> {
        match self {
            Self::Polygon(points) => points.clone(),
            Self::Rect { x, y, w, h } => {
                vec![(*x, *y), (*x + *w, *y), (*x + *w, *y + *h), (*x, *y + *h)]
            }
        }
    }

    #[must_use]
    pub fn centroid(&self) -> Option<(f32, f32)> {
        let points = self.points();
        (!points.is_empty()).then(|| {
            let (x, y) = points
                .iter()
                .fold((0.0, 0.0), |(sx, sy), (px, py)| (sx + px, sy + py));
            let count = u16::try_from(points.len()).unwrap_or(u16::MAX);
            (x / f32::from(count), y / f32::from(count))
        })
    }
}
