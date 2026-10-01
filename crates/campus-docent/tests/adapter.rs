use campus_docent::{Outcome, ask, resource_ids};
use campus_model::Catalog;

fn catalog() -> Catalog {
    Catalog::embedded().expect("catalog parses")
}

#[test]
fn finds_the_console_from_plain_language() {
    let catalog = catalog();
    let reply = ask(&catalog, "where is the console with toggle switches");
    assert_eq!(reply.outcome, Outcome::One);
    assert_eq!(reply.offer[0].0, "1130");
    assert_eq!(resource_ids(&catalog, &reply), vec!["campus:1130"]);
}

#[test]
fn offers_several_matches_when_the_question_is_broad() {
    let catalog = catalog();
    let reply = ask(&catalog, "where is the 1130");
    assert_eq!(reply.outcome, Outcome::Several);
    assert!(reply.offer.iter().any(|id| id.0 == "ibm-1130"));
}

#[test]
fn refuses_unknown_words_without_pins() {
    let catalog = catalog();
    let reply = ask(&catalog, "where is the moon observatory");
    assert_eq!(reply.outcome, Outcome::Rephrase);
    assert!(reply.offer.is_empty());
}

#[test]
fn reports_known_but_unmatched_words() {
    let catalog = catalog();
    let reply = ask(&catalog, "take me to the printer");
    assert_eq!(reply.outcome, Outcome::Several);
    assert!(reply.offer.iter().any(|id| id.0 == "1132"));
}

#[test]
fn reports_planned_destinations_without_pins() {
    let catalog = catalog();
    let reply = ask(&catalog, "where is the future digital media studio");
    assert!(matches!(reply.outcome, Outcome::NotYet(_)));
    assert!(reply.offer.is_empty());
}

#[test]
fn reports_known_words_that_point_nowhere_without_pins() {
    let catalog = catalog();
    let reply = ask(&catalog, "is that moon observatory here");
    assert_eq!(reply.outcome, Outcome::NothingHere);
    assert!(reply.offer.is_empty());
}

#[test]
fn apl_query_prefers_language_projects_over_card_machines() {
    let catalog = catalog();
    let reply = ask(&catalog, "what APL related work is there");
    assert_eq!(reply.outcome, Outcome::Several);
    assert!(reply.offer.iter().any(|id| id.0 == "apl-cor24"));
    assert!(reply.offer.iter().any(|id| id.0 == "x-etal"));
    assert!(reply.offer.iter().any(|id| id.0 == "sw-mlpl"));
    assert!(!reply.offer.iter().any(|id| id.0 == "029" || id.0 == "1442"));
}
