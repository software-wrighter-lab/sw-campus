Build the exhibit page and write the content for the open exhibits.

Read docs/plan.md ("Rendering decision", "Content for the MVP", "Known external targets").

Deliver:
- campus-ui exhibit_page.rs: title, tagline, summary paragraph(s), a prominent "Run exhibit" button when a Run link exists, then Source / Docs / Related links, then children (the 1442 page lists the radio demo). Rendering decision: leaf place with no scene -> ExhibitPage; a place with children and no scene -> Directory (unchanged).
- Content in content/catalog.ron for: 029 (IBM 029 Card Punch: what it was, why it is in the 1130 wing, keypunch/APL connection), 1130 (IBM 1130 / 1131 CPU console, links to demo-ibm-1130-system), 1442 (card read punch; links into the simulator), radio (music by RF interference from the 1442; the story, a Run link if the demo repo is found via `gh search repos`, else Placeholder). Keep summaries to a paragraph; historical accuracy over flourish.
- 1132 and 2310 remain Placeholder with one-line summaries.

Verify every exhibit URL renders and the Run links open the simulator. `just check` green. Commit, then `agentrail complete`.
