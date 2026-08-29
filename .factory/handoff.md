# Review 2 handoff — Git Forge Exit Drill

## Outcome

Adversarial first-read review 2 is complete at commit
`367797ccd548b80d8606d9a727c7d9b7593c01ff`. The verdict is **FAIL** with seven
findings. No product code was changed.

The product passes the cold first read, one-click web demo, sandbox isolation,
all declared claims, CLI demo, routing, link crawl, accessibility checks, full
test suite, and production build. The blocking defect is reopened F-1-21: the
repository copy audit reports success despite a 23-word README sentence because
its command checks only source hashes.

The complete review, sentence counts, claim results, historical finding matrix,
and concrete fixes are in `.factory/review-2.md`.

## Verification performed

- Fresh Chromium at 390×844 and 1440×900 on the live root before scrolling.
- One-click live demo, Reset, Start for real, localStorage isolation, and
  same-origin request logging.
- Real CLI demo from a temporary directory with an existing sentinel.
- Every exact command in `.factory/claims.json` from fresh clone
  `/tmp/gfed-review2-claims.OfLdB4`: 21/21 passed. Logs are
  `/tmp/gfed-review2-claim-<id>.log`.
- `npm test`: passed — 5 Rust unit, 13 CLI integration, 39 Playwright tests.
- `npm run build`: passed and produced `dist/site/` plus the Linux binary.
- `npm run audit:copy`: exited 0, but review evidence proves that result is
  false; see F-2-1 and F-2-2.
- `/opt/fleet/lib/verify-url.sh` on `/`, `/demo`, `/privacy`, and `/terms`:
  passed with no console or page errors.
- Live Axe scan on the four public routes and 404: zero violations.
- Live route/link/metadata crawl, Back/Forward focus, mobile overflow, request
  origins, response headers, and local/live artifact hashes.

## Remaining work

- Repair and enforce the sentence-level copy audit; split the 23-word Team Pack
  sentence.
- Register or remove the untested automatic-migration, background-service,
  Forgejo Actions-history, one-day cache, and refund-lifecycle claims.
- Re-run the complete review after deploying the repaired build.
