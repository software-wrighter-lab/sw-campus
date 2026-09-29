use campus_model::{Catalog, PlaceId};
use campus_scene::{Scenes, Shape, validate};

fn scenes() -> Scenes {
    Scenes::embedded().expect("embedded scenes parse")
}

#[test]
fn embedded_scenes_validate_against_catalog() {
    let catalog = Catalog::embedded().expect("catalog parses");
    for scene in scenes().all() {
        validate(scene, &catalog).expect("scene validates");
    }
}

#[test]
fn invalid_hotspot_names_the_non_child() {
    let catalog = Catalog::embedded().expect("catalog parses");
    let mut scene = scenes().get("campus").expect("campus scene").clone();
    scene.hotspots[0].place = PlaceId("1442".to_owned());
    let errors = validate(&scene, &catalog).expect_err("bad child rejected");
    assert!(errors.iter().any(|error| error.contains("1442")));
}

#[test]
fn rectangles_supply_four_points_and_a_centroid() {
    let shape = Shape::Rect {
        x: 10.0,
        y: 20.0,
        w: 30.0,
        h: 40.0,
    };
    assert_eq!(
        shape.points(),
        [(10.0, 20.0), (40.0, 20.0), (40.0, 60.0), (10.0, 60.0)]
    );
    assert_eq!(shape.centroid(), Some((25.0, 40.0)));
}
