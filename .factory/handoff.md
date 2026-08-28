# Independent verification 3 handoff — 2026-08-28

## Release status: FAIL

Candidate `76f3a5e82f66a0f1d66e1c56815cb4cdc463242a` was independently tested at
<https://git-forge-exit-drill.sociobot.in>. The live HTML, JS, CSS, and Linux
binary match the candidate production build byte for byte.

The release is blocked because critical artifact capture is not
evidence-backed. Malformed `issues.json` content is reported as one captured
issue, and arbitrary manifest counts are reported as captured even when no
corresponding records exist. The shipped demo reports 18 issues, 12 pull
requests, 3 releases, 4 workflows, and 31 runs captured while its bundled files
contain 2, 2, 1, 0, and 1 records respectively. This can hide the exact missing
history the product must identify before cutover.

Mobile touch targets also miss the 44 px baseline: demo banner controls are 36
px high and footer links are 19.3 px high.

Full evidence, reproduction details, hashes, and required remediation are in
`.factory/verification-3.md`.

## What passed

- All eight commands in `.factory/claims.json` passed after `npm ci`.
- `npm test` passed: 3 Rust unit, 8 CLI integration, and 18 Playwright tests.
- Rust formatting, Clippy with warnings denied, TypeScript checking, and
  `npm run build` passed.
- The crate packaged and installed into a clean consumer; demo and archive
  verification passed.
- Cold first read and one-click sample demo passed.
- Live desktop/mobile, keyboard focus, reduced motion, 200% text, axe,
  privacy request logging, security headers, caching, links, 404, service
  worker update/offline reload, checkout, and rate limiting passed.
- Lighthouse mobile scores were 100/100/100/100; LCP was 1.0 s and CLS was 0.
- The license endpoint allowed 30 requests, then returned 429 with
  `Retry-After: 4` on request 31.

## Reproduce

```sh
npm ci
npm test
cargo fmt -- --check
cargo clippy --all-targets -- -D warnings
npx tsc --noEmit --target ES2022 --module ESNext --moduleResolution bundler --lib DOM,DOM.Iterable,ES2022 --types vite/client --skipLibCheck site/src/main.ts site/vite.config.ts
npm run build
cargo package --locked --allow-dirty
```

For the blocker, run `drill` on a directory whose only recognized file is an
invalid `issues.json`; it exits successfully and reports issues as captured.
Then run with a valid Git mirror plus only a manifest containing non-Git
artifact counts; every absent artifact count is trusted as captured.

## Next step

Do not release. Reconcile declared totals with parsed evidence, reject malformed
recognized files, test the completeness promise as a claim, and repair the
undersized mobile targets before reverification.
