Create the semantic model: crate crates/campus-model plus content/catalog.ron describing the MVP place tree. No web dependencies; pure Rust, TDD.

Read docs/plan.md ("The thesis", "Semantic model", "Content for the MVP").

Deliver:
- crates/campus-model (lib, added to the workspace). Modules, each <= 4 fns: place.rs (PlaceId, PlaceKind, Status, Link, LinkKind, Place; serde Deserialize), catalog.rs (Catalog: root(), get(), children()), path.rs (resolve(&[&str]) walking children by slug; ancestors(id); url(id) built from the ancestor chain as "/campus/a/b/c", root -> "/"), validate.rs (every child id exists, no cycles, slugs match [a-z0-9-]+, exactly one Campus, no duplicate ids). lib.rs is a facade only.
- content/catalog.ron: the full tree from docs/plan.md "Content for the MVP" with titles, taglines, summaries and status. Open: campus, computer-history, ibm-1130, 029, 1130, 1442, radio. Everything else Placeholder, the three future sites ComingSoon. Taglines for the placeholder buildings come from the captions painted on images/sw-campus.png. `scene` set only on campus, computer-history, ibm-1130. Links for 1130 and 1442 point at https://softwarewrighter.github.io/demo-ibm-1130-system/ (Run) and https://github.com/softwarewrighter/demo-ibm-1130-system (Source). 029 and radio: no links yet unless you can find their repos under the softwarewrighter / sw-embed / sw-vibe-coding orgs with `gh search repos`.
- Catalog::embedded() parses include_str!("../../../content/catalog.ron").
- Tests: parse the embedded catalog and validate it; resolve(["computer-history","ibm-1130","1442"]) succeeds; resolve(["ibm-1130"]) at the root fails; url() of 1442 is "/campus/computer-history/ibm-1130/1442"; ancestors(radio) is campus > computer-history > ibm-1130 > 1442.

`just check` green (sw-checklist 0/0). Commit, then `agentrail complete`.
