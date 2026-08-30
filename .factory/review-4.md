# Adversarial first-read review 4 — PASS

Reviewed 2026-08-30 against commit
`31c226b5764a2b7b92667ac999418771112cb137` and
<https://git-forge-exit-drill.sociobot.in> in fresh Chromium contexts.

## Verdict

**PASS.** There are zero findings, zero untested registered claims, and zero
unlisted claim-like sentences on the live landing page or in README. The cold
read, one-click site demo, isolated CLI demo, all 23 exact claim commands,
unfiltered tests, build, routes, links, metadata, accessibility checks, and
every earlier finding pass independent verification.

## Findings

None.

## Cold first read

I opened `/` cold in separate storage-empty contexts at 390×844 and 1440×900.
I recorded the first viewport before scrolling.

| Question | Answer available from the first screen |
| --- | --- |
| What does this do? | It tests a planned GitHub move and finds missing repository history or build evidence before cutover. |
| For whom? | Small teams changing Git hosts. |
| What should I click first? | **Try it with sample data**. |

The exact supporting copy is “Test your GitHub move before cutover,” “For
small teams changing Git hosts, it finds missing repository history and build
evidence before cutover,” and “See a complete drill with no setup.” The three
facts end at y=774 in the 844 px mobile viewport and y=730 in the 900 px
desktop viewport. All required first-screen content is visible without
scrolling. There is no horizontal overflow or console error at either size.

## Copy audit

Counting rule: a word is a whitespace-delimited token after punctuation is
removed. Hyphenated terms, versions, prices, and commands count as one word.
The tables include every sentence plus the headings, controls, labels, status
messages, errors, and accessible names needed to check jargon and action copy.
No unit exceeds 22 words. No banned marketing adjective, unexplained jargon,
inconsistent product term, metaphor or mood heading, or non-result-naming
action remains.

### Landing page

| Type | Copy | Words | Flag |
| --- | --- | ---: | --- |
| Skip link | Skip to main content | 4 | — |
| Wordmark label | Git Forge Exit Drill home | 5 | — |
| Wordmark | EXIT/DRILL | 1 | — |
| Navigation label | Main navigation | 2 | — |
| Navigation | Demo | 1 | — |
| Navigation | Install | 1 | — |
| Navigation | Privacy | 1 | — |
| Section label | Git host migration check | 4 | — |
| H1 | Test your GitHub move before cutover | 6 | — |
| Sentence | For small teams changing Git hosts, it finds missing repository history and build evidence before cutover. | 16 | — |
| Action | Try it with sample data | 5 | — |
| Sentence | See a complete drill with no setup. | 7 | — |
| List label | Product facts | 2 | — |
| Fact | Local drills need no network connection. | 6 | — |
| Fact | Sample data stays in demo storage. | 6 | — |
| Fact | One-repository drills are free. | 4 | — |
| Image alt | A GitHub repository-item graph maps to a checked target grid with unsupported items marked. | 14 | — |
| Caption | The drill maps each GitHub repository item to the target and marks unsupported items. | 14 | — |
| Section label | Sample result | 2 | — |
| H2 | Sample drill results | 3 | — |
| Sentence | The sample repository has code, issues, releases, and build history. | 10 | — |
| Sentence | The drill counts an item only after it validates exported records. | 11 | — |
| Terminal label | Recorded terminal output from the bundled sample drill | 8 | — |
| Terminal label | Terminal transcript | 2 | — |
| Terminal | sample / atlas-notes | 2 | — |
| Terminal | $ git-forge-exit-drill demo | 2 | — |
| Terminal | Demo — sample data. | 3 | — |
| Terminal | No workspace files were read. | 5 | — |
| Terminal | Repository: acme-labs/atlas-notes | 2 | — |
| Terminal | Target: Forgejo 9.0 | 3 | — |
| Terminal | Outcome: BLOCKED | 2 | — |
| Terminal | Demo archive passphrase: demo-only-passphrase | 4 | — |
| Terminal | Choose a new output directory to run this demo again. | 10 | — |
| Section label | How it works | 3 | — |
| H2 | Run one repeatable drill | 4 | — |
| H3 | Inventory the source | 3 | — |
| Sentence | Read an extracted export or an authorized GitHub API repository. | 10 | — |
| H3 | Map the target | 3 | — |
| Sentence | Compare each repository item with GitLab, Gitea, or Forgejo version maps. | 11 | — |
| H3 | Plan the restore test | 4 | — |
| Sentence | Keep encrypted evidence and use the generated restore checklist. | 9 | — |
| Section label | Install | 1 | — |
| H2 | Start with the bundled sample | 5 | — |
| Sentence | Build from source, then run the sample shown on this page. | 11 | — |
| Action | Copy commands | 2 | — |
| Result | Commands copied | 2 | — |
| Live result | Install commands copied | 3 | — |
| Error | Clipboard access was denied. | 4 | — |
| Error | Select the commands above and copy them manually. | 8 | — |
| Action | Download Linux x86-64 binary | 4 | — |
| Section label | Limits | 1 | — |
| H2 | Know what the CLI writes | 5 | — |
| Sentence | The CLI writes reports and an evidence archive to the output directory. | 12 | — |
| Sentence | It does not change your selected export. | 7 | — |
| Action | Read the privacy details | 4 | — |
| Boundary | Local output stays in your chosen directory | 7 | — |
| Boundary | Review the readiness report before cutover | 6 | — |
| Boundary | No sample demo telemetry | 4 | — |
| Section label | Team Pack | 2 | — |
| H2 | Check ten repositories together | 4 | — |
| Sentence | A $39 one-time purchase adds the portfolio command and one consolidated readiness report. | 13 | — |
| Sentence | The one-repository drill stays free. | 5 | — |
| List item | Up to ten local exports per run | 7 | — |
| List item | One consolidated readiness report in Markdown | 6 | — |
| Action | Buy Team Pack — $39 | 4 | — |
| Accessible label | hosted checkout | 2 | — |
| Sentence | You buy from Sociobot through its hosted checkout. | 8 | — |
| Action | Enter Team Pack license | 4 | — |
| Form label | License token | 2 | — |
| Action | Verify license | 2 | — |
| Status | Checking license | 2 | — |
| Status | Team Pack license active. | 4 | — |
| Status | License no longer active. | 4 | — |
| Error | License check failed. | 3 | — |
| Error | Connect to the internet and try again. | 7 | — |
| H3 | Use your license in the CLI | 6 | — |
| Sentence | Copy this private token, then set it in the terminal that runs the portfolio command. | 15 | — |
| Form label | Team Pack license token | 4 | — |
| Actions | Show license / Hide license / Copy license | 2 / 2 / 2 | — |
| Sentence | Set the token before running portfolio. | 6 | — |
| Action | Copy setup command | 3 | — |
| Status | License copied. | 2 | — |
| Sentence | Keep it private. | 3 | — |
| Live result | Team Pack license copied | 4 | — |
| Status | Setup command copied. | 3 | — |
| Sentence | Run it in your terminal. | 5 | — |
| Live result | CLI setup command copied | 4 | — |
| Error | Select the license token, copy it, then paste it in your terminal. | 12 | — |
| Footer sentence | Test a GitHub move before cutover. | 6 | — |
| Footer links | Privacy / Terms / Built by Param Factory | 1 / 1 / 4 | — |
| Footer metadata | v0.1.0 · build 2026.08.28 | 3 | — |

### README

| Type | Copy | Words | Flag |
| --- | --- | ---: | --- |
| H1 | Git Forge Exit Drill | 4 | — |
| Sentence | Test a GitHub move before your team cuts over. | 9 | — |
| Sentence | Git Forge Exit Drill is a Rust command-line tool for small teams changing Git hosts. | 15 | — |
| Sentence | It checks an authorized GitHub export or API repository, writes an encrypted evidence archive, and creates readiness reports before cutover. | 20 | — |
| H2 | Try the bundled drill | 4 | — |
| Sentence | The command copies the bundled Atlas Notes sample into a new temporary directory. | 13 | — |
| Sentence | It creates a validated sample Git mirror and prints its report and archive paths. | 14 | — |
| Sentence | The demo does not read your workspace. | 7 | — |
| Sentence | Delete the printed temporary directory when finished. | 7 | — |
| Sentence | With `--output`, choose a new or empty directory. | 8 | — |
| Sentence | The command refuses a non-empty directory without changing it. | 9 | — |
| H2 | Install | 1 | — |
| Sentence | Build the single binary from source: | 6 | — |
| Sentence | The release site provides a Linux x86-64 binary. | 8 | — |
| Sentence | The download test checks that the production build includes an executable binary with the expected version. | 16 | — |
| H2 | Run a local export drill | 5 | — |
| Sentence | Set the archive passphrase in an environment variable. | 8 | — |
| Sentence | Then point the command at an export directory. | 8 | — |
| Sentence | The output directory contains: | 4 | — |
| List item | `readiness.md`: findings and a restore checklist for people. | 8 | — |
| List item | `readiness.json`: the same findings for scripts. | 6 | — |
| List item | `evidence.gfed`: source evidence protected with authenticated encryption. | 7 | — |
| Sentence | Choose an output directory outside the selected export. | 8 | — |
| Sentence | The CLI refuses an overlapping path so the source stays read-only. | 11 | — |
| Sentence | The source directory may contain `manifest.json` with expected repository-item counts. | 10 | — |
| Sentence | The CLI parses five recognized JSON exports: issues, pull requests, releases, workflows, and workflow runs. | 15 | — |
| Sentence | It compares their record counts with the manifest. | 8 | — |
| Sentence | Invalid JSON, invalid records, absent files, and count mismatches are incomplete evidence, never captured data. | 15 | — |
| Sentence | A manifest cannot prove repository history. | 6 | — |
| Sentence | The export must include a valid Git bundle or mirror. | 10 | — |
| Sentence | The mirror must contain Git objects. | 6 | — |
| Sentence | The CLI validates it with Git before counting it. | 9 | — |
| Sentence | Every regular source file enters the evidence archive. | 8 | — |
| H2 | Run an authorized API drill | 5 | — |
| Sentence | Create a fine-grained GitHub token with read-only access to the repository metadata you need. | 14 | — |
| Sentence | The token is read from the environment and never written to the reports or evidence archive. | 16 | — |
| Sentence | API mode inventories metadata but does not download Git history. | 10 | — |
| Sentence | The report stays blocked. | 4 | — |
| Sentence | Run a local drill with a validated mirror or bundle. | 10 | — |
| Sentence | Use `--json` before the subcommand for a JSON summary or error. | 11 | — |
| Sentence | Documented errors exit non-zero and give one next step. | 9 | — |
| H2 | Verify an archive | 3 | — |
| Sentence | Verification authenticates the archive and checks every recorded file digest. | 10 | — |
| H2 | Supported target services and versions | 5 | — |
| Sentence | The versioned mapping file is `mappings/targets.json`. | 6 | — |
| Sentence | A target is the Git host you plan to move to. | 11 | — |
| Sentence | A repository item is a category such as issues or releases. | 11 | — |
| Sentence | The maps mark each item as native, manual, or unsupported for GitLab 17.0, Gitea 1.22, and Forgejo 9.0. | 18 | — |
| H2 | Team Pack | 2 | — |
| Sentence | The free CLI runs one-repository drills. | 6 | — |
| Sentence | Team Pack costs $39 once. | 5 | — |
| Sentence | It adds `portfolio` drills for up to ten exports and one consolidated Markdown readiness report. | 15 | — |
| Sentence | Buy Team Pack or enter an existing license on the product site. | 12 | — |
| Sentence | After checkout, copy the shown private license token. | 8 | — |
| Sentence | Set it in the terminal that runs the portfolio command: | 10 | — |
| Sentence | The CLI reads `GFED_LICENSE` and verifies it with the Sociobot billing API. | 12 | — |
| H2 | Develop and verify | 3 | — |
| Sentence | Requirements: stable Rust, Node 22+, and Chromium for browser checks. | 10 | — |
| Sentence | `npm run build` creates the release binary and static site in `dist/site/`. | 12 | — |
| Sentence | The site build is also available as `npm run build:site`. | 11 | — |
| H2 | Privacy and security | 3 | — |
| Sentence | Local export drills make no network requests. | 7 | — |
| Sentence | API drills contact only the configured GitHub API origin. | 9 | — |
| Sentence | Portfolio license checks contact only the Sociobot billing API. | 9 | — |
| Sentence | The CLI makes no telemetry requests in these flows. | 9 | — |
| Sentence | See the site privacy page and terms. | 7 | — |
| Sentence | Review source exports before sharing them. | 6 | — |
| Sentence | They can include personal data, third-party license text, and secret material. | 11 | — |
| Sentence | Do not use command-line arguments for a token or archive passphrase. | 11 | — |
| H2 | License | 1 | — |
| Sentence | MIT. | 1 | — |
| Sentence | See `LICENSE`. | 2 | — |

Terminology is consistent: **Git host** introduces the destination and
**target** is its later shorthand; **repository item** is a data category;
**readiness report**, **evidence archive**, **drill**, **demo**, and **Team
Pack** each retain one meaning.

## Demo and sandbox

**PASS.** One click on **Try it with sample data** opens `/demo`. In the
initial 390 px viewport, the persistent banner, Atlas Notes sample, Forgejo
9.0 target, `Outcome: BLOCKED`, passphrase, and rerun instruction are already
visible. The banner says “Demo — sample data, nothing is saved” and provides
**Reset demo** and **Start for real**.

The browser demo created only `demo:gfed:started`. Reset replaced that value,
kept focus on **Reset demo**, announced the reset, and preserved both a
`real:sentinel` and a saved-license sentinel. **Start for real** removed the
demo key, retained both sentinels, opened `/#install`, and focused “Start with
the bundled sample.” An offline reload retained the demo. The complete demo
request log contained only
`https://git-forge-exit-drill.sociobot.in`.

The CLI demo ran with rejecting HTTP, HTTPS, and all-proxy endpoints. Its
working-directory sentinel remained unchanged. It created a new
`/tmp/git-forge-exit-drill-demo-*` tree containing the bundled export, a
Git-valid `atlas-notes.git`, `readiness.md`, `readiness.json`, and
`evidence.gfed`. `git fsck --no-dangling` exited 0, and the JSON report records
Git repository history as captured.

## Registered claims

I cloned the candidate to `/tmp/gfed-review4-clean.PqP8N5` and ran every exact
`test` command from `.factory/claims.json` separately and sequentially. Each
command performed its declared clean `npm ci`. The complete transcript is
`/tmp/gfed-review4-claims.log`. Every marker occurs exactly once in the test
source.

| Claim | Result | Observable evidence |
| --- | --- | --- |
| `demo-private` | PASS | Fresh demo stayed same-origin and used only the demo key. |
| `free-single` | PASS | One-repository drill completed with rejecting proxies and no license. |
| `source-read-only` | PASS | Source bytes stayed identical; overlapping output was refused. |
| `no-telemetry` | PASS | Demo requests stayed on the product origin. |
| `recorded-cli` | PASS | Displayed terminal lines matched real CLI output. |
| `encrypted-evidence` | PASS | Plaintext was absent; archive verification succeeded. |
| `evidence-complete` | PASS | Complete, malformed, mixed, and paginated records produced the promised counts. |
| `token-private` | PASS | Fixture token was absent from reports and evidence. |
| `team-portfolio` | PASS | Returned fixture license enabled ten-export portfolio output in a clean install. |
| `cli-demo-isolated` | PASS | Demo paths were isolated and an occupied-output sentinel survived. |
| `demo-valid-git-mirror` | PASS | Generated mirror passed Git checks and history was captured. |
| `target-mappings` | PASS | Named versions and native/manual/unsupported states matched. |
| `forgejo-actions-history` | PASS | Forgejo map and report both mark old Actions runs unsupported. |
| `restore-checklist` | PASS | Generated Markdown contained the required restore steps. |
| `output-boundary` | PASS | The drill created only declared outputs and preserved source. |
| `linux-download` | PASS | Production artifact executed and reported version 0.1.0. |
| `billing-contract` | PASS | Hosted checkout showed the active one-time $39 offer. |
| `archive-file-completeness` | PASS | Nested, empty, and binary files matched archive digests. |
| `api-metadata-blocks-git` | PASS | API-only output omitted Git bytes and stayed blocked. |
| `json-summary` | PASS | Success and documented errors emitted parseable JSON. |
| `actionable-errors` | PASS | Registered setup errors exited non-zero with one next step. |
| `cli-network-boundaries` | PASS | Local/API/license flows stayed within declared origins. |
| `license-browser-storage` | PASS | Returned license was stored, reused, and removable. |

I reconciled every claim-like landing and README sentence against those
entries and their tagged assertions. There is no unlisted claim.

## Historical finding verification

I read all three earlier reviews, all three polish records, and the prior
handoff. I then checked every earlier finding against both the live site and
current code; no repair is accepted solely because a polish record says so.

| Earlier ID | Independent confirmation | Status |
| --- | --- | --- |
| F-1-1 | README names the real buy-or-enter-license flow; both controls exist live. | fixed |
| F-1-2 | README gives an explicit printed-directory deletion instruction. | fixed |
| F-1-3 | The isolated CLI demo claim preserves an occupied-output sentinel. | fixed |
| F-1-4 | Named target versions and support states are registered and pass. | fixed |
| F-1-5 | The generated restore checklist has a passing dedicated claim. | fixed |
| F-1-6 | Copy uses the tested output-directory boundary. | fixed |
| F-1-7 | Built and live Linux downloads execute as version 0.1.0. | fixed |
| F-1-8 | The active one-time $39 checkout is registered and passes. | fixed |
| F-1-9 | The undefined device-entitlement promise is absent. | fixed |
| F-1-10 | Merchant jargon and the unsupported receipt promise are absent. | fixed |
| F-1-11 | Nested, empty, and binary source files match archive digests. | fixed |
| F-1-12 | API-only Git-history blocking has a passing dedicated claim. | fixed |
| F-1-13 | JSON success and failure output parses in the tagged test. | fixed |
| F-1-14 | Registered setup errors exit non-zero with one next step. | fixed |
| F-1-15 | CLI network destinations have a passing dedicated claim. | fixed |
| F-1-16 | Hero says “changing Git hosts” and “before cutover.” | fixed |
| F-1-17 | Raw deep-link HTML has route-specific metadata. | fixed |
| F-1-18 | Back to Install restores focus to the visible H2. | fixed |
| F-1-19 | All required hero facts fit at 390×844 and 1440×900. | fixed |
| F-1-20 | Mobile terminal lines wrap without page overflow. | fixed |
| F-1-21 | `audit:copy` renders routes, counts words, rejects violations, and detects stale output. | fixed |
| F-1-22 | The caption literally describes mapping and unsupported items. | fixed |
| F-1-23 | Section labels use literal names. | fixed |
| F-1-24 | The heading is “Sample drill results.” | fixed |
| F-1-25 | Sample copy names the drill and validation action. | fixed |
| F-1-26 | The step is “Plan the restore test.” | fixed |
| F-1-27 | Install copy says the sample is shown “on” the page. | fixed |
| F-1-28 | The control says “Enter Team Pack license.” | fixed |
| F-1-29 | Recognized-export copy is split below 22 words. | fixed |
| F-1-30 | Git-history guidance is split into short sentences. | fixed |
| F-1-31 | API limitation and next action are split into short sentences. | fixed |
| F-1-32 | The heading names supported target services and versions. | fixed |
| F-1-33 | “Readiness report” is used consistently. | fixed |
| F-1-34 | “Git host” and “repository item” are introduced before shorthand. | fixed |
| F-2-1 / F-1-21 | The copy audit is executable, rendered, and passed in the clean clone. | fixed |
| F-2-2 | Team Pack README scope uses sentences of 5 and 15 words. | fixed |
| F-2-3 | “No automatic migration” is absent. | fixed |
| F-2-4 | “No background service” is absent. | fixed |
| F-2-5 | Exact Forgejo Actions-history behavior is registered and passes. | fixed |
| F-2-6 | The untested one-day cache duration is absent from public copy. | fixed |
| F-2-7 | Refund, revocation, and receipt promises are absent. | fixed |
| F-3-1 / F-2-7 | Live copy now says only “You buy from Sociobot through its hosted checkout.” | fixed |
| F-3-2 | `demo-valid-git-mirror` passes a real Git check and report assertion. | fixed |
| F-3-3 | README uses the tested phrase “authenticated encryption.” | fixed |

## Structure, accessibility, links, and identity

- `/`, `/demo`, `/privacy`, and `/terms` return 200. `/not-a-route` returns a
  designed evidence-lattice page with HTTP 404 and **Return home**.
- Every route has the required title pattern, one H1, ordered headings,
  `lang="en"`, a main landmark, a working skip link, route-specific raw title,
  description, canonical, Open Graph and Twitter metadata, SVG favicon, Apple
  touch icon, and the shared header/footer.
- Browser Back restored `/#install`, scroll position, and focus on “Start with
  the bundled sample.” Forward restored `/demo` and focused its H1.
- Every navigational HTTP link returned 200 after redirects. The checkout
  reached the hosted Dodo page. The Linux download returned 200. `mailto:`
  links are explicit. The 404 page's skip link correctly targets its own main
  landmark without turning the intentional 404 into a navigational link.
- `robots.txt`, `sitemap.xml`, the web manifest, OG image, and both icons return
  200. CSP, frame protection, content-type, referrer, and permissions headers
  are present without console violations.
- The live verifier passed all four public routes. Axe reported zero WCAG
  A/AA violations at 390×844 and 1440×900 on those routes and the 404. Live
  controls meet 44 px, no route overflows at 390 px, and the full suite covers
  keyboard use, 200% text, reduced motion, and offline reload.
- Initial production JavaScript is 19,747 bytes raw and 6,560 bytes gzip.
- The dark inspection grid, clipped controls, cyan/coral evidence states,
  monospace display type, and original lattice artwork match
  `.factory/design.md`. The visual identity is not a generic SaaS template.

## Quality gates

| Check | Result |
| --- | --- |
| Every exact claim command from the clean clone | PASS — 23/23 |
| `npm test` | PASS — 5 Rust unit, 13 Rust integration, 41 Playwright |
| `npm run build` | PASS — executable binary and `dist/site/` produced |
| `cargo fmt --all -- --check` | PASS |
| `cargo clippy --all-targets --all-features -- -D warnings` | PASS |
| Live verifier on all public routes | PASS — no console errors |
| Live Axe on public routes and 404, mobile and desktop | PASS — zero violations |
| Live link and metadata crawl | PASS |
| Browser and CLI sandbox checks | PASS |

## Missed leverage

No finding. The brief calls for local export/API inventory, encrypted evidence,
versioned target maps, a restore checklist, and portfolio output; each exists.
Automatic migration and sync are explicit non-goals. An AI step would weaken a
deterministic provenance check rather than complete an implied user task. No
decorative AI feature, provider key, Azure endpoint, or direct payment-provider
integration exists in product code.

## What would make this perfect

Nothing remains within the researched scope. Preserve the exact claim tests,
rendered copy audit, sandbox checks, and live route crawl on future changes.
