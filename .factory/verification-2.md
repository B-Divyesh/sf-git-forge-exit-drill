# Independent verification 2 — FAIL

**Candidate:** `e7261c31c2f38cb4e92947e935c1d2f7debcbf58`  
**Live URL:** <https://git-forge-exit-drill.sociobot.in>  
**Verified:** 2026-08-28 from a fresh detached clone at the candidate SHA.

## Release decision

**FAIL.** The CLI reports a Git repository as captured when the selected export contains an empty bare repository with no Git objects or refs. This is a false statement about the very evidence the product is meant to validate. The paid Team Pack also accepts 11 repositories while the site, README, and terms promise a maximum of ten.

## Cold first read and demo

On a fresh live Chromium context, the first screen said: “Test your GitHub exit before cutover.” It said it is for “small teams moving forges” and that it finds missing history and build evidence before Monday. The first primary action was **Try it with sample data**, with the adjacent explanation “See a complete drill with no setup.” One click opened `/demo`, showed “See a complete exit drill,” the persistent “Demo — sample data, nothing is saved” banner, Reset demo, and Start for real. This mandatory first-read/demo check passes.

## Required claims

`.factory/claims.json` exists and contains eight entries. After `npm ci` in the fresh clone, every declared command passed independently (each also ran Rust tests and rebuilt the demo entry point):

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

The registered tests do not cover either boundary defect below.

## Quality and deployment evidence

- `npm ci`: PASS — 22 packages installed; audit reported 0 vulnerabilities.
- `npm test`: PASS — 3 Rust unit tests, 6 Rust CLI integrations, 18 Playwright tests.
- `cargo clippy --all-targets -- -D warnings`: PASS.
- `npm run build`: PASS — release binary and `dist/site/` produced.
- `cargo package --allow-dirty --no-verify`: PASS — 45 files, 182.7 KiB compressed. A clean unpacked consumer was installed with `cargo install --locked`; its installed binary ran `demo`, then verified its archive successfully (28 evidence files).
- Manual CLI checks: the bundled demo created its reports and AES-GCM archive; correct-passphrase verification passed; a wrong passphrase, nonexistent source, empty export, and unmapped target each exited non-zero with an actionable message. `demo --output` refused a non-empty directory and preserved its sentinel file. A metadata-only manifest correctly reported Git repository evidence missing.
- Candidate/live identity: local `dist/site/index.html` and live `/` are byte-identical, as are the JavaScript assets (`SHA-256 d32a7a9e707be1bedfb3082c35480b15e644b408d53e170a8f5973a5836a55ae`). The local release binary, staged download, and live download are byte-identical (`SHA-256 d5a961b1e3908f975c7bb04ca2feffa00fbed41fbc92506b10da4547ce7307fe`).
- Live browser checks: desktop and 390x844 mobile had no overflow, console errors, or page errors. Tab first reached the skip link with an amber 3px visible focus outline. The reduced-motion media query was honored (all animation/transition durations were reduced to `0.00001s`). Axe found zero serious/critical findings on `/`, `/demo`, `/privacy`, and `/terms`. `/opt/fleet/lib/verify-url.sh` against the local production preview passed: title, `lang=en`, one `h1`, `main`, zero images missing alt, zero unlabeled buttons, zero errors; load measurement 580 ms.
- PWA: after the initial live `/demo` visit, a controlled client reloaded `/demo` while offline and rendered the demo heading without errors. The deployed worker uses `skipWaiting`, `clients.claim`, and refreshes its `gfed-shell-v1` cache on installation; a real old-to-new production deploy was not available to simulate.
- Privacy/network: in fresh live contexts, `/` requested only same-origin document, JS, CSS, and product image; `/demo` requested only same-origin document/JS/CSS and created only `demo:gfed:started`. No third-party telemetry occurred. Response headers include CSP with `frame-ancestors`, HSTS, `nosniff`, strict referrer policy, and permissions policy. Hashed JS is 15,941 B raw / 5,720 B gzip, CSS 12,014 B raw / 3,420 B gzip, and JS assets have one-year immutable caching. The Team Pack checkout returned HTTP 303 to hosted Dodo; all rendered links resolved successfully.
- Unlock endpoint allowance: a single client received HTTP 200 for 30 sequential invalid-license verification requests. Request 31 returned HTTP 429 with `Retry-After: 2` and `x-ratelimit-after: 2`. Observed allowance: 30 requests before a two-second backoff.
- Live routing: `/not-a-route` returned HTTP 404 with the styled recovery page.

## Defects

### Critical — empty Git repository is treated as preserved repository history

The validator’s `validate_git_dir` accepts `git fsck` success as proof of repository object evidence. A freshly created bare repository has `HEAD`, `objects/`, and `refs/`, but has no refs and no object files; `git fsck` exits 0 while noting an unborn HEAD and no default references.

Independent reproduction:

```sh
mkdir source
printf '%s' '{"repository":"acme/empty-repository","artifacts":{"git_repository":1,"issues":1}}' > source/manifest.json
printf '[]' > source/issues.json
git init --bare --quiet source/empty.git
GFED_PASSPHRASE='a long enough test passphrase' \
  git-forge-exit-drill --json drill --source source --target forgejo:9.0 --output output
```

`find source/empty.git/objects -type f` produced no files, yet `output/readiness.json` contained:

```json
{
  "artifact": "git_repository",
  "captured": true,
  "count": 1,
  "target_support": "native",
  "result": "mapped"
}
```

This is a false-positive preservation result for a repository with no history to restore. Require at least one validated ref reachable from actual object bytes (and add a regression test using `git init --bare`) before marking Git repository evidence captured.

### Medium — Team Pack does not enforce its ten-repository limit

The landing page, README, and terms say “Up to ten local exports per run.” Using a local recorded valid Sociobot license response, `portfolio` accepted 11 repeated `--source examples/atlas-notes-export` arguments and wrote a portfolio with 11 repository rows. `#[arg(num_args = 1..=10)]` limits values on one occurrence, not the accumulated repeated `--source` occurrences. Enforce `sources.len() <= 10` after parsing and add a test for 11 sources.

## Required remediation

1. Do not report Git repository capture for an empty object database; validate reachable refs/commits and object bytes, then add the empty-bare-repo regression case.
2. Enforce the Team Pack maximum of ten total sources and test the 11-source rejection.
3. Rerun every claim command plus the full production, package-consumer, live, and boundary verification after repair.
