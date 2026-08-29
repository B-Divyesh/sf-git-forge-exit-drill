# Verification handoff — Git Forge Exit Drill

## Outcome

**FAIL.** Independent verification 10 tested the requested live URL on
2026-08-29. The requested candidate
`c06e8b7a471c3cf1b22c852694fbc60e9c813aca` is absent locally and from every
advertised ref on the stated remote. The available checkout and remote tip are
`c06e8b31bb3046de8f79623972fd752c6e0a09e8`; live artifacts match a fresh build
of that different commit.

The full evidence and reproductions are in
[`.factory/verification-10.md`](verification-10.md).

## Release blockers

1. **Candidate identity:** GitHub rejects an exact fetch of `c06e8b7...` as
   `not our ref`; it cannot be tested or matched to live.
2. **Read-only source promise:** setting `--output` equal to `--source` succeeds
   and adds `evidence.gfed`, `readiness.json`, and `readiness.md` to the selected
   export. The source changed from 7 to 10 files.
3. **JSON scripting promise:** `--json drill --source <path>` without
   `--target` exits 2 with zero stdout bytes and human text on stderr, not a
   parseable JSON error.

One moderate defect remains: denied clipboard permission makes **Copy
commands** fail without visible recovery and raises an uncaught page error.

## Verification summary

- All 21 declared claim commands passed separately, but two independent
  boundary cases above disprove the broad `source-read-only` and `json-summary`
  promises.
- `npm test` passed: 5 Rust unit, 13 Rust integration, and 38 Playwright tests.
- Typecheck, copy audit, Rust format, Clippy with denied warnings, dependency
  audit, exact production build, `cargo package`, and clean package install all
  passed.
- The clean installed CLI completed and verified a representative offline
  drill. Normal error and non-empty-output recovery cases behaved correctly.
- Cold first read and one-click sample demo passed.
- Live desktop and 390 px scans passed semantics, keyboard, visible focus,
  200% reflow, touch targets, reduced motion, and Playwright/Axe with zero
  serious/critical findings.
- Privacy request logs contained only the product origin, plus the documented
  Sociobot API during an explicit license check. Security headers and cache
  policies are present.
- Service-worker update and offline `/demo` reload passed.
- Lighthouse mobile: performance 92, accessibility 100, best practices 100,
  SEO 100; LCP 1.30 s and CLS 0.
- Live verification rate limit: requests 1–30 returned 200; request 31 returned
  429 with `Retry-After: 3`.

## Re-run

```sh
npm ci --ignore-scripts --no-audit --no-fund
npm test
npm run audit:copy
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
npm run build
cargo package --locked --allow-dirty
```

Then run every command in `.factory/claims.json`, install the generated crate
into a fresh prefix, and repeat live browser, header, identity, rate-limit,
offline, Axe, and Lighthouse checks.

## Changes in this verification

Only `.factory/verification-10.md` and this handoff were written. Product code
was not modified.
