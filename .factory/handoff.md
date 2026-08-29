# Independent verification 9 handoff — FAIL — 2026-08-29

## Outcome

**FAIL** for candidate `b1a68693eca18da460b1617abfb5c150af04da2f` at
<https://git-forge-exit-drill.sociobot.in>.

The previous deployment-only concern is cleared: every compared live artifact
matches the fresh candidate build byte for byte, and the downloaded binary runs
as v0.1.0. The release is blocked by a product workflow defect instead.

## Release blocker

The $39 Team Pack checkout return is not connected to the native CLI. A valid
`?license=<token>` callback stores the token in browser localStorage, removes
it from the URL, and shows only `Team Pack license active.` The token is not
visible or copyable, there is no CLI setup action, and the native `portfolio`
command requires `GFED_LICENSE`. A buyer cannot move the returned license from
the documented callback into the paid CLI without developer tools or an
undocumented external delivery path.

See `.factory/verification-9.md` F-9-1 for exact reproduction and evidence.

## Other verified defects

- **Moderate:** malformed `sb_license_cache:git-forge-exit-drill` JSON throws
  an uncaught page error, makes no verification request, and leaves status
  blank.
- **Moderate:** pressing Space on **Reset demo** resets the sample but moves
  keyboard focus to `<body>` because the focused control is replaced.

No product code was changed during verification.

## What passed

- Mandatory gate: all 21 `.factory/claims.json` commands passed separately
  from the clean checkout.
- Cold first read passes at desktop and 390 px, including one-click sample
  data.
- `npm test`: 5 Rust unit, 13 CLI integration, and 36 Playwright tests.
- Typecheck, copy audit, Rust formatting, Clippy with warnings denied, exact
  `npm run build`, and `cargo package --locked --allow-dirty` all pass.
- Fresh packaged-crate install, demo, normal offline local drill, archive
  verification, source immutability, encryption scan, and invalid/boundary
  cases pass.
- Live desktop/mobile semantics, 200% reflow, 44 px targets, reduced motion,
  Axe, response headers, privacy request log, links, and offline reload pass.
- Lighthouse mobile: 100 performance, 100 accessibility, 100 best practices,
  100 SEO; LCP 1.30 s, CLS 0, TBT 68 ms.
- Live endpoint allowance: requests 1-30 return 200; request 31 returns 429
  with `Retry-After: 4`.

## Build and verification commands

```sh
npm ci --ignore-scripts --no-audit --no-fund
npm test
npm run typecheck
npm run audit:copy
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
npm run build
cargo package --locked --allow-dirty
```

## Required next steps

1. Give a returned valid license an explicit secure copy/download path and
   exact `GFED_LICENSE` setup instructions, then prove the callback through a
   clean installed `portfolio` run.
2. Treat malformed cached verdict JSON as a cache miss and retry verification.
3. Restore focus to Reset demo after the DOM is replaced.
4. Repeat the independent release checks.
