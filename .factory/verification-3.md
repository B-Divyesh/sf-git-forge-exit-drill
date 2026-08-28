# Independent verification 3 — FAIL

**Candidate:** `76f3a5e82f66a0f1d66e1c56815cb4cdc463242a`  
**Live URL:** <https://git-forge-exit-drill.sociobot.in>  
**Verified:** 2026-08-28 from the clean candidate checkout.

## Release decision

**FAIL.** The CLI still gives false-positive completeness results for critical
non-Git artifacts. It accepts malformed files as captured evidence and trusts
arbitrary manifest counts when the corresponding records do not exist. The
shipped demo itself reports more captured issues, pull requests, releases,
workflows, and runs than its evidence archive contains. This defeats the core
job of finding unsupported or unexported data before cutover.

## Mandatory first read and demo

PASS. A fresh live Chromium context showed “Test your GitHub exit before
cutover,” identified small teams moving forges, and said it finds missing
history and build evidence. The visible first action was **Try it with sample
data**, with “See a complete drill with no setup” beside it. One click opened
`/demo`, which immediately showed the sample result and the persistent demo
banner with **Reset demo** and **Start for real**.

## Required claims

`.factory/claims.json` exists with eight entries. After `npm ci`, every exact
listed command passed independently against the demo entry point:

| Claim | Exact command | Result |
| --- | --- | --- |
| `demo-private` | `npm test -- --grep @claim:demo-private` | PASS |
| `free-single` | `npm test -- --grep @claim:free-single` | PASS |
| `source-read-only` | `npm test -- --grep @claim:source-read-only` | PASS |
| `no-telemetry` | `npm test -- --grep @claim:no-telemetry` | PASS |
| `recorded-cli` | `npm test -- --grep @claim:recorded-cli` | PASS |
| `encrypted-evidence` | `npm test -- --grep @claim:encrypted-evidence` | PASS |
| `token-private` | `npm test -- --grep @claim:token-private` | PASS |
| `team-portfolio` | `npm test -- --grep @claim:team-portfolio` | PASS |

Each claim tag occurs exactly once. The claims suite does not assert that a
“captured” count is backed by that many valid exported records. The
`recorded-cli` test only checks that three transcript lines match CLI output;
it does not prove the output is complete or accurate.

## Release-blocking defect

### Critical — missing and malformed artifact data is reported as captured

Two independent packaged-consumer reproductions expose the same unsafe trust
boundary:

1. A source containing only an `issues.json` file whose content was the plain
   text `this is not json` exited **0**. `readiness.json` reported the critical
   `issues` artifact as `captured: true`, `count: 1`, and `result: "mapped"`.
2. A source containing a valid Git mirror plus only a `manifest.json` claimed
   999 issues, 888 pull requests, 777 releases, 666 workflows, and 555 Actions
   runs. No corresponding artifact files existed. The command exited **0** and
   reported every claimed count as captured.

The deployed Linux binary reproduced the invalid-JSON case exactly. It is
byte-identical to the candidate release binary.

The bundled sample confirms this is not merely an adversarial fixture:

| Artifact | Manifest/report says captured | Actual bundled records |
| --- | ---: | ---: |
| Issues | 18 | 2 |
| Pull requests | 12 | 2 |
| Releases | 3 | 1 |
| Actions workflows | 4 | 0 |
| Actions runs | 31 | 1 |

The site therefore displays “Issues · 18 captured” and “Pull requests · 12
captured” although the encrypted sample evidence contains only the excerpts
above. `inventory_local` preserves manifest counts without reconciling them
with files, while filename inference treats invalid JSON as one record. A team
can be told history is captured when it is absent from the archive.

This contradicts the researched success measure (“identify all unsupported or
unexported critical artifacts before cutover”) and the landing claim that the
tool finds missing history and build evidence. That core correctness promise
also lacks a dedicated claim entry and observable completeness test.

Required remediation: parse recognized export formats, compare actual records
with declared totals, report discrepancies as missing/incomplete evidence, and
never convert invalid content or a manifest count alone into `captured: true`.
Add claim coverage that opens the produced archive/report and proves every
captured count has corresponding valid evidence.

## Additional defect

### Medium — mobile touch targets are below the 44 px contract

At 390 px, the demo banner’s **Reset demo** and **Start for real** controls are
36 px high. Footer links are 19.3 px high; the home mark and **Demo** navigation
link are only 22 px and 30.5 px wide. These fail the supplied 44 by 44 CSS-pixel
touch-target baseline even though keyboard focus and axe checks pass.

## Passing quality evidence

- Clean install: `npm ci` passed with 22 packages and 0 vulnerabilities.
- Full suite: `npm test` passed: 3 Rust unit tests, 8 CLI integrations, and 18
  Playwright tests.
- `cargo fmt -- --check`, `cargo clippy --all-targets -- -D warnings`, and the
  explicit TypeScript `tsc --noEmit` check passed.
- Exact production build: `npm run build` passed and produced `dist/site/` plus
  the release binary.
- Package/consumer: `cargo package --locked --allow-dirty` passed verification
  with 46 files (186.7 KiB compressed). A clean unpacked consumer installed
  with `cargo install --locked`; `--help`, JSON capabilities, `demo`, and
  archive `verify` passed. The demo archive verified 28 evidence files.
- Normal/recovery CLI checks: missing and empty sources, a short passphrase, an
  unknown target, and a wrong archive passphrase exited 1 with actionable
  messages. Demo output refused a non-empty directory and preserved its
  sentinel. Eleven portfolio sources were rejected before license or output;
  the registered test proved ten valid sources succeed.
- Desktop and 390 px mobile had no horizontal overflow or page errors on the
  real routes. Tab reached the visible skip link first with a 3 px amber focus
  outline, then reached and activated the sample action. At 200% text size,
  the 390 px demo retained its heading, banner, and width without clipping.
- Axe found zero serious or critical violations on `/`, `/demo`, `/privacy`,
  and `/terms`. `/opt/fleet/lib/verify-url.sh` passed on retry in 1,074 ms with
  title, `lang=en`, one H1, `main`, alt text, labels, and console clean. Its
  first invocation had an isolated browser `networkidle` timeout; direct
  Playwright loads and the immediate retry were healthy.
- Reduced-motion emulation changed line animation from 0.22 s to 0.00001 s.
  A live service worker `update()` completed, controlled the page, and served
  `/demo` on offline reload.
- A Playwright request log covering `/demo`, **Reset demo**, and **Start for
  real** contained only the product origin. It included the service worker’s
  same-origin shell prefetches. Leaving demo removed the only demo storage key.
- Live browser response headers include CSP with header-only
  `frame-ancestors 'none'`, HSTS, `nosniff`, strict referrer policy, and a
  camera/microphone/geolocation permissions policy. HTML revalidates after 30
  seconds; hashed JS and CSS use one-year immutable caching.
- Bundle sizes: JavaScript 15,941 B raw / 5.72 KB gzip; CSS 12,014 B raw /
  3.42 KB gzip; no web fonts; hero image 61,388 B. These are under budget.
- Lighthouse 12.8.2 mobile: performance 100, accessibility 100, best practices
  100, SEO 100; FCP 1.0 s, LCP 1.0 s, TBT 0 ms, CLS 0, speed index 1.3 s.
- `/`, `/demo`, `/privacy`, and `/terms` return 200. An unknown route renders
  the designed recovery page with HTTP 404. Internal links and the binary
  download return 200; the Team Pack link returns 303 to hosted Dodo checkout.
- License restore from the live page reached the Sociobot API successfully via
  CORS and displayed the invalid-license recovery state. One client received
  30 HTTP 200 invalid-verdict responses; request 31 returned HTTP 429 with
  `Retry-After: 4`. Observed allowance: 30 requests, then four-second backoff.
- Deployment identity is exact. Local/live SHA-256 values match:
  - HTML: `34cfc2309c989fb8fdaabe8110414b694c514a1e70cd911fafbd84bd1551a644`
  - JavaScript: `d32a7a9e707be1bedfb3082c35480b15e644b408d53e170a8f5973a5836a55ae`
  - CSS: `096f3abf75e831335e34ade311cc219b86b72055e8bd04b326316fdc957b55d1`
  - Linux binary: `9b1ab76593a3cb481e1d8a9d124aa395a2b9fcadb9a09c9525c0ce4444c1151e`
- Metadata, `robots.txt`, and `sitemap.xml` are present; the original social
  image is 1200 by 630 and the touch icon is 180 by 180.

## Required next steps

1. Make artifact capture evidence-backed and reject or flag count/content
   mismatches and malformed recognized files.
2. Add one claim and regression test for completeness accuracy, including the
   shipped sample, manifest-only input, and malformed JSON.
3. Increase all mobile interactive hit areas to at least 44 by 44 CSS pixels.
4. Rerun every claim, package-consumer, live identity, and accessibility check.
