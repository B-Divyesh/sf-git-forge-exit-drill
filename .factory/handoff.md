# Independent verification handoff — 2026-08-29

## Release status: FAIL

Candidate `4be1fafe73ebcd793693f1c6b3be72b68ae0c4e4` was independently tested
against <https://git-forge-exit-drill.sociobot.in>. The deployment matches the
candidate build byte-for-byte, but it is not releasable.

The release-blocking product defect is API pagination completeness. A recorded
GitHub-compatible endpoint declared 10,001 Actions runs. The CLI stopped after
100 pages, did not request page 101, then reported 10,000 runs as captured with
no incomplete or unavailable warning. This can hide missing build provenance.

The mandatory first-screen gate also fails at common desktop sizes: **Try it
with sample data** begins below the viewport at both 1366×768 and 1280×720.
The first clean-clone claim command additionally failed with `vite: not found`
before `npm ci`; all nine exact claim commands passed after installation.

Full evidence and remediation are in `.factory/verification-6.md`.

## Verification summary

- `npm ci`, `npm test`, `npm run typecheck`, Rust formatting, strict Clippy,
  and exact `npm run build` pass.
- All nine claim commands pass after install. The full suite contains 3 Rust
  unit, 13 CLI integration, and 20 Playwright tests.
- Cargo package verification and clean consumer installation pass. The
  installed CLI completes its demo, verifies its encrypted archive, and
  rejects invalid inputs and tampered/wrong-passphrase archives cleanly.
- Candidate/live hashes match for HTML, JS, CSS, and the downloadable binary.
- Live real routes have zero serious/critical Axe findings, no console/page
  errors, no 390 px overflow, designed focus, reduced-motion support, and an
  updating/offline-capable service worker.
- Lighthouse mobile scores 100 in Performance, Accessibility, Best Practices,
  and SEO; LCP is 1.3 s, TBT 80 ms, and CLS 0.
- Fresh demo traffic is same-origin only. License verification contacts only
  the documented Sociobot API. The endpoint allows 30 requests per active
  window; request 31 returns 429 with `Retry-After: 3`.

## How to reproduce the blocker

Run the release CLI against a GitHub-compatible fixture whose Actions-runs
endpoint reports `total_count: 10001`, returns 100 valid records for pages
1–100, and one record for page 101. Inspect `readiness.json`: the candidate
requests only 100 pages and marks `actions_runs` captured with count 10,000.
`src/lib.rs:964` supplies the fixed cap and does not mark an unfinished total
incomplete after the loop.

## Next steps

1. Follow pagination to the declared total, or block capped collections as
   incomplete. Follow GitHub `Link` headers for endpoints without totals.
2. Extend the evidence-completeness claim with >100-page and exact-page-boundary
   API fixtures.
3. Fit the required first-read content and sample action into 720p/768p
   desktop viewports.
4. Resolve the claims-first clean-clone command failure, then rerun independent
   verification before release.
