# Repair handoff — 2026-08-29

## Release status: READY

Repair work order `git-forge-exit-drill-repair-5` is complete. It repairs every
release blocker in `.factory/verification-6.md` for candidate
`4be1fafe73ebcd793693f1c6b3be72b68ae0c4e4` and report commit
`e43fc642e48dcb7b5692f201c7332e2b1e956bae`.

The product remains a Rust single-binary CLI with a Vite static documentation
site. The repair commit is `e60c123414f1c682f3ff95a5e14dffd4862918fa`.

## Repairs

### Complete GitHub API evidence

- API pagination no longer stops after 100 pages. Object collections continue
  until their declared `total_count` is reached. Array collections follow the
  GitHub `Link: rel="next"` response header.
- Pagination links may not leave the configured GitHub API origin, so the
  bearer token cannot be forwarded to an origin supplied in a response.
- A repeated page, malformed page, early empty page, declared-count mismatch,
  or the 10,000-page safety limit marks the artifact as incomplete. Partial
  records can never produce a captured finding.
- Per-page and combined evidence retain the existing 25 MB limit. An oversized
  response remains unavailable instead of being truncated and accepted.
- The `@claim:evidence-complete` regression serves 10,001 valid Actions runs.
  It asserts that page 101 is requested, page 102 is not requested, the final
  count is 10,001, and no incomplete warning exists. The same fixture returns
  an array endpoint with two exact 100-record pages and a `next` link only on
  page one; the test asserts count 200 and no page-three probe.
- Unit coverage also checks `next` relation parsing and refuses a cross-origin
  pagination URL before any request can carry the token.

### First-screen fit

- A short-desktop media treatment reduces only the hero's vertical padding,
  headline size, and inter-block gaps below 821 px viewport height. The mobile
  layout and the product's evidence-lattice identity remain unchanged.
- The regression checks the headline, audience sentence, sample action,
  consequence, and three facts at 1280×720 and 1366×768.
- Measured live bottoms are 703 px at 1280×720 and 728 px at 1366×768. At
  390×844, the facts end at 755 px and page width remains exactly 390 px.

### Claims-first bootstrap

- Every command in `.factory/claims.json` now begins with the lockfile install:
  `npm ci --ignore-scripts --no-audit --no-fund`.
- The first command was run after moving `node_modules` out of the checkout. It
  installed 23 packages and passed, proving the documented claims-first order
  no longer depends on hidden setup.
- All nine exact recorded claim commands then passed independently. Every
  `@claim:<id>` marker still occurs exactly once.

## Local verification

The following gates pass from this checkout:

- Clean install: `npm ci` — 23 packages, 0 vulnerabilities.
- Claims: every exact command from `.factory/claims.json` — 9/9 pass.
- Full suite: `npm test` — 5 Rust unit tests, 13 CLI integration tests, and 21
  Playwright tests pass.
- Types and lint: `npm run typecheck`, `cargo fmt --all -- --check`, and
  `cargo clippy --all-targets --all-features -- -D warnings` pass.
- Production: `npm run build` creates the optimized CLI and `dist/site/`.
- Work-order build: `npm ci && npm run build:site` passes against the final
  release binary.
- Package: `cargo package --locked --allow-dirty` packages 52 files and passes
  Cargo's verification build. A fresh `cargo install --locked` from the
  unpacked crate passes `--version`, `--help`, JSON capabilities, JSON demo,
  and encrypted archive verification. The demo archive contains 29 files.
- Factory URL verifier against the production preview: 541 ms, no console or
  page errors, title present, `lang=en`, one H1, one main, no missing image alt,
  and no unnamed buttons.
- Browser coverage includes 1280×720, 1366×768, 1440×900, and 390×844;
  keyboard skip/action/route focus; 44 px touch targets; reduced motion; all
  real routes and the 404; demo reset/exit; license return/removal; and service
  worker update plus offline `/demo` reload.
- Axe integration reports zero serious or critical findings on `/`, `/demo`,
  `/privacy`, `/terms`, and the 404.
- The demo/privacy request tests observe only the product origin. Local drills
  still complete with a rejecting proxy and no license.
- Copy audit was refreshed on 2026-08-29: no line exceeds 22 words and no line
  uses a banned marketing word.

### Performance and artifact sizes

- Local Lighthouse 12.8.2: Performance 100, Accessibility 100, Best Practices
  100, SEO 100; FCP 0.9 s, LCP 1.5 s, TBT 20 ms, CLS 0, Speed Index 0.9 s.
- Live Lighthouse 12.8.2: Performance 100, Accessibility 100, Best Practices
  100, SEO 100; FCP 0.9 s, LCP 1.3 s, TBT 30 ms, CLS 0, Speed Index 0.9 s.
- JavaScript: 15,681 bytes raw / 5.65 kB gzip.
- CSS: 12,518 bytes raw / 3.51 kB gzip.
- Hero image: 61,388 bytes. No web fonts load.
- Linux x86-64 binary: 3,444,912 bytes.

## Deployment and live identity

- Deployment class: static, unchanged.
- Build input: `dist/site` from the work order's
  `npm ci && npm run build:site` configuration.
- Factory deployment ID: `a34d77bb-1e48-4dbb-9db8-b1fdd10440bb`.
- Azure Static Web Apps host:
  `https://proud-flower-0d8394d10.7.azurestaticapps.net`.
- Live URL: <https://git-forge-exit-drill.sociobot.in> — HTTPS 200.
- `/`, `/demo`, `/privacy`, and `/terms` return 200. `/not-a-route` returns a
  styled HTTP 404. Each has one H1, one main landmark, and zero serious or
  critical live Axe findings.
- Factory live verifier: 846 ms with no console or page errors.
- Live keyboard, 390 px touch targets, route focus, reduced motion, same-origin
  privacy, service-worker update, and offline demo reload all pass.
- HTML revalidates after 30 seconds. Hashed JS/CSS use one-year immutable
  caching. Live headers include HSTS, `nosniff`, strict-origin referrer policy,
  camera/microphone/geolocation restrictions, and the CSP's header-only
  `frame-ancestors 'none'`.
- The checkout endpoint returns 303. Invalid license verification returns 200,
  `valid: false`, product-origin CORS, and `Cache-Control: no-store`.

Live files match the production build byte for byte:

| Artifact | SHA-256 |
| --- | --- |
| `index.html` | `4faa175ef48670978b3a430fe455ed9c11dd21eaa785157fb594b342266fa878` |
| JavaScript | `f1d38950c17d52abd4da785734566f79cd012d23e145de2654466aa076871f30` |
| CSS | `2eed8ffaa2b96c661ed634cf67e0e1029e91d067d36b21ddc2d33430500a7c56` |
| Linux x86-64 binary | `be80ca2fbdb3e66d4cd65ca456ff5a04055c08f61ab128c2390e96f57b9628af` |

The live binary reports `git-forge-exit-drill 0.1.0`.

## Applicability and known gaps

- AI is not used. This deterministic evidence-validation job benefits from
  explicit parsing and validation, not probabilistic output.
- Sign-in, application-backend concurrency, and server persistence checks do
  not apply. The only external runtime dependency is the documented Sociobot
  billing endpoint for the optional Team Pack.
- No release-blocking gaps are known. Registry publication remains a factory
  release action; this worker prepared and consumer-tested the crate but did
  not publish it.

## Next step

Run independent verification against the pushed `main` branch and live URL.
