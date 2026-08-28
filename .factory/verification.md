# Independent verification — FAIL

**Candidate:** `34b8db1b02cc974a3aa48e240fb8acbf9bc65cfb`  
**Live URL:** <https://git-forge-exit-drill.sociobot.in>  
**Verified:** 2026-08-28 from a fresh checkout at the candidate SHA.

## Release decision

**FAIL.** The readiness report can state that a Git repository was captured when the supplied export has no Git objects. This defeats the core job: deciding whether repository history will survive a forge exit. The live paid checkout is also HTTP 404, and `demo --output` silently deletes an existing directory.

## Cold first read

The cold landing screen says “Test your GitHub exit before cutover.” It says it is for small teams moving forges and that it finds missing history and build evidence before Monday. Its first action is “Try it with sample data,” promising a complete drill with no setup; one click opened `/demo`. This required first-read/demo check passes.

## Required claims

`.factory/claims.json` exists with five entries. After `npm ci`, each was run independently from the clean candidate checkout. Every command rebuilt the site, ran Rust tests, then ran its tagged Playwright test.

| Claim ID | Exact command | Result |
| --- | --- | --- |
| `demo-private` | `npm test -- --grep @claim:demo-private` | PASS |
| `free-single` | `npm test -- --grep @claim:free-single` | PASS |
| `encrypted-evidence` | `npm test -- --grep @claim:encrypted-evidence` | PASS |
| `token-private` | `npm test -- --grep @claim:token-private` | PASS |
| `team-portfolio` | `npm test -- --grep @claim:team-portfolio` | PASS |

The claim suite does not cover the false-positive repository-evidence case below.

## Passing verification evidence

- `npm ci`: PASS; 23 packages, 0 vulnerabilities reported.
- `npm test`: PASS — 3 Rust unit tests, 4 Rust CLI integration tests, and 12 Playwright tests.
- `npm run build`: PASS; release binary and `dist/site/` produced.
- `cargo clippy --all-targets -- -D warnings`: PASS.
- `cargo package --allow-dirty --no-verify`: PASS — 44 files, 177.5 KiB compressed.
- Clean consumer: unpacked the produced `.crate`, ran `cargo install --path … --root … --locked`, then the installed CLI’s `demo` and `verify`; both passed.
- CLI normal/recovery checks: demo and archive verification passed; wrong passphrase, missing export, unmapped target, and missing Team Pack license exit non-zero with actionable messages.
- Candidate/live identity: local `dist/site/index.html` is byte-identical to live HTML. Local/live JS SHA-256 is `d32a7a9e707be1bedfb3082c35480b15e644b408d53e170a8f5973a5836a55ae`. Local release, staged download, and live binary are byte-identical: `9ce36f8d5e0481bc799dcc7cdc0403649a1b889bab7e2ef26d109940594bb66e`.
- Browser: Chromium desktop and 390×844 mobile had no console/page errors. Mobile `scrollWidth` was 390; Tab reaches the skip link with a visible 3px amber focus ring; reduced-motion emulation worked. Live axe at `/demo`: 0 serious/critical issues.
- PWA: a fresh live client was controlled by `/sw.js`; offline reload to `/demo` rendered “See a complete exit drill” with no errors. A genuine old-to-new SW upgrade cannot be simulated against one deployed version.
- Privacy/network: a fresh live `/demo` made only same-origin document/JS/CSS requests and created only `demo:gfed:started`. The landing page made only same-origin HTML/JS/CSS/hero-image requests. Headers include CSP, HSTS, `nosniff`, referrer policy, and permissions policy. Hashed assets use one-year immutable caching. JS is 15,941 B raw / 5,720 B gzip; CSS is 12,014 B raw / 3,420 B gzip.
- Unlock API allowance: one client received 200 for 30 sequential verify requests, then 429 on request 31 with `Retry-After: 2` and `x-ratelimit-after: 2`.

## Defects

### Critical — repository evidence is a false positive

`inventory_local` trusts artifact counts in `manifest.json` instead of proving corresponding bytes exist. I created an export with only a manifest claiming `"git_repository": 1` and an `issues.json`; it had no `.bundle`, Git objects, refs, or clone. `drill` succeeded and its report wrote:

```text
| Git repository | Yes (1) | native | mapped |
```

The shipped Atlas Notes sample has the same problem: its manifest claims `git_repository: 1`, but its six files have no Git object data; the report still says Git repository captured. API mode also inserts `git_repository: 1` after metadata retrieval without obtaining the object graph. Require and validate a mirror/bundle/refs before reporting repository capture; metadata-only mode must block repository-preservation readiness.

### High — Team Pack checkout is broken live

The visible `$39` purchase link targets `https://api.sociobot.in/api/v1/products/git-forge-exit-drill/checkout`. A fresh GET on 2026-08-28 returned **404**. The advertised paid portfolio feature cannot be purchased.

### High — `demo --output` deletes existing data

With a disposable output directory containing `important.txt`, `git-forge-exit-drill demo --output <directory>` exited 0 and removed that sentinel. There is no confirmation, empty-directory check, or destructive flag. Refuse non-empty output directories by default.

### Medium — styled unknown route returns HTTP 200

`/not-a-route` correctly renders “This route has no evidence,” but its response status is 200 rather than 404. Preserve the styled recovery view while returning a real 404.

### Medium — visitor claims lack claim registration/tests

The supplied claims contract requires tests for every rely-on claim. Landing copy contains unlisted claims including “No account is needed,” “No Telemetry,” “The CLI … reads only the source you provide,” and “This recording comes from the real CLI.” Add observable claim tests or remove/narrow those statements.

## Required remediation

1. Validate actual Git object evidence and block on its absence.
2. Register/fix the Sociobot checkout, or remove the live purchase offer.
3. Make demo output non-destructive.
4. Return HTTP 404 for unknown routes and complete claim coverage.
