# Independent verification 9 — FAIL

**Candidate:** `b1a68693eca18da460b1617abfb5c150af04da2f`  
**Live URL:** <https://git-forge-exit-drill.sociobot.in>  
**Verified:** 2026-08-29 from the clean work-order checkout.

## Release decision

**FAIL.** The mandatory claims, first-read gate, core CLI, clean package,
deployment identity, privacy boundary, accessibility scans, performance, PWA,
and endpoint rate limit all pass. The paid Team Pack flow is not usable end to
end from its documented checkout return: the site saves and hides the returned
license, but the native CLI requires the user to supply that token through
`GFED_LICENSE`. The product provides no way to view, copy, download, or apply
the returned token to the CLI.

Two smaller recovery defects are also independently reproducible: malformed
license-cache data causes an uncaught page error and suppresses verification,
and Reset demo drops keyboard focus to the document body.

## Release-blocking defect

### F-9-1 — Major — a checkout-returned license cannot be handed to the paid CLI

The landing page promises that a $39 one-time purchase adds the CLI's
`portfolio` command. The checkout contract returns the buyer to the site with
`?license=<token>`, while the CLI accepts the license only from
`GFED_LICENSE`.

Fresh reproduction with a recorded valid verification response:

1. Open `/?license=qa-valid-cli-handoff` in a new browser context.
2. The page saves `qa-valid-cli-handoff` under
   `sb_license:git-forge-exit-drill`, strips it from the URL, and shows
   `Team Pack license active.`
3. The token is absent from visible page text. The license input remains
   hidden and empty. No copy-token, download-license, or CLI setup action is
   available, and `GFED_LICENSE` is not mentioned on the page.
4. The native CLI cannot read browser localStorage. Its `portfolio` command
   requires the separate `GFED_LICENSE` environment variable.

Observed browser state:

```json
{
  "url": "https://git-forge-exit-drill.sociobot.in/",
  "saved": "qa-valid-cli-handoff",
  "status": "Team Pack license active.",
  "tokenVisible": false,
  "gfedInstructionsVisible": false,
  "licenseInputVisible": false
}
```

Relevant implementation: `site/src/main.ts:270-276` stores the callback token
and removes it from the URL; `site/src/main.ts:248-262` only displays a verdict;
`src/main.rs:81-83` makes the native CLI read `GFED_LICENSE`. README lines
108-112 name both halves but do not explain how to transfer the returned token.

This breaks the paid job immediately after a successful return. A buyer should
receive an explicit, secure way to copy or download the license for the CLI,
plus the exact setup command. Add a test that begins with a returned valid
license and ends with a clean installed CLI successfully running `portfolio`.

The existing claim tests do not catch the gap: `billing-contract` stops at the
hosted order summary, `license-browser-storage` stops at browser storage, and
`team-portfolio` injects a recorded license directly into the CLI. Each passes
in isolation, but they do not prove the public purchase-to-portfolio claim as
one observable workflow.

## Other defects

### F-9-2 — Moderate — malformed cached license data causes an uncaught page error

With a saved license and `sb_license_cache:git-forge-exit-drill` set to
`{not json`, reloading the live home page produces the page error
`Expected property name or '}' in JSON at position 1`. No verification request
is made and the license status stays blank. The free page remains visible, and
the user can recover by opening Privacy and removing the license, but normal
startup should discard an unreadable cache and perform a fresh check.

The parse at `site/src/main.ts:250` happens before the existing `try` block.
Move defensive cache parsing inside error handling, clear invalid cached data,
and add a regression test.

### F-9-3 — Moderate — Reset demo loses keyboard focus

On live `/demo` at 390 x 844, focus **Reset demo** and press Space. The sample
resets and the live region announces the result, but `document.activeElement`
becomes `<body>` because `render(false)` replaces the focused button. The next
Tab starts again at the skip link. Restore focus to the new Reset demo button
after rendering and assert it in the keyboard test.

The skip link itself is correct: after activation and the browser's focus
settles, the H1 is focused, `#main` is at the top, and the designed 3 px amber
focus ring is visible.

## Mandatory claims gate

`.factory/claims.json` exists, parses, and contains 21 unique IDs. Before
general QA, every listed `test` command was run separately and sequentially;
each performed its declared clean `npm ci --ignore-scripts --no-audit
--no-fund` bootstrap. Result: **21/21 passed**.

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
| `cli-demo-isolated` | PASS |
| `target-mappings` | PASS |
| `restore-checklist` | PASS |
| `output-boundary` | PASS |
| `linux-download` | PASS |
| `billing-contract` | PASS |
| `archive-file-completeness` | PASS |
| `api-metadata-blocks-git` | PASS |
| `json-summary` | PASS |
| `actionable-errors` | PASS |
| `cli-network-boundaries` | PASS |
| `license-browser-storage` | PASS |

Each ID appears exactly once as `@claim:<id>` in the Playwright suite. There
are no unregistered claim tags. F-9-1 is a claim-composition gap: the suite
proves three disconnected pieces instead of the advertised paid workflow.

## Cold first read and demo

The mandatory gate passes cold at 1440 x 900 and 390 x 844:

- What: `Test your GitHub move before cutover`.
- For whom: `For small teams changing Git hosts...`.
- First action: `Try it with sample data`, beside `See a complete drill with
  no setup.`
- Three visible facts state the local-network, demo-storage, and free-tier
  boundaries.

One click opens `/demo`, immediately showing Atlas Notes, Forgejo 9.0, a
blocked outcome, concrete risks, and restore steps. The persistent banner says
`Demo — sample data, nothing is saved` and provides Reset demo and Start for
real. Demo storage uses only `demo:gfed:started`; Start for real removes it.

## Clean repository gates

All commands below passed at the candidate SHA:

- `npm ci --ignore-scripts --no-audit --no-fund`
- `npm test`: 5 Rust unit tests, 13 CLI integration tests, and 36 Playwright
  tests
- `npm run typecheck`
- `npm run audit:copy`
- `cargo fmt --all -- --check`
- `cargo clippy --all-targets --all-features -- -D warnings`
- Exact `npm run build`, producing `dist/site/` and the Linux binary
- `cargo package --locked --allow-dirty`: 74 files, 3.0 MiB, followed by a
  successful package verification build

There is no separate JavaScript lint command. Clippy is the available Rust
lint gate.

## Clean consumer and CLI exercise

The generated crate was extracted and installed with `cargo install --locked
--path ...` into a fresh temporary prefix. The installed binary reported
`git-forge-exit-drill 0.1.0`; help documented drill, demo, verify,
capabilities, portfolio, JSON output, and non-interactive options.

The packaged `--json demo` wrote the three declared files. Archive verification
reported 29 evidence files. A separate GitLab 17.0 drill used the exact minimum
12-character passphrase behind rejecting HTTP/HTTPS proxies; it completed,
kept every source hash unchanged, generated seven restore steps, and exposed
none of a known issue title in a plaintext archive scan.

Independent error and boundary cases behaved correctly:

| Case | Exit | Result |
| --- | ---: | --- |
| 11-character passphrase | 1 | JSON says to set a longer value. |
| Missing source | 1 | Names the missing path and next step. |
| Source is a file | 1 | Requests an extracted export directory. |
| Empty source | 1 | Requests an extracted export. |
| Unknown target | 1 | Lists all three supported targets. |
| Invalid `owner/name` shape | 1 | Shows the accepted example. |
| Wrong archive passphrase | 1 | Names passphrase or integrity recovery. |
| Demo into occupied directory | 1 | Refuses it and preserves the sentinel. |
| Source file over 25 MiB | 1 | Explains the limit and next step. |

A hostile cross-origin redirect from a configured GitHub fixture received no
`Authorization` header, confirming the API token is stripped on redirect.

## Live browser, accessibility, privacy, and offline checks

Fresh live checks covered `/`, `/demo`, `/privacy`, `/terms`, and an unknown
route at 1440 x 900 and 390 x 844.

- Product routes return 200; the designed unknown route returns 404.
- Every page has `lang=en`, a route-specific title, one H1, one main landmark,
  ordered headings, complete image alt text, and no ordinary overflow.
- Every route reflows without horizontal overflow at 200% text size at both
  viewports.
- Every visible link, button, and input is at least 44 x 44 CSS px.
- Axe reports zero serious or critical findings across all routes and sizes.
- Normal route loads have no console or page errors.
- Reduced motion sets smooth scrolling to `auto`, removes button transforms,
  and reduces the terminal line animation to `0.00001s`.
- The factory `verify-url.sh` passes `/`, `/demo`, `/privacy`, and `/terms`
  with zero browser errors.
- A fresh demo flow requests only the product origin and stores only the demo
  namespace. No telemetry, CDN font, Azure AI, or other third-party runtime
  request occurs.
- A real invalid-license return makes one CORS-enabled, `no-store` request to
  `api.sociobot.in`, strips the query token, shows the invalid notice, and
  suppresses a second request on reload through its daily cache.
- The service worker updates, controls the page, exposes `gfed-shell-v1`, and
  reloads `/demo` offline with status 200, its H1, and no-save banner.

The product uses no sign-in, so the Entra authority rule is not applicable.
It has no product-owned backend, so backend concurrency, persistence, and
health checks do not apply. AI would not improve this deterministic evidence
validation job.

## Headers, caching, links, budgets, and performance

The live HTML response includes HSTS, `nosniff`, strict-origin referrer policy,
restrictive permissions policy, and the product CSP. `frame-ancestors 'none'`
is a response header. CSP connections are limited to self and the documented
Sociobot billing API.

HTML uses `public, must-revalidate, max-age=30`; hashed JS and CSS use a
one-year immutable cache; WebP art uses one day. Every crawled internal link,
asset, manifest, service worker, icon, and binary download returns 200. The
checkout returns the expected 303 to hosted Dodo; its registered claim test
confirmed product name, `$39.00`, and `One-time unlock` without purchasing.

Production sizes:

- JavaScript: 16,160 bytes raw / 5,687 bytes gzip
- CSS: 12,898 bytes raw / 3,606 bytes gzip
- Hero WebP: 61,388 bytes
- Fonts: none

Fresh mobile Lighthouse: performance 100, accessibility 100, best practices
100, SEO 100; FCP 1.03 s, LCP 1.30 s, CLS 0, TBT 68 ms, speed index 1.03 s,
with no run warnings.

## Deployment identity

`HEAD`, local `main`, and `origin/main` were the candidate SHA before this
report. The candidate only changes prior handoff documentation over the
deployed source parent; nevertheless, identity was checked from a fresh build.
All compared artifacts match byte for byte:

| Artifact | SHA-256 |
| --- | --- |
| `index.html` | `a84b36cccbf1c4f7fecff530d9fbf78238b8666d71cc9c7c404a18babc6f029a` |
| `demo/index.html` | `f2ccac3eab63bc484a930457f1ca03e430cad23fbe64b7e847f142a989d8f974` |
| `privacy/index.html` | `cc173eff89af3c49d62d19afe2791b74056f86576195f82c241b66e648a9579f` |
| `terms/index.html` | `6180776b7b295c6501910eeda3cd3a4b9503dd4211328741ff013cda7f56bf7c` |
| `404.html` | `2f185171b4e4a1077c9ad2a9498f2ea0c59a505ad457b34ff5ae8e42fd7503aa` |
| `assets/index-B_J9ZtOV.js` | `b69fea87e93421c7045c9367c6bd694c6290ca713dcbb36c30b7076379967dab` |
| `assets/index-D_If0Hr5.css` | `8ad1851be9b1f387cf8c6ad8afec64aaf873d83213e461e0b2f2c5796cbcb69d` |
| `geometry-exit-drill.webp` | `69a1452e5c9c0df2023198be491e977cacc3af9913110e8c608d10b9d4cb5443` |
| Linux x86-64 binary | `11d47f861d1d1a7a627d3cb2b0e74bedb034d21f1a20b6428b074f3ce6269cc4` |

The downloaded live binary executes as version 0.1.0 and lists the three
expected mapping versions.

## Billing endpoint allowance

From one client, live verification requests 1 through 30 returned 200. Request
31 returned 429 with `Retry-After: 4` and `X-RateLimit-After: 4`. Observed
allowance: **30 requests per active rate window**.

## Required next steps

1. Complete the paid callback-to-CLI handoff and test a returned valid license
   through a clean installed `portfolio` run.
2. Recover safely from malformed browser license-cache JSON.
3. Preserve focus on Reset demo after replacing the demo DOM.
4. Re-run every claim command, the full clean gates, packaged consumer test,
   and live verification before release.
