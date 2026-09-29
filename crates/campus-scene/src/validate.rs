use crate::Scene;
use campus_model::Catalog;

/// Checks scene dimensions, geometry bounds, and catalog containment.
///
/// # Errors
///
/// Returns every scene invariant that failed.
pub fn validate(scene: &Scene, catalog: &Catalog) -> Result<(), Vec<String>> {
    let mut errors = Vec::new();
    if scene.image.is_empty() {
        errors.push(format!("{} has an empty image", scene.id));
    }
    if scene.width == 0 || scene.height == 0 {
        errors.push(format!("{} has no image dimensions", scene.id));
    }
    if catalog.get(&scene.place).is_none() {
        errors.push(format!(
            "{} names missing place {}",
            scene.id, scene.place.0
        ));
    }
    for hotspot in &scene.hotspots {
        errors.extend(validate_hotspot(scene, catalog, hotspot));
    }
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

fn validate_hotspot(scene: &Scene, catalog: &Catalog, hotspot: &crate::Hotspot) -> Vec<String> {
    let mut errors = points_error(scene, &hotspot.place, &hotspot.shape.points());
    let place = catalog.get(&hotspot.place);
    let parent = catalog.get(&scene.place);
    if place.is_none() {
        errors.push(format!(
            "{} hotspot names missing place {}",
            scene.id, hotspot.place.0
        ));
    }
    if parent.is_some_and(|parent| !parent.children.contains(&hotspot.place)) {
        errors.push(format!(
            "{} hotspot {} is not a child of {}",
            scene.id, hotspot.place.0, scene.place.0
        ));
    }
    errors
}

#[allow(clippy::cast_precision_loss)]
fn points_error(
    scene: &Scene,
    place: &campus_model::PlaceId,
    points: &[(f32, f32)],
) -> Vec<String> {
    points
        .iter()
        .enumerate()
        .filter(|(_, (x, y))| {
            *x < 0.0 || *y < 0.0 || *x > scene.width as f32 || *y > scene.height as f32
        })
        .map(|(index, _)| {
            format!(
                "{} hotspot {} point {} is outside image",
                scene.id, place.0, index
            )
        })
        .collect()
}
