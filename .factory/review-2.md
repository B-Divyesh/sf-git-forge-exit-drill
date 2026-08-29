# Adversarial first-read review 2 — FAIL

Reviewed 2026-08-29 against commit
`367797ccd548b80d8606d9a727c7d9b7593c01ff` and
<https://git-forge-exit-drill.sociobot.in> in fresh Chromium contexts.

## Verdict

**FAIL.** The product itself is clear, immediately tryable, isolated, and
operational. All 21 declared claim commands pass from a clean clone. The live
routes, links, accessibility checks, build, and CLI demo also pass.

Seven documentation and claim-governance findings remain. One is **BLOCKING**
because earlier finding F-1-21 is only half-fixed: the repository copy audit
states that no sentence exceeds 22 words, but the README contains a 23-word
sentence, and the audit command only compares hashes. PASS requires zero
findings and no untested claims.

## Cold first read

I opened `/` without prior site state at 390×844 and 1440×900. I recorded the
viewport before scrolling.

| Question | First-read answer |
| --- | --- |
| What does this do? | It checks a planned move from GitHub and finds missing repository history or build evidence before cutover. |
| For whom? | Small teams changing Git hosts. |
| What should I click first? | **Try it with sample data**. |

The exact first-screen text is “Test your GitHub move before cutover,” “For
small teams changing Git hosts, it finds missing repository history and build
evidence before cutover,” and “Try it with sample data,” followed by “See a
complete drill with no setup.” All three answers are clear at both widths. The
three facts also fit above the fold at both widths. This check passes.

## Findings

### F-2-1 / F-1-21 reopened — BLOCKING — the copy audit is still not an executable audit

- **Quote/location:** `.factory/copy-audit.md`: “No reader-facing sentence
  exceeds 22 words.” `scripts/check-copy-audit.mjs` only checks whether two
  source hashes appear in that file. It does not extract copy, count words, or
  enforce the 22-word cap. The audit contains grouped descriptions such as
  “price sentence” instead of the promised sentence-by-sentence counts.
- **Counterexample:** README, Team Pack contains a 23-word sentence: “A $39
  one-time Team Pack purchase adds the portfolio command for up to ten export
  directories and one consolidated readiness report in Markdown.” Despite
  that contradiction, `npm run audit:copy` exits 0.
- **Why this matters:** The previous review required a generated audit of each
  rendered string and README sentence. The repair added a source lock but not
  the requested audit. It therefore allows a plain-words regression while
  reporting success.
- **Concrete fix:** Generate `.factory/copy-audit.md` from rendered accessible
  strings and README prose, include every unit and word count, and make
  `npm run audit:copy` fail on a count above 22 or a banned word. Keep the hash
  check only as an additional stale-audit guard.

### F-2-2 — minor — the Team Pack sentence exceeds the hard copy limit

- **Quote/location:** README, **Team Pack**, 23 words: “A $39 one-time Team
  Pack purchase adds the portfolio command for up to ten export directories
  and one consolidated readiness report in Markdown.”
- **Why this matters:** It combines price, command access, repository limit,
  and output format in one sentence. This violates the attached 22-word hard
  cap and is harder to scan on a phone.
- **Concrete rewrite:** “Team Pack costs $39 once. It adds portfolio drills for
  up to ten exports and one consolidated Markdown readiness report.”

### F-2-3 — high — “No automatic migration” is an unlisted claim

- **Quote/location:** Landing, **Limits**: “NO Automatic migration.”
- **Why this matters:** A team may rely on this safety boundary. The
  `output-boundary` test covers local output files and `cli-network-boundaries`
  covers observed request destinations, but no single claim entry states or
  tests that every command avoids writes to a target Git host.
- **Concrete fix:** Add a `no-automatic-migration` claim and a tagged test that
  records every request method and filesystem write for local, API, demo, and
  portfolio commands, or remove the label and retain only the narrower tested
  output statement.

### F-2-4 — high — “No background service” is an unlisted claim

- **Quote/location:** Landing, **Limits**: “NO Background service.”
- **Why this matters:** No `.factory/claims.json` entry promises this, and no
  tagged test confirms that commands terminate without leaving child
  processes, listeners, or scheduled work.
- **Concrete fix:** Add a `no-background-service` claim with a test that runs
  every command class and checks process exit, descendants, and listeners, or
  remove the label.

### F-2-5 — high — the demo’s showcased Forgejo result is not tested

- **Quote/location:** `/demo`: “Past Actions runs cannot become native
  Forgejo history.”
- **Why this matters:** This is the main concrete risk shown after the demo
  action. The `target-mappings` test only checks that each target has at least
  one `native`, `manual`, and `unsupported` value. It would still pass if
  `actions_runs` changed to `native` and another item remained unsupported.
- **Concrete fix:** Add the exact statement to a claim entry and assert that
  Forgejo 9.0 maps `actions_runs` to `unsupported` with the matching reason.
  Also assert that the bundled sample report emits that finding.

### F-2-6 — medium — the one-day license-cache claim is unlisted and untested

- **Quote/location:** `/privacy`, **License storage**: “It stores the last
  verdict for one day.”
- **Why this matters:** `license-browser-storage` verifies immediate cache
  reuse and removal, but does not advance time past 86,400,000 ms or verify a
  fresh request. The quantitative one-day promise is absent from
  `.factory/claims.json`.
- **Concrete fix:** Add `license-cache-ttl` and a tagged fake-clock test that
  confirms reuse before one day and revalidation at or after one day, or omit
  the duration from the page.

### F-2-7 — medium — refund handling and revocation are unlisted claims

- **Quote/location:** `/privacy`: “Sociobot and Dodo handle checkout, receipts,
  refunds, and payment data.” `/terms`: “Approved refunds revoke the license.”
- **Why this matters:** `billing-contract` verifies a checkout redirect, price,
  and one-time order text. It does not verify receipt delivery, a refund path,
  or license revocation after refund. These are post-purchase behaviors a buyer
  may rely on.
- **Concrete fix:** Add a sandboxed billing lifecycle claim that verifies
  receipt, approved refund, and subsequent invalid-license behavior, or narrow
  the copy to the tested hosted-checkout behavior and link to Sociobot’s
  governing refund terms.

## Copy audit

Counting rule: words are whitespace-delimited after punctuation is removed;
hyphenated terms, versions, and prices count as one word. Commands are listed
where they are reader-visible. The landing audit includes headings, controls,
accessible labels, terminal copy, dynamic messages, and footer text. Only
F-2-2 exceeds 22 words. No banned marketing adjective, unexplained mood
heading, inconsistent product term, or non-result-naming button was found.

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
| Primary action | Try it with sample data | 5 | — |
| Sentence | See a complete drill with no setup. | 7 | — |
| Fact | Local drills need no network connection. | 6 | — |
| Fact | Sample data stays in demo storage. | 6 | — |
| Fact | One-repository drills are free. | 4 | — |
| List label | Product facts | 2 | — |
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
| Terminal | Demo — sample data. No workspace files were read. | 8 | — |
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
| Command | cargo install --path . | 3 | — |
| Command | git-forge-exit-drill demo | 2 | — |
| Button | Copy commands | 2 | — |
| Button result | Commands copied | 2 | — |
| Live-region result | Install commands copied | 3 | — |
| Error | Clipboard access was denied. | 4 | — |
| Error | Select the commands above and copy them manually. | 8 | — |
| Link | Download Linux x86-64 binary | 4 | — |
| Section label | Limits | 1 | — |
| H2 | Know what the CLI writes | 5 | — |
| Sentence | The CLI writes reports and an evidence archive to the output directory. | 12 | — |
| Sentence | It does not change your selected export. | 7 | — |
| Link | Read the privacy details | 4 | — |
| Boundary | No automatic migration | 3 | F-2-3 (claim) |
| Boundary | No background service | 3 | F-2-4 (claim) |
| Boundary | No site telemetry | 3 | — |
| Section label | Team Pack | 2 | — |
| H2 | Check ten repositories together | 4 | — |
| Sentence | A $39 one-time purchase adds the portfolio command and one consolidated readiness report. | 13 | — |
| Sentence | The one-repository drill stays free. | 5 | — |
| List item | Up to ten local exports per run | 7 | — |
| List item | One consolidated readiness report in Markdown | 6 | — |
| Button | Buy Team Pack — $39 | 4 | — |
| Link label | hosted checkout | 2 | — |
| Sentence | You buy from Sociobot, which handles payment and receipts. | 9 | — |
| Button | Enter Team Pack license | 4 | — |
| Label | License token | 2 | — |
| Button | Verify license | 2 | — |
| H3 | Use your license in the CLI | 6 | — |
| Sentence | Copy this private token, then set it in the terminal that runs the portfolio command. | 15 | — |
| Label | Team Pack license token | 4 | — |
| Button | Show license | 2 | — |
| Button | Hide license | 2 | — |
| Button | Copy license | 2 | — |
| Sentence | Set the token before running portfolio. | 6 | — |
| Command | export GFED_LICENSE='paste-license-here' | 4 | — |
| Button | Copy setup command | 3 | — |
| Status | Checking license | 2 | — |
| Status | Team Pack license active. | 4 | — |
| Status | License no longer active. | 4 | — |
| Error | License check failed. | 3 | — |
| Error | Connect to the internet and try again. | 7 | — |
| Status | License copied. | 2 | — |
| Live-region result | Team Pack license copied | 4 | — |
| Sentence | Keep it private. | 3 | — |
| Status | Setup command copied. | 3 | — |
| Live-region result | CLI setup command copied | 4 | — |
| Sentence | Run it in your terminal. | 5 | — |
| Error | Select the license token, copy it, then paste it in your terminal. | 12 | — |
| Footer sentence | Test a GitHub move before cutover. | 6 | — |
| Footer link | Privacy | 1 | — |
| Footer link | Terms | 1 | — |
| Footer link | Built by Param Factory | 4 | — |
| Footer label | external | 1 | — |
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
| List item | `evidence.gfed`: source evidence protected with AES-256-GCM after Argon2id key derivation. | 10 | — |
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
| Sentence | A $39 one-time Team Pack purchase adds the portfolio command for up to ten export directories and one consolidated readiness report in Markdown. | 23 | F-2-2 |
| Sentence | Buy Team Pack or enter an existing license on the product site. | 12 | — |
| Sentence | After checkout, copy the shown private license token. | 8 | — |
| Sentence | Set it in the terminal that runs the portfolio command: | 10 | — |
| Sentence | The CLI reads `GFED_LICENSE` and verifies it with the Sociobot billing API. | 13 | — |
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
**target** is the later shorthand; **repository item** is the data category;
**readiness report**, **evidence archive**, **drill**, **demo**, and **Team
Pack** each have one meaning.

## Demo and sandbox

**PASS.** One click on **Try it with sample data** opens `/demo`. The initial
390 px screen already shows the Atlas Notes repository, Forgejo 9.0 target,
`Outcome: BLOCKED`, passphrase, and realistic next actions. The persistent
banner says “Demo — sample data, nothing is saved” and provides **Reset demo**
and **Start for real**.

In a fresh context, entering and resetting the demo touched only
`demo:gfed:started`. Reset replaced that timestamp, retained the sample, kept
focus on the replacement reset button, and announced the reset. A separate
real-data sentinel was preserved, and the demo code path does not read the
license namespace. The complete browser request log contained only
`https://git-forge-exit-drill.sociobot.in`.

The clean-clone CLI demo ran from `/tmp/gfed-review2-demo.hdju9A`, left its
pre-existing `sentinel.txt` unchanged, and wrote its report, JSON report, and
encrypted evidence only below a new
`/tmp/git-forge-exit-drill-demo-*` directory. The isolation claim also proves
that a non-empty explicit output is refused without deleting its sentinel.

## Declared claims

I cloned the reviewed commit to `/tmp/gfed-review2-claims.OfLdB4` and ran every
exact `test` command from `.factory/claims.json`, separately and sequentially.
Each command performed its declared clean `npm ci` first. Claim markers occur
exactly once each.

| Claim | Result | Evidence |
| --- | --- | --- |
| `demo-private` | PASS | Browser demo is immediate, same-origin, and uses the demo namespace. |
| `free-single` | PASS | One-repository drill succeeds with rejecting HTTP/HTTPS proxies. |
| `source-read-only` | PASS | Complete source snapshot stays unchanged; overlapping outputs fail. |
| `no-telemetry` | PASS | Demo request-origin log contains only the site origin. |
| `recorded-cli` | PASS | Six displayed transcript lines match real CLI output. |
| `encrypted-evidence` | PASS | Source text is absent from the archive and verification succeeds. |
| `evidence-complete` | PASS | Invalid, complete, and paginated fixtures produce the expected counts. |
| `token-private` | PASS | Unique API token is absent from all reports and evidence. |
| `team-portfolio` | PASS | Checkout-returned fixture license enables ten-repository portfolio output. |
| `cli-demo-isolated` | PASS | New output paths are printed; occupied output remains untouched. |
| `target-mappings` | PASS | Named versions and the three state values are present. |
| `restore-checklist` | PASS | Generated report contains the required restore steps. |
| `output-boundary` | PASS | Local drill writes the three declared outputs and leaves source unchanged. |
| `linux-download` | PASS | Built download is executable and reports version 0.1.0. |
| `billing-contract` | PASS | Live checkout redirects to the active one-time $39 Dodo order. |
| `archive-file-completeness` | PASS | Nested, empty, and binary source files match archive digests. |
| `api-metadata-blocks-git` | PASS | API-only report lacks Git bytes and remains blocked. |
| `json-summary` | PASS | Success, runtime failure, and parser failures emit parseable JSON. |
| `actionable-errors` | PASS | Tested setup errors exit non-zero with a next action. |
| `cli-network-boundaries` | PASS | Local, API, and license flows stay within their declared network boundaries. |
| `license-browser-storage` | PASS | Returned license is stored, reused, and removable. |

Logs are `/tmp/gfed-review2-claim-<id>.log`. F-2-3 through F-2-7 cover live
claim-like copy that is absent from the registry or materially narrower than
its tagged assertion.

## Historical finding verification

I read all of `.factory/review-1.md`, `.factory/polish-1.md`, and the prior
`.factory/handoff.md`, then checked every earlier finding against the deployed
site and current source.

| Earlier ID | Live and code confirmation | Status |
| --- | --- | --- |
| F-1-1 | README now says buy or enter an existing license; both controls exist. | fixed |
| F-1-2 | README gives an explicit deletion instruction for the printed temp path. | fixed |
| F-1-3 | `cli-demo-isolated` passes and preserves an occupied-output sentinel. | fixed |
| F-1-4 | Named versions and support states are registered and tested. | fixed |
| F-1-5 | `restore-checklist` asserts the generated report steps. | fixed |
| F-1-6 | Broad cutover prose was replaced with the tested output boundary. | fixed; new narrower claim gaps are F-2-3/F-2-4 |
| F-1-7 | Live binary returns 200; the executable version test passes. | fixed |
| F-1-8 | Live checkout and $39 one-time order test pass. | fixed |
| F-1-9 | Undefined device entitlement is absent. | fixed |
| F-1-10 | Plain payment wording replaced merchant jargon; checkout is verified. | fixed |
| F-1-11 | Every nested, empty, and binary source file is compared by digest. | fixed |
| F-1-12 | API-only Git-history block has a dedicated passing claim. | fixed |
| F-1-13 | JSON success and runtime/parser errors parse successfully. | fixed |
| F-1-14 | Registered setup-error cases exit non-zero with a next action. | fixed |
| F-1-15 | Local/API/license request boundaries have a dedicated passing claim. | fixed |
| F-1-16 | Hero now says “changing Git hosts” and “before cutover.” | fixed |
| F-1-17 | Raw HTML for demo/privacy/terms/404 has route-specific metadata. | fixed |
| F-1-18 | Back and Forward focus the visible install H2 and restore its scroll position. | fixed |
| F-1-19 | All hero facts fit at 390×844 and 1440×900. | fixed |
| F-1-20 | Mobile terminal wraps with no horizontal page overflow. | fixed |
| F-1-21 | Audit is grouped, falsely reports the 23-word sentence as passing, and the checker only validates hashes. | **REOPENED — BLOCKING (F-2-1)** |
| F-1-22 | Figure caption now literally describes mapping and unsupported items. | fixed |
| F-1-23 | Section labels are literal: Sample result, How it works, Install, Limits, Team Pack. | fixed |
| F-1-24 | Heading is “Sample drill results.” | fixed |
| F-1-25 | Sample explanation names the drill and its validation action. | fixed |
| F-1-26 | Step is “Plan the restore test.” | fixed |
| F-1-27 | Install copy says “shown on this page.” | fixed |
| F-1-28 | Control is “Enter Team Pack license.” | fixed |
| F-1-29 | The old recognized-export sentence was split below 22 words. | fixed; separate new overlength copy is F-2-2 |
| F-1-30 | Git-history warning is three short sentences. | fixed |
| F-1-31 | API limitation is three short sentences. | fixed |
| F-1-32 | Heading is “Supported target services and versions.” | fixed |
| F-1-33 | “Readiness report” is used consistently, with Markdown as a format. | fixed |
| F-1-34 | “Git host” and “repository item” precede the shorter target terminology. | fixed |

## Structure, accessibility, links, and visual identity

- `/`, `/demo`, `/privacy`, and `/terms` return 200. An unknown route returns
  a designed page with HTTP 404 and a working **Return home** action.
- Every route has the required title pattern, one H1, one main landmark,
  route-specific raw HTML title/description/canonical/Open Graph metadata,
  Twitter card metadata, SVG favicon, and 180×180 apple-touch icon. The Open
  Graph image is 1200×630.
- `robots.txt` and `sitemap.xml` are present; the sitemap lists all four public
  routes. The same header and footer appear on every route with Privacy and
  Terms links.
- Browser Back and Forward restore the home H1 or Install H2 with the target in
  view. Route changes update focus and the polite live region.
- Every discovered internal and external HTTP link returns 200 after redirects;
  the checkout reaches hosted Dodo. Explicit `mailto:` links were excluded
  from HTTP status checks.
- `/opt/fleet/lib/verify-url.sh` passes all four public routes with no console
  or page errors. A live Axe scan reports zero violations on those routes and
  the 404. The full tests also cover 44 px controls, reduced motion, skip-link
  focus, 200% text reflow, and offline demo reload.
- Live page width equals the 390 px viewport on every route. No horizontal page
  overflow was observed.
- The evidence lattice, inspection-grid background, clipped controls, cyan and
  coral signal palette, and monospace display type match `.factory/design.md`.
  This is product-specific and not a generic SaaS template.
- Production JavaScript is 19,823 bytes raw and 6,615 bytes gzip. Live JS, CSS,
  hero art, and binary hashes match the local production build.

## Quality gates

| Check | Result |
| --- | --- |
| Every exact claim command from a clean clone | PASS — 21/21 |
| `npm test` | PASS — 5 Rust unit, 13 CLI integration, 39 Playwright tests |
| `npm run build` | PASS — `dist/site/` and executable Linux binary produced |
| `npm run audit:copy` | **False PASS** — F-2-1; hash-only check misses F-2-2 |
| Live `verify-url.sh` on `/`, `/demo`, `/privacy`, `/terms` | PASS |
| Live Axe on four routes plus 404 | PASS — zero violations |
| Link crawl | PASS — no dead HTTP links |
| Live demo request-origin log | PASS — same origin only |

## Missed leverage

No missed-leverage finding. The CLI already accepts local exports and
authorized API metadata, emits human and JSON reports, creates an encrypted
archive, and supports multi-repository portfolio output. An AI feature would
weaken a deterministic evidence check rather than complete an implied user
step. No decorative AI or embedded provider key exists.

## What would make this perfect

1. Replace the hash-only copy audit with generated, enforceable per-unit
   counts and fix the 23-word Team Pack sentence.
2. Remove or register and test the automatic-migration and background-service
   boundaries.
3. Register and test the demo’s exact Forgejo Actions result, the one-day
   license cache, and the post-purchase refund lifecycle claims.
4. Deploy the exact repaired build and repeat the cold read, demo sandbox,
   claim commands, history matrix, route crawl, and accessibility checks.

At that point there should be no finding or untested claim left.
