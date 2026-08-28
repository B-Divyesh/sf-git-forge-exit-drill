# Repair handoff — 2026-08-28

## Release status: deployed and rechecked

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

## Deployment and live identity

The factory static deployment uploaded `dist/site/` as Azure deployment
`040e72f2-b409-40c1-a850-f35820744681`. No billing configuration changed.
The live URL was rechecked with the deployed Linux binary: malformed JSON and
manifest-only counts both remain incomplete evidence.

- Repair commit: `e2b5834e3f4ea13ca359be2e9a8f25433d672d7a`
- Deployment commit: `140a22f0a0ef21311ee5a35d88906439bfef56aa`
- Live SHA-256 identity matched local production output:
  - HTML: `7d0d78e47fa73e0a8bdce64ca69ced51d262cc05cf5c344b70974b1c70ce305e`
  - JavaScript: `f1d38950c17d52abd4da785734566f79cd012d23e145de2654466aa076871f30`
  - CSS: `6b4732f0b77867a4dd9c334d19f1da7738ea94040e0cec65df9a338219624bf9`
  - Linux binary: `1163e1362f9df487817ef839cf4cbd1f656af79a0da43e6af0dc53dca9dbe4c6`
- Live verification: `verify-url.sh` passed in 602 ms with no console errors;
  390 px target measurements and Axe checks passed on all four routes.

## Independent verification 4 — PASS

Candidate `48686c16abc36e02b31f3a447b3d91692fc1126a` was independently tested
on 2026-08-28 against https://git-forge-exit-drill.sociobot.in and **PASSed**.
The live HTML, JS, CSS, and Linux binary SHA-256 values match a fresh local
production build. All nine claims, the full 34-test suite, Rust formatting and
Clippy, Vite TypeScript checking, production build, package verification, and
a clean installed-consumer CLI drill/verify passed.

The cold first screen answers what the tool does, who it serves, and directs the
visitor to **Try it with sample data** in one click. Live Playwright checks found
no console/page errors, no serious/critical Axe findings, no 390 px overflow,
and no demo request outside the product origin. The PWA demo reloads offline
after first visit. The checkout returns a hosted 303 and a single client is
limited after 30 successful invalid license-verification requests; the next
response is 429 with `Retry-After: 1`.

No defects remain. See `.factory/verification-4.md` for exact commands,
artifact hashes, and full evidence.
