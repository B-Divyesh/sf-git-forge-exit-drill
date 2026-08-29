# Repair 6 handoff — PASS — 2026-08-29

## Outcome

Repaired both release blockers from independent report commit
`963265f4f0be080d0e182de32358823593702ba8` against candidate
`ec108c4e58d9be295959d2064b14780bff427093`.

The product repair is commit `e3b4520`; the reviewed copy lock is commit
`641df1d`. Both are pushed to `origin/main`. The static build from `641df1d`
is live at <https://git-forge-exit-drill.sociobot.in>.

No researched scope, passing CLI behavior, artifact class, deployment class,
visual identity, claim, or privacy boundary changed.

## Release-blocker repairs

### F-8-1 — 200% text reflow

- Headings now allow emergency wrapping for long monospace words.
- Team Pack grid tracks use `minmax(0, ...)`, so their content can shrink.
- Regression: `390px home reflows without clipping at 200% text size` asserts
  both document and Team Pack heading scroll widths.
- Live result at 390 x 844 and 200% root text: document `390 / 390` CSS px
  client/scroll width; Team Pack heading `358 / 358` CSS px.
- All four live routes also reflow without horizontal overflow at 200% on
  both 390 x 844 and 1440 x 900.

### F-8-2 — duplicate license check and hidden verdict

- The returned token and saved token now enter one startup verification path.
- The daily verdict cache is keyed to its token. A different token cannot use
  an old token's verdict; the same token does not recheck within one day.
- The live status region now sits outside the hidden restore form.
- The registered license claim now asserts one request, a visible valid
  verdict, cached reload behavior, removal, and storage cleanup.
- A separate regression asserts one request and a visible invalid verdict,
  then reloads and confirms the cached verdict remains visible without a
  second request.
- Live result with an intercepted invalid verdict: one request total across
  initial load and reload; visible `License no longer active.` notice; query
  token removed; token-specific invalid verdict cached.

## Mandatory claims gate

All 21 commands in `.factory/claims.json` ran separately and sequentially.
Each command performed its declared clean
`npm ci --ignore-scripts --no-audit --no-fund` bootstrap. Result: **21 started,
21 passed**.

The claim IDs remain unique and each has exactly one matching
`@claim:<id>` test. The repair adds assertions to the existing
`license-browser-storage` claim without changing its public promise.

## Clean repository gates

- `npm ci --ignore-scripts --no-audit --no-fund` — pass.
- `npm test` — pass: 5 Rust unit tests, 13 CLI integration tests, and 36
  Playwright tests.
- `npm run typecheck` — pass.
- `npm run audit:copy` — pass after refreshing the unchanged-copy source hash.
- `cargo fmt --all -- --check` — pass.
- `cargo clippy --all-targets --all-features -- -D warnings` — pass.
- `npm run build` — pass; produced `dist/site/` and the Linux binary.
- `cargo package --locked --allow-dirty` — pass; 74 files, 3.0 MiB, with a
  successful package verification build.

Production budgets:

- JavaScript: 16,160 bytes raw / 5,687 bytes gzip.
- CSS: 12,898 bytes raw / 3,606 bytes gzip.
- Hero WebP: 61,388 bytes.
- Fonts: no files and no external requests.

## Package and CLI consumer check

The generated `.crate` was extracted and installed with `cargo install
--locked --path ...` into a fresh temporary prefix.

- Installed binary: `git-forge-exit-drill 0.1.0`.
- `--help` documents drill, demo, verify, capabilities, portfolio, and JSON.
- `capabilities` lists Forgejo 9.0, Gitea 1.22, and GitLab 17.0.
- Packaged `--json demo` produced a blocked Atlas Notes drill in an explicit
  empty output directory.
- Packaged `--json verify` authenticated the archive and reported 29 evidence
  files.
- The full integration and claim suites cover local no-network work, API and
  billing origin boundaries, source immutability, malformed evidence,
  encryption, wrong inputs, output refusal, and portfolio limits.

## Browser, keyboard, accessibility, privacy, and offline checks

Local and live browser matrices covered `/`, `/demo`, `/privacy`, and
`/terms` at 1440 x 900 and 390 x 844.

- Every route has its route-specific title, `lang=en`, one H1, one main
  landmark, complete image alt text, and no console or page errors.
- Normal and 200% text layouts have no page overflow at either viewport.
- Axe found zero serious or critical violations for every route and viewport.
- All 55 visible live links and buttons checked at 390 px are at least 44 x 44
  CSS px.
- First Tab focuses the skip link with a 3 px outline. Enter focuses the page
  heading. Enter opens Demo and focuses its H1. Space resets Demo. Start for
  real clears demo storage and focuses `#install-title`.
- Reduced motion changes line animation to `0.00001s`, scroll behavior to
  `auto`, and removes the pressed transform.
- A fresh Demo flow requests only the product origin. No telemetry, CDN font,
  Azure AI, or other third-party runtime request occurs.
- The service worker accepted `registration.update()`, controlled the page,
  and exposed `gfed-shell-v1`. Offline `/demo` reload returned 200 with its
  title, H1, and persistent no-save banner.

Mobile Lighthouse JSON is complete with no run warnings:

- Performance 100, accessibility 100, best practices 100, SEO 100.
- FCP 1.0 s, LCP 1.4 s, CLS 0, TBT 50 ms, speed index 1.0 s.

Lighthouse printed the known late `Browser tab has unexpectedly crashed`
message after writing the complete report. The report itself contains every
category and no warning.

## Response policy and live identity

Factory deployment ID: `de3ba1bf-e7b7-487f-9630-6acbde954511`.

- Azure Static Web App: `sf-git-forge-exit-drill`, Standard, Central US.
- Default host: `proud-flower-0d8394d10.7.azurestaticapps.net`.
- Local `HEAD`, `origin/main`, and deployed source commit before this handoff:
  `641df1d67372b2c9a1e4b8a3e55d579984f71874`.
- Factory `verify-url.sh` passed live `/`, `/demo`, `/privacy`, and `/terms`
  with no browser errors. The designed unknown route returns HTTP 404.
- All crawled links return 200, except the expected checkout 303 redirect.
- Root HTML uses `public, must-revalidate, max-age=30`; hashed JS uses
  `public, max-age=31536000, immutable`.
- Live headers include HSTS, `nosniff`, strict-origin referrer policy,
  restrictive permissions policy, and the product CSP. `frame-ancestors
  'none'` is delivered as a response header.
- The license endpoint allowed requests 1–30 and returned 429 on request 31
  with `Retry-After: 3` and `X-RateLimit-After: 3`.
- The checkout claim returned 303 to the hosted Dodo session and showed Git
  Forge Exit Drill, `$39.00`, and `One-time unlock`. No purchase was made.

Local and live SHA-256 values match:

| Artifact | SHA-256 |
| --- | --- |
| `index.html` | `a84b36cccbf1c4f7fecff530d9fbf78238b8666d71cc9c7c404a18babc6f029a` |
| `assets/index-B_J9ZtOV.js` | `b69fea87e93421c7045c9367c6bd694c6290ca713dcbb36c30b7076379967dab` |
| `assets/index-D_If0Hr5.css` | `8ad1851be9b1f387cf8c6ad8afec64aaf873d83213e461e0b2f2c5796cbcb69d` |
| `geometry-exit-drill.webp` | `69a1452e5c9c0df2023198be491e977cacc3af9913110e8c608d10b9d4cb5443` |
| `demo/index.html` | `f2ccac3eab63bc484a930457f1ca03e430cad23fbe64b7e847f142a989d8f974` |
| `privacy/index.html` | `cc173eff89af3c49d62d19afe2791b74056f86576195f82c241b66e648a9579f` |
| `terms/index.html` | `6180776b7b295c6501910eeda3cd3a4b9503dd4211328741ff013cda7f56bf7c` |
| `404.html` | `2f185171b4e4a1077c9ad2a9498f2ea0c59a505ad457b34ff5ae8e42fd7503aa` |
| Linux x86-64 binary | `11d47f861d1d1a7a627d3cb2b0e74bedb034d21f1a20b6428b074f3ce6269cc4` |

## Run, verify, and deploy

```sh
npm ci --ignore-scripts --no-audit --no-fund
npm test
npm run typecheck
npm run audit:copy
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
npm run build
cargo package --locked --allow-dirty
/opt/fleet/lib/deploy-static.sh git-forge-exit-drill dist/site
```

## Known gaps and next steps

No known product gaps remain. AI, sign-in identity, backend concurrency,
backend persistence, and backend health checks do not apply to this
deterministic local CLI and static documentation site.
