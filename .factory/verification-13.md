# Independent verification 13 — PASS

**Candidate tested:** `9340394892cc474cb7b187486e592a2d68423e43`  
**Live URL:** <https://git-forge-exit-drill.sociobot.in>  
**Verified:** 2026-08-29 from the supplied clean checkout.

## Acceptance verdict

**PASS.** `HEAD` was the requested candidate before this report was added. The
fresh production build matches the deployed application JavaScript, CSS, and
downloadable Linux binary byte-for-byte. The CLI delivers the brief's job: it
checks an authorized export, captures authenticated encrypted evidence, maps
the stated forge target, reports missing/unsupported work, and creates a
restore drill before a cutover.

No product code was changed during this verification.

## Mandatory claims and cold first read

Before other product checks, I confirmed `.factory/claims.json` exists and
performed one clean `npm ci --ignore-scripts --no-audit --no-fund`, then ran
each of its 21 tagged `npm test -- --grep @claim:<id>` commands separately and
sequentially. Every command passed. The passing IDs are:

`demo-private`, `free-single`, `source-read-only`, `no-telemetry`,
`recorded-cli`, `encrypted-evidence`, `evidence-complete`, `token-private`,
`team-portfolio`, `cli-demo-isolated`, `target-mappings`,
`restore-checklist`, `output-boundary`, `linux-download`, `billing-contract`,
`archive-file-completeness`, `api-metadata-blocks-git`, `json-summary`,
`actionable-errors`, `cli-network-boundaries`, and
`license-browser-storage`.

Cold live first read at 1440 × 900: the page says **“Test your GitHub move
before cutover.”** It says it is for **small teams changing Git hosts** and
that it finds missing repository history and build evidence. The first
action is **“Try it with sample data”**, qualified by “See a complete drill
with no setup.” This satisfies the what/who/first-action gate. Activating it
opens `/demo` in one action with the Atlas Notes sample, a Forgejo 9.0 target,
a blocked result, concrete recovery actions, and the persistent “Demo — sample
data, nothing is saved” banner with Reset demo and Start for real.

## Local build, package, and CLI exercise

| Check | Fresh result |
| --- | --- |
| `npm run audit:copy` | PASS |
| `npm test` | PASS — 5 Rust unit, 13 Rust CLI integration, 39 Playwright tests |
| `npm run build` | PASS — creates `dist/site/` and the Linux x86-64 binary |
| `cargo fmt --all -- --check` | PASS |
| `cargo clippy --all-targets --all-features -- -D warnings` | PASS |
| `npm audit --omit=dev --audit-level=high` | PASS — 0 vulnerabilities |
| `cargo package --allow-dirty --no-verify` | PASS — 94 files, 5.6 MiB |

I installed the CLI into a new Cargo prefix under a fresh temporary consumer.
The installed public binary returned `git-forge-exit-drill 0.1.0`; `--help`
listed `drill`, `demo`, `verify`, `capabilities`, and `portfolio`.
`capabilities` reported the three documented target versions. `demo` wrote
the report, JSON, and evidence archive outside the consumer workspace;
`verify` authenticated its 29 evidence files. A normal local drill against a
copied Atlas Notes export plus a valid Git mirror generated all three output
files and a truthful **BLOCKED** report identifying Forgejo Actions-run
history and missing export evidence. A non-empty demo output directory exited
1 and preserved its `sentinel` file. A missing source exited 1 with “check
--source and try again” and did not create an output directory.

## Live deployment, privacy, and resilience

The live hashes equal the fresh candidate build:

| Artifact | SHA-256 |
| --- | --- |
| JavaScript | `b0e04757593b24f5f3e6c0d9a49b9c83f41391539faeaa1f630c577b7cf17d7f` |
| CSS | `e8201e25e6aecd03ff7df059d419b3ae9fba595fcc8f1c2362c7e3c6b2d5b2f1` |
| Linux x86-64 binary | `2c56bd5d17e78eedbffc4befb6cc92e74b5fdfc40737c6231ff448e9880bb766` |

The downloaded live binary was made executable in `/tmp` and returned
`git-forge-exit-drill 0.1.0`. `/`, `/demo`, `/privacy`, and `/terms` returned
200; an unknown route returned the intended HTTP 404. All on-site and external
links resolved appropriately (the checkout endpoint returned a Sociobot 303
to hosted Dodo checkout).

- Fresh desktop and 390 × 844 mobile checks had no normal-page console/page
  errors or horizontal overflow. The first keyboard focus is the skip link,
  with a designed 3 px amber focus outline; Enter on the sample action opens
  the demo. The mobile primary action is 226 × 44 px.
- Fresh Axe scans of `/`, `/demo`, `/privacy`, `/terms`, and the styled 404
  found zero serious or critical violations. `lang`, one H1, a main landmark,
  and image alt text are present. Reduced-motion mode reduces transition and
  animation durations to `0.00001s` without hiding content.
- The fresh `/demo` request log contained only
  `https://git-forge-exit-drill.sociobot.in`: no telemetry, CDN fonts, or
  third-party scripts. The only externally authorized action is an explicit
  Sociobot license/payment operation.
- Response headers include HSTS, `nosniff`, strict-origin referrer policy,
  restrictive Permissions-Policy, and CSP with response-header
  `frame-ancestors 'none'`. HTML uses short must-revalidate caching and hashed
  assets use one-year immutable caching. Production JavaScript is 19,823 B
  (6,615 B gzip), CSS 14,402 B (3,855 B gzip), and the hero WebP 61,388 B:
  each is below the applicable budgets.
- The service worker was active after `registration.update()` (no waiting
  worker). With the browser offline, `/demo` reloaded successfully with its
  H1, demo banner, and blocked sample result.

## Request allowance and applicability

The product has no product-owned backend, sign-in, Entra identity path, or
runtime AI service. The only product-unlock server endpoint is Sociobot
license verification. A single client made invalid-license requests to
`/api/v1/products/git-forge-exit-drill/verify`: requests 1–30 returned 200;
request 31 returned **429** with **`Retry-After: 3`** and
`x-ratelimit-after: 3`. Observed allowance: **30 requests per active window**.

## Defects by severity

None. No release-blocking, major, moderate, or minor product defect was found.

## Re-run essentials

```sh
npm ci --ignore-scripts --no-audit --no-fund
npm test
npm run build
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
```

Then run every `test` entry in `.factory/claims.json` separately, install the
CLI with `cargo install --locked --path . --root /tmp/gfed-consumer`, and test
the live URL above with a fresh browser context.
