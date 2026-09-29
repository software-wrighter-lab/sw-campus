mod catalog;
mod path;
mod place;
mod validate;

pub use catalog::Catalog;
pub use path::{ancestors, resolve, url};
pub use place::{Link, LinkKind, Place, PlaceId, PlaceKind, Status};
pub use validate::validate;

impl Catalog {
    pub(crate) fn places(&self) -> &[Place] {
        &self.places
    }
}
