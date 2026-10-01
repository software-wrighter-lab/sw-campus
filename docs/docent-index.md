# Docent index refresh

The catalog is the physical map; the Docent index is derived from it. Keep
these responsibilities separate:

1. Refresh the campus catalog from the current sw-atlas export when
   `dist/catalog.json` is available. Until CATALOG-EXPORT ships in sw-atlas,
   inspect its repository cache and record only projects that have a real
   campus destination.
2. Put repository names and URLs in a place's links. The matcher indexes link
   labels and URL path components automatically, so a new repository name does
   not require a duplicate alias entry.
3. Use a local LLM as a vocabulary assistant, not as the runtime matcher. Give
   it the place title, tagline, summary, atlas descriptions, and current
   aliases. Ask it to propose likely visitor phrases, abbreviations, spelling
   variants, and rejected false friends. Review the proposal, then add only
   accepted human vocabulary to `aliases` or `example_queries`.
4. Run the link-index regression and the full `just check` gate. The regression
   requires every declared link label to resolve back to its owning place;
   this catches stale or invisible repository links without hand-writing a
   test for every repository.

The local LLM proposal should be treated like an editorial patch: it may
suggest synonyms, but it must not invent a project, repository, or physical
destination. The deterministic matcher remains the final authority and never
calls a model at query time.
