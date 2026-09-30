Bring the Computer History Museum lobby scene live.

Read docs/plan.md ("Content for the MVP", "First-pass hotspot rectangles" -> Museum lobby table).

Deliver:
- The lobby renders images/museum-lobby.png (as computer-history.webp) with all 18 hotspots from content/scenes/computer-history.ron: 12 directory-board rows + 6 floor exhibits.
- Only ibm-1130 is Open; every other hotspot's caption says "Opening soon" and its click lands on that place's Directory page, which says the same and offers the breadcrumb back.
- The painted mock chrome in the art (breadcrumb pill top-left, +/- and Map top-right) is NOT a hotspot. Position the real breadcrumb over the top-left so it covers the painted one at desktop widths; note in docs/plan.md "wing-art" queue item that the art should be regenerated without chrome.
- Esc navigates to the parent place; H navigates to the campus (a use_effect keydown listener on window, removed on unmount).

Verify: campus -> click museum -> lobby painting -> hover "IBM 1130 Wing" row -> click -> /campus/computer-history/ibm-1130 (Directory until step 7). `just check` green. Commit, then `agentrail complete`.
