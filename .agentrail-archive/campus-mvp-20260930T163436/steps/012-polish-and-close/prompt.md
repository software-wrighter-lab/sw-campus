Polish and close the campus-mvp saga.

Read docs/plan.md ("Interaction", "Non-goals"). This is a validation step: no new features beyond the list below.

Deliver:
- Transitions: a short fade (CSS only) when the scene image changes; no layout shift (reserve the 3:2 box).
- Mobile: at 400px width the scene scales, captions stay on-screen (clamp the card inside the viewBox), breadcrumb wraps, footer wraps. No horizontal scroll.
- A11y pass: every hotspot has aria-label and a visible focus ring; the Directory under each scene is the text equivalent of the picture; images have alt text; colour contrast on the caption card >= 4.5:1.
- Keyboard: Tab/Enter/Esc/H verified on campus, lobby, wing.
- sw-checklist: 0 failed, 0 warnings across the workspace; cargo clippy pedantic clean on host and wasm32.
- docs/plan.md: mark the saga shipped, move anything undone into the "Saga queue" section with a sentence on why. README status paragraph updated.
- Run `agentrail audit` and fix any saga/git drift before closing.

`just check` green. Commit, then `agentrail complete --done`.
