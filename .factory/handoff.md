# Build handoff

## What shipped

- Rust 0.1.0 single binary with `drill`, `demo`, `verify`, `capabilities`, and licensed `portfolio` commands.
- Local export inventory with an optional explicit manifest and tolerant common-file discovery.
- Authorized GitHub REST inventory with pagination and clear unavailable-scope findings.
- Authenticated evidence archives using Argon2id key derivation and AES-256-GCM encryption.
- SHA-256 file checks during archive verification.
- Versioned GitLab 17.0, Gitea 1.22, and Forgejo 9.0 capability maps.
- Markdown and JSON readiness reports with a restore checklist.
- Bundled Atlas Notes demo in an isolated temporary directory.
- $39 one-time Team Pack portfolio command for up to ten repositories.
- Sociobot checkout, returned-license storage, restore form, daily verdict cache, and CLI verification.
- Responsive static site with `/`, `/demo`, `/privacy`, `/terms`, and styled unknown-route views.
- Original generated geometry art, local metadata assets, a service worker, and security headers.

## Run and build

```sh
npm install
npm test
npm run build
```

The exact deployment command is `npm run build`. Static output lands in `dist/site/`, with `index.html` at that root. The release binary is `dist/site/downloads/git-forge-exit-drill-linux-x86_64`.

Run the product demo with:

```sh
cargo run -- demo
```

The demo passphrase is `demo-only-passphrase` and protects sample data only.

## Verification completed

- `npm test`: passed.
- Rust: 3 unit tests and 4 CLI integration tests passed.
- Browser: 12 Playwright tests passed in Chromium 1.58.2.
- All five `.factory/claims.json` claim commands are covered by unique tagged tests.
- Playwright axe checks found no serious or critical issues on `/`, `/demo`, `/privacy`, or `/terms`.
- `/opt/fleet/lib/verify-url.sh`: passed with title, `lang`, one `h1`, `main`, alt text, and zero console errors.
- `cargo clippy --all-targets -- -D warnings`: passed.
- `npm audit --audit-level=high`: no vulnerabilities.
- `cargo package --allow-dirty --no-verify`: produced a 176.2 KiB compressed source package.
- Production build: passed. Initial JS is 15.94 KiB raw / 5.72 KiB gzip. CSS is 12.01 KiB raw / 3.42 KiB gzip. Hero WebP is 61.4 KiB.
- Lighthouse 13 mobile simulation: performance 100, accessibility 100, best practices 100, SEO 100. FCP 0.9 s, LCP 1.5 s, TBT 0 ms, CLS 0.
- Desktop and 390×844 screenshots were reviewed. Content remained readable with no page-level horizontal overflow.

## Known gaps and next steps

- The factory still needs to register the paid product before checkout can complete.
- Capability maps are dated planning baselines. Recheck and update them when target forge versions change.
- API mode archives repository and artifact metadata. It does not download Git object packs or release binaries. Use a local mirror/export when those bytes must enter the evidence archive.
- GitHub API collection stops after 100 pages per artifact type. This covers the intended small-team scope but should be raised for very large repositories.
- Only a Linux x86-64 binary is placed on the static site. The release pipeline can add macOS, Windows, ARM, and checksums.
- Lighthouse ran against the local production preview. Repeat it against the deployed HTTPS URL after factory deployment.
