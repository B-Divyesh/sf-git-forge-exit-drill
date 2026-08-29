# Independent verification 11 — PASS

**Candidate tested:** `8ea309a0aa33f003f86be67ce65df4c88aa2037d`  
**Live URL:** <https://git-forge-exit-drill.sociobot.in>  
**Verified:** 2026-08-29 from the supplied clean work-order checkout.

## Release decision

**PASS.** The checkout `HEAD` is the requested candidate and fresh production
artifacts match its production build byte-for-byte. The mandatory claims gate,
CLI package-consumer exercise, browser/accessibility/privacy/PWA checks, and
available local quality gates all passed. No defects were observed.

## Mandatory claims gate

`.factory/claims.json` exists, parses, and contains 21 unique IDs. Before
general product QA, I ran every registered command separately and sequentially
from this checkout, including the clean `npm ci --ignore-scripts --no-audit
--no-fund` step embedded in each command. Every command returned zero. The
registry has exactly one corresponding `@claim:<id>` tag per ID.

| Claim IDs that passed |
| --- |
| `demo-private`, `free-single`, `source-read-only`, `no-telemetry`, `recorded-cli`, `encrypted-evidence`, `evidence-complete` |
| `token-private`, `team-portfolio`, `cli-demo-isolated`, `target-mappings`, `restore-checklist`, `output-boundary` |
| `linux-download`, `billing-contract`, `archive-file-completeness`, `api-metadata-blocks-git`, `json-summary` |
| `actionable-errors`, `cli-network-boundaries`, `license-browser-storage` |

## Cold first read and demo

At a cold 1440 x 900 live load, the first screen says **“Test your GitHub move
before cutover”**. It says it is for **small teams changing Git hosts** and
that it finds missing repository history and build evidence before cutover.
The first action is **“Try it with sample data”**, with the immediate outcome
“See a complete drill with no setup.” This satisfies what, who, and what to
click first in plain words.

The single click opens `/demo` with the realistic Atlas Notes sample, Forgejo
9.0 target, blocked outcome, concrete risks, and restore actions. The
persistent banner reads “Demo — sample data, nothing is saved,” offers **Reset
demo** and **Start for real**, uses only `demo:gfed:started`, and removes that
key when leaving demo mode.

## Local build, tests, and CLI consumer

| Check | Result |
| --- | --- |
| `npm test` | PASS — 5 Rust unit tests, 13 Rust CLI integration tests, 39 Playwright tests |
| `npm run build` | PASS — created `dist/site/` and the Linux x86-64 binary |
| `npm run typecheck` / `npm run audit:copy` | PASS |
| `cargo fmt --check` | PASS |
| `cargo clippy --all-targets -- -D warnings` | PASS |
| `npm audit --omit=dev --audit-level=high` | PASS — 0 vulnerabilities |
| `cargo package --allow-dirty --no-verify` | PASS — packaged 92 files, 5.6 MiB |

I extracted the generated crate into a fresh temporary consumer, installed it
to a separate Cargo prefix, and exercised its public CLI. `--version` returned
`git-forge-exit-drill 0.1.0`; `--help` documented the five public commands;
`demo --output <new-dir>` produced `result/readiness.md`,
`result/readiness.json`, and `result/evidence.gfed`; and `verify` with the
demo passphrase authenticated the archive and reported 29 evidence files.
The invalid `--source /no/such/export` recovery exited 1 with the actionable
“check --source and try again” message and did not create the output directory.

## Live browser, accessibility, privacy, and PWA

- Live `/`, `/demo`, `/privacy`, and `/terms` returned 200, each had the
  expected route title, `lang="en"`, exactly one H1, one main landmark, and no
  page or console errors. The designed unknown route returned HTTP 404.
- Independent Axe scans of those four successful routes plus the 404 found
  zero serious or critical findings (and no lower-severity Axe findings).
- At desktop and 390 x 844 mobile, the primary action was visible on the first
  screen, mobile width equalled scroll width (390 px), and all *visible*
  interactive controls met the 44 px target. The visual inspection showed no
  clipping or horizontal overflow.
- Keyboard testing reaches the visible skip link first; its Enter path reaches
  `#main`, then the page H1. The sample action is operable by Enter and route
  navigation focuses its destination H1. Focus uses a visible 3 px amber
  outline. Reduced-motion mode reduced animation and transition duration to
  `0.00001s` while leaving the content visible.
- A clean `/demo` request log contained only the product origin (HTML, local
  JS/CSS, and local WebP). Explicitly verifying an invalid pasted license made
  exactly the documented Sociobot verification request and recovered with
  “License no longer active,” without page errors. No analytics, CDN scripts,
  or third-party fonts were requested.
- The response uses HSTS, `nosniff`, strict-origin referrer policy,
  Permissions-Policy, and a restrictive response-header CSP including
  `frame-ancestors 'none'`. Hashed assets have one-year immutable caching;
  HTML and the service worker have short must-revalidate caching.
- The service worker was active after forced `registration.update()` (no
  waiting worker) and `/demo` reloaded offline with its demo title intact.

The production initial assets are 19,823 B JavaScript (6,615 B gzip), 14,402 B
CSS (3,855 B gzip), 0 B fonts, and a 61,388 B hero WebP: all within the stated
static-product budgets.

## Deployment identity and rate limit

Freshly built artifacts match live production exactly:

| Artifact | SHA-256 |
| --- | --- |
| app JS | `b0e04757593b24f5f3e6c0d9a49b9c83f41391539faeaa1f630c577b7cf17d7f` |
| app CSS | `e8201e25e6aecd03ff7df059d419b3ae9fba595fcc8f1c2362c7e3c6b2d5b2f1` |
| hero WebP | `69a1452e5c9c0df2023198be491e977cacc3af9913110e8c608d10b9d4cb5443` |
| Linux binary | `2c56bd5d17e78eedbffc4befb6cc92e74b5fdfc40737c6231ff448e9880bb766` |

The live downloaded binary is executable and returns version `0.1.0`. A
single client made 30 invalid live license-verification requests that returned
200; request 31 returned **429** with **`Retry-After: 4`**. Observed allowance:
**30 requests per active window**.

There is no product-owned backend, sign-in, Entra flow, or AI feature;
backend concurrency/persistence/health and Entra checks do not apply.

## Defects by severity

None observed. There are no known release blockers, major, moderate, or minor
defects from this verification.
