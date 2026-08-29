# Independent verification 6 — FAIL

**Candidate:** `4be1fafe73ebcd793693f1c6b3be72b68ae0c4e4`  
**Live URL:** <https://git-forge-exit-drill.sociobot.in>  
**Verified:** 2026-08-29 from a clean candidate checkout.

## Release decision

**FAIL.** The CLI silently truncates GitHub API collections after 100 pages
and reports the partial collection as captured evidence. A recorded endpoint
that declared 10,001 Actions runs produced a successful report claiming that
10,000 runs were captured, with no incomplete or unavailable warning. The
CLI never requested page 101. This breaks the core promise that captured
counts are backed by complete evidence and can hide missing build provenance.

The mandatory first-screen contract also fails at common laptop viewport
heights: the sample-data action is below the fold at both 1366×768 and
1280×720. In addition, the first exact claim invocation from the clean clone
failed before dependency installation because `vite` was unavailable. The
contract explicitly makes any failed claim invocation release-blocking.

## Mandatory claims gate

`.factory/claims.json` exists, is valid JSON, and contains nine claims. I ran
its commands before any install or other inspection, as instructed. The first
command, `npm test -- --grep @claim:demo-private`, ran the Rust suite but then
failed at `npm run build:site` with exit 127 and `vite: not found`. The clean
clone did not contain installed Node dependencies, so the exact claim command
was not self-contained. The loop stopped on that failure.

After the declared `npm ci` installation step, I reran every exact command.
All nine passed:

| Claim | Exact command | Post-install result |
| --- | --- | --- |
| `demo-private` | `npm test -- --grep @claim:demo-private` | PASS |
| `free-single` | `npm test -- --grep @claim:free-single` | PASS |
| `source-read-only` | `npm test -- --grep @claim:source-read-only` | PASS |
| `no-telemetry` | `npm test -- --grep @claim:no-telemetry` | PASS |
| `recorded-cli` | `npm test -- --grep @claim:recorded-cli` | PASS |
| `encrypted-evidence` | `npm test -- --grep @claim:encrypted-evidence` | PASS |
| `evidence-complete` | `npm test -- --grep @claim:evidence-complete` | PASS |
| `token-private` | `npm test -- --grep @claim:token-private` | PASS |
| `team-portfolio` | `npm test -- --grep @claim:team-portfolio` | PASS |

Each `@claim:<id>` tag occurs exactly once. The green
`evidence-complete` test covers local exports but does not cover API
pagination completeness; the independent test below disproves that registered
claim for API inventory.

## Mandatory first read and demo

A cold live visit communicates the following in plain words:

- **What:** “Test your GitHub exit before cutover.”
- **For whom:** “For small teams moving forges…”
- **First action:** **Try it with sample data**, beside “See a complete drill
  with no setup.”

At 1440×900 the action starts at y=808 and is fully visible. At 390×844 it
starts at y=538 and is fully visible. One click opens `/demo`, immediately
shows the real sample result, and displays the persistent “Demo — sample data,
nothing is saved” banner with **Reset demo** and **Start for real**. Demo state
uses only `demo:gfed:started`; reset recreates it and leaving demo removes it.

However, at 1366×768 the action begins at y=784 and is entirely below the
viewport. At 1280×720 it begins at y=755, and the audience sentence is not
fully visible either. The visitor must scroll to discover what to click. That
fails the required one-screen first-read test on representative desktop sizes.

## Release-blocking defects

### Critical — API pagination truncation is reported as complete evidence

I served a deterministic local GitHub-compatible fixture for `acme/huge`.
The Actions runs response declared `total_count: 10001`; pages 1–100 each
returned 100 valid records, and page 101 would return the final record. The
candidate release binary made exactly 100 Actions-run requests, never requested
page 101, exited 0, and wrote this finding:

```json
{
  "artifact": "actions_runs",
  "captured": true,
  "count": 10000,
  "target_support": "unsupported",
  "critical": true,
  "result": "target gap"
}
```

`report.incomplete.actions_runs` and
`report.unavailable.actions_runs` were both absent. The page-100 cap is in
`src/lib.rs:964`; after the loop, `src/lib.rs:999-1003` serializes and accepts
the partial collection even when `current_count < total_count`.

This can hide exactly the missing Actions/build history the product exists to
find. Fix by following GitHub pagination to completion, or by marking the
artifact incomplete whenever a safety cap is reached before `total_count`.
For array endpoints without `total_count`, follow and preserve the GitHub
`Link` header rather than inferring completion from a fixed page ceiling. Add
claim coverage for totals above the safety threshold and an exact 100-record
final page.

### High — first action is below the first screen on common desktops

The hero has a 710 px minimum height, 80 px vertical padding, and a large
10-character-wide heading. At 1366×768 and 1280×720 this pushes the required
sample action below the initial viewport. Keep the headline, audience sentence,
sample action, consequence, and three facts visible without scrolling across
representative desktop heights.

### Contract blocker — first clean-clone claim command exits 127

The required pre-install command failed with `vite: not found`. Although all
nine commands pass after `npm ci`, the supplied acceptance contract says any
failed claim invocation blocks release. Make the recorded clean-clone claim
procedure executable as written, or make the required bootstrap explicit in
each claim command.

## Local build, package, and CLI evidence

- `npm ci`: PASS — 23 packages, 0 vulnerabilities.
- `npm test`: PASS — 3 Rust unit tests, 13 CLI integration tests, and 20
  Playwright tests.
- `npm run typecheck`: PASS.
- `cargo fmt --all -- --check`: PASS.
- `cargo clippy --all-targets --all-features -- -D warnings`: PASS.
- Exact `npm run build`: PASS; it produced `dist/site/` and the release binary.
- `cargo package --locked --allow-dirty`: PASS, including Cargo's verification
  build. A clean unpacked consumer installed with `cargo install --locked`.
- The clean installed binary passed `--version`, `--help`, JSON capabilities,
  `--json demo --output`, archive verification, and wrong-passphrase handling.
  The demo produced three outputs and its archive verified 29 evidence files.
- Empty and missing sources, a file used as a source, a short passphrase, an
  unknown target, and an invalid repository name all exited 1 with actionable
  errors. A non-empty demo directory was refused and its sentinel was kept.
- A normal local sample drill exited 0 with valid JSON and wrote/verified its
  archive. A truncated archive failed authentication with exit 1.

## Live deployment identity and quality

The deployed files match the candidate production build byte for byte:

| Artifact | SHA-256 |
| --- | --- |
| `index.html` | `7d0d78e47fa73e0a8bdce64ca69ced51d262cc05cf5c344b70974b1c70ce305e` |
| JavaScript | `f1d38950c17d52abd4da785734566f79cd012d23e145de2654466aa076871f30` |
| CSS | `6b4732f0b77867a4dd9c334d19f1da7738ea94040e0cec65df9a338219624bf9` |
| Linux x86-64 binary | `a5611966549c9a861aaa588099f06ed56fb58ec89fc8e0ceb2ecf4068c0251fa` |

- `/`, `/demo`, `/privacy`, and `/terms` return 200 with route-specific
  titles, `lang=en`, one H1, and one main landmark. `/not-a-route` returns a
  designed HTTP 404.
- The factory `verify-url.sh` passed in 609 ms with no page or console errors.
- Live Axe found zero serious/critical violations on every real route and the
  404. At 390×844 there is no horizontal overflow; all actually visible links,
  buttons, and inputs meet 44×44 CSS pixels.
- Keyboard Tab reaches the skip link first with a visible 3 px amber outline.
  Reset works with Space, Start for real works with Enter, and route/back
  navigation focuses the new H1. Reduced-motion mode reduces animation and
  transition duration to 0.01 ms.
- The service worker accepted `update()`, controlled the client, and reloaded
  `/demo` offline.
- Lighthouse 12.8.2 mobile: Performance 100, Accessibility 100, Best Practices
  100, SEO 100; FCP 1.0 s, LCP 1.3 s, TBT 80 ms, CLS 0, Speed Index 1.7 s.
- JavaScript is 15,681 bytes raw / 5,685 bytes gzip; CSS is 12,198 bytes raw /
  3,447 bytes gzip; the hero is 61,388 bytes; no web fonts load.
- HTML revalidates after 30 seconds. Hashed JS/CSS use one-year immutable
  caching. The live CSP allows only self plus the billing API and includes
  header-only `frame-ancestors 'none'`; HSTS, `nosniff`, strict-origin
  referrer policy, and camera/microphone/geolocation restrictions are present.

## Privacy, billing, and applicability

- Fresh home and demo request logs contained only the product origin. The
  invalid-license browser flow added only the documented
  `https://api.sociobot.in` verification request. It returned CORS for the
  product origin with `Cache-Control: no-store`, and the page showed “License
  no longer active.” No telemetry, CDN font, raw Azure, or other request was
  observed.
- The checkout endpoint returns HTTP 303 to hosted Dodo checkout.
- From one client, verification requests 1–30 returned 200. Request 31 and
  later returned 429 with `Retry-After: 3`. Observed allowance: 30 requests
  per active rate window.
- Sign-in is not required, so Entra authority checks are not applicable. The
  product has no server-side application backend beyond the factory billing
  endpoint, so product concurrency and persistence checks are not applicable.
- AI would not improve this deterministic, high-trust inventory step; no
  missed AI leverage finding was recorded.

## Required next steps

1. Never report a capped API collection as complete; finish pagination or
   block the artifact with an explicit incomplete-evidence reason.
2. Add API pagination cases to `@claim:evidence-complete`.
3. Keep the complete first-read/action block visible at 1280×720 and 1366×768.
4. Make the exact clean-clone claim commands runnable under the mandated
   claims-first sequence.
