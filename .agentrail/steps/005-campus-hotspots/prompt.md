Make the campus scene interactive: painting + SVG hotspot layer, hover caption, click to enter, keyboard access.

Read docs/plan.md ("What changed from the research", "Interaction").

Deliver:
- campus-ui: scene_view.rs (<svg viewBox="0 0 W H" preserveAspectRatio="xMidYMid meet"> with <image href=assets/{image}> then <HotspotLayer>), hotspot_layer.rs (one <a href=url role="link" tabindex="0" aria-label=title> per hotspot wrapping a <polygon points=...>; hover/focus state lifted to the parent), caption_card.rs (foreignObject or positioned <div> near the hotspot centroid: title, tagline, and up to four child titles joined with " - "; Placeholder/ComingSoon show "Opening soon"). If campus-ui hits 5 modules, split into campus-ui-scene and campus-ui-chrome now, before adding the fifth.
- Clicking navigates with yew_router (prevent default on the anchor, push route) so it is an SPA transition, while the href stays real for middle-click / new tab.
- CSS: polygons transparent by default; hovered/focused polygon gets a soft fill and a 2px stroke; focus ring visible.
- app.rs rendering decision: place has scene -> SceneView + Directory below it; else Directory. Campus now shows the painting with 10 hotspots.
- Assets: crates/campus-web/assets/campus.webp exists from step 1; add computer-history.webp (images/museum-lobby.png -> WebP < 400 KB).

Verify in the browser: hover each building shows its caption; click Computer History Museum lands on /campus/computer-history (Directory for now); Tab cycles hotspots; Enter enters. `just check` green. Commit, then `agentrail complete`.
