# Independent verification 4 — PASS

**Verified:** 2026-08-28  
**Candidate commit:** `48686c16abc36e02b31f3a447b3d91692fc1126a`  
**Live URL:** https://git-forge-exit-drill.sociobot.in

## Release decision

**PASS.** This candidate meets the researched brief's core job: a small team can
run a local, encrypted migration-readiness drill for a GitHub export/API
repository, see evidence gaps before cutover, and get a versioned target map and
restore checklist. The prior false-positive evidence defects are covered by the
current product and independent regression checks.

No Critical, High, Medium, or Low defects were found in this verification.

## Mandatory cold read and one-click demo

PASS. A fresh Chromium visit to the live root showed:

- **What:** “Test your GitHub exit before cutover.”
- **For whom:** “For small teams moving forges…”
- **First action:** the visible, one-click **Try it with sample data** link,
  with “See a complete drill with no setup.” beside it.

It opens `/demo` directly. The demo shows the actual bundled CLI's Atlas Notes
output, the persistent “Demo — sample data, nothing is saved” banner, Reset
demo, and Start for real controls.

## Required claims

`npm ci` completed from this clean candidate (22 packages, 0 vulnerabilities).
Every command in `.factory/claims.json` was run independently through the
product's demo entry point and passed.

| Claim | Exact command | Result |
| --- | --- | --- |
| `demo-private` | `npm test -- --grep @claim:demo-private` | PASS |
| `free-single` | `npm test -- --grep @claim:free-single` | PASS |
| `source-read-only` | `npm test -- --grep @claim:source-read-only` | PASS |
| `no-telemetry` | `npm test -- --grep @claim:no-telemetry` | PASS |
| `recorded-cli` | `npm test -- --grep @claim:recorded-cli` | PASS |
| `encrypted-evidence` | `npm test -- --grep @claim:encrypted-evidence` | PASS |
| `evidence-complete` | `npm test -- --grep @claim:evidence-complete` | PASS |
| `token-private` | `npm test -- --grep @claim:token-private` | PASS |
| `team-portfolio` | `npm test -- --grep @claim:team-portfolio` | PASS |

The evidence-completeness claim specifically rejects malformed JSON and
manifest-only totals as incomplete evidence; the sample's displayed counts are
backed by parseable export records.

## Local quality, CLI, and error-path evidence

- `npm test`: PASS — 3 Rust unit tests, 11 CLI integration tests, 20 Playwright
  tests.
- `cargo fmt -- --check`: PASS.
- `cargo clippy --all-targets -- -D warnings`: PASS.
- `npx tsc --noEmit --target ES2022 --moduleResolution bundler --module ESNext --lib ES2022,DOM --types vite/client --skipLibCheck site/src/main.ts`: PASS.
- `npm run build`: PASS — release Rust binary and `dist/site/` produced.
- `cargo package --locked --allow-dirty`: PASS — verified package with 48 files,
  192.0 KiB compressed.
- A clean `cargo install --root <temporary consumer>` installation passed
  `--help`, `--json capabilities`, `demo --output`, and `verify` of the demo's
  resulting `result/evidence.gfed` archive (29 evidence files).
- A normal local GitLab drill returned valid JSON/reports. Invalid target
  `nowhere:1` and a missing `GFED_PASSPHRASE` both failed safely with exit code
  1 and an actionable message. The test suite also covers malformed export
  recovery, non-empty demo-output refusal, no source mutation, encryption,
  invalid archive authentication, and the Team Pack's 10-repository boundary.

## Live deployment, privacy, security, and accessibility

The locally built artifact matches the live candidate byte-for-byte:

| Artifact | SHA-256 |
| --- | --- |
| `index.html` | `7d0d78e47fa73e0a8bdce64ca69ced51d262cc05cf5c344b70974b1c70ce305e` |
| JavaScript | `f1d38950c17d52abd4da785734566f79cd012d23e145de2654466aa076871f30` |
| CSS | `6b4732f0b77867a4dd9c334d19f1da7738ea94040e0cec65df9a338219624bf9` |
| Linux x86-64 download | `1163e1362f9df487817ef839cf4cbd1f656af79a0da43e6af0dc53dca9dbe4c6` |

- Live `/`, `/demo`, `/privacy`, and `/terms` each return 200 with a route
  title, exactly one H1, and one main landmark. A nonexistent route returns a
  real HTTP 404 with the styled recovery page.
- Fresh `/demo` request logging observed only
  `https://git-forge-exit-drill.sociobot.in`; no telemetry or third-party data
  request occurred. The actual invalid-license form recovers with “License no
  longer active.”
- The live CSP restricts scripts/styles/images to self (plus `data:` images) and
  permits only Sociobot billing in `connect-src`; it also sends HSTS,
  `nosniff`, strict-origin referrer policy, permissions policy, and
  `frame-ancestors 'none'`. HTML is short cached; hashed JS/CSS are
  `max-age=31536000, immutable`.
- One client made 30 successful invalid-license verification requests to the
  documented Sociobot endpoint; the next request received **429** with
  **`Retry-After: 1`**. Observed allowance: 30 requests per active rate window.
- All live links were checked: internal routes/downloads returned 200, the
  checkout returned 303 to hosted Dodo checkout, and the Param Factory link
  returned 200.
- Independent Playwright Axe scans found no serious or critical issues on all
  four routes. At desktop and 390 px mobile there was no horizontal overflow;
  visible interactive controls met the 44 px baseline. Keyboard Tab reaches the
  skip link, routing moves focus to the H1, focus styling is visible, and no
  console or page errors were observed.
- Reduced-motion media mode applies the site's 0.01 ms motion fallback. The
  service worker activated, accepted an update check, and `/demo` reloaded
  successfully offline after its first online visit.
- Initial JS is 5.65 KiB gzip and CSS is 3.43 KiB gzip, within the 200 KiB JS
  and 50 KiB CSS budgets.

## Known gaps / next steps

None for release. The normal deploy process may publish this verification
commit; no product-code change is required.
