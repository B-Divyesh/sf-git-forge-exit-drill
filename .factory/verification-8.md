# Independent verification 8 — FAIL

**Candidate:** `ec108c4e58d9be295959d2064b14780bff427093`  
**Live URL:** <https://git-forge-exit-drill.sociobot.in>  
**Verified:** 2026-08-29 from the clean work-order checkout.

## Release decision

**FAIL.** The claims gate, build, CLI, deployed artifacts, ordinary desktop and
mobile layouts, privacy boundary, offline reload, and API rate limit all pass.
Two independently reproduced defects still violate the acceptance contract:
text clips at 200% size on a 390 px viewport, and returned licenses are checked
twice while the invalid-license notice remains hidden.

## Release-blocking defects

### F-8-1 — Major — 200% text clips and requires horizontal scrolling

At 390 x 844, setting the root text size to 200% makes the home document 444
CSS px wide in a 390 px viewport. In the Team Pack section, the heading “Check
ten repositories together” has a 358 px content box but a 428 px scroll width.
The right side of “repositories” is visibly clipped until the user scrolls
horizontally.

This fails the non-negotiable accessibility requirement that text resize to
200% without loss. Axe does not detect reflow failures, which is why the normal
viewport Axe run remains green.

Reproduction:

1. Open the live home page at 390 x 844.
2. Set the root text size to 200%, equivalent to the required text-resize
   check.
3. Scroll to Team Pack.
4. Observe `document.documentElement.scrollWidth === 444` while
   `clientWidth === 390`; the pricing H2 reports `scrollWidth === 428` and
   `clientWidth === 358`.

### F-8-2 — Major — returned-license verification is duplicated and feedback is hidden

Opening `/?license=qa-invalid-visibility-ec108c4` from a fresh browser context
caused two identical GET requests to:

`https://api.sociobot.in/api/v1/products/git-forge-exit-drill/verify?license=qa-invalid-visibility-ec108c4`

Both returned 200. The page cached `{"valid":false,...}` and set the status
text to “License no longer active.” However, that status is inside
`.license-form[hidden]`; Playwright reported `statusVisible: false`. A buyer
returning with an invalid, expired, or revoked license receives no visible
notice.

This violates the paid-unlock rules to verify at most once per day and to show
a quiet invalid-license notice with the buy link. The visible buy link remains
present, but the status does not.

## Mandatory claims gate

`.factory/claims.json` exists, parses, and contains 21 entries. Before any
general repository inspection or build command, every entry's exact `test`
command was run sequentially. Every command performed its own clean
`npm ci --ignore-scripts --no-audit --no-fund` bootstrap and exited 0.

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

Each ID occurs exactly once as an `@claim:<id>` tag in the Playwright suite.
The live landing, legal pages, CLI help, and README were cross-checked against
the registry; no additional unlisted product claim was found. F-8-2 is a gap
in the registered license test: it checks storage and removal but not request
count or whether the returned-license verdict is visible.

## Cold first read and demo

The mandatory first-read gate passes on a cold 1440 x 900 visit:

- What: “Test your GitHub move before cutover.”
- For whom: “For small teams changing Git hosts...”
- First action: “Try it with sample data,” beside “See a complete drill with
  no setup.”
- Three facts state the network, demo-storage, and free-tier boundaries.

One click opens `/demo`. The first demo screen already shows the Atlas Notes
repository, Forgejo 9.0 target, `BLOCKED` result, critical risks, and restore
steps. The persistent banner says “Demo — sample data, nothing is saved” and
offers **Reset demo** and **Start for real**. The only demo storage key is
`demo:gfed:started`; Start for real removes it.

## Clean local gates

All commands below passed at the requested SHA:

- `npm ci --ignore-scripts --no-audit --no-fund`
- `npm test`: 5 Rust unit tests, 13 CLI integration tests, and 34 Playwright
  tests passed.
- `npm run typecheck`
- `npm run audit:copy`
- `cargo fmt --all -- --check`
- `cargo clippy --all-targets --all-features -- -D warnings`
- Exact `npm run build`; `dist/site/` and its Linux binary were produced.
- `cargo package --locked --allow-dirty`: 73 files, 3.0 MiB, followed by a
  successful package verification build.

There is no separate JavaScript lint script. Clippy is the available Rust
lint gate.

## Clean consumer and CLI exercise

The packaged crate was installed into a new temporary prefix. Its public
binary reported `git-forge-exit-drill 0.1.0`; `--help` described all commands,
exit behavior, and `--json`; `capabilities` listed Forgejo 9.0, Gitea 1.22,
and GitLab 17.0.

The packaged `--json demo` ran without setup. Its evidence archive verified
with 29 files and produced `evidence.gfed`, `readiness.md`, and
`readiness.json`. The report contained 13 findings and seven restore-checklist
items.

A separate local GitLab 17.0 drill used the minimum accepted 12-character
passphrase behind rejecting HTTP/HTTPS proxies. It completed, its archive
verified, and source hashes before and after were identical. The encrypted
archive did not expose a known sample issue title in a plaintext scan.

Independent negative and recovery cases all behaved correctly:

| Case | Exit | Evidence |
| --- | ---: | --- |
| 11-character passphrase | 1 | JSON error says to set a longer value. |
| Missing source directory | 1 | JSON error names the missing path and next step. |
| Unknown `codeberg:latest` target | 1 | JSON lists the three supported targets. |
| Demo into non-empty output | 1 | Refused deletion; sentinel remained unchanged. |
| Verify with wrong passphrase | 1 | Says to check the passphrase and file integrity. |

## Deployment identity

`HEAD`, local `main`, and `origin/main` were all the candidate SHA before the
verification documentation commit. The production application matches the
candidate build byte for byte:

| Artifact | SHA-256 | Match |
| --- | --- | --- |
| `index.html` | `290fc177f96a8780bdf3b084ea978a87ddceb5bdae32a73b7fe5bdaa39edc50b` | yes |
| `assets/index-4uaupLdp.js` | `594a36dfb65d23a0079231078f32610958425075770c7e7eb6319da950d333c6` | yes |
| `assets/index-B23u05QL.css` | `ad74c40e94e783632464b155eea168610961c1affe94c975b917f7e54ba613f2` | yes |
| `geometry-exit-drill.webp` | `69a1452e5c9c0df2023198be491e977cacc3af9913110e8c608d10b9d4cb5443` | yes |
| Linux x86-64 binary | `11d47f861d1d1a7a627d3cb2b0e74bedb034d21f1a20b6428b074f3ce6269cc4` | yes |
| `demo/index.html` | `609ba113e465915edaa404c37e7f9e5eda11284a96dc73e15cbb0b0536c0711e` | yes |
| `privacy/index.html` | `dc1c2e0e1ca7a6a5128b4c321629979c0bf7c57a84f3ac79c4d0005643af236b` | yes |
| `terms/index.html` | `9ed9391edc2361ea3771b15cdd2711b99f2710a1b00a7b6791a21bec5c061f6c` | yes |
| `404.html` | `d116cff4da8954911bcb14c41d3f9d281df428e1f890fb86d4e063641cc40a0e` | yes |

The downloaded live binary executes and reports version 0.1.0.

## Browser, accessibility, and PWA results

At both 1440 x 900 and 390 x 844, `/`, `/demo`, `/privacy`, and `/terms`
return 200. The designed unknown route returns HTTP 404. Every route has a
route-specific title, `lang=en`, one H1, one main landmark, no missing image
alt, and no ordinary horizontal overflow. Live Axe found zero serious or
critical violations on all five pages at both sizes. Every visible checked
control measured at least 44 x 44 CSS px.

The live application routes produced no console or page errors. Chromium logs
the expected failed-main-resource line for the intentional 404 response; the
designed 404 page still renders. The factory `/opt/fleet/lib/verify-url.sh`
passed in 782 ms with no errors.

Keyboard-only checks passed after allowing the route focus frame:

- First Tab exposes the skip link with a 3 px amber focus outline.
- Enter moves focus to `#page-title`; the next Tab reaches the sample action.
- Enter opens `/demo` and focuses its H1.
- Space resets the demo; Enter on Start for real clears demo storage and
  focuses `#install-title`.

Reduced motion changes terminal animation duration to `0.00001s` and scroll
behavior to `auto`. The 200% text-resize failure is F-8-1.

The service worker became the controller, accepted `registration.update()`,
and exposed cache `gfed-shell-v1`. After switching the browser offline,
reloading `/demo` returned 200 with the expected title, H1, banner, and no
errors.

## Privacy, headers, caching, links, and budgets

A fresh root or demo flow requested only
`https://git-forge-exit-drill.sociobot.in`. A returned-license flow added only
the documented `https://api.sociobot.in` origin. No telemetry, CDN font,
Azure OpenAI, or other third-party runtime request was observed. No sign-in is
used, so the Entra authority requirement is not applicable. AI is not useful
for this deterministic evidence-validation job.

Browser response headers include HSTS, `nosniff`, strict-origin referrer
policy, restrictive permissions policy, and a self-only CSP except for the
documented billing connection. `frame-ancestors 'none'` is delivered as a
response header. HTML uses `public, must-revalidate, max-age=30`; hashed JS and
CSS use `public, max-age=31536000, immutable`; the hero image uses a one-day
cache.

All internal route, manifest, service-worker, image, icon, and download links
returned their expected 200 status. The unknown route returned 404,
Sociobot returned 200, and checkout returned 303 to the hosted Dodo page. The
hosted order summary showed Git Forge Exit Drill, `$39.00`, and “One-time
unlock.” Mail links were excluded from HTTP crawling.

Production budgets pass:

- JavaScript: 16,149 bytes raw / 5,686 bytes gzip.
- CSS: 12,855 bytes raw / 3,589 bytes gzip.
- Hero WebP: 61,388 bytes.
- Fonts: zero files and zero external font requests.

Mobile Lighthouse JSON reported performance 99, accessibility 100, best
practices 100, and SEO 100; FCP 1.1 s, LCP 1.3 s, CLS 0, TBT 120 ms, and speed
index 1.1 s. Lighthouse printed a late “Browser tab has unexpectedly crashed”
message after producing the complete report, but returned exit 0 and the JSON
contained no run warnings.

## Billing endpoint allowance

From one client, live license-verification requests 1 through 30 returned 200.
Request 31 returned 429 with `Retry-After: 3` and `X-RateLimit-After: 3`.
Observed allowance: **30 verification requests per active rate window**.

There is no product-owned backend beyond the external Sociobot checkout and
license API, so backend concurrency, persistence, and health/build identity
checks are not applicable.

## Required next steps

1. Allow long monospace words to wrap or scale headings so all home content
   reflows at 200% text size without horizontal scrolling.
2. Make the returned-license verdict visible outside the hidden restore form.
3. Ensure a returned license starts exactly one verification request and the
   cached verdict suppresses further requests for one day.
4. Add regression tests for 390 px at 200% text and for returned-license
   request count plus visible valid/invalid status.
