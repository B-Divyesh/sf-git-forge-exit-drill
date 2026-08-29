# Polish 1 — review-1 closure map

Candidate repaired from `beb49611058280ebf675c5c471d00fc76c031183` through
`d69df29dfb195f58fb07be2845e8b5c134decf63` and finalised in
`3b105d904bb336c1edec2c46ee0b1ff07baf1e6b`. There are no earlier
`review-*.md` or `polish-*.md` files to carry forward. The evidence paths
below are captured after deployment; all listed automated tests also passed
from `/tmp/gfed-polish-clean` at commit `3b105d9`.

| Finding | Change made | Evidence |
| --- | --- | --- |
| F-1-1 | Replaced the nonexistent restore-link promise with the real purchase-or-enter-license path. | `@claim:license-browser-storage`; live `/` and README check. |
| F-1-2 | Replaced vague cleanup wording with the printed temporary-directory deletion instruction. | `@claim:cli-demo-isolated`; README check. |
| F-1-3 | Registered CLI demo isolation and tested printed paths plus an untouched non-empty sentinel. | `@claim:cli-demo-isolated`. |
| F-1-4 | Registered named versioned target maps and asserted all published support states. | `@claim:target-mappings`; live `/` check. |
| F-1-5 | Registered the generated restore checklist and asserted its three required steps. | `@claim:restore-checklist`. |
| F-1-6 | Replaced broad non-migration language with the observable output boundary. | `@claim:output-boundary`; live `/` check. |
| F-1-7 | Registered the Linux artifact and build now makes a release binary before testing it. | `@claim:linux-download`. |
| F-1-8 | Registered a live billing contract test: Sociobot checkout returns 303 and the Dodo order summary says $39.00 and one-time. | `@claim:billing-contract`. |
| F-1-9 | Removed the undefined device-entitlement bullet. | Live `/` check. |
| F-1-10 | Rewrote merchant jargon as plain payment-and-receipt language and verifies the hosted contract. | `@claim:billing-contract`; live `/` check. |
| F-1-11 | Compared every nested, empty, and binary source path and SHA-256 against verified archive output. | `@claim:archive-file-completeness`. |
| F-1-12 | Registered API-only blocked Git-history readiness and asserted no Git-object request. | `@claim:api-metadata-blocks-git`. |
| F-1-13 | Registered JSON output, then made runtime failures emit parseable JSON too. | `@claim:json-summary`. |
| F-1-14 | Registered the documented missing-source and short-passphrase error paths. | `@claim:actionable-errors`. |
| F-1-15 | Registered local/API/license network boundaries; the test records configured API and billing fixture calls. | `@claim:cli-network-boundaries`. |
| F-1-16 | Rewrote the hero audience sentence without “before Monday” or unexplained “forge”. | `.factory/evidence/polish-1-home/screenshot-desktop.png`; live `/` check. |
| F-1-17 | Build now emits route-specific static HTML metadata for demo, privacy, terms, and 404. | `built deep-link documents have route-specific source metadata`; live route-source checks. |
| F-1-18 | Hash restoration focuses and announces the install heading after Back. | `browser Back restores focus to the install heading`; live keyboard check. |
| F-1-19 | Made hero sizing continuous and assert all first-screen facts fit at 1280×720, 1366×768, and 1440×900. | `required first-screen content fits common desktop viewports`; `.factory/evidence/polish-1-home/screenshot-desktop.png`. |
| F-1-20 | Shortened the safety line and wraps mobile terminal lines; test asserts no terminal overflow. | `desktop and 390px mobile render without page overflow or console errors`; `.factory/evidence/polish-1-demo/screenshot-mobile.png`. |
| F-1-21 | Expanded the audit scope, documented counting, added source locks, and made `audit:copy` fail on source drift. | `npm run audit:copy`. |
| F-1-22 | Replaced the boundary metaphor with a literal target-map caption. | `@claim:target-mappings`; live `/` check. |
| F-1-23 | Replaced decorative numbered labels with literal section names. | `polish-1-home-1440.png`; live `/` check. |
| F-1-24 | Renamed the sample heading to “Sample drill results.” | Live `/` check. |
| F-1-25 | Rewrote the ambiguous sample explanation with an explicit subject and action. | `@claim:evidence-complete`; live `/` check. |
| F-1-26 | Renamed the checklist step to “Plan the restore test.” | `@claim:restore-checklist`; live `/` check. |
| F-1-27 | Corrected the install sentence to say the sample is shown “on” the page. | Live `/` check. |
| F-1-28 | Renamed the reveal control “Enter Team Pack license.” | `@claim:license-browser-storage`; live `/` check. |
| F-1-29 | Split the long recognized-export sentence in README. | `npm run audit:copy`; README check. |
| F-1-30 | Split the Git-history evidence warning into three short README sentences. | `@claim:api-metadata-blocks-git`; README check. |
| F-1-31 | Split API-mode limitation into plain, short README sentences. | `@claim:api-metadata-blocks-git`; README check. |
| F-1-32 | Renamed “Targets” to “Supported target services and versions.” | `@claim:target-mappings`; README check. |
| F-1-33 | Used “readiness report” consistently, with Markdown named as its format. | `@claim:team-portfolio`; live `/` and README check. |
| F-1-34 | Uses “Git host” and “repository item” before the shorter target terminology. | `npm run audit:copy`; live `/` and README check. |

## Final suite

- Fresh clone: `/tmp/gfed-polish-clean`, SHA `3b105d9`.
- Claims: every `@claim:` test ran with `npm test -- --grep @claim`.
- Full suite: `npm test` — TypeScript, 5 Rust unit tests, 13 CLI integration
  tests, release build, and 34 Playwright tests.
- Accessibility: Axe serious/critical checks on `/`, `/demo`, `/privacy`, and
  `/terms` are in the Playwright suite.
- Performance: production initial JS is 5.65 kB gzip and CSS is 3.57 kB gzip.
