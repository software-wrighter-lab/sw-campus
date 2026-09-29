use crate::Scene;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Scenes {
    scenes: Vec<Scene>,
}

impl Scenes {
    /// Parses the two scenes compiled into the scene crate.
    ///
    /// # Errors
    ///
    /// Returns the RON parser error when either embedded scene is malformed.
    pub fn embedded() -> Result<Self, ron::error::SpannedError> {
        let campus: Scene = ron::from_str(include_str!("../../../content/scenes/campus.ron"))?;
        let museum: Scene =
            ron::from_str(include_str!("../../../content/scenes/computer-history.ron"))?;
        Ok(Self {
            scenes: vec![campus, museum],
        })
    }

    #[must_use]
    pub fn get(&self, id: &str) -> Option<&Scene> {
        self.scenes.iter().find(|scene| scene.id == id)
    }

    #[must_use]
    pub fn all(&self) -> &[Scene] {
        &self.scenes
    }
}
