# Independent verification 12 — PASS

**Candidate tested:** `c7f35bf50d651c31131db7c0c47880a5dff3c1ef`  
**Live URL:** <https://git-forge-exit-drill.sociobot.in>  
**Verified:** 2026-08-29 from the supplied clean checkout.

## Release decision

**PASS.** `HEAD` equals the requested candidate. A fresh production build matches the live JavaScript, CSS, hero artwork, and downloadable Linux binary byte-for-byte. The CLI performs the researched job: it inventories an authorized export, preserves encrypted evidence, reports target capability gaps, and produces a restore drill.

## Mandatory claims gate

Before any general product checks, I confirmed `.factory/claims.json` exists, then ran every declared `test` command separately and sequentially. Each command includes its declared clean `npm ci --ignore-scripts --no-audit --no-fund` installation and its tagged demo-entry-point test. All returned zero. Every one of the 21 IDs has exactly one matching `@claim:<id>` test.

| Claim IDs passed |
| --- |
| `demo-private`, `free-single`, `source-read-only`, `no-telemetry`, `recorded-cli`, `encrypted-evidence`, `evidence-complete` |
| `token-private`, `team-portfolio`, `cli-demo-isolated`, `target-mappings`, `restore-checklist`, `output-boundary` |
| `linux-download`, `billing-contract`, `archive-file-completeness`, `api-metadata-blocks-git`, `json-summary` |
| `actionable-errors`, `cli-network-boundaries`, `license-browser-storage` |

The landing-page and README claim audit found their testable statements covered by this registry; no unlisted landing/README claim was found.

## Cold first read and demo

A fresh 1440 × 900 live load says **“Test your GitHub move before cutover.”** It says this is for **small teams changing Git hosts** and explains that it finds missing repository history and build evidence before cutover. The first primary action is **“Try it with sample data”**, immediately qualified by “See a complete drill with no setup.” This passes the plain-words what/who/first-action gate.

One click opens `/demo`, showing the realistic Atlas Notes sample, a Forgejo 9.0 target, a blocked result, concrete risks, and restore actions. The persistent banner says “Demo — sample data, nothing is saved,” with Reset demo and Start for real. Its only demo storage key is `demo:gfed:started`; demo requests are same-origin only.

## Local quality and consumer checks

| Check | Fresh result |
| --- | --- |
| `npm test` | PASS — 5 Rust unit, 13 Rust CLI integration, 39 Playwright tests |
| `npm run build` | PASS — produced `dist/site/` and Linux x86-64 binary |
| `npm run typecheck`; `npm run audit:copy` | PASS |
| `cargo fmt --check`; `cargo clippy --all-targets -- -D warnings` | PASS |
| `npm audit --omit=dev --audit-level=high` | PASS — 0 vulnerabilities |
| `cargo package --allow-dirty --no-verify` | PASS — 93 files, 5.6 MiB |

I extracted that generated crate into a clean temporary consumer, installed it into an isolated Cargo prefix, and used the installed binary. `--version` returned `git-forge-exit-drill 0.1.0`; `--help` documents its five public commands. `demo --output <new-dir>` created `readiness.md`, `readiness.json`, and `evidence.gfed`; `verify` authenticated the archive and reported 29 evidence files. A missing-source drill exited 1, said “check --source and try again,” and did not create its output directory.

## Live deployment, privacy, accessibility, and performance

- `/`, `/demo`, `/privacy`, and `/terms` each returned 200 with their route title, `lang="en"`, exactly one H1, one main landmark, no JavaScript/page errors, and zero Axe violations (including zero serious/critical) in fresh Playwright scans. The designed unknown route returned HTTP 404; its expected document 404 network message was the only console entry on that route.
- Desktop inspection and a 390 × 844 mobile pass showed no clipping or horizontal overflow (`scrollWidth = clientWidth = 390`). The visible mobile controls were all at least 44 × 44 px.
- Keyboard-only use reached the skip link first, Enter moved focus to the H1, and focusing/activating the sample action by Enter opened `/demo` and moved focus to its H1. Reset demo returned focus to Reset demo. The implemented focus outline is 3 px amber. Reduced-motion computed durations were `0.00001s` while all content remained visible.
- A fresh demo request log contained only the product origin: no analytics, CDN scripts, or remote fonts. An invalid pasted license made exactly one request to the documented `https://api.sociobot.in` verification endpoint, stored only the documented browser keys, recovered with “License no longer active,” and the privacy control removed both keys without errors.
- Live headers include HSTS, `nosniff`, strict-origin referrer policy, restrictive Permissions-Policy, and response-header CSP with `frame-ancestors 'none'`. HTML and `sw.js` use short must-revalidate caching; hashed assets use one-year immutable caching. The service worker was active after `registration.update()` with no waiting worker; `/demo` reloaded offline with its title, H1, and demo banner intact.
- Production assets are 19,823 B JavaScript (6,615 B gzip), 14,402 B CSS (3,855 B gzip), 0 B font files, and a 61,388 B hero WebP: all below the applicable static-product bundle budgets.

## Deployment identity and request allowance

| Artifact | SHA-256 |
| --- | --- |
| App JS | `b0e04757593b24f5f3e6c0d9a49b9c83f41391539faeaa1f630c577b7cf17d7f` |
| App CSS | `e8201e25e6aecd03ff7df059d419b3ae9fba595fcc8f1c2362c7e3c6b2d5b2f1` |
| Hero WebP | `69a1452e5c9c0df2023198be491e977cacc3af9913110e8c608d10b9d4cb5443` |
| Linux binary | `2c56bd5d17e78eedbffc4befb6cc92e74b5fdfc40737c6231ff448e9880bb766` |

The live download has the identical binary hash and returned `git-forge-exit-drill 0.1.0`. The only product server endpoint is Sociobot license verification. From one client, requests 1–30 with invalid test licenses returned 200; request 31 returned **429** with **`Retry-After: 3`** (subsequent retries remained 429). Observed allowance: **30 requests per active window**. The product has no product-owned backend, sign-in, Entra flow, or runtime AI feature, so those checks do not apply.

## Defects by severity

None. No release-blocking, major, moderate, or minor product defect was found.
