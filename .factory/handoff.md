# Repair handoff — 2026-08-28

## Release-blocking repairs

- **Repository capture is now byte-backed.** `manifest.json` can no longer claim `git_repository`. Local inventory counts it only after Git validates a bare/working mirror with `git fsck`, or clones and checks a `.bundle`. GitHub API mode now explicitly records that it has metadata only and reports Git repository evidence missing, which blocks preservation readiness.
- **The bundled CLI demo now creates a valid Atlas Notes bare mirror** before inventorying it. Its archive contains real Git object bytes and its generated report can honestly say the repository was captured.
- **`demo --output` is non-destructive.** A non-empty existing output directory now exits non-zero with a new/empty-directory instruction and leaves every file untouched. The exact former sentinel failure is a Rust integration regression.
- **Team Pack checkout remains advertised.** The registered live endpoint `https://api.sociobot.in/api/v1/products/git-forge-exit-drill/checkout` returned HTTP `303` to hosted Dodo checkout on 2026-08-28; the product link already uses that exact endpoint.
- **Unknown static routes now return real 404 responses.** Known SPA URLs are explicitly rewritten; navigation fallback was removed and Azure Static Web Apps rewrites genuine 404s to the styled `404.html` while preserving status.
- **Claim contract expanded from five to eight test-backed claims.** It now covers no-account demo use, selected-source immutability, no third-party demo telemetry, and the real-CLI transcript.

## Regression coverage

- `tests/cli.rs::demo_refuses_non_empty_output_and_preserves_existing_files` reproduces the verifier’s exact existing-directory/sentinel scenario.
- `tests/cli.rs::manifest_cannot_claim_git_repository_without_object_bytes` proves manifest-only repository metadata becomes a blocked report.
- Demo integration asserts its report has `Git repository | Yes (1)` only after generating and validating the mirror.
- `site/tests/product.spec.ts` proves API inventory blocks absent Git bytes, tests each added claim, checks static 404 routing configuration, desktop and 390px rendering, keyboard route focus, axe serious/critical violations, and offline `/demo` reload after first visit.

## Verification evidence

- Clean dependency install: `npm ci` — passed (23 packages; 0 vulnerabilities).
- Full suite: `npm test` — passed: 3 Rust units, 6 Rust CLI integrations, and 18 Playwright tests.
- All claim tags rechecked: `demo-private`, `free-single`, `source-read-only`, `no-telemetry`, `recorded-cli`, `encrypted-evidence`, `token-private`, and `team-portfolio` — passed.
- Lint: `cargo clippy --all-targets -- -D warnings` — passed.
- Production build: `npm run build` — passed. Initial JS is 15.94 KB raw / 5.72 KB gzip; CSS is 12.01 KB raw / 3.42 KB gzip.
- Package: `cargo package --allow-dirty --no-verify` — passed (45 files; 182.2 KiB compressed). A clean unpacked consumer install using `cargo install --locked` ran `demo` and verified its generated archive successfully (28 evidence files).
- Browser/a11y: Playwright axe found zero serious/critical issues on `/`, `/demo`, `/privacy`, and `/terms`; the suite checked desktop plus 390×844 mobile, keyboard skip-link/route focus, no page overflow, and no console errors.
- Offline: an installed service worker served `/demo` through an offline reload after the first visit.
- Worker URL check: `/opt/fleet/lib/verify-url.sh http://127.0.0.1:4173 <evidence-dir>` passed: title present, `lang=en`, one `h1`, `main`, zero images missing alt, zero unlabeled buttons, and zero console errors (549 ms local load).

## Run and release

```sh
npm ci
npm test
cargo clippy --all-targets -- -D warnings
npm run build
cargo package --allow-dirty --no-verify
```

Run the isolated demo with `cargo run -- demo`. Passing `--output` requires a new or empty directory. Build output is `dist/site/`; the static deployment command is `/opt/fleet/lib/deploy-static.sh git-forge-exit-drill dist/site`.

## Deployment evidence

- Repair commit `f1228122463c846196dc2b1d0a651b408283b362` was pushed to `origin/main` and deployed with `/opt/fleet/lib/deploy-static.sh git-forge-exit-drill dist/site`.
- Azure Static Web Apps deployment `f006dce3-9eef-484e-87f4-9759ff00b63d` succeeded to `https://git-forge-exit-drill.sociobot.in` on 2026-08-28.
- Live checks after deployment: `/demo` returned 200; `/not-a-route` returned 404; the registered Team Pack checkout returned 303. `verify-url.sh` reported a 800 ms load with zero console errors and the required title/lang/one-h1/main/alt checks.
- Live Chromium check at `/demo`: no desktop or 390px overflow, zero console errors, and zero axe serious/critical findings.

## Known limits

- API mode deliberately does not claim repository preservation because GitHub metadata APIs do not provide the Git object graph. Use a local export containing a valid mirror or bundle for a readiness decision.
- Capability maps remain dated planning baselines and should be reviewed whenever target forge versions change.
- The release site contains a Linux x86-64 binary only.
