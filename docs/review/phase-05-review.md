Updated both Phase 5 documents and added the requested regression coverage.

  Changes:

  - Added an exact {value:escape:escape} one-call regression in crates/htmlc/tests/htmlc.rs:242.
  - Made the three filename fixtures dynamic and added multi-template integration coverage in tests/templates.rs:33.
  - Documented repeated :escape semantics in docs/phases/05-html-rendering.md:221.
  - Updated all stale findings in docs/review/phase-05-review.md:13.

  Finding 1 is not fully fixed. build.rs:47 now produces:

   Template    Render suffix    View suffix
  ━━━━━━━━━━  ━━━━━━━━━━━━━━━  ━━━━━━━━━━━━━
   page-2      page2            Page2View
  ──────────  ───────────────  ─────────────
   page_2      page_2           Page2View
  ──────────  ───────────────  ─────────────
   page2       page2            Page2View

  Therefore:

  - page-2 and page_2 still collide on the view struct.
  - page-2 and page2 collide on both the function and view struct.

  I left your production normalization unchanged and recorded the remaining issue in the review document.

  Validation passed:

  - htmlc: 18/18 tests
  - Root suite: 39/39 tests
  - cargo check
  - Both Clippy checks with warnings denied
