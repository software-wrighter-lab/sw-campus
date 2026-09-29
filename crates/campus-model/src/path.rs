use crate::{Catalog, Place, PlaceId};
use std::collections::HashSet;

#[must_use]
pub fn resolve<'a>(catalog: &'a Catalog, segments: &[&str]) -> Option<&'a Place> {
    let mut current = catalog.root()?;
    for segment in segments {
        let next = current.children.iter().find(|id| id.0 == *segment)?;
        current = catalog.get(next)?;
    }
    Some(current)
}

#[must_use]
pub fn ancestors<'a>(catalog: &'a Catalog, id: &PlaceId) -> Option<Vec<&'a Place>> {
    let mut chain = Vec::new();
    let mut current = catalog.get(id)?;
    let mut seen = HashSet::new();
    loop {
        if !seen.insert(current.id.clone()) {
            return None;
        }
        chain.push(current);
        current = catalog
            .places()
            .iter()
            .find(|place| place.children.iter().any(|child| child == &current.id))?;
        if current.kind == crate::PlaceKind::Campus {
            chain.push(current);
            break;
        }
    }
    chain.reverse();
    Some(chain)
}

#[must_use]
pub fn url(catalog: &Catalog, id: &PlaceId) -> Option<String> {
    let chain = ancestors(catalog, id)?;
    let tail = chain
        .iter()
        .skip(1)
        .map(|place| place.id.0.as_str())
        .collect::<Vec<_>>();
    (!tail.is_empty()).then(|| format!("/campus/{}", tail.join("/")))
}
