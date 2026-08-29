# Repair handoff — Git Forge Exit Drill

## Outcome

The release blockers in independent verification 9 were repaired from base
`a689c6335712bc62fd10868bc4755fca71915ede`. The CLI and static landing site
remain the original artifact and deployment classes.

## Repairs

### F-9-1 — checkout license handoff to the CLI

- A valid returned `?license=` callback still saves the token locally and
  removes it from the address bar.
- After verification, the Team Pack panel exposes a masked, labelled token
  control. A buyer can explicitly show it, copy it, or copy a safely quoted
  `export GFED_LICENSE='…'` setup command.
- The handoff explains that `GFED_LICENSE` is needed for `portfolio`; README
  now documents the same terminal step.
- The token is not rendered as page text by default. Removing the saved
  license clears the handoff control too.
- The `@claim:team-portfolio` regression begins at a mocked checkout return,
  copies the browser token and setup command, installs the CLI into a new
  temporary prefix, then runs a ten-source `portfolio` report against a
  recorded valid billing response.

### F-9-2 — malformed license cache

- Browser verdict-cache parsing is now defensive. Invalid JSON or an invalid
  cache shape is deleted and treated as a cache miss.
- A regression seeds `{not json`, reloads, asserts one fresh verification,
  an active verdict, a valid replacement cache, and no page errors.

### F-9-3 — reset-demo keyboard focus

- Resetting the demo now focuses the replacement **Reset demo** button after
  the DOM is rendered.
- A 390 px keyboard regression focuses the control, presses Space, and
  asserts both restored focus and the reset announcement.

## Verification evidence

All verification ran in `/work/repo` on 2026-08-29.

| Check | Result |
| --- | --- |
| `npm ci --ignore-scripts --no-audit --no-fund` | pass |
| `npm test` | pass: 5 Rust unit, 13 Rust CLI integration, 38 Playwright tests |
| Every one of 21 declared claim commands, separately and sequentially | pass |
| `npm run typecheck` | pass |
| `npm run audit:copy` | pass |
| `cargo fmt --all -- --check` | pass |
| `cargo clippy --all-targets --all-features -- -D warnings` | pass |
| `npm run build` | pass; creates `dist/site/` and executable Linux binary |
| `cargo package --locked --allow-dirty` | pass: 75 files, 3.1 MiB; verification build passed |
| Fresh package consumer | pass: extracted `.crate`, `cargo install --locked --path … --root …`, then `git-forge-exit-drill --version` returned `0.1.0` |

The claim registry has 21 unique IDs and exactly 21 matching `@claim:` tests;
there are no missing, extra, or duplicate tags.

### Browser, accessibility, privacy, and offline

- `verify-url.sh` passed local production `/`, `/demo`, `/privacy`, and
  `/terms`: 200 response, route title, `lang=en`, one main, one H1, complete
  image alt text, and no browser console or page errors.
- Playwright/Axe scanned those four routes at 1440×900 and 390×844. Every
  scan had zero serious or critical violations, no horizontal overflow, one
  H1, one main, and `lang=en`.
- The standalone `@axe-core/cli` was attempted but its Selenium launcher
  could not find a Chrome binary. The project’s installed Playwright/Axe
  integration used the provisioned Chromium and completed the required scan.
- Full browser tests cover keyboard navigation, the new reset-focus path,
  44 px visible controls, 200% text reflow, reduced motion, service-worker
  update, and offline `/demo` reload.
- Claim tests record demo requests and storage: demo requests stay same-origin
  and demo storage uses `demo:gfed:started`; no telemetry is requested.
- `staticwebapp.config.json` keeps the production CSP, including response
  header `frame-ancestors 'none'`, only self plus Sociobot billing in
  `connect-src`, `nosniff`, and strict-origin referrer policy.

### Size and performance

- Production JavaScript: 19,234 bytes raw / 6,446 bytes gzip.
- Production CSS: 14,245 bytes raw / 3,808 bytes gzip.
- Lighthouse mobile against the local production build: performance 100,
  accessibility 100, best practices 100, SEO 100; FCP 1.0 s, LCP 1.5 s,
  CLS 0, TBT 70 ms, no run warnings.

## Deployment

Static deployment uses the work-order configuration:

```sh
/opt/fleet/lib/deploy-static.sh git-forge-exit-drill dist/site
```

- Code repair commit `9a56fdf675752d0d2a03f1d957c25d70817425de` was pushed to
  `origin/main` before deployment.
- Azure Static Web Apps deployment `fc2ad30b-b827-407d-b7ca-07271aaa7b90`
  completed successfully at `https://git-forge-exit-drill.sociobot.in`.
- The factory verifier passed live `/`, `/demo`, `/privacy`, and `/terms`.
  Each returned 200 with no browser errors; `/missing` returns 404.
- Live headers include HSTS, `nosniff`, strict-origin referrer policy,
  restrictive permissions policy, and the expected CSP with response-header
  `frame-ancestors 'none'`.
- Deployment identity is exact: live `index.html` SHA-256 is
  `21cadaf9a043c45ffee32a5012f571dfcc3ad8a24698f63f12f10884f6ee56cf`,
  matching `dist/site/index.html`; live app JS SHA-256 is
  `2c219ad028a15b625bc9ec0762eb50dfd227aa963550dcc9da92ef7fdc1863c5`,
  matching the built asset.
- A live browser fixture confirmed the returned-license URL is stripped, the
  token stays masked rather than page text, **Copy setup command** returns the
  correct `GFED_LICENSE` export, malformed cache recovery works, and mobile
  demo reset preserves keyboard focus.

## Known gaps / next steps

None. The product has no sign-in or product-owned backend, so Entra identity,
backend concurrency, persistence, and health checks do not apply. AI is not
used because deterministic evidence validation is the core job.
