use campus_model::{Catalog, Place, PlaceId, Status, resource_id};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Outcome {
    One,
    Several,
    NotYet(String),
    Rephrase,
    NothingHere,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Reply {
    pub outcome: Outcome,
    pub offer: Vec<PlaceId>,
    pub because: String,
}

#[must_use]
pub fn ask(catalog: &Catalog, question: &str) -> Reply {
    let tokens = tokens(question);
    if tokens.is_empty() {
        return refusal(Outcome::Rephrase, "Try asking where a place or project is.");
    }
    let mut matches = catalog
        .all_places()
        .iter()
        .filter_map(|place| score(place, &tokens).map(|score| (score, *place)))
        .collect::<Vec<_>>();
    matches.sort_by(|left, right| {
        right
            .0
            .cmp(&left.0)
            .then_with(|| left.1.id.0.cmp(&right.1.id.0))
    });
    reply_for_matches(catalog, &tokens, &matches)
}

fn reply_for_matches(catalog: &Catalog, tokens: &[String], matches: &[(u32, &Place)]) -> Reply {
    let Some((top_score, top)) = matches.first() else {
        let known = catalog
            .all_places()
            .iter()
            .any(|place| searchable(place).iter().any(|word| tokens.contains(word)));
        return if known || tokens.iter().any(|token| token == "here") {
            refusal(
                Outcome::NothingHere,
                "I know those words, but nothing here matches them.",
            )
        } else {
            refusal(
                Outcome::Rephrase,
                "I do not recognize that yet. Try a campus place or project.",
            )
        };
    };
    if asks_not_yet(tokens) && is_unavailable(top) {
        return refusal(
            Outcome::NotYet(format!("{} is planned, but it is not open yet.", top.title)),
            "The catalog knows this destination but marks it as not open.",
        );
    }
    let offer = matches
        .iter()
        .filter(|(score, _)| *score * 4 >= *top_score * 3)
        .take(3)
        .map(|(_, place)| place.id.clone())
        .collect::<Vec<_>>();
    let outcome = if offer.len() == 1 {
        Outcome::One
    } else {
        Outcome::Several
    };
    let names = offer
        .iter()
        .filter_map(|id| catalog.get(id))
        .map(|place| place.title.as_str())
        .collect::<Vec<_>>()
        .join(", ");
    Reply {
        outcome,
        offer,
        because: format!("Matching campus vocabulary: {names}."),
    }
}

fn score(place: &Place, query_tokens: &[String]) -> Option<u32> {
    let explicit = indexed_tokens(place, true);
    let descriptive = indexed_tokens(place, false);
    let explicit_overlap = query_tokens
        .iter()
        .filter(|token| explicit.contains(token))
        .count();
    let descriptive_overlap = query_tokens
        .iter()
        .filter(|token| descriptive.contains(token))
        .count();
    let phrase = place
        .aliases
        .iter()
        .chain(place.example_queries.iter())
        .any(|text| contains_sequence(&tokens(text), query_tokens))
        || place.links.iter().any(|link| {
            contains_sequence(&tokens(&link.label), query_tokens)
                || contains_sequence(&tokens(&link.url), query_tokens)
        });
    let score = explicit_overlap * 10 + descriptive_overlap;
    (score > 0).then_some(u32::try_from(score).unwrap_or(u32::MAX) + u32::from(phrase) * 20)
}

fn searchable(place: &Place) -> Vec<String> {
    indexed_tokens(place, true)
}

fn indexed_tokens(place: &Place, include_explicit: bool) -> Vec<String> {
    let mut texts = place
        .links
        .iter()
        .flat_map(|link| [&link.label, &link.url])
        .chain([&place.title, &place.tagline]);
    let mut words = texts
        .by_ref()
        .flat_map(|text| tokens(text))
        .collect::<Vec<_>>();
    if include_explicit {
        words.extend(
            place
                .aliases
                .iter()
                .chain(place.example_queries.iter())
                .flat_map(|text| tokens(text)),
        );
    }
    words.extend(acronyms(&words));
    words
}

fn acronyms(words: &[String]) -> Vec<String> {
    words
        .windows(2)
        .filter_map(|pair| {
            Some(format!(
                "{}{}",
                pair[0].chars().next()?,
                pair[1].chars().next()?
            ))
        })
        .collect()
}

fn contains_sequence(words: &[String], query: &[String]) -> bool {
    !query.is_empty() && words.windows(query.len()).any(|window| window == query)
}

fn tokens(text: &str) -> Vec<String> {
    normalize(text)
        .split_whitespace()
        .filter(|token| !STOP_WORDS.contains(token))
        .map(str::to_owned)
        .collect()
}

fn normalize(text: &str) -> String {
    text.chars()
        .map(|character| {
            if character.is_alphanumeric() {
                character.to_ascii_lowercase()
            } else {
                ' '
            }
        })
        .collect()
}

fn asks_not_yet(tokens: &[String]) -> bool {
    ["coming", "future", "planned", "soon", "yet"]
        .iter()
        .any(|word| tokens.iter().any(|token| token == word))
}

fn is_unavailable(place: &Place) -> bool {
    matches!(place.status, Status::Placeholder | Status::ComingSoon)
}

fn refusal(outcome: Outcome, because: &str) -> Reply {
    Reply {
        outcome,
        offer: Vec::new(),
        because: because.to_owned(),
    }
}

const STOP_WORDS: &[&str] = &[
    "a", "an", "and", "at", "can", "do", "for", "i", "in", "is", "me", "of", "on", "show", "take",
    "that", "the", "to", "where", "what", "there", "related", "work", "with",
];

#[must_use]
pub fn resource_ids(catalog: &Catalog, reply: &Reply) -> Vec<String> {
    reply
        .offer
        .iter()
        .filter_map(|id| resource_id(catalog, id))
        .collect()
}
