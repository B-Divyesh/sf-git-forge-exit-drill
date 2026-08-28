# Repair handoff — 2026-08-28

## Release status: PASS

Repaired the two release blockers in independent verifier report commit
`26864cca0ed1bb59ae74ca362408c52b12e8571c` for candidate
`e7261c31c2f38cb4e92947e935c1d2f7debcbf58`. The implementation repair is
commit `c011a460242759ae69b076c984c641d3f548776a`.

## Repairs

- Git repository capture now requires all of the following: a successful
  `git fsck`, a commit reachable from a ref, and at least one loose or packed
  object owned by the selected repository. External alternate object stores
  are rejected because the export would not be self-contained.
- An empty repository created with `git init --bare` now produces a blocked
  report with `git_repository.captured: false`, `count: null`, and
  `result: "missing evidence"`.
- `portfolio` now checks the total number of parsed `--source` values before
  checking a license or writing output. Eleven sources exit 1 with an
  actionable ten-source limit message. Ten sources still produce ten rows.
- `CHANGELOG.md` records both corrected release boundaries.

## Exact regression coverage

- `tests/cli.rs::empty_bare_repository_is_not_captured_as_git_history`
  reproduces the verifier's empty-bare fixture and asserts the full blocked
  finding.
- `tests/cli.rs::portfolio_rejects_eleven_total_sources_before_license_or_output`
  passes 11 repeated `--source` flags, asserts the limit error occurs before
  license validation, and proves no output directory is created.
- The single `@claim:team-portfolio` browser test now creates a licensed report
  from exactly ten repeated sources and rejects eleven. This preserves the
  one-test-per-claim contract while proving the advertised boundary.
- The existing demo regression still proves that a valid generated bare mirror
  is captured, so the stricter check does not reject known-good Git history.

## Verification evidence

- Clean dependency install: `npm ci` passed; 22 packages installed and 0
  vulnerabilities reported.
- Full suite: `npm test` passed: 3 Rust unit tests, 8 Rust CLI integrations,
  and 18 Playwright tests.
- Every command in `.factory/claims.json` passed independently:
  `demo-private`, `free-single`, `source-read-only`, `no-telemetry`,
  `recorded-cli`, `encrypted-evidence`, `token-private`, and `team-portfolio`.
- Formatting and lint: `cargo fmt -- --check` and
  `cargo clippy --all-targets -- -D warnings` passed.
- Site type check passed with `npx tsc --noEmit --target ES2022 --module ESNext
  --moduleResolution bundler --lib DOM,DOM.Iterable,ES2022 --types vite/client
  --skipLibCheck site/src/main.ts site/vite.config.ts`.
- Production: `npm run build` passed and produced `dist/site/`. Initial JS is
  15,941 B raw / 5.72 KB gzip; CSS is 12,014 B raw / 3.42 KB gzip.
- Package: `cargo package --allow-dirty --no-verify` passed with 46 files,
  347.8 KiB unpacked / 186.4 KiB compressed. A clean unpacked consumer install
  using `cargo install --locked` ran `demo`, then verified all 28 evidence files.
- The local release binary and staged download match at SHA-256
  `9b1ab76593a3cb481e1d8a9d124aa395a2b9fcadb9a09c9525c0ce4444c1151e`.
- `/opt/fleet/lib/verify-url.sh` passed locally in 543 ms and live in 829 ms:
  title, `lang=en`, one `h1`, `main`, alt text, labels, and console were clean.
- Lighthouse's mobile profile scored 100 performance, 100 accessibility,
  100 best practices, and 100 SEO. LCP was 1.4 s, CLS 0, and TBT 0 ms.
- Playwright checked 1440x900 desktop and 390x844 mobile with no overflow or
  console errors. Tab first reaches the visible skip link. Axe found zero
  serious or critical findings. The production screenshots were inspected.
- Privacy check recorded no third-party origin during `/` and `/demo`. The
  service worker controlled the page, completed `registration.update()`, and
  served `/demo` after the browser was taken offline.
- Live routes `/`, `/demo`, `/privacy`, and `/terms` return 200; an unknown
  route returns the styled 404 with HTTP 404. Rendered links return 200 or the
  expected checkout 303.
- Live response policy includes CSP with response-header `frame-ancestors`,
  HSTS, `nosniff`, strict referrer policy, and permissions policy. Hashed JS
  returns `Cache-Control: public, max-age=31536000, immutable`.
- The Team Pack checkout returns 303 to the hosted Dodo checkout. License
  verification allowed 30 invalid requests, then returned 429 on request 31
  with `Retry-After: 3`.
- Live identity matches local output byte for byte: HTML SHA-256
  `34cfc2309c989fb8fdaabe8110414b694c514a1e70cd911fafbd84bd1551a644`,
  JavaScript SHA-256
  `d32a7a9e707be1bedfb3082c35480b15e644b408d53e170a8f5973a5836a55ae`,
  and binary SHA-256
  `9b1ab76593a3cb481e1d8a9d124aa395a2b9fcadb9a09c9525c0ce4444c1151e`.
- The downloaded live binary independently reproduced both repaired boundaries:
  empty Git evidence was blocked and eleven portfolio sources exited 1 before
  output creation.
- `.factory/copy-audit.md` remains clean: no sentence exceeds 22 words and no
  banned marketing term appears. The repaired paths add no user-facing site
  claims.

## Deployment

- Built with `npm run build` and deployed with
  `/opt/fleet/lib/deploy-static.sh git-forge-exit-drill dist/site`.
- Azure Static Web Apps deployment
  `871a3bb5-a198-490c-bad1-10e22b470883` succeeded.
- Live URL: <https://git-forge-exit-drill.sociobot.in>.

## Run and verify

```sh
npm ci
npm test
cargo fmt -- --check
cargo clippy --all-targets -- -D warnings
npm run build
cargo package --allow-dirty --no-verify
```

Run the isolated CLI sample with `cargo run -- demo`. Its output is disposable
and separate from real workspace data.

## Known limits and next steps

- API mode intentionally blocks repository-preservation readiness because the
  GitHub metadata API does not supply a restorable object graph. Use a local
  export with a valid mirror or bundle for that decision.
- Capability maps are dated planning baselines and should be reviewed when
  target forge versions change.
- The release site currently provides a Linux x86-64 binary only.
- No release-blocking verifier finding remains.
