# Independent verification 10 — FAIL

**Requested candidate:** `c06e8b7a471c3cf1b22c852694fbc60e9c813aca`  
**Available checkout tested:** `c06e8b31bb3046de8f79623972fd752c6e0a09e8`  
**Live URL:** <https://git-forge-exit-drill.sociobot.in>  
**Verified:** 2026-08-29 from the clean work-order checkout.

## Release decision

**FAIL.** The requested candidate commit is not available locally or from any
advertised ref on the stated GitHub remote, so its contents cannot be tested
or matched to production. GitHub rejected an exact-SHA fetch with `not our
ref`; `origin/main` and the only advertised commit are `c06e8b31...`.

Fresh verification of that available base and its byte-identical live build
also found two release-blocking claim gaps:

1. A valid local drill can write its output directly into the selected source
   export, contradicting the published read-only-source promise.
2. `--json` does not return JSON for command-line argument errors, contradicting
   its scripting promise.

The install-command copy action also has no recovery when browser clipboard
permission is denied and produces an uncaught page error.

## Mandatory claims gate

`.factory/claims.json` exists, parses, and contains 21 unique IDs. Before
general QA, every listed `test` command was run separately and sequentially,
including its declared `npm ci --ignore-scripts --no-audit --no-fund` step.
All registered tests passed.

| Claim | Registered test |
| --- | --- |
| `demo-private` | PASS |
| `free-single` | PASS |
| `source-read-only` | PASS — incomplete coverage; see F-10-2 |
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
| `json-summary` | PASS — incomplete coverage; see F-10-3 |
| `actionable-errors` | PASS |
| `cli-network-boundaries` | PASS |
| `license-browser-storage` | PASS |

The registry has exactly one matching `@claim:<id>` test per ID, with no
missing, duplicate, or extra claim tags. The page and README copy audit found
no additional material promise outside the registry. Passing the registered
tests does not override the independently reproduced counterexamples below.

## Cold first read and one-click demo

The mandatory first-read gate passes at 1440 x 900 and 390 x 844:

- What: **Test your GitHub move before cutover**.
- For whom: small teams changing Git hosts.
- First action: **Try it with sample data**, beside “See a complete drill with
  no setup.”

One click opens `/demo`, showing Atlas Notes, Forgejo 9.0, a blocked outcome,
specific risks, and restore steps. The persistent banner says “Demo — sample
data, nothing is saved” and includes **Reset demo** and **Start for real**.
Only `demo:gfed:started` is written; reset restores keyboard focus, and leaving
the demo removes that key.

## Findings

### F-10-1 — Release blocker — requested candidate cannot be obtained or identified

`git fetch origin c06e8b7a471c3cf1b22c852694fbc60e9c813aca`
returns:

```text
fatal: remote error: upload-pack: not our ref c06e8b7a471c3cf1b22c852694fbc60e9c813aca
```

`git ls-remote origin` advertises only `HEAD` and `refs/heads/main`, both at
`c06e8b31bb3046de8f79623972fd752c6e0a09e8`. No local object or remote ref
matches `c06e8b7...`. The requested candidate therefore cannot be reviewed or
proven live.

### F-10-2 — Major — a valid output path mutates the selected source export

The landing page says the CLI “does not change your selected export,” and the
registered `source-read-only` claim says it reads the selected export without
changing it. The test checks one pre-existing file only and does not check the
source tree for new files.

Fresh reproduction with the clean installed package:

```sh
cp -a examples/atlas-notes-export /tmp/gfed-overlap/source
GFED_PASSPHRASE=123456789012 git-forge-exit-drill --json drill \
  --source /tmp/gfed-overlap/source \
  --target forgejo:9.0 \
  --output /tmp/gfed-overlap/source
```

Observed: exit 0; the source changed from 7 to 10 regular files. The command
added `evidence.gfed`, `readiness.json`, and `readiness.md` inside the selected
export. `--output` equal to or nested below `--source` should be rejected before
writing, or the read-only claim must be narrowed everywhere.

### F-10-3 — Major — `--json` is not JSON for argument-validation errors

The registered claim says `--json` prints a parseable summary or error. A
representative invalid invocation fails before the application's JSON error
handler:

```sh
git-forge-exit-drill --json drill --source examples/atlas-notes-export
```

Observed: exit 2, zero stdout bytes, and human-formatted Clap help on stderr:

```text
error: the following required arguments were not provided:
  --target <TARGET>
```

This breaks scripts that select `--json` specifically to parse all outcomes.
The registered test covers a runtime missing-source error but not parser
errors. Handle Clap failures as JSON when `--json` is present, or narrow the
published claim.

### F-10-4 — Moderate — denied clipboard access has no fallback

In a fresh live Chromium context without clipboard permission, clicking
**Copy commands** leaves the button text unchanged and raises:

```text
Failed to execute 'writeText' on 'Clipboard': Write permission denied.
```

There is no visible explanation or selectable fallback, and the error is an
uncaught page error. The license-copy controls already implement a suitable
catch-and-select fallback; the install copy action needs equivalent recovery.

## Clean repository, build, and package evidence

All checks below ran against the available clean commit `c06e8b31...`:

| Check | Result |
| --- | --- |
| Every one of 21 claim commands | PASS |
| `npm test` | PASS: 5 Rust unit, 13 Rust integration, 38 Playwright |
| `npm run typecheck` | PASS |
| `npm run audit:copy` | PASS |
| `cargo fmt --all -- --check` | PASS |
| `cargo clippy --all-targets --all-features -- -D warnings` | PASS |
| `npm audit --omit=dev --audit-level=high` | PASS: 0 vulnerabilities |
| Exact `npm run build` | PASS; populated `dist/site/` and Linux binary |
| `cargo package --locked --allow-dirty` | PASS: 75 files, 3.1 MiB; verification build passed |
| Fresh package consumer | PASS: installed extracted crate; version `0.1.0` |

The clean consumer ran `--json demo`, a normal GitLab 17.0 local drill behind
rejecting HTTP/HTTPS proxies, and archive verification. The drill preserved all
pre-existing source hashes, produced a 29-file authenticated archive and both
reports, hid known source text from a raw archive scan, and generated seven
restore steps. Correct passphrase verification passed.

Independent boundary cases for an 11-character passphrase, missing source,
source-as-file, unknown target, wrong archive password, and non-empty demo
output all exited non-zero with an actionable error. The non-empty demo output
preserved its sentinel file.

## Live browser, accessibility, privacy, and PWA

- `/opt/fleet/lib/verify-url.sh` passed live `/`, `/demo`, `/privacy`, and
  `/terms`: 200, correct titles, `lang=en`, one H1, one main, complete alt text,
  and no console/page errors.
- Playwright/Axe scanned those four routes plus the designed 404 at 1440 x 900
  and 390 x 844. Successful routes had zero serious/critical findings, no
  horizontal overflow, no missing alt text, and no sub-44 px visible controls.
- Keyboard-only checks reached the skip link first; Enter focused the H1. The
  primary demo action, Reset demo, Start for real, and back-navigation focus
  paths worked. The focus indicator is a visible 3 px amber outline.
- The full suite passed 200% text reflow. Reduced-motion media matched and
  reduced animation/transition durations to `0.00001s`.
- Cold home and the complete demo flow requested only the product origin. A
  license callback requested only the documented Sociobot API, stripped the
  token from the URL, cached an invalid verdict, and removed both stored items
  from `/privacy`. No analytics, CDN font, or third-party script request was
  observed.
- The service worker was active, accepted `registration.update()`, and reloaded
  `/demo` offline with its heading and blocked sample result intact.
- Internal pages, assets, icons, manifest, service worker, sitemap, robots file,
  and Linux download returned their expected 200 responses. Checkout returned
  the expected 303 to hosted Dodo. Unknown routes returned the designed 404.

Live response headers include HSTS, `nosniff`, strict-origin referrer policy,
a restrictive permissions policy, and a CSP whose response-header
`frame-ancestors 'none'` is correct. HTML and the service worker use
`public, must-revalidate, max-age=30`; hashed JS/CSS use one-year immutable
caching; WebP art uses one day.

## Performance and budgets

| Measure | Observed | Budget/result |
| --- | ---: | --- |
| JavaScript | 19,234 B raw / 6,446 B gzip | PASS, under 200 KB |
| CSS | 14,245 B raw / 3,808 B gzip | PASS, under 50 KB |
| Hero WebP | 61,388 B | PASS, under 300 KB |
| Fonts | 0 B | PASS |
| Lighthouse mobile performance | 92 | PASS, at least 90 |
| Lighthouse accessibility / best practices / SEO | 100 / 100 / 100 | PASS |
| FCP / LCP / CLS | 0.89 s / 1.30 s / 0 | PASS |
| Total transferred page weight | 73,428 B | PASS |

The live Lighthouse run had no warnings. Lab TBT was 338 ms; field INP is not
available from a cold synthetic run.

## Deployment identity

A fresh build of the available base matches production byte for byte, but this
does not establish identity for the missing requested candidate.

| Artifact | SHA-256 | Result |
| --- | --- | --- |
| `index.html` | `21cadaf9a043c45ffee32a5012f571dfcc3ad8a24698f63f12f10884f6ee56cf` | MATCH |
| app JS | `2c219ad028a15b625bc9ec0762eb50dfd227aa963550dcc9da92ef7fdc1863c5` | MATCH |
| app CSS | `80ea8bd57671050a034f37c4dac77b8e87bbc1ddee96ec5aa02ca050db6f2832` | MATCH |
| hero WebP | `69a1452e5c9c0df2023198be491e977cacc3af9913110e8c608d10b9d4cb5443` | MATCH |
| Linux binary | `11d47f861d1d1a7a627d3cb2b0e74bedb034d21f1a20b6428b074f3ce6269cc4` | MATCH |

The demo, privacy, terms, and 404 HTML also matched. The live binary executes
as `git-forge-exit-drill 0.1.0`.

## Billing rate limit and non-applicable checks

Fresh calls from one client to the live license-verification endpoint returned
200 for requests 1–30. Request 31 returned 429 with `Retry-After: 3` and
`X-RateLimit-After: 3`. Observed allowance: **30 requests per active window**.

The product has no sign-in or product-owned backend, so Entra authority,
backend concurrency, server persistence, and health/build endpoints do not
apply. AI is not part of this deterministic evidence-validation job.

## Required before release

1. Make the exact candidate commit available and redeploy it with verifiable
   build identity.
2. Reject output paths equal to or nested under any selected source, and expand
   `source-read-only` to assert the entire source tree is unchanged.
3. Make every `--json` failure parseable, including Clap argument errors, and
   expand the claim test with missing/invalid argument cases.
4. Add denied-clipboard recovery and a regression test for **Copy commands**.
5. Re-run all claims, clean gates, package consumer, and live verification.
