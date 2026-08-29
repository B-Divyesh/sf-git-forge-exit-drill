# Adversarial first-read review 3 — FAIL

Reviewed 2026-08-29 against commit
`9371a8ccae8d056da1ca6d56cb6adc5599bf4e44` and
<https://git-forge-exit-drill.sociobot.in> in fresh Chromium contexts.

## Verdict

**FAIL.** The cold read, one-click sandbox, CLI, all 22 registered claim
commands, routing, accessibility, links, build, and visual identity pass.
Three claim-governance findings remain. F-3-1 reopens historical finding
F-2-7 as **BLOCKING**: the repair record says receipt promises were removed,
but the live landing page still promises that Sociobot handles receipts and no
registered claim tests receipt delivery. The README also makes two narrower,
untested implementation promises about the demo Git mirror and archive
algorithms. PASS requires zero findings and no untested claim.

## Findings

### F-3-1 / F-2-7 reopened — BLOCKING — the receipt promise remains unlisted and untested

- **Exact quote/location:** landing page, Team Pack: “You buy from Sociobot,
  which handles payment and receipts.”
- **Historical evidence:** F-2-7 found untested receipt, refund, and revocation
  promises. `.factory/polish-2.md` says it “removed receipt, refund, and
  revocation lifecycle promises.” Refund and revocation copy is gone, but the
  receipt promise remains live and in `site/src/main.ts`.
- **Why this misleads:** `billing-contract` proves the active $39 one-time
  checkout and its order summary. It does not complete a purchase or verify
  that a receipt is issued. A buyer can rely on “handles receipts,” while the
  claim registry and sandbox cannot prove it.
- **Concrete fix:** write “You buy from Sociobot through its hosted checkout.”
  Alternatively, register `purchase-receipt` and test a sandbox purchase
  through receipt issuance without live spend.

### F-3-2 — high — the validated demo Git mirror is an unlisted claim

- **Exact quote/location:** README, **Try the bundled drill**: “It creates a
  validated sample Git mirror and prints its report and archive paths.”
- **Why this misleads:** `cli-demo-isolated` tests paths and non-deletion, while
  `recorded-cli` compares six terminal lines. Neither registered claim asserts
  that the demo-created mirror passes Git validation or that the demo report
  records Git repository history as captured. Both claim commands could pass
  after this part of the sample regressed.
- **Concrete fix:** add `demo-valid-git-mirror` to `.factory/claims.json`. Its
  tagged test should run `git fsck` on the generated mirror and assert
  `git_repository.captured=true` in the demo readiness JSON. Otherwise delete
  “validated sample Git mirror.”

### F-3-3 — high — the named archive algorithms are stronger than the registered encryption claim

- **Exact quote/location:** README, output list: “`evidence.gfed`: source
  evidence protected with AES-256-GCM after Argon2id key derivation.”
- **Why this misleads:** `encrypted-evidence` promises only authenticated
  encryption. Its tagged browser test checks that one plaintext string is
  absent and that the product's verifier accepts the archive. The always-run
  Rust unit test also rejects a wrong password. Nothing registered asserts the
  named cipher and KDF, so an algorithm change could leave every claim command
  green while this sentence became false.
- **Concrete fix:** either change the sentence to “`evidence.gfed`: source
  evidence protected with authenticated encryption,” or expand the claim and
  tagged test to assert the archive format identifies AES-256-GCM and Argon2id,
  including authentication failure after ciphertext tampering.

## Cold first read

I opened `/` at 390×844 and 1440×900 in separate, storage-empty browser
contexts and recorded the viewport before scrolling.

| Question | First-read answer |
| --- | --- |
| What does this do? | It tests a planned GitHub move and finds missing repository history or build evidence before cutover. |
| For whom? | Small teams changing Git hosts. |
| What should I click first? | **Try it with sample data**. |

The exact first-screen copy is “Test your GitHub move before cutover,” “For
small teams changing Git hosts, it finds missing repository history and build
evidence before cutover,” and “Try it with sample data,” followed by “See a
complete drill with no setup.” All three facts are also visible without
scrolling. The facts end at y=774 in the 844 px mobile viewport and y=730 in
the 900 px desktop viewport. This check passes.

## Copy audit

Counting rule: a word is a whitespace-delimited token after punctuation is
removed. Hyphenated terms, versions, prices, and commands count as one word.
No sentence exceeds 22 words. No banned marketing adjective, mood heading,
metaphor heading, inconsistent product term, or non-result-naming action was
found. “Claim” flags below are the three untested promises listed above.

### Landing page

| Type | Copy | Words | Flag |
| --- | --- | ---: | --- |
| Eyebrow | Git host migration check | 4 | — |
| H1 | Test your GitHub move before cutover | 6 | — |
| Sentence | For small teams changing Git hosts, it finds missing repository history and build evidence before cutover. | 16 | — |
| Action | Try it with sample data | 5 | — |
| Sentence | See a complete drill with no setup. | 7 | — |
| Fact | Local drills need no network connection. | 6 | — |
| Fact | Sample data stays in demo storage. | 6 | — |
| Fact | One-repository drills are free. | 4 | — |
| Image alt | A GitHub repository-item graph maps to a checked target grid with unsupported items marked. | 14 | — |
| Caption | The drill maps each GitHub repository item to the target and marks unsupported items. | 14 | — |
| Section label | Sample result | 2 | — |
| H2 | Sample drill results | 3 | — |
| Sentence | The sample repository has code, issues, releases, and build history. | 10 | — |
| Sentence | The drill counts an item only after it validates exported records. | 11 | — |
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
| Action | Download Linux x86-64 binary | 4 | — |
| Section label | Limits | 1 | — |
| H2 | Know what the CLI writes | 5 | — |
| Sentence | The CLI writes reports and an evidence archive to the output directory. | 12 | — |
| Sentence | It does not change your selected export. | 7 | — |
| Action | Read the privacy details | 4 | — |
| Boundary | Local output stays in your chosen directory | 7 | — |
| Boundary | CHECK Review the readiness report before cutover | 7 | — |
| Boundary | No sample demo telemetry | 4 | — |
| Section label | Team Pack | 2 | — |
| H2 | Check ten repositories together | 4 | — |
| Sentence | A $39 one-time purchase adds the portfolio command and one consolidated readiness report. | 13 | — |
| Sentence | The one-repository drill stays free. | 5 | — |
| List item | Up to ten local exports per run | 7 | — |
| List item | One consolidated readiness report in Markdown | 6 | — |
| Action | Buy Team Pack — $39 | 4 | — |
| Sentence | You buy from Sociobot, which handles payment and receipts. | 9 | F-3-1 / F-2-7 |
| Action | Enter Team Pack license | 4 | — |
| Form label | License token | 2 | — |
| Action | Verify license | 2 | — |
| H3 | Use your license in the CLI | 6 | — |
| Sentence | Copy this private token, then set it in the terminal that runs the portfolio command. | 15 | — |
| Form label | Team Pack license token | 4 | — |
| Actions | Show license / Hide license / Copy license | 2 / 2 / 2 | — |
| Sentence | Set the token before running portfolio. | 6 | — |
| Action | Copy setup command | 3 | — |
| Status | Checking license. | 2 | — |
| Status | Team Pack license active. | 4 | — |
| Status | License no longer active. | 4 | — |
| Error | License check failed. | 3 | — |
| Error | Connect to the internet and try again. | 7 | — |
| Status | License copied. | 2 | — |
| Sentence | Keep it private. | 3 | — |
| Status | Setup command copied. | 3 | — |
| Sentence | Run it in your terminal. | 5 | — |
| Error | Select the license token, copy it, then paste it in your terminal. | 12 | — |
| Error | Clipboard access was denied. | 4 | — |
| Error | Select the commands above and copy them manually. | 8 | — |
| Footer sentence | Test a GitHub move before cutover. | 6 | — |

Navigation and other result-naming controls are **Demo**, **Install**,
**Privacy**, **Terms**, and **Built by Param Factory**. The install command,
license environment command, version/build label, and decorative terminal
labels are fragments rather than sentences; the generated repository audit
still counts them.

### README

| Type | Copy | Words | Flag |
| --- | --- | ---: | --- |
| H1 | Git Forge Exit Drill | 4 | — |
| Sentence | Test a GitHub move before your team cuts over. | 9 | — |
| Sentence | Git Forge Exit Drill is a Rust command-line tool for small teams changing Git hosts. | 15 | — |
| Sentence | It checks an authorized GitHub export or API repository, writes an encrypted evidence archive, and creates readiness reports before cutover. | 20 | — |
| H2 | Try the bundled drill | 4 | — |
| Sentence | The command copies the bundled Atlas Notes sample into a new temporary directory. | 13 | — |
| Sentence | It creates a validated sample Git mirror and prints its report and archive paths. | 14 | F-3-2 |
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
| List item | `evidence.gfed`: source evidence protected with AES-256-GCM after Argon2id key derivation. | 10 | F-3-3 |
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

Terminology is consistent: **Git host** introduces the destination;
**target** is its later shorthand; **repository item** is a data category;
**readiness report**, **evidence archive**, **drill**, **demo**, and **Team
Pack** retain one meaning.

## Demo and sandbox

**PASS.** One click on **Try it with sample data** opens `/demo`. The initial
390 px viewport already shows Atlas Notes, Forgejo 9.0, `Outcome: BLOCKED`, the
archive passphrase, and a real finding. The persistent banner says “Demo —
sample data, nothing is saved” and provides **Reset demo** and **Start for
real**.

In a fresh context, the demo created only `demo:gfed:started`. Reset replaced
that value, kept focus on **Reset demo**, announced the reset, and left a
separate `real:sentinel` value unchanged. **Start for real** removed the demo
key, retained the real sentinel, opened `/#install`, and focused “Start with
the bundled sample.” Direct `?demo=1` entry normalized to `/demo`. An offline
reload after the first visit retained the complete demo. The whole request log
contained only `https://git-forge-exit-drill.sociobot.in`.

The CLI demo ran from `/tmp/gfed-review3-demo.*` with rejecting HTTP, HTTPS,
and all-proxy endpoints. Its working-directory sentinel remained the only file
there. The command created a new `/tmp/git-forge-exit-drill-demo-*` tree with
the sample export, valid Git objects, `readiness.md`, `readiness.json`, and
`evidence.gfed`; it printed each output path.

## Registered claims

I cloned the candidate to a new temporary directory and ran every exact `test`
command in `.factory/claims.json` separately and sequentially. Each command
performed its declared `npm ci`. Every marker occurs once.

| Claim | Result | Observable evidence |
| --- | --- | --- |
| `demo-private` | PASS | Fresh demo stayed same-origin and used only the demo key. |
| `free-single` | PASS | One-repository drill completed with rejecting proxies and no license. |
| `source-read-only` | PASS | Source snapshot stayed identical; overlapping output failed. |
| `no-telemetry` | PASS | Demo request origins contained only the site origin. |
| `recorded-cli` | PASS | Six displayed transcript lines matched real CLI output. |
| `encrypted-evidence` | PASS | Known plaintext was absent; archive verification and wrong-password rejection passed. |
| `evidence-complete` | PASS | Invalid, mixed, complete, and paginated records produced the expected counts. |
| `token-private` | PASS | Fixture API token was absent from every output. |
| `team-portfolio` | PASS | Returned fixture license enabled an installed CLI to report ten repositories. |
| `cli-demo-isolated` | PASS | Printed paths were isolated and an occupied-output sentinel survived. |
| `target-mappings` | PASS | Three named versions and native/manual/unsupported states matched. |
| `forgejo-actions-history` | PASS | Forgejo 9.0 mapping and sample report both mark old Actions runs unsupported. |
| `restore-checklist` | PASS | Generated Markdown contained the required restore steps. |
| `output-boundary` | PASS | Local drill created the three declared outputs and did not change source. |
| `linux-download` | PASS | Production download was executable and reported version 0.1.0. |
| `billing-contract` | PASS | Live checkout redirected to the active one-time $39 order. |
| `archive-file-completeness` | PASS | Nested, empty, and binary source files matched archive digests. |
| `api-metadata-blocks-git` | PASS | API-only report lacked Git bytes and remained blocked. |
| `json-summary` | PASS | Success and documented failures emitted parseable JSON. |
| `actionable-errors` | PASS | Registered setup errors exited non-zero with one next step. |
| `cli-network-boundaries` | PASS | Local/API/license flows stayed within their declared network origins. |
| `license-browser-storage` | PASS | Returned license was stored, reused, and removable. |

F-3-1 through F-3-3 are not failures of those commands; they are live or
README promises that are absent from, or materially stronger than, the claim
registry and its tagged assertions.

## Historical finding verification

I read both earlier reviews, both polish records, and the prior handoff. I then
checked each finding against the live deployment and current code.

| Earlier ID | Live and code confirmation | Status |
| --- | --- | --- |
| F-1-1 | README names the real buy-or-enter-license path; both controls exist. | fixed |
| F-1-2 | README tells the user to delete the printed temporary directory. | fixed |
| F-1-3 | Registered demo isolation preserves an occupied-output sentinel. | fixed |
| F-1-4 | Named target versions and support states are registered and tested. | fixed |
| F-1-5 | The generated restore checklist has a dedicated passing claim. | fixed |
| F-1-6 | Broad migration prose is replaced by the tested output boundary. | fixed |
| F-1-7 | Live and built Linux downloads execute with version 0.1.0. | fixed |
| F-1-8 | Active one-time $39 checkout is registered and passes. | fixed |
| F-1-9 | Undefined device-entitlement copy is absent. | fixed |
| F-1-10 | Merchant jargon is gone and checkout ownership is registered; receipt scope is tracked under reopened F-2-7. | fixed |
| F-1-11 | Nested, empty, and binary source files are compared with archive digests. | fixed |
| F-1-12 | API-only Git-history blocking has a dedicated passing claim. | fixed |
| F-1-13 | JSON success and error output parses in the tagged test. | fixed |
| F-1-14 | Registered setup errors exit non-zero with a next action. | fixed |
| F-1-15 | CLI network destinations have a dedicated passing claim. | fixed |
| F-1-16 | Hero says “changing Git hosts” and “before cutover.” | fixed |
| F-1-17 | Raw demo/privacy/terms/404 documents have route-specific metadata. | fixed |
| F-1-18 | Back to Install restores focus to the visible H2. | fixed |
| F-1-19 | All required hero facts fit at 390×844 and 1440×900. | fixed |
| F-1-20 | Mobile terminal wraps; page width remains 390 px. | fixed |
| F-1-21 | Audit now renders routes, splits sentences, counts words, rejects banned words, and detects stale output. | fixed |
| F-1-22 | Caption literally describes mapping and unsupported items. | fixed |
| F-1-23 | Section labels are literal names. | fixed |
| F-1-24 | Heading is “Sample drill results.” | fixed |
| F-1-25 | Sample explanation names the drill and validation action. | fixed |
| F-1-26 | Step is “Plan the restore test.” | fixed |
| F-1-27 | Install copy says the sample is shown “on” the page. | fixed |
| F-1-28 | Control is “Enter Team Pack license.” | fixed |
| F-1-29 | Recognized-export sentence is under 22 words. | fixed |
| F-1-30 | Git-history warning is split into short sentences. | fixed |
| F-1-31 | API limitation is split into short sentences. | fixed |
| F-1-32 | Heading names supported target services and versions. | fixed |
| F-1-33 | “Readiness report” is used consistently. | fixed |
| F-1-34 | “Git host” and “repository item” are introduced before shorthand. | fixed |
| F-2-1 / F-1-21 | `audit:copy` is now an executable rendered-copy and README audit. | fixed |
| F-2-2 | Team Pack scope is split into sentences of 5 and 15 words. | fixed |
| F-2-3 | “No automatic migration” is absent. | fixed |
| F-2-4 | “No background service” is absent. | fixed |
| F-2-5 | Exact Forgejo Actions-history behavior is registered and passes. | fixed |
| F-2-6 | One-day cache duration is absent from public copy. | fixed |
| F-2-7 | Refund and revocation copy is gone, but the landing receipt promise remains unregistered despite the polish record saying receipt promises were removed. | **REOPENED — BLOCKING (F-3-1)** |

## Structure, accessibility, links, and identity

- `/`, `/demo`, `/privacy`, and `/terms` return 200. An unknown route returns
  a designed evidence-lattice page with HTTP 404 and a working **Return home**
  link.
- Every route has the required title pattern, one H1, one main landmark,
  `lang="en"`, route-specific raw title/description/canonical/Open Graph and
  Twitter metadata, SVG favicon, apple-touch icon, and shared header/footer.
  `robots.txt` and `sitemap.xml` are present and list all public routes.
- Browser Back restored `/#install`, scroll, and focus on “Start with the
  bundled sample.” Forward restored `/demo` and focused its H1. Direct deep
  links and reloads retained the correct route.
- Every discovered expected HTTP link returned 200 after redirects. The
  checkout reached hosted Dodo. `mailto:` links were identified explicitly.
  The skip link on the intentionally missing URL naturally retains that 404.
- `/opt/fleet/lib/verify-url.sh` passed all four public routes with no console
  or page errors. Independent Axe scans reported zero WCAG A/AA violations on
  all four routes and the 404. No route overflowed 390 px.
- The full browser suite covers keyboard focus, 44 px controls, 200% text,
  reduced motion, service-worker updates, and offline reload. It passed.
- Initial production JavaScript is 19,755 bytes raw and 6.51 kB gzip. Live JS,
  CSS, and Linux binary hashes match the clean build exactly.
- The dark inspection grid, clipped controls, cyan/coral evidence signals,
  monospace type, and original source-to-target lattice match
  `.factory/design.md`. This is distinct from a generic SaaS template.

## Quality gates

| Check | Result |
| --- | --- |
| Every exact claim command from a clean clone | PASS — 22/22 |
| `npm test` | PASS — 5 Rust unit, 13 Rust integration, 40 Playwright tests |
| `npm run build` | PASS — `dist/site/` and executable Linux binary produced |
| `cargo fmt --all -- --check` | PASS |
| `cargo clippy --all-targets --all-features -- -D warnings` | PASS |
| Live verifier on `/`, `/demo`, `/privacy`, `/terms` | PASS |
| Live Axe on four routes plus 404 | PASS — zero violations |
| Live link crawl | PASS — no dead expected link |
| Live demo request log and offline reload | PASS — same-origin only; reload works offline |

## Missed leverage

No missed-leverage finding. The product accepts local export evidence and
authorized API metadata, creates human and JSON reports, builds an encrypted
archive, maps three target services, and offers ten-repository portfolio
output. The brief does not imply sync or automatic migration. AI would weaken
a deterministic evidence check rather than complete a missing user step. No
decorative AI or embedded provider key exists.

## What would make this perfect

1. Remove “and receipts,” or register and prove receipt issuance in a no-spend
   sandbox purchase.
2. Register and test that the CLI demo creates a Git-valid mirror whose history
   is captured in the readiness report, or remove that README promise.
3. Align the README's named archive algorithms with a claim and tamper test, or
   use the narrower tested phrase “authenticated encryption.”
4. Deploy the repaired candidate and repeat the cold read, all exact claim
   commands, copy reconciliation, sandbox checks, route crawl, and historical
   finding matrix.

Only then is there actually nothing left to do.
