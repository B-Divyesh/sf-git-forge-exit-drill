# Git Forge Exit Drill

Test a GitHub move before your team cuts over.

Git Forge Exit Drill is a Rust command-line tool for small teams changing Git
hosts. It checks an authorized GitHub export or API repository, writes an
encrypted evidence archive, and creates readiness reports before cutover.

## Try the bundled drill

```sh
cargo run -- demo
```

The command copies the bundled Atlas Notes sample into a new temporary
directory. It creates a validated sample Git mirror and prints its report and
archive paths. The demo does not read your workspace. Delete the printed
temporary directory when finished.

With `--output`, choose a new or empty directory. The command refuses a
non-empty directory without changing it.

## Install

Build the single binary from source:

```sh
cargo install --path .
git-forge-exit-drill --help
```

The release site provides a Linux x86-64 binary. The download test checks that
the production build includes an executable binary with the expected version.

## Run a local export drill

Set the archive passphrase in an environment variable. Then point the command
at an export directory.

```sh
export GFED_PASSPHRASE='use-a-long-team-passphrase'
git-forge-exit-drill drill \
  --source ./github-export \
  --target forgejo:9.0 \
  --output ./exit-drill
```

The output directory contains:

- `readiness.md`: findings and a restore checklist for people.
- `readiness.json`: the same findings for scripts.
- `evidence.gfed`: source evidence protected with authenticated encryption.

Choose an output directory outside the selected export. The CLI refuses an
overlapping path so the source stays read-only.

The source directory may contain `manifest.json` with expected repository-item
counts. The CLI parses five recognized JSON exports: issues, pull requests,
releases, workflows, and workflow runs. It compares their record counts with
the manifest. Invalid JSON, invalid records, absent files, and count mismatches
are incomplete evidence, never captured data.

A manifest cannot prove repository history. The export must include a valid Git
bundle or mirror. The mirror must contain Git objects. The CLI validates it
with Git before counting it. Every regular source file enters the evidence
archive.

## Run an authorized API drill

Create a fine-grained GitHub token with read-only access to the repository
metadata you need. The token is read from the environment and never written to
the reports or evidence archive. API mode inventories metadata but does not
download Git history. The report stays blocked. Run a local drill with a
validated mirror or bundle.

```sh
export GITHUB_TOKEN='github_pat_...'
export GFED_PASSPHRASE='use-a-long-team-passphrase'
git-forge-exit-drill drill \
  --repo owner/repository \
  --target gitlab:17.0 \
  --output ./exit-drill
```

Use `--json` before the subcommand for a JSON summary or error. Documented
errors exit non-zero and give one next step.

## Verify an archive

```sh
export GFED_PASSPHRASE='use-a-long-team-passphrase'
git-forge-exit-drill verify ./exit-drill/evidence.gfed
```

Verification authenticates the archive and checks every recorded file digest.

## Supported target services and versions

The versioned mapping file is [`mappings/targets.json`](mappings/targets.json).
A target is the Git host you plan to move to. A repository item is a category
such as issues or releases. The maps mark each item as native, manual, or
unsupported for GitLab 17.0, Gitea 1.22, and Forgejo 9.0.

```sh
git-forge-exit-drill capabilities
```

## Team Pack

The free CLI runs one-repository drills. Team Pack costs $39 once. It adds
`portfolio` drills for up to ten exports and one consolidated Markdown
readiness report. Buy Team Pack or enter an existing license on the product
site. After checkout, copy the shown private license token. Set it in the
terminal that runs the portfolio command:

```sh
export GFED_LICENSE='paste-license-here'
```

The CLI reads `GFED_LICENSE` and verifies it with the Sociobot billing API.

## Develop and verify

Requirements: stable Rust, Node 22+, and Chromium for browser checks.

```sh
npm ci
npm test
npm run build
```

`npm run build` creates the release binary and static site in `dist/site/`.
The site build is also available as `npm run build:site`.

## Privacy and security

Local export drills make no network requests. API drills contact only the
configured GitHub API origin. Portfolio license checks contact only the
Sociobot billing API. The CLI makes no telemetry requests in these flows. See
the site [privacy page](https://git-forge-exit-drill.sociobot.in/privacy) and
[terms](https://git-forge-exit-drill.sociobot.in/terms).

Review source exports before sharing them. They can include personal data,
third-party license text, and secret material. Do not use command-line
arguments for a token or archive passphrase.

## License

MIT. See [`LICENSE`](LICENSE).
