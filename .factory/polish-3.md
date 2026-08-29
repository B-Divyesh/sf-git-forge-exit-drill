# Polish 3 — cumulative review closure map

Repair commit: `cc0a1c5a43c7bce90b3afe4b8f453d8a30e6ce5f`.

All 23 exact claim commands passed from the fresh clone at
`/tmp/gfed-polish3-clean.36Fkf8`; the command transcript is
`/tmp/gfed-polish3-clean-claims.log`. The unfiltered suite passed 41
Playwright tests, 5 Rust unit tests, and 13 Rust integration tests there.

Evidence shorthand: **live** means `https://git-forge-exit-drill.sociobot.in`;
the post-deploy verifier screenshots and machine-readable checks are under
`.factory/evidence/polish-3/live/`. Local matching-build screenshots are under
`.factory/evidence/polish-3/local/`.

- **Home screenshot** =
  `.factory/evidence/polish-3/live/home/screenshot-{desktop,mobile}.png`, live
  check `/`.
- **Demo screenshot** =
  `.factory/evidence/polish-3/live/demo/screenshot-{desktop,mobile}.png`, live
  check `/demo` and direct `?demo=1` in `live-recheck.json`.
- **Legal screenshot** =
  `.factory/evidence/polish-3/live/{privacy,terms}/screenshot-{desktop,mobile}.png`,
  live checks `/privacy` and `/terms`.
- **404 screenshot** =
  `.factory/evidence/polish-3/live/404/screenshot-mobile.png`, live check
  `/not-a-route` (HTTP 404).

| Finding | Change made | Evidence |
| --- | --- | --- |
| F-1-1 | Kept the real buy-or-enter-license path; no restore-link promise remains. | `@claim:license-browser-storage`; live `/`; home screenshot. |
| F-1-2 | Kept the explicit instruction to delete the printed demo directory. | `@claim:cli-demo-isolated`; README audit. |
| F-1-3 | Kept isolated CLI output and non-empty-directory protection. | `@claim:cli-demo-isolated`; live `/demo`. |
| F-1-4 | Kept named versioned target maps and support-state coverage. | `@claim:target-mappings`; live `/`. |
| F-1-5 | Kept generated restore-checklist coverage. | `@claim:restore-checklist`; live `/`. |
| F-1-6 | Kept the observable output-directory boundary. | `@claim:output-boundary`; live `/`. |
| F-1-7 | Kept the executable Linux download. | `@claim:linux-download`; live download returned 200 and hash matched build. |
| F-1-8 | Kept the active $39 one-time hosted checkout contract. | `@claim:billing-contract`; live `/`. |
| F-1-9 | Kept undefined device-entitlement copy removed. | `npm run audit:copy`; live home screenshot. |
| F-1-10 | Kept plain hosted-checkout wording. | `@claim:billing-contract`; live `/`. |
| F-1-11 | Kept full source-file archive comparison. | `@claim:archive-file-completeness`; README audit. |
| F-1-12 | Kept API-only Git-history blocking. | `@claim:api-metadata-blocks-git`; README audit. |
| F-1-13 | Kept parseable success and error JSON. | `@claim:json-summary`; README audit. |
| F-1-14 | Kept non-zero actionable documented errors. | `@claim:actionable-errors`; README audit. |
| F-1-15 | Kept local/API/license request-boundary coverage. | `@claim:cli-network-boundaries`; live `/privacy`. |
| F-1-16 | Kept the plain-language hero audience sentence. | first-screen Playwright test; live home desktop/mobile screenshots. |
| F-1-17 | Kept route-specific source metadata. | `built deep-link documents have route-specific source metadata`; live `/demo`, `/privacy`, `/terms`. |
| F-1-18 | Kept Back/Forward focus restoration. | `browser Back restores focus to the install heading`; live `/`. |
| F-1-19 | Kept first-screen facts above the fold. | first-screen Playwright test; live home mobile screenshot. |
| F-1-20 | Kept wrapped mobile terminal output. | mobile render Playwright test; live demo mobile screenshot. |
| F-1-21 | Kept generated, enforced rendered-copy and README audit. | `npm run audit:copy`. |
| F-1-22 | Kept literal target-map figure caption. | `@claim:target-mappings`; live home screenshot. |
| F-1-23 | Kept literal section labels. | `npm run audit:copy`; live home screenshot. |
| F-1-24 | Kept the `Sample drill results` heading. | `npm run audit:copy`; live `/`. |
| F-1-25 | Kept the explicit sample-validation sentence. | `@claim:evidence-complete`; live `/`. |
| F-1-26 | Kept the `Plan the restore test` step. | `@claim:restore-checklist`; live `/`. |
| F-1-27 | Kept corrected install wording. | `npm run audit:copy`; live `/`. |
| F-1-28 | Kept the result-naming license control. | `@claim:license-browser-storage`; live `/`. |
| F-1-29 | Kept recognized-export copy below 22 words. | `npm run audit:copy`; README audit. |
| F-1-30 | Kept split Git-history guidance. | `npm run audit:copy`; README audit. |
| F-1-31 | Kept split API limitation guidance. | `npm run audit:copy`; README audit. |
| F-1-32 | Kept the specific target-services heading. | `@claim:target-mappings`; README audit. |
| F-1-33 | Kept `readiness report` terminology. | `@claim:team-portfolio`; `npm run audit:copy`. |
| F-1-34 | Kept plain first mentions of Git host and repository item. | `npm run audit:copy`; live `/`. |
| F-2-1 / F-1-21 reopened | Kept the executable audit rather than a source-hash-only check. | `npm run audit:copy` in fresh clone. |
| F-2-2 | Kept Team Pack prose split into short sentences. | `npm run audit:copy`; README audit. |
| F-2-3 | Kept the untested automatic-migration label removed. | `npm run audit:copy`; live `/`. |
| F-2-4 | Kept the untested background-service label removed. | `npm run audit:copy`; live `/`. |
| F-2-5 | Kept the exact Forgejo Actions-history finding covered. | `@claim:forgejo-actions-history`; live `/demo`. |
| F-2-6 | Kept the untested one-day cache duration removed from public copy. | `@claim:license-browser-storage`; live `/privacy`. |
| F-2-7 | Replaced the receipt promise with `You buy from Sociobot through its hosted checkout.` | `@claim:billing-contract`; live recheck JSON and home screenshots. |
| F-3-1 / F-2-7 reopened | Removed the unsupported receipt-delivery promise while retaining the tested hosted-checkout statement. | `@claim:billing-contract`; `live-recheck.json` records one hosted-checkout sentence and zero receipt text. |
| F-3-2 | Added the `demo-valid-git-mirror` claim and a real test that runs `git fsck --no-dangling` on demo output and checks `git_repository.captured=true`. | `@claim:demo-valid-git-mirror`; README live-source check. |
| F-3-3 | Narrowed README archive wording from named algorithms to the tested `authenticated encryption` guarantee. | `@claim:encrypted-evidence`; README live-source check. |

## Post-deploy evidence

- `verify-url.sh` passed live `/`, `/demo`, `/privacy`, and `/terms` with no
  console errors, one H1, language, main landmark, and complete image/button
  labeling. See each route's `verify.json` and screenshots.
- Live Axe found no serious or critical violations on `/`, `/demo`, `/privacy`,
  `/terms`, or `/not-a-route`; see `live/axe.json`.
- `live-recheck.json` confirms cold direct `?demo=1`, demo namespace isolation,
  Reset focus, Start-for-real cleanup, the hosted-checkout wording, and the
  designed 404. `offline-demo.json` confirms same-origin requests and offline
  demo reload after the first visit.
- Live asset and Linux-binary SHA-256 values matched the local production build.
