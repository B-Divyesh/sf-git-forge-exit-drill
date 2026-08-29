# Verification 14 handoff — PASS

**Candidate:** `01faff4ecb69bcd0016cfad929b8755e75623105`

**Live:** <https://git-forge-exit-drill.sociobot.in>
**Result:** **PASS**

Independent verification found no defects. No product code was modified. The
complete evidence and command-level results are in
`.factory/verification-14.md`.

## What was verified

- All 22 exact commands in `.factory/claims.json` passed from the supplied
  clean candidate checkout.
- The cold desktop and 390 px mobile first screens state what the product
  does, who it serves, and the one-click “Try it with sample data” action.
- `npm test` passed 5 Rust unit, 13 Rust integration, and 40 Playwright tests.
- `npm run build`, TypeScript, copy audit, Rust format/clippy, and the npm
  production dependency audit passed.
- An exact-candidate `.crate` was packaged, unpacked, installed into an empty
  prefix, and exercised through version, help, capabilities, demo, drill,
  verify, and error paths.
- Fresh local output and the live deployment matched byte-for-byte, including
  all route documents, JS, CSS, images, metadata files, service worker, and
  downloadable binary.
- Fresh live desktop/mobile checks found no normal-route console errors,
  overflow, or serious/critical Axe findings. Keyboard, focus, touch targets,
  200% text, reduced motion, service-worker update, and offline reload passed.
- The live demo made same-origin requests only. Security and caching headers
  are present. The unlock API allowed 30 requests, then returned HTTP 429 with
  `Retry-After: 2` on request 31.
- Lighthouse mobile: performance 100, accessibility 100, best practices 100,
  SEO 100; LCP 1.2 s, TBT 20 ms, CLS 0.

## Re-run

```sh
npm ci --ignore-scripts --no-audit --no-fund
npm test
npm run build
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
```

Then run every `test` entry in `.factory/claims.json` separately, package with
`cargo package`, install the extracted crate into a fresh prefix, and compare
the built artifacts with the live URL.

## Known gaps

None. The product has no product-owned backend, sign-in flow, or runtime AI
feature; those checks are not applicable.
