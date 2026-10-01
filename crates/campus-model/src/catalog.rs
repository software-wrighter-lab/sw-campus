use crate::place::{Place, PlaceId};
use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Catalog {
    pub(crate) places: Vec<Place>,
}

impl Catalog {
    /// Parses the catalog compiled into the model crate.
    ///
    /// # Errors
    ///
    /// Returns the RON parser error when the embedded content is malformed.
    pub fn embedded() -> Result<Self, ron::error::SpannedError> {
        ron::from_str(include_str!("../../../content/catalog.ron"))
    }

    #[must_use]
    pub fn root(&self) -> Option<&Place> {
        self.places
            .iter()
            .find(|place| place.kind == crate::PlaceKind::Campus)
    }

    #[must_use]
    pub fn get(&self, id: &PlaceId) -> Option<&Place> {
        self.places.iter().find(|place| &place.id == id)
    }

    #[must_use]
    pub fn children(&self, id: &PlaceId) -> Vec<&Place> {
        self.get(id)
            .map(|place| {
                place
                    .children
                    .iter()
                    .filter_map(|child| self.get(child))
                    .collect()
            })
            .unwrap_or_default()
    }

    #[must_use]
    pub fn all_places(&self) -> Vec<&Place> {
        self.places.iter().collect()
    }
}
