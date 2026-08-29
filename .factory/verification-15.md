# Independent verification 15 — PASS

**Candidate:** `1ba0d173a95dda2e1021f65ce82a9a145973786e`  
**Live URL:** <https://git-forge-exit-drill.sociobot.in>  
**Verified:** 2026-08-29 from the supplied clean checkout.

## Verdict

**PASS.** The deployed product matches the tested candidate and performs the
brief's real job: it inventories an authorized export or API repository,
validates evidence and Git history, creates authenticated encrypted local
evidence, maps the stated GitLab/Gitea/Forgejo targets, and emits a readiness
and restore drill. No deployment-only failure reproduced.

## Mandatory opening checks

`.factory/claims.json` is present with 23 unique claim IDs and matching
`@claim:` tests. Before broader QA, I ran every declared command from the
clean checkout. All passed: `demo-private`, `free-single`,
`source-read-only`, `no-telemetry`, `recorded-cli`, `encrypted-evidence`,
`evidence-complete`, `token-private`, `team-portfolio`,
`cli-demo-isolated`, `demo-valid-git-mirror`, `target-mappings`,
`forgejo-actions-history`, `restore-checklist`, `output-boundary`,
`linux-download`, `billing-contract`, `archive-file-completeness`,
`api-metadata-blocks-git`, `json-summary`, `actionable-errors`,
`cli-network-boundaries`, and `license-browser-storage`. The final two were
also rerun individually verbatim; each passed.

Cold live first read at 1440x900 answered all required questions in plain
words:

- What: “Test your GitHub move before cutover.”
- Who: “For small teams changing Git hosts…”
- First action: visible “Try it with sample data,” with “See a complete drill
  with no setup.”

One click opened `/demo`, with a completed Atlas Notes drill and persistent
“Demo — sample data, nothing is saved” banner. `Reset demo` and `Start for
real` were present.

## Clean checks and CLI exercise

| Check | Result |
| --- | --- |
| `npm ci --ignore-scripts --no-audit --no-fund` | PASS |
| `npm test` | PASS: 5 Rust unit, 13 Rust integration, 41 Playwright tests (`test-results/.last-run.json`: passed) |
| `npm run typecheck`, `npm run audit:copy` | PASS |
| `npm run build` | PASS; produced `dist/site/` and executable Linux binary |
| `cargo fmt --all -- --check` | PASS |
| `cargo clippy --all-targets --all-features -- -D warnings` | PASS |
| `cargo package --allow-dirty` | PASS |

I unpacked the generated crate into a clean consumer directory, installed it
with `cargo install --locked --path`, and exercised the installed binary. It
reported `0.1.0`, listed all three target maps, completed `demo`, and verified
the generated archive with 29 evidence files. The declared suite also covered
normal drills, malformed records, source/output overlap, short passphrases,
missing sources, token redaction, API metadata blocking Git-history readiness,
ten-repository portfolio limits, JSON errors, and rejected network access.

## Live, privacy, accessibility, and resilience

The locally built JS, CSS, hero image, route documents, service worker,
manifest, robots/sitemap, icons, and Linux binary matched the live bytes for
all 15 exposed artifacts checked. The downloaded live binary executes as
`git-forge-exit-drill 0.1.0`.

Fresh `/demo` request logging observed only
`https://git-forge-exit-drill.sociobot.in`; its sole browser storage key was
`demo:gfed:started`. It reloaded offline after the service worker had first
activated, without console or page errors. The root loaded only same-origin
HTML, JS, CSS, and hero image. No analytics, CDN fonts, or third-party scripts
were observed.

Live `/`, `/demo`, `/privacy`, and `/terms` each had one H1 and main landmark,
no overflow at desktop or 390px mobile, no console/page errors, and zero axe
serious or critical findings. Keyboard Tab begins with the skip link; the
sample drill is operable by Enter; Reset demo retains focus. The full suite
also passed the 200% text, 44px target, reduced-motion, service-worker update,
and 404 checks.

Headers include HSTS, `X-Content-Type-Options: nosniff`, strict referrer
policy, permissions policy, and response-header CSP with
`frame-ancestors 'none'`. HTML/service worker cache for 30 seconds; hashed
JS/CSS cache for one year immutable. Built sizes: JS 19,747 B (6,500 B gzip),
CSS 14,402 B (3,830 B gzip), all within budget. Unknown routes return HTTP
404 with the designed recovery page.

The hosted Team Pack checkout returns HTTP 303 to Sociobot/Dodo. On the
Sociobot license-verify endpoint, requests 1–30 from one client returned 200;
request 31 returned HTTP 429 with `Retry-After: 3`. Observed allowance: **30
requests per active window**.

## Defects and follow-up

No release-blocking, major, or moderate defect found.

Minor packaging hygiene: `cargo package` includes 120 `.factory` verification
and screenshot files (15.58 MiB uncompressed), resulting in a 14.5 MiB crate.
The crate installs and works, but a future release should exclude factory
evidence from the published crate using Cargo include/exclude rules.

This static CLI site has no product-owned backend, sign-in, or runtime AI
feature; backend-concurrency, Entra, and AI checks are not applicable.
