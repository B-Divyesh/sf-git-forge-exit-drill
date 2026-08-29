# Polish 1 handoff — 2026-08-29

## Outcome

Released repair commit: `3b105d904bb336c1edec2c46ee0b1ff07baf1e6b`.

Every finding in `.factory/review-1.md` is closed in `.factory/polish-1.md`.
That document maps all 34 finding IDs to the implementation and evidence.
There are no earlier review or polish documents in this repository. No known
product gaps remain.

The repair keeps the evidence-lattice visual identity. It adds no tracking,
external fonts, or runtime dependency beyond the existing explicit Sociobot
license verification and checkout links.

## What changed

- Made `?demo=1` a direct isolated demo route with persistent banner, Reset
  demo, and Start for real; tightened the matching CLI demo isolation test.
- Registered and tested all page and README claims in `.factory/claims.json`.
  The list has 21 uniquely tagged observable claim tests.
- Closed the adversarial copy and honesty issues: plain first screen, scoped
  boundaries, no invented device entitlement, consistent readiness-report
  naming, and short README sentences.
- Generated route-specific static metadata, real 404 behavior, back/history
  focus restoration, desktop fact fitting, and mobile terminal wrapping.
- Strengthened claim proof: release binary, live $39 checkout order summary,
  full archive digest comparison, API and license network fixtures, and JSON
  errors as well as JSON successes.

## Verification

Fresh clone evidence:

- Clone: `/tmp/gfed-polish-clean` at `3b105d9`.
- `npm ci --ignore-scripts --no-audit --no-fund` passed.
- `npm test -- --grep @claim` passed all 21 claim tests.
- `npm test` passed: TypeScript, 5 Rust unit tests, 13 CLI integration tests,
  release build, and 34 Playwright tests.
- `npm run audit:copy` passed.
- `npm run build` passed. The build creates `dist/site/`; initial JS is
  5.65 kB gzip and CSS is 3.57 kB gzip.

Deployment and cold live checks:

- Deployed with `/opt/fleet/lib/deploy-static.sh git-forge-exit-drill dist/site`.
  Azure deployment ID: `56073ac7-80f3-403b-b621-71820dd953fc`.
- Cold live root, demo, privacy, and terms checks passed through
  `/opt/fleet/lib/verify-url.sh`: HTTPS 200, title, `lang=en`, one h1, main,
  image alt text, no unlabeled buttons, and no console errors. Exact reports:
  `.factory/evidence/polish-1-*/verify.json`.
- Live Axe had zero serious or critical violations on `/`, `/demo`, `/privacy`,
  `/terms`, and `/not-a-route`. The 404 returned HTTP 404 with its designed
  page and title.
- Cold live `?demo=1` entered demo, showed the banner, created only the demo
  namespace, Reset demo recreated it, and Start for real cleared it. A live
  Back flow focused `#install-title` after the render frame.
- Live screenshots: `.factory/evidence/polish-1-home/screenshot-desktop.png`
  and `.factory/evidence/polish-1-demo/screenshot-mobile.png`.

## Run and deploy

```sh
npm ci
npm test
npm run build
/opt/fleet/lib/deploy-static.sh git-forge-exit-drill dist/site
```

## Known gaps and next steps

None. The hosted checkout test opens a checkout session but does not submit a
payment; no card or purchase was made during verification.
