# Polish 2 handoff — Git Forge Exit Drill

## Outcome

Polish 2 repairs every finding in `review-1.md` and `review-2.md`. The full
closure map is `.factory/polish-2.md`. This handoff is completed after the
fresh-clone claim matrix and live deployment recheck.

## Changes

- Replaced the non-executable copy-audit hash check with a browser-rendered
  route and README audit that enforces every sentence’s word count and banned
  words.
- Split the overlong Team Pack README sentence.
- Removed untested automatic-migration, background-service, one-day cache,
  refund, receipt, and revocation promises.
- Added the exact Forgejo 9.0 Actions-history claim and report assertion.
- Updated the catalog description to the verb-first sentence: “Check a GitHub
  move before cutover with local evidence.”

## Verification

- `npm run audit:copy`: passed.
- `npm test`: passed — 5 Rust unit tests, 13 CLI integration tests, and 40
  Playwright tests; `test-results/.last-run.json` records `passed`.
- `npm run build`: passed; production JS is 6.51 kB gzip and CSS is 3.83 kB
  gzip.
- Focused claims passed: `@claim:forgejo-actions-history`,
  `@claim:billing-contract`, and `@claim:license-browser-storage`.

## Remaining release evidence

The final commit, clean-clone per-claim logs, local screenshots, push, deploy,
and cold live recheck are appended below before handoff.
