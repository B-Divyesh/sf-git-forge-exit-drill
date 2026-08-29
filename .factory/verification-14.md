# Independent verification 14 — PASS

**Candidate tested:** `01faff4ecb69bcd0016cfad929b8755e75623105`

**Live URL:** <https://git-forge-exit-drill.sociobot.in>
**Verified:** 2026-08-29 from the supplied clean checkout.

## Acceptance verdict

**PASS.** The candidate performs the brief's real job end to end and the live
deployment matches it. The CLI inventories authorized local exports or API
metadata, rejects unsupported evidence as incomplete, validates real Git
object capture, writes authenticated encrypted evidence, maps all three
documented target versions, and emits actionable readiness and restore output.

No release-blocking, major, moderate, or minor product defect was found. No
product code was changed during this verification. The earlier reported
deployment-only concern does not reproduce.

## Mandatory opening gates

`.factory/claims.json` exists. Before broader inspection, I ran each of its 22
exact `test` commands separately and sequentially from the clean candidate
checkout. Every command exited 0:

`demo-private`, `free-single`, `source-read-only`, `no-telemetry`,
`recorded-cli`, `encrypted-evidence`, `evidence-complete`, `token-private`,
`team-portfolio`, `cli-demo-isolated`, `target-mappings`,
`forgejo-actions-history`, `restore-checklist`, `output-boundary`,
`linux-download`, `billing-contract`, `archive-file-completeness`,
`api-metadata-blocks-git`, `json-summary`, `actionable-errors`,
`cli-network-boundaries`, and `license-browser-storage`.

There are exactly 22 matching and unique `@claim:<id>` test tags. A manual
cross-check of the rendered site and README found no material unlisted claim.

Cold first read, before scrolling or interaction:

- What it does: **“Test your GitHub move before cutover.”**
- Who it is for: **“For small teams changing Git hosts…”**
- What to do first: **“Try it with sample data,”** followed by **“See a
  complete drill with no setup.”**

All three appeared in the initial 1440×900 and 390×844 viewports. The sample
action is one click from `/`; it opened `/demo` with a completed Atlas Notes
drill, a clear BLOCKED result, and the persistent “Demo — sample data, nothing
is saved” banner. Reset demo and Start for real were both present.

## Clean build and automated checks

| Check | Result |
| --- | --- |
| `npm ci --ignore-scripts --no-audit --no-fund` | PASS on every claim invocation |
| `npm test` | PASS: 5 Rust unit, 13 Rust CLI integration, 40 Playwright tests |
| `npm run typecheck` | PASS |
| `npm run audit:copy` | PASS |
| `npm run build` | PASS; created `dist/site/` and the Linux x86-64 binary |
| `cargo fmt --all -- --check` | PASS |
| `cargo clippy --all-targets --all-features -- -D warnings` | PASS |
| `npm audit --omit=dev --audit-level=high` | PASS: 0 vulnerabilities |

An additional exact-commit clone packaged successfully: 121 files, 10.8 MiB
unpacked, 10,201,831-byte `.crate`. I unpacked that package, installed it with
`cargo install --locked --path ...` into an empty consumer prefix, and ran the
installed binary. It reported version `0.1.0`, listed Forgejo 9.0, Gitea 1.22,
and GitLab 17.0, completed `demo`, and verified all 29 files in its generated
evidence archive.

## Independent CLI exercise

The packaged CLI's demo produced `readiness.md`, `readiness.json`, and
`evidence.gfed` in an explicitly empty temporary directory. It reported the
realistic Atlas Notes fixture as BLOCKED for Forgejo 9.0 and printed the
archive passphrase and output paths.

I then ran the copied demo export through all three supported target maps.
Each run generated all three output files, returned parseable JSON, and its
archive independently verified. The sample was truthfully BLOCKED with 3
mapped items, 6 missing-evidence items, 3 restore-test items, and 1 target gap.

Recovery and boundary probes behaved safely:

- Wrong archive passphrase: exit 1 with a passphrase/integrity recovery
  message.
- Non-empty demo destination: exit 1; the sentinel file remained unchanged.
- Missing source: JSON error, exit 1, no output created.
- Passphrase shorter than 12 characters: JSON error, exit 1, no output.
- Unknown target: JSON error naming all valid targets, exit 1, no output.
- Eleven portfolio sources: rejected before license or output with exit 1.
- A 25 MiB + 1 byte source file: rejected with the documented 25 MiB limit
  and no output directory.

The declared tests additionally cover valid and malformed records, manifest
count mismatches, empty Git repositories, output/source overlap, pagination,
token redaction, local no-network operation, ten-repository licensed
portfolio generation, and source-byte preservation.

## Live deployment and privacy

The live deployment is the candidate. Fresh-build and live SHA-256 hashes
matched exactly:

| Artifact | SHA-256 |
| --- | --- |
| JavaScript | `57ba35eee4c620d7e40ffb5920e66b2626e91ca8e1857b4f37f17cdd92112d1b` |
| CSS | `e8201e25e6aecd03ff7df059d419b3ae9fba595fcc8f1c2362c7e3c6b2d5b2f1` |
| Linux x86-64 binary | `3ed95a28b7332f2a096e1e60d9d75a4424fd236fa0ec444ab3b0c8601fa60772` |

The root, demo, privacy, terms, 404, service worker, manifest, robots, sitemap,
both images, icons, hashed assets, and binary also matched byte-for-byte. The
downloaded live binary executed and returned `git-forge-exit-drill 0.1.0`.

During fresh `/demo` browser runs, every outgoing request was same-origin: the
HTML plus the candidate JS and CSS. The root additionally requested only its
same-origin hero image. No telemetry, CDN font, analytics, or third-party
script request occurred. The only external runtime operations are explicit
GitHub API access and Sociobot checkout/license verification, as documented.

Responses include HSTS, `X-Content-Type-Options: nosniff`,
`Referrer-Policy: strict-origin-when-cross-origin`, a restrictive
Permissions-Policy, and a CSP delivered as a response header with
`frame-ancestors 'none'`. HTML and the service worker use 30-second
must-revalidate caching. Hashed JS/CSS use one-year immutable caching. No
`Set-Cookie` header was observed.

The live checkout returned HTTP 303 to hosted Dodo checkout, and the claim
test confirmed the Git Forge Exit Drill $39 one-time order. On the Sociobot
verify endpoint, requests 1–30 from one client returned 200; request 31
returned **429** with **`Retry-After: 2`**. Observed allowance: **30 requests
per active window**.

## Accessibility, responsive behavior, and resilience

- Desktop 1440×900 and mobile 390×844 had no horizontal overflow or
  application console/page errors on `/`, `/demo`, `/privacy`, or `/terms`.
- Fresh Axe scans at both widths found zero serious or critical findings on
  those four routes and the styled 404.
- Every route has `lang="en"`, one H1, one main landmark, a route-specific
  title, and complete image alt text. Heading order and bound form labels are
  valid.
- Keyboard Tab order begins with “Skip to main content.” Every sampled focus
  target had a visible 3 px amber outline. Enter activated the sample link;
  Reset demo retained focus; Start for real cleared demo storage and focused
  the install heading.
- Visible controls at 390 px met the 44×44 px target baseline. The 200% text
  test passed without document or pricing-heading clipping.
- Reduced-motion media matching was active, transitions/animations were
  reduced to effectively zero, and no infinite animation remained.
- The updated service worker was active with no waiting worker. After going
  offline, `/demo` reloaded with its title, H1, banner, and sample content.
- `/`, `/demo`, `/privacy`, `/terms`, the binary download, Sociobot homepage,
  and checkout link resolved. An unknown route returned the designed HTTP 404.

## Performance

Fresh production sizes were 19,755 B JavaScript (6,570 B gzip), 14,402 B CSS
(3,855 B gzip), and 61,388 B for the mobile hero image. They are below all
specified budgets.

A fresh Lighthouse 12.8.2 mobile run against the live root scored:

| Category / metric | Result |
| --- | ---: |
| Performance | 100 |
| Accessibility | 100 |
| Best practices | 100 |
| SEO | 100 |
| First Contentful Paint | 0.8 s |
| Largest Contentful Paint | 1.2 s |
| Total Blocking Time | 20 ms |
| Cumulative Layout Shift | 0 |

## Applicability and defects

This product has no product-owned backend, sign-in flow, or runtime AI
feature. Backend concurrency, persistence/health identity, and Entra checks
are therefore not applicable. The only server endpoint in product scope is
the Sociobot unlock endpoint, whose allowance was verified above.

Defects by severity: **none**.
