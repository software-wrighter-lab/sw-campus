Wire routing so every place in the catalog renders at its URL, with a breadcrumb and a plain Directory page. No hotspots yet.

Read docs/plan.md ("Hosting and URLs", "Web crate", "Rendering decision").

Deliver:
- crates/campus-web/src/route.rs: Route { Home "/", Campus "/campus", Place "/campus/*path", NotFound }. The switch splits path on '/', drops empty segments, calls Catalog::resolve; None -> NotFound page.
- crates/campus-web/src/base.rs: basename() reads web_sys document().base_uri(), returns the path without trailing slash ("/sw-campus" or "" for root). Passed to <BrowserRouter basename=...>.
- crates/campus-ui (lib, new crate, Yew components): breadcrumb.rs (Campus > Computer History Museum > IBM 1130 Wing, each ancestor a yew_router Link), directory.rs (title, tagline, summary, then the children as a list of Links with status badges "Opening soon" for Placeholder/ComingSoon), not_found.rs. Keep <= 4 modules; footer moves here from campus-web if that helps the count.
- app.rs renders header (breadcrumb) + main (Directory for now, for every place) + footer. Catalog is constructed once and provided via ContextProvider.
- Visit /sw-campus/campus/computer-history/ibm-1130/1442 in `just serve` and see the 1442 directory page; /sw-campus/nope shows NotFound; Back works.

`just check` green. Commit, then `agentrail complete`.
