# Adversarial first-read review 1 — FAIL

Reviewed 2026-08-29 against commit `595e4128056ed55c1fbcf45cc6c7a68e814665f0`
and <https://git-forge-exit-drill.sociobot.in> in fresh Chromium contexts.

## Verdict

**FAIL.** The first read, one-click demo, declared claim tests, core CLI, visual
identity, accessibility baseline, links, and historical repairs work. The
product still has 34 findings. Most importantly, the landing page and README
make product, paid-plan, isolation, and network promises that have no entries
in `.factory/claims.json`; one paid-license sentence promises a restore link
that is not present. PASS requires zero findings and no untested claim.

No finding below is marked BLOCKING under the review's special criteria: the
first screen answers all three questions, the demo is immediate and isolated,
all nine listed claim tests pass, routing is usable, and the design is not a
generic template. Every finding still blocks a PASS verdict.

## Cold first read

I opened the live root in separate fresh contexts at 390×844 and 1440×900 and
recorded the initial viewport before scrolling.

- **What does it do?** It tests a move away from GitHub and reports missing
  history or build evidence before the move.
- **For whom?** Small teams moving to another Git hosting service.
- **What should I click first?** **Try it with sample data**.

The exact first-screen copy was “Test your GitHub exit before cutover,” “For
small teams moving forges, it finds missing history and build evidence before
Monday,” and “Try it with sample data,” followed by “See a complete drill with
no setup.” All three questions are answerable, so this check is not blocking.
The 1440×900 layout still hides the three required fact lines below the fold;
see F-1-19.

## Findings — high

### F-1-1 — README promises a license-restore link that does not exist

- **Quote/location:** README, Team Pack: “Purchase and restore links live on
  the product site.”
- **Why this misleads:** The live site has a checkout link and a form for
  pasting a license already in hand. It has no link or flow to recover a lost
  purchase or license.
- **Fix:** Add a working **Restore Team Pack purchase** link and verify it, or
  rewrite this as “Buy Team Pack or paste an existing license on the product
  site.” Register the remaining purchase claim.

### F-1-2 — temporary-data cleanup is an untestable guarantee

- **Quote/location:** README, Try the bundled drill: “Demo data is removed on
  the next system cleanup and never enters your real workspace.”
- **Why this misleads:** “Next system cleanup” has no defined time or mechanism
  and may never happen. No claim entry tests it.
- **Fix:** Write “The demo writes to the printed temporary directory. Delete
  that directory when finished.” Keep workspace isolation as a separately
  tested claim.

### F-1-3 — CLI demo isolation and non-deletion promises are unlisted claims

- **Quote/location:** README: “The command loads the bundled Atlas Notes
  metadata and creates a validated sample Git mirror in a temporary folder.”;
  “It prints the report and archive paths.”; “If you use `--output`, it must be
  a new or empty directory; the command never deletes an existing directory.”
- **Why this misleads:** These are useful safety promises, but no
  `.factory/claims.json` entry names or scopes them. A normal integration test
  is not a registered claim test.
- **Fix:** Add `cli-demo-isolated` with one tagged test that runs from a temp
  working directory, checks the printed output paths, snapshots that directory,
  and confirms a non-empty `--output` is refused without changing its sentinel.

### F-1-4 — target mapping behavior is an unlisted claim

- **Quote/location:** Landing, Map the target: “Compare each artifact with a
  versioned GitLab, Gitea, or Forgejo map.” README: “The versioned mapping file
  is `mappings/targets.json`.” and “Mappings describe native support, manual
  conversion, and unsupported artifacts.”
- **Why this misleads:** Target-version mapping is central to the result, but
  none of the nine claims promises or directly tests the published mapping.
- **Fix:** Add a `target-mappings` claim and a tagged fixture test that asserts
  the named versions and each support state, or remove “versioned” and narrow
  the copy to behavior already tested.

### F-1-5 — generated restore-checklist behavior is an unlisted claim

- **Quote/location:** Landing, Prove the restore: “Keep encrypted evidence,
  then follow the generated restore checklist.” README output list:
  “`readiness.md`: findings and a restore checklist for people.”
- **Why this misleads:** `encrypted-evidence` tests the archive, not the
  contents or usefulness of a generated restore checklist.
- **Fix:** Add a `restore-checklist` claim whose tagged sample test asserts the
  required restore steps in `readiness.md`, or remove the promise.

### F-1-6 — the non-migration boundary is an unlisted claim

- **Quote/location:** Landing: “The CLI does not cut over repositories,
  forward webhooks, or host a forge.” README: “The tool does not migrate data
  or change either forge.”
- **Why this misleads:** This boundary is important to informed use, but no
  claim test scopes the command's writes or outbound operations broadly enough
  to support it.
- **Fix:** Replace it with the observable statement “The command writes only
  reports and an archive to the output directory,” then add a tagged filesystem
  and request-log test.

### F-1-7 — binary availability is an unlisted claim

- **Quote/location:** Landing action: “Download Linux x86-64 binary.” README:
  “The release site also provides a Linux x86-64 binary.”
- **Why this misleads:** The link currently works, but no claim entry prevents
  a release from shipping a missing, HTML, or non-runnable download.
- **Fix:** Add `linux-download` with a tagged production-build test that checks
  the file exists, is executable, and prints the expected version.

### F-1-8 — the $39 one-time price is an unlisted claim

- **Quote/location:** Landing: “One $39 purchase adds the portfolio command
  and one ordered risk list.”; “Buy Team Pack — $39”; “One-time purchase.”
  README: “A $39 one-time Team Pack license adds the portfolio command for up
  to ten export directories and one consolidated risk list.”
- **Why this misleads:** `team-portfolio` tests what a valid fixture license
  unlocks, not the live amount or one-time billing model.
- **Fix:** Add a billing-contract claim that verifies the Sociobot product is
  active, costs exactly $39 once, and redirects to the expected hosted checkout.

### F-1-9 — device entitlement is an unlisted and undefined claim

- **Quote/location:** Landing Team Pack bullet: “License use on your own
  devices.”
- **Why this misleads:** The copy gives no device limit or definition of “your
  own,” and the Team Pack test does not verify device entitlement.
- **Fix:** State the exact entitlement in Terms and test that license behavior,
  or delete the bullet.

### F-1-10 — merchant-of-record copy is an unlisted claim and uses legal jargon

- **Quote/location:** Landing: “Sociobot is the merchant of record.”
- **Why this misleads:** It is a factual payment promise with no claim entry,
  and “merchant of record” is not explained for a first-time buyer.
- **Fix:** Use “You buy from Sociobot, which handles payment and receipts.” Add
  a checkout-contract test that confirms the merchant shown before payment.

### F-1-11 — complete source-file inclusion is an unlisted claim

- **Quote/location:** README, local export drill: “All regular source files
  enter the evidence archive.”
- **Why this misleads:** `encrypted-evidence` confirms that one known source
  string is hidden and the archive verifies; it does not compare every regular
  input file with archive entries.
- **Fix:** Add `archive-file-completeness` with nested, empty, and binary files,
  then compare the archived path/digest set with the source set.

### F-1-12 — API-mode Git-history limitation is not directly claimed or tested

- **Quote/location:** README, authorized API drill: “API mode inventories
  metadata but does not download Git object bytes, so its readiness report
  remains blocked until you run a local drill with a validated mirror or
  bundle.”
- **Why this misleads:** This critical safety behavior is broader than the
  `evidence-complete` claim and its tagged browser fixture does not assert the
  Git-repository finding or overall blocked outcome for API mode.
- **Fix:** Add `api-metadata-blocks-git` with a local API fixture that asserts
  no Git object bytes, `git_repository.captured=false`, and a blocked report.

### F-1-13 — machine-readable output is an unlisted claim

- **Quote/location:** README: “Use `--json` before the subcommand for a
  machine-readable summary.”
- **Why this misleads:** No listed claim defines the schema or verifies that
  standard output remains parseable JSON.
- **Fix:** Add `json-summary` and parse stdout from success and failure paths in
  its tagged test.

### F-1-14 — error behavior is an unlisted claim

- **Quote/location:** README: “Errors use a non-zero exit code and include one
  next step.”
- **Why this misleads:** This promises behavior across all errors, while no
  claim entry defines which errors are covered.
- **Fix:** Add `actionable-errors` with a table of documented error classes and
  assertions for non-zero status plus exactly one next action, or narrow the
  sentence to tested cases.

### F-1-15 — CLI network destinations and telemetry are only partly registered

- **Quote/location:** README, Privacy and security: “API mode contacts only
  the configured GitHub API origin.”; “Portfolio license checks contact the
  Sociobot billing API.”; “The CLI has no telemetry.”
- **Why this misleads:** `free-single` proves that one local drill survives a
  rejecting proxy, and `no-telemetry` covers the site demo only. Neither claim
  covers all CLI modes or the two stated destinations.
- **Fix:** Add a `cli-network-boundaries` claim with local HTTP fixtures and a
  request log for local, API, and license flows. Narrow “no telemetry” to the
  modes actually observed.

### F-1-16 — “before Monday” is an unlisted timing claim and vague first-screen copy

- **Quote/location:** Landing hero: “For small teams moving forges, it finds
  missing history and build evidence before Monday.”
- **Why this misleads:** The result depends on what day the visitor arrives and
  implies a completion time that no test measures. “Forges” is also unexplained
  jargon on the first screen.
- **Fix:** Write “For small teams changing Git hosts, it finds missing
  repository history and build evidence before cutover.”

## Findings — medium

### F-1-17 — deep-link source metadata describes the home page

- **Quote/location:** Raw HTML at `/demo`, `/privacy`, and `/terms` contains
  `<title>Git Forge Exit Drill — test a GitHub move</title>`, the root canonical,
  root `og:url`, and the home Open Graph/Twitter title and description.
- **Why this misleads:** JavaScript corrects the visible title, description,
  and canonical after load, but link unfurlers and crawlers commonly read the
  initial HTML and will label every shared route as the home page.
- **Fix:** Generate route-specific HTML documents or server-render each route.
  Give every route its own initial title, canonical, Open Graph URL/title/
  description, and Twitter title/description.

### F-1-18 — Back to the Install anchor drops keyboard focus

- **Quote/location:** Live flow `/` → **Install** (`/#install`) → `/demo` →
  browser Back. Scroll returned to `#install`, but `document.activeElement` was
  `BODY`, not the page H1 or install heading.
- **Why this loses a visitor:** A keyboard or screen-reader user gets the right
  scroll position without a corresponding focus position or announcement.
- **Fix:** Handle hash state in `popstate`; after restoring the anchor, focus
  `#install-title` (or consistently focus the H1 and announce the destination).
  Add this exact Back/Forward sequence to Playwright.

### F-1-19 — the three first-screen facts disappear at 1440×900

- **Quote/location:** Live home at 1440×900: the primary action ends at y=852,
  but `.facts` spans y=890–966. At 1280×720 it spans y=628–703 and at 1366×768
  y=653–728.
- **Why this loses a visitor:** The responsive rule stops at 820 px high, so a
  taller desktop unexpectedly shows less of the required first-screen content
  than shorter laptops.
- **Fix:** Make hero sizing continuous rather than switching at 820 px. Assert
  that headline, audience, action, consequence, and all three facts fit at
  1440×900 as well as the current short-desktop sizes.

### F-1-20 — the mobile demo transcript hides the end of important lines

- **Quote/location:** `/demo` at 390×844 clips “Demo — sample data, nothing was
  read from your workspace.” after “worksp…”. The focusable `<pre>` is 356 px
  wide with a 389 px scroll width.
- **Why this loses a visitor:** Horizontal scrolling is possible but has no
  visible cue; the safety statement appears cut off during the 30-second demo.
- **Fix:** Wrap terminal lines at 390 px or shorten this line to “No workspace
  files were read.” Keep horizontal scrolling only for strings that cannot wrap.

### F-1-21 — the repository copy audit is incomplete and miscounts copy

- **Quote/location:** `.factory/copy-audit.md` says headings, buttons, labels,
  and fragments are included, but omits navigation, section-index labels,
  terminal text, alt text, code, form labels, and footer metadata. It counts
  “Try it with sample data” as 6 words (actual 5), “See the gap before it
  becomes downtime” as 8 (actual 7), and “It marks a count captured only when
  valid records back it” as 10 (actual 11).
- **Why this misleads:** The handoff cites this audit as proof of plain copy,
  but it did not inspect all reader-visible or accessible strings and missed
  the violations below.
- **Fix:** Generate the audit from rendered accessible strings, document the
  counting rule, include README sentences, and fail its check when copy changes.

## Findings — minor copy defects

### F-1-22 — the figure caption uses a migration metaphor

- **Quote/location:** Landing figure: “Source artifacts cross the forge
  boundary. Gaps stay visible.”
- **Why this is unclear:** The CLI explicitly does not move artifacts, so
  “cross” can imply migration; “gaps stay visible” does not say where or how.
- **Fix:** Write “The drill maps each GitHub artifact to the target and marks
  unsupported items.” Register the mapping claim described in F-1-4.

### F-1-23 — section labels are decorative factory lore

- **Quote/location:** Landing labels: “FIELD TEST / 001,” “METHOD / 003,”
  “TERMINAL / START,” and “BOUNDARY / CLEAR.”
- **Why this is unclear:** The numbering has no sequence a visitor can use and
  the labels do not name the sections more clearly than their headings.
- **Fix:** Remove them. If labels remain, use literal names such as “Sample
  result,” “How it works,” “Install,” and “Limits.”

### F-1-24 — the sample section heading is a slogan

- **Quote/location:** Landing H2: “See the gap before it becomes downtime.”
- **Why this is unclear:** It describes a mood or hoped-for consequence, not
  the section's content when headings are read alone.
- **Fix:** Rename it “Sample drill results.”

### F-1-25 — the sample explanation has an ambiguous pronoun

- **Quote/location:** Landing: “It marks a count captured only when valid
  records back it.”
- **Why this is unclear:** “It” may refer to the repository or the tool, and
  “marks a count captured” is hard to parse.
- **Fix:** Write “The drill counts an artifact only after it validates the
  exported records.”

### F-1-26 — “Prove the restore” overstates a checklist

- **Quote/location:** Landing H3: “Prove the restore.”
- **Why this misleads:** The product generates a restore checklist; the copy
  does not show that it performs or proves a restore.
- **Fix:** Rename the step “Plan the restore test.”

### F-1-27 — install copy uses the wrong preposition

- **Quote/location:** Landing: “Build from source, then run the same sample
  used in this page.”
- **Why this is awkward:** A sample is shown **on** a page, not “in” it.
- **Fix:** Write “Build from source, then run the sample shown on this page.”

### F-1-28 — the license button does not name its result

- **Quote/location:** Landing button: “Have a license? Paste it.”
- **Why this is unclear:** The control first reveals a form; it does not paste
  anything itself, and the question is not a result-naming verb.
- **Fix:** Label it “Enter Team Pack license.”

### F-1-29 — one README sentence exceeds 22 words

- **Quote/location:** README, local export drill, 23 words: “The CLI parses
  recognized JSON exports such as `issues.json`, `pull_requests.json`,
  `releases.json`, `workflows.json`, and `workflow_runs.json`, then compares
  their actual records with any declared totals.”
- **Why this is hard to scan:** The sentence mixes accepted files and count
  validation.
- **Fix:** “The CLI parses five recognized JSON exports: issues, pull requests,
  releases, workflows, and workflow runs. It compares their record counts with
  the manifest.”

### F-1-30 — the Git-evidence README sentence exceeds 22 words

- **Quote/location:** README, local export drill, 29 words: “A manifest cannot
  prove repository history: to report **Git repository** captured, the export
  must also contain a valid Git bundle or a valid bare/working mirror with Git
  object bytes.”
- **Why this is hard to scan:** It combines the warning, accepted formats, and
  internal evidence requirement.
- **Fix:** “A manifest cannot prove repository history. The export must include
  a valid Git bundle or mirror. The mirror must contain Git objects.”

### F-1-31 — the API limitation README sentence exceeds 22 words

- **Quote/location:** README, authorized API drill, 29 words: “API mode
  inventories metadata but does not download Git object bytes, so its readiness
  report remains blocked until you run a local drill with a validated mirror or
  bundle.”
- **Why this is hard to scan:** The operating limit and required next action
  are buried in one sentence.
- **Fix:** “API mode inventories metadata but does not download Git history.
  The report stays blocked. Run a local drill with a validated mirror or
  bundle.”

### F-1-32 — “Targets” is vague out of context

- **Quote/location:** README H2: “Targets.”
- **Why this is unclear:** A screen-reader heading list does not reveal whether
  these are repositories, services, versions, or goals.
- **Fix:** Rename it “Supported target services and versions.”

### F-1-33 — output names are inconsistent

- **Quote/location:** README introduction says “two readiness reports.” The
  landing Team Pack says “one ordered risk list” and “One consolidated Markdown
  report.”
- **Why this is unclear:** A buyer cannot tell whether a readiness report, risk
  list, and Markdown report are the same artifact.
- **Fix:** Use “readiness report” everywhere; describe its format separately,
  for example “one consolidated readiness report in Markdown.”

### F-1-34 — “forge” and “artifact” are used before plain definitions

- **Quote/location:** Landing says “Migration readiness / CLI,” “moving
  forges,” “forge boundary,” and “each artifact.” README starts with “change
  either forge” and later refers to “artifact totals.”
- **Why this is unclear:** These are domain terms, but the first-time copy does
  not define them and alternates between “forge” and “target.”
- **Fix:** Use “Git host” on first mention and “repository item” in explanatory
  copy. Define “target” and “artifact” once before using the shorter terms.

## Landing-page copy audit

Counts exclude punctuation-only marks, shell prompts, and decorative list
ordinals; count a hyphenated token as one word; and include visible or
accessibility copy, terminal text, headings, controls, labels, and footer text.
Repeated Privacy/Terms labels are listed once per component rather than once
per duplicate occurrence.

| Copy unit | Words | Result |
| --- | ---: | --- |
| Skip to main content | 4 | pass |
| Git Forge Exit Drill home | 5 | pass |
| EXIT/DRILL | 1 | pass |
| Main navigation | 2 | pass |
| Demo | 1 | pass |
| Install | 1 | pass |
| Privacy | 1 | pass |
| Migration readiness / CLI | 3 | F-1-34 |
| Test your GitHub exit before cutover | 6 | pass |
| For small teams moving forges, it finds missing history and build evidence before Monday. | 14 | F-1-16, F-1-34 |
| Try it with sample data | 5 | pass |
| See a complete drill with no setup. | 7 | pass |
| Local exports stay on your machine. | 6 | covered by `free-single` |
| No account is needed. | 4 | covered by `demo-private` |
| One-repository drills are free. | 4 | covered by `free-single` |
| Product facts | 2 | pass |
| A tangled artifact graph crosses a boundary and becomes a checked geometric grid. | 13 | pass (descriptive alt) |
| Source artifacts cross the forge boundary. | 6 | F-1-22, F-1-34 |
| Gaps stay visible. | 3 | F-1-22 |
| FIELD TEST / 001 | 3 | F-1-23 |
| See the gap before it becomes downtime | 7 | F-1-24 |
| The sample repository has code, issues, releases, and build history. | 10 | covered by `recorded-cli` / `evidence-complete` |
| It marks a count captured only when valid records back it. | 11 | F-1-25 |
| Recorded terminal output from the bundled sample drill | 8 | covered by `recorded-cli` |
| Terminal transcript | 2 | pass |
| sample / atlas-notes | 2 | pass |
| $ git-forge-exit-drill demo | 2 | pass |
| Demo — sample data, nothing was read from your workspace. | 9 | covered by `recorded-cli` |
| Repository: acme-labs/atlas-notes | 2 | covered by `recorded-cli` |
| Target: Forgejo 9.0 | 3 | covered by `recorded-cli` |
| Outcome: BLOCKED | 2 | covered by `recorded-cli` |
| Demo archive passphrase: demo-only-passphrase | 4 | covered by `recorded-cli` |
| Choose a new output directory to run this demo again. | 10 | F-1-3 |
| METHOD / 003 | 2 | F-1-23 |
| Run one repeatable drill | 4 | pass |
| Inventory the source | 3 | pass |
| Read an extracted export or an authorized GitHub API repository. | 10 | covered by `evidence-complete` / `token-private` |
| Map the target | 3 | F-1-4 |
| Compare each artifact with a versioned GitLab, Gitea, or Forgejo map. | 11 | F-1-4, F-1-34 |
| Prove the restore | 3 | F-1-26 |
| Keep encrypted evidence, then follow the generated restore checklist. | 9 | F-1-5 |
| TERMINAL / START | 2 | F-1-23 |
| Start with the bundled sample | 5 | pass |
| Build from source, then run the same sample used in this page. | 12 | F-1-27 |
| cargo install --path . | 3 | pass |
| git-forge-exit-drill demo | 2 | pass |
| Copy commands | 2 | pass |
| Download Linux x86-64 binary | 4 | F-1-7 |
| BOUNDARY / CLEAR | 2 | F-1-23 |
| Know what stays untouched | 4 | pass |
| The CLI does not cut over repositories, forward webhooks, or host a forge. | 13 | F-1-6, F-1-34 |
| It reads only the source you provide. | 7 | covered by `source-read-only` |
| Read the privacy details | 4 | pass |
| NO Automatic migration | 3 | pass |
| NO Background service | 3 | pass |
| NO Telemetry | 2 | covered for the demo by `no-telemetry` |
| TEAM PACK / $39 | 3 | F-1-8 |
| Check ten repositories together | 4 | covered by `team-portfolio` |
| One $39 purchase adds the portfolio command and one ordered risk list. | 12 | F-1-8, F-1-33 |
| The complete one-repository drill stays free. | 6 | covered by `free-single` |
| Up to ten local exports per run | 7 | covered by `team-portfolio` |
| One consolidated Markdown report | 4 | covered by `team-portfolio`; F-1-33 |
| License use on your own devices | 6 | F-1-9 |
| Buy Team Pack — $39 | 4 | F-1-8 |
| hosted checkout | 2 | pass |
| One-time purchase. | 2 | F-1-8 |
| Sociobot is the merchant of record. | 6 | F-1-10 |
| Have a license? | 3 | F-1-28 |
| Paste it | 2 | F-1-28 |
| License token | 2 | pass |
| Verify license | 2 | pass |
| Privacy / Terms | 2 | pass |
| Test a GitHub move before cutover. | 6 | pass |
| Built by Param Factory | 4 | pass |
| external | 1 | pass |
| v0.1.0 · build 2026.08.28 | 3 | pass |

No landing unit exceeds 22 words and no banned marketing adjective appears.

## README copy audit

Code blocks are commands rather than sentences and are excluded. Headings and
list fragments are included because they must make sense out of context.

| Copy unit | Words | Result |
| --- | ---: | --- |
| Git Forge Exit Drill | 4 | pass |
| Test a GitHub move before your team cuts over. | 9 | pass |
| Git Forge Exit Drill is a single Rust CLI for small teams planning a move to GitLab, Gitea, or Forgejo. | 20 | F-1-34 |
| It inventories an authorized GitHub export or API repository. | 9 | pass |
| It then writes an encrypted evidence archive and two readiness reports. | 11 | F-1-33 |
| The tool does not migrate data or change either forge. | 10 | F-1-6, F-1-34 |
| Try the bundled drill | 4 | pass |
| The command loads the bundled Atlas Notes metadata and creates a validated sample Git mirror in a temporary folder. | 19 | F-1-3 |
| It prints the report and archive paths. | 7 | F-1-3 |
| Demo data is removed on the next system cleanup and never enters your real workspace. | 15 | F-1-2 |
| If you use `--output`, it must be a new or empty directory; the command never deletes an existing directory. | 19 | F-1-3 |
| Install | 1 | pass |
| Build the single binary from source: | 6 | pass |
| The release site also provides a Linux x86-64 binary. | 9 | F-1-7 |
| Run a local export drill | 5 | pass |
| Set the archive passphrase in an environment variable. | 8 | pass |
| Then point the command at an export directory. | 8 | pass |
| The output directory contains: | 4 | pass |
| `readiness.md`: findings and a restore checklist for people. | 8 | F-1-5 |
| `readiness.json`: the same findings for scripts. | 6 | pass |
| `evidence.gfed`: the source evidence encrypted with AES-256-GCM after an Argon2id key derivation. | 12 | covered by `encrypted-evidence` |
| The source directory may contain a `manifest.json` with expected artifact totals. | 11 | F-1-34 |
| The CLI parses recognized JSON exports such as `issues.json`, `pull_requests.json`, `releases.json`, `workflows.json`, and `workflow_runs.json`, then compares their actual records with any declared totals. | 23 | F-1-29 |
| Invalid JSON, structurally invalid records, absent record files, and count mismatches are reported as incomplete evidence, never as captured data. | 20 | covered by `evidence-complete` |
| A record must carry the identity and restore fields for its artifact, including issue and pull-request author attribution. | 18 | covered by `evidence-complete`; F-1-34 |
| A manifest cannot prove repository history: to report **Git repository** captured, the export must also contain a valid Git bundle or a valid bare/working mirror with Git object bytes. | 29 | F-1-30 |
| The CLI runs `git fsck` (or clones and checks a bundle) before counting it. | 14 | covered by `evidence-complete` |
| All regular source files enter the evidence archive. | 8 | F-1-11 |
| Run an authorized API drill | 5 | pass |
| Create a fine-grained GitHub token with read-only access to the repository metadata you need. | 14 | pass |
| The token is read from the environment and is never written to the report. | 14 | covered by `token-private` |
| API mode inventories metadata but does not download Git object bytes, so its readiness report remains blocked until you run a local drill with a validated mirror or bundle. | 29 | F-1-12, F-1-31 |
| Use `--json` before the subcommand for a machine-readable summary. | 9 | F-1-13 |
| Errors use a non-zero exit code and include one next step. | 11 | F-1-14 |
| Verify an archive | 3 | pass |
| Verification authenticates the archive and checks every recorded file digest. | 10 | covered by `encrypted-evidence` |
| Targets | 1 | F-1-32 |
| The versioned mapping file is `mappings/targets.json`. | 6 | F-1-4 |
| See the installed choices with: | 5 | pass |
| Mappings describe native support, manual conversion, and unsupported artifacts. | 9 | F-1-4, F-1-34 |
| They are a planning baseline, not a promise from a forge vendor. | 12 | F-1-34 |
| Team Pack | 2 | pass |
| The free CLI runs complete one-repository drills. | 7 | covered by `free-single` |
| A $39 one-time Team Pack license adds the portfolio command for up to ten export directories and one consolidated risk list. | 21 | F-1-8, F-1-33 |
| Purchase and restore links live on the product site. | 9 | F-1-1 |
| The CLI reads the license from `GFED_LICENSE` and verifies it with the Sociobot billing API. | 15 | covered for unlock behavior by `team-portfolio`; network part F-1-15 |
| Develop and verify | 3 | pass |
| Requirements: stable Rust, Node 22+, and Chromium for browser checks. | 10 | pass |
| `npm run build` creates the release binary and the static site in `dist/site/`. | 13 | verified by the build gate |
| The site build is also available as `npm run build:site`. | 10 | verified by the build gate |
| Privacy and security | 3 | pass |
| The local export path makes no network request. | 8 | covered by `free-single` |
| API mode contacts only the configured GitHub API origin. | 9 | F-1-15 |
| Portfolio license checks contact the Sociobot billing API. | 8 | F-1-15 |
| The CLI has no telemetry. | 5 | F-1-15 |
| See the site privacy page and terms. | 7 | pass |
| Review source exports before sharing them. | 6 | pass |
| They can include personal data, third-party license text, and secret material. | 11 | pass |
| Do not use a command-line argument for a token or archive passphrase. | 12 | pass |
| License | 1 | pass |
| MIT. | 1 | pass |
| See `LICENSE`. | 2 | pass |

The README has three sentences above 22 words (F-1-29 through F-1-31). No
banned marketing adjective appears.

## Demo and sandbox evidence

**PASS.** One click on **Try it with sample data** opens `/demo`. Its first
screen already shows the Atlas Notes repository, Forgejo 9.0 target, and
`Outcome: BLOCKED`. The persistent banner says “Demo — sample data, nothing is
saved” and offers **Reset demo** and **Start for real**.

In a fresh mobile context I seeded `real:sentinel=keep`. Entering and resetting
the demo changed only `demo:gfed:started`; the real sentinel survived. **Start
for real** removed the demo key and preserved the sentinel. The complete live
request log contained only `https://git-forge-exit-drill.sociobot.in`.

From `/tmp/gfed-review-demo.HjXzKr`, `cargo run --quiet --manifest-path
/work/repo/Cargo.toml -- demo` left the working directory empty, created the
sample under a separately printed `/tmp/git-forge-exit-drill-demo-…` path, and
printed report, JSON, encrypted archive, checksum, and passphrase locations.

## Claims audit

I cloned the repository to `/tmp/gfed-claims-clone.5H3IlI` and ran every exact
`test` string from `.factory/claims.json`. Each command performed its own clean
`npm ci` first.

| Claim | Result | Evidence |
| --- | --- | --- |
| `demo-private` | PASS | 1 tagged Playwright test passed |
| `free-single` | PASS | 1 tagged Playwright test passed with rejecting proxy |
| `source-read-only` | PASS | 1 tagged Playwright test passed |
| `no-telemetry` | PASS | 1 tagged Playwright test passed |
| `recorded-cli` | PASS | 1 tagged Playwright test passed |
| `encrypted-evidence` | PASS | 1 tagged Playwright test passed |
| `evidence-complete` | PASS | 1 tagged Playwright test passed, including page 101 |
| `token-private` | PASS | 1 tagged Playwright test passed |
| `team-portfolio` | PASS | 1 tagged Playwright test passed |

Logs are `/tmp/gfed-claim-<id>.log`. Each claim marker appears once in product
tests. The additional unlisted claim findings are F-1-1 through F-1-16.

## Structure, links, accessibility, and visual identity

- `/`, `/demo`, `/privacy`, and `/terms` return 200. A random path returns the
  designed recovery page with HTTP 404 and a working home action.
- After JavaScript runs, every route has the correct title pattern, one H1, one
  main landmark, a route-specific description and canonical, `lang=en`,
  favicons, and the same header/footer. F-1-17 covers the incorrect initial
  deep-link metadata.
- Every crawled internal asset and page returns 200. The Linux binary returns
  200; the Team Pack checkout reaches hosted Dodo with 200 after redirect;
  Sociobot returns 200. Mail links are explicit.
- Live Axe returned zero violations on `/`, `/demo`, `/privacy`, `/terms`, and
  the 404. Every measured mobile link, button, and input was at least 44×44 px.
  The skip link and 3 px focus ring work. Reduced motion is present.
- `/opt/fleet/lib/verify-url.sh` passed the live root in 599 ms with no console
  errors, `lang=en`, one H1, one main, complete image alt text, and named
  buttons.
- The live root and demo made same-origin requests only. The CSP is delivered
  as a response header and contains `frame-ancestors 'none'`. There are no CDN
  scripts or fonts.
- The evidence-lattice art, clipped geometry, dark inspection palette,
  monospace display type, and coral gap language match `.factory/design.md`.
  This is recognizable product-specific work, not a generic SaaS template.
- Raw JavaScript is 15,681 bytes (5.65 kB gzip), well below the static-product
  limit.

## Historical finding verification

There were no earlier `.factory/review-*.md` or `.factory/polish-*.md` files.
I read the complete prior handoff and all seven verification reports so that
their recorded defects could be checked against the live artifact and current
code.

| Earlier defect | Current confirmation | Status |
| --- | --- | --- |
| Manifest-only Git repository counted as captured | `manifest_cannot_claim_git_repository_without_object_bytes` and `@claim:evidence-complete` pass | fixed |
| Empty bare repository counted as captured | `empty_bare_repository_is_not_captured_as_git_history` passes | fixed |
| Missing, malformed, or arbitrary JSON counted as captured | malformed, null, mixed-record, and structural-invalid tests pass | fixed |
| API pagination stopped at page 100 | `@claim:evidence-complete` observes page 101 and exact counts | fixed |
| Demo deleted a non-empty output | Live binary exits 1 and preserved `sentinel.txt` | fixed |
| Team Pack accepted 11 sources | `portfolio_rejects_eleven_total_sources_before_license_or_output` passes | fixed |
| Checkout returned 404 | Live endpoint redirects to hosted Dodo, final HTTP 200 | fixed |
| Unknown routes returned 200 | Live unknown route returns styled HTTP 404 | fixed |
| Four visitor claims were unregistered | `demo-private`, `source-read-only`, `no-telemetry`, and `recorded-cli` now exist and pass | fixed |
| Mobile controls were under 44 px | No measured interactive target is below 44×44 on all routes | fixed |
| Required first action was below 1280×720 and 1366×768 | Action/facts end at 608/703 and 633/728 respectively | fixed; see new 1440×900 F-1-19 |
| First clean-clone claim command lacked dependencies | All nine exact commands bootstrap with `npm ci` and pass | fixed |

The live `index.html`, JavaScript, CSS, and downloaded Linux binary hashes match
the fresh production build byte for byte.

## Full quality gates

- `npm test`: PASS — 5 Rust unit tests, 13 CLI integration tests, and 21
  Playwright tests.
- `npm run build`: PASS — release binary and `dist/site/` produced.
- Production site JavaScript: 15,681 bytes raw / 5.65 kB gzip.
- Live root at desktop and mobile: no console or page errors.
- Live Axe: zero violations on all routes tested.

## Missed leverage

No missed-leverage finding. The product already imports local exports or
authorized API metadata and exports encrypted evidence plus human/JSON reports.
An AI step would weaken a deterministic, audit-oriented evidence check; no sync
feature is implied by a local exit drill. No decorative AI or embedded provider
key exists.

## What would make this perfect

Resolve F-1-1 through F-1-34, regenerate the complete copy audit, add one tagged
test for every remaining product claim, generate route-specific initial social
metadata, and add the 1440×900 plus hash-history/mobile-terminal regressions.
Then deploy that exact build and rerun this entire review from fresh browser
contexts and a fresh clone. At that point there should be no copy, claim,
structure, demo, accessibility, privacy, or historical finding left.
