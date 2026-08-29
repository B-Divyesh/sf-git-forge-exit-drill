# Polish 2 — cumulative review closure map

This round repairs candidate `367797ccd548b80d8606d9a727c7d9b7593c01ff`.
Every finding in `review-1.md` and `review-2.md` is acceptance work. The
evidence column names the durable automated check; the final live checks and
screenshots are recorded in the handoff after deployment.

| Finding | Change made | Evidence |
| --- | --- | --- |
| F-1-1 | Replaced the nonexistent restore-link promise with the actual buy-or-enter-license flow. | `@claim:license-browser-storage`; live `/` |
| F-1-2 | Replaced vague demo cleanup wording with deletion of the printed temporary directory. | `@claim:cli-demo-isolated`; README |
| F-1-3 | Registered isolated CLI demo output and non-empty-directory preservation. | `@claim:cli-demo-isolated` |
| F-1-4 | Registered target versions and support-state mapping. | `@claim:target-mappings` |
| F-1-5 | Registered the generated restore checklist. | `@claim:restore-checklist` |
| F-1-6 | Replaced the broad boundary with the tested output-directory statement. | `@claim:output-boundary` |
| F-1-7 | Registered the executable Linux release artifact. | `@claim:linux-download` |
| F-1-8 | Registered the active $39 hosted checkout contract. | `@claim:billing-contract` |
| F-1-9 | Removed the undefined device-entitlement promise. | live `/` copy check |
| F-1-10 | Rewrote merchant jargon in plain words and checks the hosted offer. | `@claim:billing-contract` |
| F-1-11 | Compares every regular source path and digest with archive contents. | `@claim:archive-file-completeness` |
| F-1-12 | Registers API metadata-only Git-history blocking. | `@claim:api-metadata-blocks-git` |
| F-1-13 | Registers parseable JSON on success and error. | `@claim:json-summary` |
| F-1-14 | Registers non-zero, actionable documented errors. | `@claim:actionable-errors` |
| F-1-15 | Registers local/API/license network boundaries. | `@claim:cli-network-boundaries` |
| F-1-16 | Rewrote the hero audience sentence without timing or unexplained jargon. | first-screen Playwright check; live `/` |
| F-1-17 | Emits route-specific static metadata documents. | `built deep-link documents have route-specific source metadata` |
| F-1-18 | Restores heading focus on Back to the install anchor. | `browser Back restores focus to the install heading` |
| F-1-19 | Keeps all required first-screen facts inside common desktop viewports. | `required first-screen content fits common desktop viewports` |
| F-1-20 | Wraps demo terminal copy on mobile and shortens its safety line. | `desktop and 390px mobile render without page overflow or console errors` |
| F-1-21 | Replaced hash-only checking with an executable rendered-copy and README audit. | `npm run audit:copy` |
| F-1-22 | Replaced migration metaphor with a literal target-map caption. | `@claim:target-mappings` |
| F-1-23 | Replaced decorative labels with literal section names. | copy audit; live `/` |
| F-1-24 | Renamed the sample section to “Sample drill results.” | copy audit; live `/` |
| F-1-25 | Rewrote the ambiguous sample sentence with an explicit subject. | `@claim:evidence-complete` |
| F-1-26 | Renamed the checklist step to “Plan the restore test.” | `@claim:restore-checklist` |
| F-1-27 | Corrected the install sentence to use “shown on this page.” | copy audit; live `/` |
| F-1-28 | Renamed the license disclosure control to name its result. | `@claim:license-browser-storage` |
| F-1-29 | Split the recognized-export README sentence. | `npm run audit:copy` |
| F-1-30 | Split the Git-history evidence warning. | `@claim:api-metadata-blocks-git` |
| F-1-31 | Split the API-mode limitation and next action. | `@claim:api-metadata-blocks-git` |
| F-1-32 | Renamed the target-services heading. | `@claim:target-mappings` |
| F-1-33 | Uses “readiness report” consistently. | `@claim:team-portfolio`; copy audit |
| F-1-34 | Introduces “Git host” and “repository item” before shorthand. | `npm run audit:copy` |
| F-2-1 / F-1-21 reopened | `audit:copy` now launches the rendered router, reads public accessible copy plus README prose, counts every unit, rejects banned words and >22 words, and rejects stale audit output. | `npm run audit:copy` |
| F-2-2 | Split the 23-word Team Pack sentence into price, scope, and format sentences. | `npm run audit:copy`; README |
| F-2-3 | Removed the unprovable “No automatic migration” label and retained only the tested output boundary. | `@claim:output-boundary`; live `/` |
| F-2-4 | Removed the untested “No background service” label. | live `/` copy check |
| F-2-5 | Added the exact Forgejo Actions-history claim; it asserts the mapping and emitted sample report finding. | `@claim:forgejo-actions-history`; live `/demo` |
| F-2-6 | Removed the untested one-day cache duration while retaining tested browser storage and removal behavior. | `@claim:license-browser-storage`; live `/privacy` |
| F-2-7 | Removed receipt, refund, and revocation lifecycle promises; payment copy now states only the tested hosted-checkout path. | `@claim:billing-contract`; live `/privacy`, `/terms` |

## Evidence artifacts

- Local screenshots: `.factory/evidence/polish-2/{home,demo,privacy,terms}-{desktop,mobile}.png`.
- Live screenshots and verifier records: `.factory/evidence/polish-2/live/{home,demo,privacy,terms}/`.
- Live recheck: `https://git-forge-exit-drill.sociobot.in/`, `/demo`, `/privacy`, `/terms`, and `/not-a-route` after deployment `845ccaa0-84e3-4b4f-a432-6a10504012b9`.
