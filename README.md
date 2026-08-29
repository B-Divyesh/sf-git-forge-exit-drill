# Git Forge Exit Drill

Test a GitHub move before your team cuts over.

Git Forge Exit Drill is a single Rust CLI for small teams planning a move to GitLab, Gitea, or Forgejo. It inventories an authorized GitHub export or API repository. It then writes an encrypted evidence archive and two readiness reports.

The tool does not migrate data or change either forge.

## Try the bundled drill

```sh
cargo run -- demo
```

The command loads the bundled Atlas Notes metadata and creates a validated sample Git mirror in a temporary folder. It prints the report and archive paths. Demo data is removed on the next system cleanup and never enters your real workspace. If you use `--output`, it must be a new or empty directory; the command never deletes an existing directory.

## Install

Build the single binary from source:

```sh
cargo install --path .
git-forge-exit-drill --help
```

The release site also provides a Linux x86-64 binary.

## Run a local export drill

Set the archive passphrase in an environment variable. Then point the command at an export directory.

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
- `evidence.gfed`: the source evidence encrypted with AES-256-GCM after an Argon2id key derivation.

The source directory may contain a `manifest.json` with expected artifact totals. The CLI parses recognized JSON exports such as `issues.json`, `pull_requests.json`, `releases.json`, `workflows.json`, and `workflow_runs.json`, then compares their actual records with any declared totals. Invalid JSON, structurally invalid records, absent record files, and count mismatches are reported as **incomplete evidence**, never as captured data. A record must carry the identity and restore fields for its artifact, including issue and pull-request author attribution. A manifest cannot prove repository history: to report **Git repository** captured, the export must also contain a valid Git bundle or a valid bare/working mirror with Git object bytes. The CLI runs `git fsck` (or clones and checks a bundle) before counting it. All regular source files enter the evidence archive.

## Run an authorized API drill

Create a fine-grained GitHub token with read-only access to the repository metadata you need. The token is read from the environment and is never written to the report. API mode inventories metadata but does not download Git object bytes, so its readiness report remains blocked until you run a local drill with a validated mirror or bundle.

```sh
export GITHUB_TOKEN='github_pat_...'
export GFED_PASSPHRASE='use-a-long-team-passphrase'
git-forge-exit-drill drill \
  --repo owner/repository \
  --target gitlab:17.0 \
  --output ./exit-drill
```

Use `--json` before the subcommand for a machine-readable summary. Errors use a non-zero exit code and include one next step.

## Verify an archive

```sh
export GFED_PASSPHRASE='use-a-long-team-passphrase'
git-forge-exit-drill verify ./exit-drill/evidence.gfed
```

Verification authenticates the archive and checks every recorded file digest.

## Targets

The versioned mapping file is [`mappings/targets.json`](mappings/targets.json). See the installed choices with:

```sh
git-forge-exit-drill capabilities
```

Mappings describe native support, manual conversion, and unsupported artifacts. They are a planning baseline, not a promise from a forge vendor.

## Team Pack

The free CLI runs complete one-repository drills. A $39 one-time Team Pack license adds the `portfolio` command for up to ten export directories and one consolidated risk list. Purchase and restore links live on the product site. The CLI reads the license from `GFED_LICENSE` and verifies it with the Sociobot billing API.

## Develop and verify

Requirements: stable Rust, Node 22+, and Chromium for browser checks.

```sh
npm install
npm test
npm run build
```

`npm run build` creates the release binary and the static site in `dist/site/`. The site build is also available as `npm run build:site`.

## Privacy and security

The local export path makes no network request. API mode contacts only the configured GitHub API origin. Portfolio license checks contact the Sociobot billing API. The CLI has no telemetry. See the site [privacy page](https://git-forge-exit-drill.sociobot.in/privacy) and [terms](https://git-forge-exit-drill.sociobot.in/terms).

Review source exports before sharing them. They can include personal data, third-party license text, and secret material. Do not use a command-line argument for a token or archive passphrase.

## License

MIT. See [`LICENSE`](LICENSE).
