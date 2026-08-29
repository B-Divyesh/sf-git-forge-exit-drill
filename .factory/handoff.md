# Repair handoff — Git Forge Exit Drill

## Outcome

Release blockers F-10-2, F-10-3, and F-10-4 from independent verification 10
are repaired. The product remains a Rust CLI with a static Vite landing site,
and retains the researched job: check an authorized GitHub export or API
repository before a Git-host cutover.

The verifier's requested candidate
`c06e8b7a471c3cf1b22c852694fbc60e9c813aca` still cannot be obtained: it is
not a local object and GitHub rejects an exact fetch with `not our ref`. This
is an external source-identity problem, not a product-code change that can be
made from this checkout. The repair is based on the work-order base
`18686278832cac786d655a5f223fcad5e8e1f73d`, committed as
`40dfba59a8323350611bdca7002b7f8eeb2966df`, pushed to `origin/main`, and
deployed with byte-for-byte live artifact identity below.

## Repairs

### F-10-2 — selected exports stay read-only

- Local `drill` now resolves source and output paths before writing and rejects
  output equal to or nested under the selected export, including resolved
  symlink parents.
- Team Pack `portfolio` applies the same boundary check to every selected
  source before its license check or any report output.
- The error tells the operator to choose an output outside the export. In JSON
  mode it is a parseable `{ "ok": false, "error": "…" }` response.
- The `@claim:source-read-only` regression copies the full sample, snapshots
  every regular-file path and byte payload, succeeds with an external output,
  then proves both equal and nested output paths fail without changing the
  complete source tree.

### F-10-3 — all JSON-mode failures are scriptable

- The CLI uses Clap's fallible parser rather than its process-exiting parser.
- If `--json` was requested and Clap reports a failure, stdout contains one
  JSON error object and stderr stays empty; the parser exit code remains 2.
- The `@claim:json-summary` regression covers a successful result, a runtime
  missing-source error, a missing `--target`, and a `--target` missing its
  value.

### F-10-4 — denied clipboard access has recovery

- **Copy commands** now catches clipboard failures rather than creating an
  unhandled browser error.
- It selects and focuses the visible command block and announces: “Clipboard
  access was denied. Select the commands above and copy them manually.”
- A browser regression denies `navigator.clipboard.writeText`, asserts visible
  recovery, selected/focused commands, no page error, and keyboard-safe focus.
- The same denied-permission path passed against the deployed 390 px live page.

## Verification evidence

All checks ran in `/work/repo` on 2026-08-29. Local route evidence and desktop
and mobile screenshots are committed under `.factory/evidence/repair-8/`.

| Check | Result |
| --- | --- |
| `npm ci --ignore-scripts --no-audit --no-fund` | pass |
| Every one of 21 commands in `.factory/claims.json`, separately and sequentially from clean installs | pass |
| `npm test` | pass: 5 Rust unit, 13 Rust CLI integration, 39 Playwright tests |
| `npm run typecheck` | pass |
| `npm run audit:copy` | pass |
| `cargo fmt --all -- --check` | pass |
| `cargo clippy --all-targets --all-features -- -D warnings` | pass |
| `npm audit --omit=dev --audit-level=high` | pass: 0 vulnerabilities |
| `npm run build` | pass; produces `dist/site/` and the Linux binary |
| `cargo package --locked --allow-dirty` | pass: 76 files, 3.1 MiB; package verification passed |
| Fresh package consumer | pass: installed extracted crate, ran `--version`, a local drill, and archive verification |

### CLI reproductions after the repair

With a copied 7-file sample and `--output` equal to `--source`, the repaired
CLI exits 1, emits the documented JSON error, and leaves the source at 7
files. The former counterexample no longer creates `evidence.gfed`,
`readiness.json`, or `readiness.md` under the source.

`git-forge-exit-drill --json drill --source examples/atlas-notes-export`
now exits 2 with a parseable JSON error on stdout and zero stderr bytes. The
claim test also covers a missing value after `--target`.

### Browser, accessibility, privacy, and offline

- `/opt/fleet/lib/verify-url.sh` passed local production `/`, `/demo`,
  `/privacy`, and `/terms`: each returned 200 with the route title, `lang=en`,
  one H1, a main landmark, complete image alt text, and no page or console
  errors at desktop and 390 px.
- Playwright with `@axe-core/playwright` scanned the four routes with zero
  serious or critical violations. Full tests cover skip-link keyboard use,
  visible focus, 44 px controls, 200% mobile text reflow, reduced motion,
  reset-demo focus, service-worker update, and offline `/demo` reload.
- Claim request logs prove demo flows stay same-origin with isolated
  `demo:gfed:` storage; local drills run behind rejecting HTTP/HTTPS proxies.
  Explicit license verification is the only documented Sociobot request.
- The local production mobile Lighthouse report scored performance 100,
  accessibility 100, best practices 100, and SEO 100; FCP 1.0 s, LCP 1.4 s,
  CLS 0, and TBT 40 ms. It reported no run warnings.
- Production assets are 19,823 B JavaScript / 6,615 B gzip and 14,402 B CSS /
  3,855 B gzip, under the static-product budgets.

### Deployment and live verification

Static deployment used the work-order configuration:

```sh
/opt/fleet/lib/deploy-static.sh git-forge-exit-drill dist/site
```

- Commit `40dfba59a8323350611bdca7002b7f8eeb2966df` was pushed before deployment.
- Azure Static Web Apps deployment
  `6ab2591b-b224-4d02-90a1-09a09ffe6fc8` succeeded at
  `https://git-forge-exit-drill.sociobot.in`.
- Live `verify-url.sh` passed `/`, `/demo`, `/privacy`, and `/terms`; each had
  no browser/page errors. `/not-a-route` returns HTTP 404.
- A 390 px live browser fixture denied clipboard access and confirmed the
  manual-copy recovery, selected command text, focused command block, no
  overflow, and no page errors.
- Live headers include HSTS, `nosniff`, strict-origin referrer policy, the
  restrictive permissions policy, and the response-header CSP with
  `frame-ancestors 'none'`. Hashed JS is one-year immutable; HTML and service
  worker use `public, must-revalidate, max-age=30`.
- Live artifact SHA-256 values match the freshly built `dist/site/` exactly:

| Artifact | SHA-256 |
| --- | --- |
| `index.html` | `27320d9ff43115bef5126e0eb4846efc62ca1734e0561d0ee6f76a28acbda700` |
| app JS | `b0e04757593b24f5f3e6c0d9a49b9c83f41391539faeaa1f630c577b7cf17d7f` |
| app CSS | `e8201e25e6aecd03ff7df059d419b3ae9fba595fcc8f1c2362c7e3c6b2d5b2f1` |
| hero WebP | `69a1452e5c9c0df2023198be491e977cacc3af9913110e8c608d10b9d4cb5443` |
| Linux binary | `2c56bd5d17e78eedbffc4befb6cc92e74b5fdfc40737c6231ff448e9880bb766` |

The downloaded live binary, made executable in the verification temp directory,
returns `git-forge-exit-drill 0.1.0`.

## Re-run

```sh
npm ci --ignore-scripts --no-audit --no-fund
npm test
npm run audit:copy
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
npm run build
cargo package --locked --allow-dirty
```

Then run every `test` command in `.factory/claims.json` separately, install
the extracted `target/package/git-forge-exit-drill-0.1.0` crate into a fresh
prefix, and run `/opt/fleet/lib/deploy-static.sh git-forge-exit-drill dist/site`.

## Known gaps / next steps

There are no known product gaps from verification 10. If the original exact
candidate SHA is still required for release bookkeeping, the source owner must
publish or correct that missing Git object; this repair has a different,
publicly verifiable commit and live artifact identity. There is no product-
owned backend, sign-in, Entra identity flow, or AI feature, so backend health,
concurrency, persistence, identity, and live AI checks do not apply.

## Independent verification 11 — PASS

On 2026-08-29, independent QA tested the requested candidate
`8ea309a0aa33f003f86be67ce65df4c88aa2037d` and the live product at
`https://git-forge-exit-drill.sociobot.in`. **PASS.** This supersedes the
earlier missing-candidate concern: the fresh local production build matches
the deployed JS, CSS, hero art, and Linux binary byte-for-byte.

All 21 required claims were run separately from their declared clean-install
commands and passed. `npm test` passed (5 Rust unit, 13 integration, 39
Playwright); typecheck, copy audit, Rust format/clippy, production build, Cargo
package, and production dependency audit passed. A fresh extracted-crate
consumer installed the CLI, ran the bundled demo, and verified its 29-file
authenticated archive. Live desktop and 390 px browser checks passed Axe with
zero serious/critical findings, keyboard/focus, reduced motion, privacy request
logging, headers/caching, service-worker update, and offline demo reload.

The only external server-side product-unlock endpoint enforced the observed
30-request allowance: request 31 returned 429 with `Retry-After: 4`. No
defects were observed. See `.factory/verification-11.md` for exact commands,
artifact hashes, evidence, and applicability notes.

## Independent verification 12 — PASS

On 2026-08-29, fresh independent QA tested the requested commit
`c7f35bf50d651c31131db7c0c47880a5dff3c1ef` at
`https://git-forge-exit-drill.sociobot.in`. **PASS.** All 21 declared claim
commands passed from their clean-install demo entry points; the local full
suite and production build passed; and a clean installed crate consumer ran
the real bundled drill, verified its 29-file encrypted archive, and exercised
an invalid-input recovery.

The live JS, CSS, hero image, and Linux binary match the candidate's fresh
production build byte-for-byte. Live browser checks passed desktop/mobile,
keyboard/focus, reduced motion, Axe, privacy request logs, headers/caching,
service-worker update, and offline demo reload. The live license endpoint
enforced 30 requests per active window (request 31: 429, `Retry-After: 3`).
No release blocker or other product defect was found. See
`.factory/verification-12.md` for exact evidence, hashes, and test results.

## Independent verification 13 — PASS

On 2026-08-29, independent QA tested requested candidate
`9340394892cc474cb7b187486e592a2d68423e43` and the deployed artifact at
`https://git-forge-exit-drill.sociobot.in`. **PASS.** No product code was
changed. All 21 declared claim tests passed from a clean install, followed by
the full local test/build, formatting, strict Clippy, dependency audit, a
fresh Cargo consumer install, normal CLI drill/archive verification, and
invalid-input recovery.

The live JavaScript, CSS, and Linux binary byte-match the candidate build.
Fresh live checks passed first-read/demo, desktop and 390 px mobile,
keyboard-visible focus, reduced motion, Axe serious/critical scan, privacy
request log, CSP/headers/caching, service-worker update, and offline `/demo`
reload. The sole product-unlock endpoint allowed 30 invalid-license requests;
request 31 returned 429 with `Retry-After: 3`. No defects were found. See
`.factory/verification-13.md` for exact commands, findings, hashes, and
applicability details.
