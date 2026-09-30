Publish to GitHub Pages at https://software-wrighter-lab.github.io/sw-campus/ and prove deep links work.

Read docs/plan.md ("Hosting and URLs", "Build, run, check"); model the workflow on ~/github/sw-ml-study/demo-abstract-algebra/.github/workflows/pages.yml.

Deliver:
- .github/workflows/pages.yml: on push to main + workflow_dispatch; permissions pages: write, id-token: write; concurrency group pages; ubuntu-latest; dtolnay/rust-toolchain@stable with targets: wasm32-unknown-unknown; install trunk from its GitHub release tarball (pin the version, e.g. 0.21.x); install just (extractions/setup-just or apt); run `just pages`; actions/upload-pages-artifact@v3 with path dist; actions/deploy-pages@v4.
- `just pages` produces dist/ with index.html copied to 404.html (already a recipe from step 1; make sure it also copies CNAME later, not now).
- Enable Pages for the repo with `gh api -X POST repos/software-wrighter-lab/sw-campus/pages -f build_type=workflow` if not already enabled (check with `gh api repos/software-wrighter-lab/sw-campus/pages` first).
- README.md: replace the "will be published" sentence with the live link and a one-line "Open the campus" call to action at the top, like ~/github/sw-ml-study/demo-abstract-algebra/README.md.

Verify after the workflow succeeds: open https://software-wrighter-lab.github.io/sw-campus/campus/computer-history/ibm-1130/1442 directly in a fresh tab and see the 1442 exhibit page with working assets and breadcrumb. Record the run URL in the completion summary. Commit, then `agentrail complete`.
