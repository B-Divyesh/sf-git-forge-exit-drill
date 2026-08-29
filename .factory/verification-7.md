# Independent verification 7 — PASS

**Candidate:** `beb49611058280ebf675c5c471d00fc76c031183`  
**Live URL:** <https://git-forge-exit-drill.sociobot.in>  
**Verified:** 2026-08-29 from a clean checkout.

## Release decision

**PASS.** The deployed static site and Linux CLI binary match the production
build of this candidate byte for byte. The previous API-pagination,
claims-bootstrap, and short-desktop defects are repaired in fresh,
independent testing. No release-blocking defects were found.

## Required claims gate

`.factory/claims.json` exists, parses, and contains nine claims. Before any
general test or build step, I ran every recorded command from the clean
checkout. Each command begins with its own `npm ci --ignore-scripts --no-audit
--no-fund` bootstrap and passed.

| Claim | Result |
| --- | --- |
| `demo-private` | PASS |
| `free-single` | PASS |
| `source-read-only` | PASS |
| `no-telemetry` | PASS |
| `recorded-cli` | PASS |
| `encrypted-evidence` | PASS |
| `evidence-complete` | PASS |
| `token-private` | PASS |
| `team-portfolio` | PASS |

The `evidence-complete` claim now drives a 10,001-record Actions fixture
through page 101 and an exact-100-record array fixture through its `Link:
rel=next` page. It asserts the exact completed counts and no unwanted extra
page. This directly covers the prior release-blocking truncation.

## First read and demo

A cold live visit answers all required questions in plain words:

- **What:** “Test your GitHub exit before cutover.”
- **For whom:** “For small teams moving forges…”
- **First action:** **Try it with sample data**, with “See a complete drill
  with no setup.” beside it.

The action is fully visible at 1280×720 (y=564–608), 1366×768
(y=589–633), and 390×844 (y=538–582). It opens `/demo` in one click. The demo
immediately displays the real Atlas Notes drill and the persistent “Demo —
sample data, nothing is saved” banner. Space activates **Reset demo**; Enter
activates **Start for real**; the sole demo key is `demo:gfed:started`, and it
is removed on exit.

## Local product verification

- `npm ci --ignore-scripts --no-audit --no-fund`: PASS (23 packages).
- `npm test`: PASS — 5 Rust unit tests, 13 CLI integration tests, and 21
  Playwright tests.
- `npm run typecheck`, `cargo fmt --all -- --check`, and
  `cargo clippy --all-targets --all-features -- -D warnings`: PASS.
- Exact `npm run build`: PASS; it builds `dist/site/` and the release binary.
- `cargo package --locked --allow-dirty`: PASS (52 files), including Cargo's
  verification build.
- A clean consumer installation of the packaged crate passed `--version` and
  `--help`; `--json demo` produced `result/evidence.gfed`, `readiness.md`, and
  `readiness.json`; archive verification succeeded with 29 evidence files.
- Recovery paths were independently exercised: a nonexistent `--source` and
  a passphrase under 12 characters exit 1 with an actionable next step. A
  non-empty demo output directory is refused without removal.

The release build is small: JavaScript is 15,681 bytes raw / 5,685 bytes gzip,
CSS is 12,518 bytes raw / 3,510 bytes gzip, and the hero WebP is 61,388 bytes.
No remote fonts or scripts load.

## Live deployment, accessibility, and PWA

All production artifacts match local `npm run build` output:

| Artifact | SHA-256 |
| --- | --- |
| `index.html` | `4faa175ef48670978b3a430fe455ed9c11dd21eaa785157fb594b342266fa878` |
| JavaScript | `f1d38950c17d52abd4da785734566f79cd012d23e145de2654466aa076871f30` |
| CSS | `2eed8ffaa2b96c661ed634cf67e0e1029e91d067d36b21ddc2d33430500a7c56` |
| Linux x86-64 binary | `be80ca2fbdb3e66d4cd65ca456ff5a04055c08f61ab128c2390e96f57b9628af` |

- `/`, `/demo`, `/privacy`, and `/terms` return 200; `/not-a-route` returns
  the designed HTTP 404. Every checked route has one H1 and one main landmark;
  the real routes have no page or console errors and no 390px overflow.
- The factory `verify-url.sh` passed in 685 ms: title, `lang=en`, H1, main,
  image alt text, button labels, and console checks all passed.
- Live Playwright Axe found zero serious or critical violations on `/`,
  `/demo`, `/privacy`, `/terms`, and the 404. At 390px the checked controls
  pass the shipped 44px-target test. Keyboard starts at the visible 3px amber
  skip-link outline; Space/Enter work on demo controls. Reduced motion sets
  transition and animation duration to `0.00001s`.
- The service worker controlled `/demo`, accepted `registration.update()`,
  and reloaded that route offline with HTTP 200 and the expected H1.

## Privacy, headers, caching, and billing

- A fresh home or demo request log contained only
  `https://git-forge-exit-drill.sociobot.in`. An invalid returned license added
  only the documented `https://api.sociobot.in` verification request; it
  received product-origin CORS and `Cache-Control: no-store`. No telemetry,
  CDN, font, raw Azure, or other third-party request was observed.
- Live response headers include a self-only CSP (with only the documented
  billing API in `connect-src`), HSTS, `nosniff`, strict-origin referrer policy,
  `frame-ancestors 'none'`, and restrictive camera/microphone/geolocation
  permissions. HTML revalidates after 30 seconds; hashed JS/CSS cache for one
  year with `immutable`.
- The optional Team Pack checkout endpoint returns HTTP 303 to hosted Dodo
  checkout. A single client received 200 on verification requests 1–30 and
  HTTP 429 with `Retry-After: 3` on request 31 and later. Observed allowance:
  **30 verification requests per active rate window**.
- No sign-in is used, so an Entra tenant check is not applicable. There is no
  product application backend beyond the documented billing endpoint. AI is
  not appropriate for this deterministic evidence-validation job.

## Defects by severity

None. The browser reports the expected network-status console line when
loading the intentional HTTP 404, but the live application routes and the
factory verifier have no console or page errors; this is not a product defect.

