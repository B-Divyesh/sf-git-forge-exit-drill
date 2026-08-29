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

- `site/src/main.ts: d241ae84e836c7f0e8153e6fe9f5c826e50871245a980a917c3e550217c15759`
- `README.md: 8915abba49b5de0adc71efacd3f9d7e10ccbed91ab1d9aebbe476c980cb74014`

## Landing and route strings

| Surface | Copy units reviewed | Result |
| --- | --- | --- |
| Header and footer | Git Forge Exit Drill home; Demo; Install; Privacy; Terms; Built by Param Factory; version | pass |
| First screen | Git host migration check; Test your GitHub move before cutover; audience sentence; Try it with sample data; action consequence; three facts | pass |
| Image and sample | descriptive alt; target-map caption; Sample result; Sample drill results; sample explanation; terminal label and all seven transcript lines | pass |
| How it works | Run one repeatable drill; Inventory the source; Map the target; Plan the restore test; three explanations | pass |
| Install and limits | Start with the bundled sample; install explanation; copy control; Linux download; Know what the CLI writes; limit list | pass |
| Team Pack | Check ten repositories together; price sentence; two scope bullets; checkout label; license form; legal links | pass |
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
| Team Pack and privacy | free scope; $39 one-time scope; license route; three network boundaries; telemetry boundary | pass |
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
