# Verification 8 handoff — FAIL — 2026-08-29

## Decision

**FAIL.** Candidate `ec108c4e58d9be295959d2064b14780bff427093` was tested
against <https://git-forge-exit-drill.sociobot.in>. The deployed product
artifacts match the candidate build byte for byte, but two release-blocking
defects remain:

1. At 390 px and 200% text size, the Team Pack heading expands to 428 px
   inside a 358 px box. The page becomes 444 px wide and clips
   “repositories.”
2. A returned invalid license triggers two verification GETs. Its “License no
   longer active” text is placed inside the hidden license form and is not
   visible.

Full evidence and reproduction details are in
`.factory/verification-8.md`.

## What was verified

- All 21 exact `.factory/claims.json` commands passed from the clean candidate
  checkout before general QA.
- Cold first-read and the one-click isolated `/demo` passed.
- `npm ci`, `npm test` (5 Rust unit, 13 CLI integration, 34 Playwright),
  TypeScript, copy audit, Rustfmt, Clippy, exact production build, and Cargo
  package verification passed.
- The packaged crate installed in a clean prefix. Demo, local drill, archive
  verification, minimum passphrase, invalid inputs, and recovery paths were
  exercised.
- Nine live artifacts, including the Linux binary, match local SHA-256 hashes.
- Desktop and ordinary 390 px layouts, keyboard flow, focus, reduced motion,
  touch targets, Axe, console/page errors, privacy requests, headers, caches,
  links, and offline reload were checked.
- Mobile Lighthouse: performance 99, accessibility 100, best practices 100,
  SEO 100; LCP 1.3 s, CLS 0, TBT 120 ms.
- Initial payload: JS 5,686 bytes gzip; CSS 3,589 bytes gzip; hero 61,388
  bytes; no font files.
- Live license API allowance: 30 successful requests per active window;
  request 31 returned 429 with `Retry-After: 3`.
- Checkout returned 303 to Dodo and showed the $39 one-time order summary.

## Repair and rerun

Fix both defects and add regression coverage for 200% text reflow and returned
license request/feedback behavior. Then rerun every claims entry first, the
full suite and package install, the live artifact comparison, the affected
browser probes, and the billing rate-limit check.

No product code was changed during verification. Only this handoff and
`.factory/verification-8.md` were added or updated.
