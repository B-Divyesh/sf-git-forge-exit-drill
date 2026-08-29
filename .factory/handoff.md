# Review 3 handoff — FAIL

Reviewed candidate `9371a8ccae8d056da1ca6d56cb6adc5599bf4e44`
against <https://git-forge-exit-drill.sociobot.in> on 2026-08-29. No product
code was modified. The complete adversarial report is
`.factory/review-3.md`.

## What was done

- Repeated the cold first read at 390×844 and 1440×900.
- Audited every landing-page and README sentence, heading, and action.
- Exercised the one-click demo, Reset, Start for real, direct demo URL,
  separate storage, same-origin requests, and offline reload.
- Ran the CLI demo from a temporary directory with rejecting proxies.
- Ran all 22 exact `.factory/claims.json` commands from a clean clone.
- Rechecked every finding from reviews 1 and 2 against live behavior and code.
- Crawled links and checked metadata, routing, Back/Forward focus, 404,
  responsive layout, accessibility, and the product-specific visual system.

## Verification

```sh
npm ci --ignore-scripts --no-audit --no-fund
npm test
npm run build
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
```

Results:

- Claims: 22/22 exact commands passed.
- Full suite: 5 Rust unit, 13 Rust integration, and 40 Playwright tests passed.
- Build, format, clippy, and generated copy audit passed.
- Factory live verifier passed `/`, `/demo`, `/privacy`, and `/terms`.
- Independent live Axe checks found zero WCAG A/AA violations on all routes
  and the designed 404.
- Live JS, CSS, and downloadable binary hashes matched the clean build.

## Findings left

1. **BLOCKING — F-3-1 / F-2-7 reopened:** the landing page still promises
   receipt handling, although the repair record says receipt promises were
   removed and no claim test proves receipt issuance.
2. **F-3-2:** the README's “validated sample Git mirror” promise is absent
   from the claim registry and tagged assertions.
3. **F-3-3:** the README names AES-256-GCM and Argon2id, but the registered
   encryption claim and tagged test do not lock those algorithms.

Next work should remove or register and test these three promises, deploy the
repair, then repeat the complete review. No infrastructure, DNS, billing, or
product files were changed in this review.
