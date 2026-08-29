# Polish 2 handoff — Git Forge Exit Drill

## Outcome

Polish 2 repairs every finding in `review-1.md` and `review-2.md`. The full
closure map is `.factory/polish-2.md`. This handoff is completed after the
fresh-clone claim matrix and live deployment recheck.

## Changes

- Replaced the non-executable copy-audit hash check with a browser-rendered
  route and README audit that enforces every sentence’s word count and banned
  words.
- Split the overlong Team Pack README sentence.
- Removed untested automatic-migration, background-service, one-day cache,
  refund, receipt, and revocation promises.
- Added the exact Forgejo 9.0 Actions-history claim and report assertion.
- Updated the catalog description to the verb-first sentence: “Check a GitHub
  move before cutover with local evidence.”

## Verification

- `npm run audit:copy`: passed.
- `npm test`: passed — 5 Rust unit tests, 13 CLI integration tests, and 40
  Playwright tests; `test-results/.last-run.json` records `passed`.
- `npm run build`: passed; production JS is 6.51 kB gzip and CSS is 3.83 kB
  gzip.
- Focused claims passed: `@claim:forgejo-actions-history`,
  `@claim:billing-contract`, and `@claim:license-browser-storage`.

## Clean-clone claim matrix

- Fresh clone: `/tmp/gfed-polish2-clean.xB6VuR` at
  `b8e819cb7d4f587628ee85b429aae71438aee674`.
- Ran `npm ci --ignore-scripts --no-audit --no-fund` and then every exact
  `test` command in `.factory/claims.json` separately and sequentially.
- All 22 claims passed. The sequential log is
  `/tmp/gfed-polish2-claims.log`; the initial `demo-private` run also passed
  before that loop. The IDs are `demo-private`, `free-single`,
  `source-read-only`, `no-telemetry`, `recorded-cli`, `encrypted-evidence`,
  `evidence-complete`, `token-private`, `team-portfolio`,
  `cli-demo-isolated`, `target-mappings`, `forgejo-actions-history`,
  `restore-checklist`, `output-boundary`, `linux-download`,
  `billing-contract`, `archive-file-completeness`,
  `api-metadata-blocks-git`, `json-summary`, `actionable-errors`,
  `cli-network-boundaries`, and `license-browser-storage`.

## Local verification

- `npm run audit:copy`: passed; it now renders every public route and checks
  README prose instead of merely comparing hashes.
- `npm test`: passed — 5 Rust unit tests, 13 CLI integration tests, and 40
  Playwright tests. `test-results/.last-run.json` says `passed`.
- `npm run build`: passed and made `dist/site/` plus the Linux binary.
- `cargo fmt --check` and `cargo clippy --all-targets --all-features -- -D
  warnings`: passed.
- `npm audit --omit=dev --audit-level=high`: 0 vulnerabilities.
- `cargo package --locked --allow-dirty --no-verify`: passed; 121 files,
  10.8 MiB unpacked.
- Production CSS is 3.83 kB gzip and JavaScript is 6.51 kB gzip.
- Current local desktop and mobile screenshots are under
  `.factory/evidence/polish-2/`.

## Deployment and cold live recheck

- Repair commit `b8e819cb7d4f587628ee85b429aae71438aee674` was pushed to
  `origin/main`.
- Static deployment used the work-order configuration:
  `/opt/fleet/lib/deploy-static.sh git-forge-exit-drill dist/site`.
- Azure deployment `845ccaa0-84e3-4b4f-a432-6a10504012b9` succeeded. The
  custom domain returned HTTPS 200 and now serves
  `assets/index-DtMobjRO.js`, the repaired build.
- Cold `verify-url.sh` passed `/`, `/demo`, `/privacy`, and `/terms`: every
  route had its expected title, `lang=en`, one H1, a main landmark, image alt
  text, no unlabeled buttons, and no page or console errors. The generated
  desktop/mobile screenshots and JSON records are committed under
  `.factory/evidence/polish-2/live/`.
- Fresh-browser Axe scans on `/`, `/demo`, `/privacy`, `/terms`, and
  `/not-a-route` found zero serious or critical violations. The styled unknown
  route returns HTTP 404; its expected document-status console message was
  excluded from normal-page console-error checks.
- Cold live root at 1440×900 showed the headline, audience, action, action
  consequence, and all three facts above the fold. The one-click demo showed
  its banner, reset action, start-for-real action, and the Forgejo Actions
  history warning. Live privacy has no one-day cache promise; live terms has
  no refund, receipt, or revocation promise.

## Known gaps

None. The product remains a Rust CLI with a static Vite documentation site;
it has no product-owned backend, sign-in flow, or runtime AI feature.
