# Copy audit

Checked 2026-08-29 against the rendered route strings and README. A word is a
whitespace-delimited token after punctuation is removed; hyphenated terms and
prices count as one word. Navigation, controls, labels, terminal text, alt
text, footer metadata, and README prose are included. Code blocks and URLs are
excluded. No reader-facing sentence exceeds 22 words and none uses a banned
marketing word.

## Source lock

Run `npm run audit:copy` after changing copy. It fails until this audit is
updated for the exact reviewed sources.

- `site/src/main.ts: d3fa0f8ce9838c85e54bda3f68373a49fc741d681509509597ef65d9100eb5fc`
- `README.md: ae2fe773bad0e143b5dccf374c77f63c1ac66411d5a5abd9beeaa922abd389d1`

## Landing and route strings

| Surface | Copy units reviewed | Result |
| --- | --- | --- |
| Header and footer | Git Forge Exit Drill home; Demo; Install; Privacy; Terms; Built by Param Factory; version | pass |
| First screen | Git host migration check; Test your GitHub move before cutover; audience sentence; Try it with sample data; action consequence; three facts | pass |
| Image and sample | descriptive alt; target-map caption; Sample result; Sample drill results; sample explanation; terminal label and all seven transcript lines | pass |
| How it works | Run one repeatable drill; Inventory the source; Map the target; Plan the restore test; three explanations | pass |
| Install and limits | Start with the bundled sample; install explanation; copy control and clipboard recovery; Linux download; Know what the CLI writes; limit list | pass |
| Team Pack | Check ten repositories together; price sentence; two scope bullets; checkout label; license form; hidden license handoff; legal links | pass |
| Demo | demo banner; See a complete exit drill; recording explanation; blocked finding; three restore actions; Run your own drill | pass |
| Privacy | all five headings, summaries, saved-license control, and contact line | pass |
| Terms | all five headings, scope statements, Team Pack terms, and contact line | pass |
| Not found | Route / missing; This route has no evidence; recovery sentence and Return home | pass |

## README strings

| Section | Reviewed sentence groups | Result |
| --- | --- | --- |
| Introduction and demo | job statement; user; temporary-directory behavior; empty-output refusal | pass |
| Install and local drill | installation; Linux binary; output names; encryption; record validation; Git-history evidence; archive file coverage | pass |
| API, verification, and targets | token handling; metadata limitation; JSON output; error behavior; archive verification; target and repository-item definitions | pass |
| Team Pack and privacy | free scope; $39 one-time scope; checkout-to-terminal license handoff; three network boundaries; telemetry boundary | pass |

## Repair copy checks

| String | Words | Result |
| --- | ---: | --- |
| Use your license in the CLI | 6 | pass |
| Copy this private token, then set it in the terminal that runs the portfolio command. | 15 | pass |
| Team Pack license token | 4 | pass |
| Set the token before running portfolio. | 6 | pass |
| License copied. Keep it private. | 5 | pass |
| Setup command copied. Run it in your terminal. | 8 | pass |
| Clipboard access was denied. Select the commands above and copy them manually. | 11 | pass |
| Choose an output directory outside the selected export. | 8 | pass |
| The CLI refuses an overlapping path so the source stays read-only. | 11 | pass |
| After checkout, copy the shown private license token. | 8 | pass |
| Set it in the terminal that runs the portfolio command: | 10 | pass |
| Development and license | requirements; build outputs; MIT notice | pass |

## Terminology table

| Concept | One term |
| --- | --- |
| Destination service | Git host on first use; target afterwards |
| Data category | repository item on first use; item afterwards |
| Human output | readiness report |
| Encrypted collected bytes | evidence archive |
| Dry run | drill |
| Example state | demo |
| Paid multi-repository feature | Team Pack |
