use campus_model::PlaceId;
use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Scene {
    pub id: String,
    pub place: PlaceId,
    pub image: String,
    pub width: u32,
    pub height: u32,
    pub hotspots: Vec<Hotspot>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Hotspot {
    pub place: PlaceId,
    pub shape: Shape,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub enum Shape {
    Polygon(Vec<(f32, f32)>),
    Rect { x: f32, y: f32, w: f32, h: f32 },
}
