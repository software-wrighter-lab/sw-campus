Add a Rust-only hotspot tracing overlay and use it to refine the campus and lobby hotspots from rectangles into polygons.

Read docs/plan.md ("First-pass hotspot rectangles", saga step 10).

Deliver:
- When the URL has ?edit (read via web_sys location search), SceneView shows an editor overlay: each click on the image appends a vertex (in viewBox coordinates, computed from the svg's getScreenCTM / bounding rect so it is correct at any scale); the in-progress polygon draws live; Enter closes it and logs a ready-to-paste RON Hotspot (with a place: "TODO" field) via web_sys::console::log_1; Backspace removes the last vertex; Esc clears. Keep this in its own module(s); if campus-ui-scene would exceed 4 modules, make campus-ui-editor.
- No effect at all without ?edit (no listeners registered).
- Use it: retrace every campus building as a polygon following its actual roofline/footprint, and the lobby's directory rows as tight rects; update content/scenes/*.ron. Validation tests still pass.
- Document ?edit in docs/plan.md under "Interaction" and in README "Development".

`just check` green. Commit, then `agentrail complete`.
