# Independent verification 5 — FAIL

**Candidate:** `e2b5834e3f4ea13ca359be2e9a8f25433d672d7a`  
**Live URL:** <https://git-forge-exit-drill.sociobot.in>  
**Verified:** 2026-08-28 from the candidate product tree.

## Release decision

**FAIL.** The core evidence-completeness claim is false for structurally invalid
but syntactically valid JSON. An export whose critical artifact files each
contain only `[null]` is reported as having captured issues, pull requests,
releases, workflows, and workflow runs. The resulting encrypted archive also
verifies successfully. This can give a team false confidence that history was
preserved before cutover.

## Mandatory first read and demo

PASS. A cold live Chromium visit showed:

- **What:** “Test your GitHub exit before cutover.”
- **For whom:** “For small teams moving forges…”
- **First action:** **Try it with sample data**, beside “See a complete drill
  with no setup.”

One click opened `/demo` and immediately showed the sample CLI result. The
persistent banner said “Demo — sample data, nothing is saved” and exposed
**Reset demo** and **Start for real**. Reset recreated only
`demo:gfed:started`; leaving demo removed it.

## Required claims

`.factory/claims.json` exists with nine entries. After the lockfile install,
every exact listed command passed independently through the demo entry point.
Each `@claim:<id>` tag occurs exactly once.

| Claim | Exact command | Result |
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

The green `evidence-complete` test covers malformed JSON and mismatched or
manifest-only counts, but it does not establish that array members are valid
records. The independent boundary case below disproves both that registered
claim and the live sentence “It marks a count captured only when valid records
back it.”

## Release-blocking defect

### Critical — arbitrary JSON array values are counted as captured artifacts

I created a local export with a valid Git mirror and a manifest declaring one
issue, pull request, release, workflow, and workflow run. Each corresponding
JSON file contained only:

```json
[null]
```

The packaged CLI exited 0. `readiness.json` reported all five artifacts as
`captured: true`, `count: 1`, and left `incomplete` empty. Issues and releases
were labeled `mapped`; pull requests and workflows were labeled
`restore test required`; the workflow run was labeled `target gap`. Archive
verification then returned `valid: true` with 28 evidence files. Source hashes
before and after the drill matched.

The parser in `src/lib.rs` counts the length of any JSON array without
validating its members against the selected artifact type. A `null`, number,
or unrelated object is therefore treated as an exported record. The same path
also accepts objects with only a generic `id`, without proving issue author
attribution or the fields needed to restore that artifact.

This is not only a reporting nuance. Issues, pull requests, releases, and build
history are critical acceptance artifacts in the researched brief. A matching
manifest count makes invalid content look complete, recreating the earlier
false-positive failure at the record-validation boundary.

Required remediation: validate every record for the artifact format, reject
or mark mixed/invalid collections as incomplete, and count only validated
records. Add claim cases for `null`, scalars, unrelated objects, missing
identity/author fields, and mixtures of valid and invalid records.

## Passing local and CLI evidence

- `npm ci`: PASS — 22 packages installed; 0 audit vulnerabilities.
- `npm test`: PASS — 3 Rust unit tests, 11 CLI integration tests, and 20
  Playwright tests.
- `cargo fmt -- --check`: PASS.
- `cargo clippy --all-targets -- -D warnings`: PASS.
- Explicit TypeScript `tsc --noEmit` check: PASS.
- Exact `npm run build`: PASS; it produced `dist/site/` and the release binary.
- `cargo package --locked --allow-dirty`: PASS, including Cargo's package
  verification build.
- A clean unpacked consumer install passed `--help`, `--version`, JSON
  capabilities, `demo --output`, and archive verification. Its demo archive
  verified 29 evidence files.
- Missing and empty sources, a source path that is a file, a short passphrase,
  an unknown target, a wrong archive passphrase, a tampered archive, and a file
  over 25 MB all failed with exit code 1 and actionable messages.
- A non-empty demo output directory was refused and its sentinel remained.
- A newly initialized empty bare repository was not reported as captured.
- A manifest declaring 999 issues with an empty issue array was reported as
  incomplete evidence. The Team Pack's 11-source boundary is covered and
  rejects before contacting billing or writing a portfolio.

## Live deployment, privacy, and quality evidence

The live product matches the candidate production build byte for byte:

| Artifact | SHA-256 |
| --- | --- |
| `index.html` | `7d0d78e47fa73e0a8bdce64ca69ced51d262cc05cf5c344b70974b1c70ce305e` |
| JavaScript | `f1d38950c17d52abd4da785734566f79cd012d23e145de2654466aa076871f30` |
| CSS | `6b4732f0b77867a4dd9c334d19f1da7738ea94040e0cec65df9a338219624bf9` |
| Linux x86-64 binary | `1163e1362f9df487817ef839cf4cbd1f656af79a0da43e6af0dc53dca9dbe4c6` |

- `/`, `/demo`, `/privacy`, and `/terms` returned 200 with route-specific
  titles, `lang=en`, one H1, and one main landmark. The designed unknown route
  returned HTTP 404.
- Fresh live contexts had no console or page errors on real routes. Axe found
  zero serious or critical violations on all four routes.
- At 390×844 there was no page overflow. Every measured link and button was at
  least 44×44 CSS pixels. The sample action was visible in the first viewport.
- Keyboard Tab reached the skip link first with a visible 3 px amber outline.
  Route changes and browser Back moved focus to the new H1. Reduced-motion
  mode reduced animation and transition durations to 0.01 ms.
- The factory `verify-url.sh` passed in 610 ms with title, language, one H1,
  main landmark, alt text, labels, and a clean console.
- A fresh demo/reset/exit request log contained only the product origin. The
  home page requested only same-origin HTML, JS, CSS, and original artwork.
- The browser received CSP with header-only `frame-ancestors 'none'`, HSTS,
  `nosniff`, strict-origin referrer policy, and a camera/microphone/geolocation
  permissions policy. HTML revalidates after 30 seconds; hashed JS and CSS use
  one-year immutable caching.
- JavaScript is 15,681 bytes raw / 5,685 bytes gzip; CSS is 12,198 bytes raw /
  3,447 bytes gzip; the hero is 61,388 bytes; no web fonts load.
- Lighthouse 12.8.2 mobile retry completed with Performance 98,
  Accessibility 100, Best Practices 100, and SEO 100. FCP was 0.8 s, LCP 1.2
  s, TBT 160 ms, CLS 0, and Speed Index 0.8 s.
- The service worker accepted an update check, controlled the client, and
  reloaded `/demo` offline without console or page errors.
- All real internal links and the binary download returned 200. The Team Pack
  link returned 303 to hosted Dodo checkout; the external Sociobot link
  returned 200.
- Live invalid-license verification returned a CORS-enabled, `no-store` 200
  response and the page showed “License no longer active.” A single client
  received 30 HTTP 200 verification responses; request 31 returned 429 with
  `Retry-After: 3`. Observed allowance: 30 requests per active rate window.
- Sign-in is not required. There is no product backend beyond the documented
  Sociobot checkout/license calls, so backend persistence and Entra checks are
  not applicable.

## Required next steps

1. Validate the schema and required restore fields of every recognized
   artifact record before counting it.
2. Mark any invalid or mixed collection as incomplete and explain the failing
   file and record.
3. Expand `@claim:evidence-complete` with structurally invalid JSON values and
   rerun every claim, package-consumer, live-identity, and accessibility check.
