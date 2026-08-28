# Repair handoff — 2026-08-28

## Release status: ready for deployment

This repair addresses every finding in independent verification 3 for candidate
`76f3a5e82f66a0f1d66e1c56815cb4cdc463242a`.

## Fixed

- Local inventory now parses recognized JSON export records. A malformed file
  is reported as **incomplete evidence**, never as one captured record.
- `manifest.json` is now an expected-total check, not evidence. A declared
  count with no matching parseable records is incomplete evidence; critical
  artifacts block the report.
- Release records also derive release-asset counts from their actual `assets`
  arrays. The bundled Atlas Notes manifest and sample now agree: issues 2,
  pull requests 2, releases 1, release assets 2, workflows 1, and runs 1.
- Reports expose the evidence reason in JSON and Markdown. Team portfolios now
  include incomplete evidence in their evidence-gap total.
- The recorded site transcript contains only exact stable output lines from the
  bundled CLI demo. It no longer displays fabricated capture totals.
- Every link and button is at least 44 by 44 CSS pixels at 390 px, including
  the demo banner, header navigation, wordmark, and footer links.

## Regression coverage

- Rust CLI integrations reproduce both verifier failures: malformed
  `issues.json`, and a valid Git mirror with manifest-only totals. Both produce
  a blocked report with `captured: false` and `result: "incomplete evidence"`.
- The bundled sample test verifies every displayed captured count against its
  parsed evidence.
- New `@claim:evidence-complete` opens each generated archive and report for
  malformed JSON, manifest-only totals, and the sample export.
- A 390 px Playwright test measures every visible link and button on `/`,
  `/demo`, `/privacy`, and `/terms` against the 44 px baseline.

## Verification run locally

- Clean install: `npm ci` (22 packages, 0 vulnerabilities).
- Full suite: `npm test` passed: 3 Rust unit tests, 11 CLI integrations, and
  20 Playwright tests.
- All nine claim commands in `.factory/claims.json` passed independently,
  including `npm test -- --grep @claim:evidence-complete`.
- `cargo fmt -- --check`, `cargo clippy --all-targets -- -D warnings`, and the
  explicit TypeScript `tsc --noEmit` check passed.
- `npm run build` produced `dist/site/` and the release Linux binary.
- `cargo package --locked --allow-dirty` packaged and verified 48 files
  (191.3 KiB compressed). A clean installed consumer passed `--help`, JSON
  capabilities, `demo`, and archive `verify`.
- `/opt/fleet/lib/verify-url.sh` against the local production preview passed:
  HTTP 200, title, `lang=en`, one H1, `main`, image alt text, labeled buttons,
  and zero page errors. Playwright axe integration found no serious or critical
  issues on `/`, `/demo`, `/privacy`, or `/terms`; it also covers offline demo
  reload, desktop/390 px overflow, keyboard, privacy requests, and reduced
  motion.

`@axe-core/cli` and Lighthouse could not launch their Chrome-driver path in
this container despite the Playwright Chromium being installed. The equivalent
in-repo Playwright axe coverage passed; the previous independent live run
recorded Lighthouse 100/100/100/100. Re-run Lighthouse in the deploy worker
before final catalog acceptance.

## Reproduce the repaired boundary

```sh
npm ci
npm test -- --grep @claim:evidence-complete
cargo test --test cli malformed_recognized_json_is_incomplete_not_captured
cargo test --test cli manifest_totals_without_records_are_incomplete_not_captured
npm run build
```

## Deployment and known gaps

Push `main`; the static deployment is produced from `dist/site/` by the
factory. No infrastructure, DNS, or billing configuration was changed. The
post-push live URL, artifact hashes, and deployment commit are recorded below
after the deployment check.

- Repair commit: `e2b5834e3f4ea13ca359be2e9a8f25433d672d7a`
- Deployment commit: pending
- Live identity: pending
