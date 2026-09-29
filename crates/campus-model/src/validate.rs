use crate::{Catalog, PlaceKind};
use std::collections::HashSet;

/// Checks IDs, references, roots, and containment cycles.
///
/// # Errors
///
/// Returns every catalog invariant that failed.
pub fn validate(catalog: &Catalog) -> Result<(), Vec<String>> {
    let mut errors = validate_ids(catalog);
    errors.extend(validate_children(catalog));
    errors.extend(validate_cycles(catalog));
    if catalog
        .places()
        .iter()
        .filter(|p| p.kind == PlaceKind::Campus)
        .count()
        != 1
    {
        errors.push("catalog must contain exactly one campus".to_owned());
    }
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

fn validate_ids(catalog: &Catalog) -> Vec<String> {
    let mut seen = HashSet::new();
    catalog
        .places()
        .iter()
        .filter_map(|place| {
            let valid = place
                .id
                .0
                .chars()
                .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-');
            (!valid || !seen.insert(place.id.clone()))
                .then(|| format!("invalid or duplicate id: {}", place.id.0))
        })
        .collect()
}

fn validate_children(catalog: &Catalog) -> Vec<String> {
    catalog
        .places()
        .iter()
        .flat_map(|place| {
            place
                .children
                .iter()
                .filter(|child| {
                    !catalog
                        .places()
                        .iter()
                        .any(|candidate| candidate.id == **child)
                })
                .map(|child| format!("{} references missing child {}", place.id.0, child.0))
        })
        .collect()
}

fn validate_cycles(catalog: &Catalog) -> Vec<String> {
    catalog
        .places()
        .iter()
        .filter_map(|start| {
            let mut seen = HashSet::new();
            let mut current = start;
            loop {
                if !seen.insert(current.id.clone()) {
                    return Some(format!("cycle includes {}", current.id.0));
                }
                let parent = catalog
                    .places()
                    .iter()
                    .find(|place| place.children.contains(&current.id));
                match parent {
                    Some(place) => current = place,
                    None => return None,
                }
            }
        })
        .collect()
}

impl Catalog {
    /// Validates every invariant in the catalog.
    ///
    /// # Errors
    ///
    /// Returns every catalog invariant that failed.
    pub fn validate(&self) -> Result<(), Vec<String>> {
        validate(self)
    }
}
