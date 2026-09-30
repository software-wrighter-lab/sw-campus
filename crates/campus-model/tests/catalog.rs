use campus_model::{Catalog, PlaceId, ancestors, resolve, resource_id, url};

fn catalog() -> Catalog {
    Catalog::embedded().expect("embedded catalog parses")
}

#[test]
fn embedded_catalog_is_valid() {
    catalog().validate().expect("catalog validates");
}

#[test]
fn paths_walk_only_declared_children() {
    let catalog = catalog();
    assert!(resolve(&catalog, &["computer-history", "ibm-1130", "1442"]).is_some());
    assert!(resolve(&catalog, &["ibm-1130"]).is_none());
}

#[test]
fn urls_and_ancestors_are_canonical() {
    let catalog = catalog();
    let id = PlaceId("1442".to_owned());
    assert_eq!(
        url(&catalog, &id).as_deref(),
        Some("/campus/computer-history/ibm-1130/1442")
    );
    let radio = ancestors(&catalog, &PlaceId("radio".to_owned())).expect("radio ancestors");
    let ids: Vec<_> = radio.iter().map(|place| place.id.0.as_str()).collect();
    assert_eq!(
        ids,
        ["campus", "computer-history", "ibm-1130", "1442", "radio"]
    );
}

#[test]
fn docent_vocabulary_and_resource_ids_are_stable() {
    let catalog = catalog();
    let console = catalog.get(&PlaceId("1130".to_owned())).expect("1130");
    assert!(
        console
            .aliases
            .iter()
            .any(|alias| alias == "toggle switches")
    );
    assert!(
        console
            .example_queries
            .iter()
            .any(|query| query.contains("console"))
    );
    assert_eq!(
        resource_id(&catalog, &PlaceId("1130".to_owned())).as_deref(),
        Some("campus:1130")
    );
}
