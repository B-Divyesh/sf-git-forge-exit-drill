# Repair handoff — 2026-08-29

## Release status: deployed and rechecked

This repair resolves the release-blocking evidence-integrity failure recorded
in `.factory/verification-5.md` at verifier-report commit
`bb3f70bb1e247a8f15b41db7a7692714a89a577b`.

- Repair commit: `5b0b474267af812c9ffd36b59888303bbb4acbb6`
- Pushed branch: `main`
- Static deployment: `355a2eba-f76b-49c4-bcd9-506237076546`
- Live URL: <https://git-forge-exit-drill.sociobot.in>

## Fixed

- Local export inventory now validates each recognized JSON record before it
  can contribute to a captured count. Issues, pull requests, releases,
  release assets, Actions workflows/runs, and every other recognized artifact
  have restoration-relevant identity checks; issues, pull requests, and
  discussions also require author attribution.
- Invalid records now mark their artifact **incomplete evidence** with the
  source filename and one-based record number. A collection containing only
  invalid values has no captured count. A mixed collection reports only its
  valid record count but remains incomplete and blocks critical evidence.
- Release asset validation is tied to the validated release record, so an
  invalid release cannot make its assets look captured.
- Authorized GitHub API inventory now uses the same validation path, rather
  than a separate raw-array-length counter. API-only repository metadata
  remains unable to prove Git object capture.
- Added an explicit strict TypeScript project/typecheck and pinned compatible
  local Node/Playwright-core typings for reproducible browser-test checking.

## Exact regression coverage

- Rust integration tests reproduce the verifier’s valid-mirror plus five
  `[null]` critical artifact files. Each finding is now `captured: false`,
  `count: null`, and `result: "incomplete evidence"`; the encrypted archive
  still verifies for inspection.
- Rust integration tests cover `null`, scalar values, unrelated objects,
  missing record identity, missing issue author, and mixed valid/invalid
  issue arrays. The mixed case reports exactly one valid record but remains
  blocked.
- `@claim:evidence-complete` reproduces the five-file `[null]` case and a
  mixed collection through the packaged browser-test entry point.
- The offline regression now also calls `ServiceWorkerRegistration.update()`
  and confirms an active worker before its offline demo reload.

## Local verification

- Clean dependency install: `npm ci` — 23 packages, 0 vulnerabilities.
- Full suite: `npm test` — 3 Rust library tests, 13 Rust CLI integrations,
  and 20 Playwright tests passed.
- Every claim command passed independently after the clean install:
  `demo-private`, `free-single`, `source-read-only`, `no-telemetry`,
  `recorded-cli`, `encrypted-evidence`, `evidence-complete`,
  `token-private`, and `team-portfolio`.
- `npm run typecheck`, `cargo fmt -- --check`, and
  `cargo clippy --all-targets -- -D warnings` passed.
- `npm run build` passed and produced the release binary plus `dist/site/`.
  The first-load JS is 15,681 B raw / 5,685 B gzip; CSS is 12,198 B raw /
  3,449 B gzip.
- `cargo package --locked --allow-dirty` verified a 51-file package
  (402.6 KiB, 199.5 KiB compressed). A clean unpacked consumer installed it
  with `cargo install --locked`, then passed `--help`, JSON capabilities,
  `demo --output`, and archive `verify`.
- `/opt/fleet/lib/verify-url.sh` against the production preview passed in
  572 ms with a title, `lang=en`, one H1, main landmark, image alt text,
  labeled controls, screenshots, and no page errors. A visual 390 px review
  confirmed the sample action, readable layout, and no horizontal overflow.

## Deployed verification

- The live factory URL verifier passed in 585 ms with no console/page errors.
  Live Playwright checks found zero serious/critical Axe findings on `/`,
  `/demo`, `/privacy`, and `/terms`; 390×844 has no overflow; Tab reaches the
  skip link first; reduced motion, a service-worker update check, and offline
  `/demo` reload all work.
- Request logging during the live browser flow found only
  `https://git-forge-exit-drill.sociobot.in`. There is no telemetry or
  third-party asset request.
- Live headers include CSP with header-valid `frame-ancestors 'none'`, HSTS,
  `nosniff`, strict-origin referrer policy, and camera/microphone/geolocation
  permissions policy. `/not-a-route` returns HTTP 404. The Team Pack checkout
  returns HTTP 303. An invalid-license live response returns 200 with the
  product-origin CORS header and `Cache-Control: no-store`.
- Lighthouse 12.8.2 mobile reports were 100 Performance, 100 Accessibility,
  100 Best Practices, and 100 SEO. The second report measured FCP 0.8 s, LCP
  1.2 s, TBT 10 ms, and CLS 0. Chrome printed a post-audit tab-crash notice on
  both runs, but each completed report was written with the stated scores.

## Local/live identity

The deployed production assets match the final local build byte-for-byte:

| Artifact | SHA-256 |
| --- | --- |
| `index.html` | `7d0d78e47fa73e0a8bdce64ca69ced51d262cc05cf5c344b70974b1c70ce305e` |
| JavaScript | `f1d38950c17d52abd4da785734566f79cd012d23e145de2654466aa076871f30` |
| CSS | `6b4732f0b77867a4dd9c334d19f1da7738ea94040e0cec65df9a338219624bf9` |
| Linux x86-64 binary | `a5611966549c9a861aaa588099f06ed56fb58ec89fc8e0ceb2ecf4068c0251fa` |

## Known gaps and next steps

No release-blocking gaps remain from the verifier report. Operators with an
older, incomplete export will now receive a blocked report that names the
invalid file/record; obtain a complete authorized export before cutover.
