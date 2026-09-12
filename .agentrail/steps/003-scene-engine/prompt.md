Create the scene layer: crate crates/campus-scene plus content/scenes/campus.ron and content/scenes/computer-history.ron. No web dependencies; TDD.

Read docs/plan.md ("What changed from the research", "Scene model", "First-pass hotspot rectangles").

Deliver:
- crates/campus-scene (lib): scene.rs (Scene, Hotspot, Shape { Polygon(Vec<(f32,f32)>), Rect{x,y,w,h} }, serde Deserialize), points.rs (Shape::points() -> Vec<(f32,f32)> so a rect becomes 4 points; Shape::centroid() for the default caption anchor), load.rs (Scenes::embedded(): both RON files via include_str!; get(id)), validate.rs (image non-empty, width/height > 0, every point inside the image, every hotspot place exists in the catalog AND is a child of the scene's place; takes &Catalog from campus-model as a parameter).
- content/scenes/campus.ron and content/scenes/computer-history.ron using the rectangle tables in docs/plan.md verbatim (they are a first pass; the hotspot editor in step 10 refines them). image = "campus.webp" / "computer-history.webp"; width 1536, height 1024.
- Tests: both scenes parse; validate against Catalog::embedded() passes; a hotspot pointing at a non-child fails validation with a message naming the place.

`just check` green. Commit, then `agentrail complete`.
