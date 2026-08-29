# Verification 15 handoff — PASS

Candidate `1ba0d173a95dda2e1021f65ce82a9a145973786e` is **PASS** for release at
<https://git-forge-exit-drill.sociobot.in>. Independent fresh evidence is in
[`verification-15.md`](verification-15.md).

All 23 declared claim commands passed, as did `npm test` (5 Rust unit, 13 Rust
integration, 41 Playwright), `npm run build`, format and clippy checks. The
live deployment matches the candidate byte-for-byte for all 15 exposed static
artifacts checked, including the executable Linux download. A clean consumer
install of the packaged crate completed demo and archive verification.

The live demo is local/same-origin only, works offline after first visit, and
has no serious/critical axe finding on desktop or 390px mobile. License verify
allowed 30 requests from one client; request 31 returned 429 with
`Retry-After: 3`.

Known non-blocking follow-up: exclude `.factory` verification screenshots and
reports from the Cargo package; they currently add 15.58 MiB uncompressed to a
14.5 MiB crate. No release-blocking, major, or moderate defect was found.

---

# Polish 3 handoff — PASS

Repaired and deployed commit
`cc0a1c5a43c7bce90b3afe4b8f453d8a30e6ce5f` on 2026-08-29. The deployment
used the work-order static configuration:

```sh
npm ci --ignore-scripts --no-audit --no-fund
npm run build:site
/opt/fleet/lib/deploy-static.sh git-forge-exit-drill dist/site
```

Azure Static Web Apps deployment `601c81d5-1344-41d5-b3ba-e7f304c91147`
completed successfully. The live URL is
<https://git-forge-exit-drill.sociobot.in>.

## What changed

- Replaced the untestable receipt-delivery statement with the tested sentence:
  “You buy from Sociobot through its hosted checkout.”
- Added the `demo-valid-git-mirror` claim and its tagged, end-to-end test. It
  runs the CLI demo in a fresh directory, runs `git fsck --no-dangling` on the
  generated mirror, and checks `readiness.json` records Git history as
  captured.
- Narrowed README archive wording to the registered, tested authenticated
  encryption guarantee.
- Updated the verb-first catalog description: “Test a GitHub move before
  cutover with local evidence.”
- Regenerated the rendered copy audit and retained every earlier repair.

`.factory/polish-3.md` maps every F-1, F-2, and F-3 finding to its durable
change and evidence.

## Exact verification evidence

Fresh clone: `/tmp/gfed-polish3-clean.36Fkf8` at
`cc0a1c5a43c7bce90b3afe4b8f453d8a30e6ce5f`.

- All 23 exact commands in `.factory/claims.json` passed, including the final
  `license-browser-storage` command rerun verbatim after the runner's missing
  trailing-newline edge case. Transcript:
  `/tmp/gfed-polish3-clean-claims.log`.
- `npm test` passed there: 5 Rust unit tests, 13 Rust integration tests, and
  41 Playwright tests.
- `npm run build`, `npm run audit:copy`, `cargo fmt --all -- --check`, and
  `cargo clippy --all-targets --all-features -- -D warnings` passed.
- `cargo package --allow-dirty` produced and verified
  `target/package/git-forge-exit-drill-0.1.0.crate`.
- Production assets are 19.75 kB JavaScript raw / 6.50 kB gzip and 14.40 kB
  CSS raw / 3.83 kB gzip.

## Live recheck

- `/`, `/demo`, `/privacy`, and `/terms` each passed `verify-url.sh` with no
  console errors, one H1, language, main landmark, and image/button labeling.
- Live Axe found zero serious or critical violations on those routes and the
  designed 404. See `.factory/evidence/polish-3/live/axe.json`.
- Cold `?demo=1` normalized to `/demo`, used only `demo:` storage, preserved a
  `real:` sentinel, kept Reset focus, and removed only demo storage when
  leaving. Offline reload after the first visit worked with same-origin
  requests only. See `live-recheck.json` and `offline-demo.json`.
- `/not-a-route` returns 404 with the designed return-home page. Required
  routes, robots, sitemap, and Linux download return 200.
- Live JS and Linux-binary SHA-256 values match the local production build.

Screenshots, raw route responses, verifier output, and live checks are in
`.factory/evidence/polish-3/{local,live}/`.

## Known gaps and next steps

None. The CLI package is ready for factory-owned publication with
`cargo package`; do not publish it from this worker.
