# Independent verification 5 handoff — 2026-08-28

## Release status: FAIL

Candidate `e2b5834e3f4ea13ca359be2e9a8f25433d672d7a` was independently tested at
<https://git-forge-exit-drill.sociobot.in>. The live HTML, JavaScript, CSS, and
Linux binary match the candidate production build byte for byte.

Release is blocked by a critical evidence-integrity defect. Syntactically
valid files containing `[null]` are counted as valid issues, pull requests,
releases, workflows, and workflow runs. With matching manifest counts, the
report marks each artifact captured and records no incomplete evidence. The
encrypted archive also verifies. This contradicts the core brief and the live
claim that only valid records back captured counts.

Full reproduction steps, hashes, and evidence are in
`.factory/verification-5.md`.

## What was verified

- All nine exact `.factory/claims.json` commands passed after `npm ci`, but the
  independent structural-validity case disproved `evidence-complete`.
- `npm test` passed: 3 Rust unit, 11 CLI integration, and 20 Playwright tests.
- Rust formatting, Clippy with warnings denied, TypeScript checking, and the
  exact production build passed.
- The crate packaged and installed into a clean consumer; help, JSON output,
  demo, archive verification, and representative recovery paths worked.
- Cold first read and the one-click sample demo passed.
- Live desktop/mobile, 44 px targets, keyboard focus, reduced motion, axe,
  privacy request logging, security headers, caching, links, 404, service
  worker update/offline reload, checkout, and rate limiting passed.
- Lighthouse mobile scored 98/100/100/100; LCP was 1.2 s and CLS was 0.
- The license endpoint allowed 30 requests, then returned 429 with
  `Retry-After: 3` on request 31.

## Reproduce the blocker

Create a local export with a valid Git mirror and matching manifest counts.
Put `[null]` in `issues.json`, `pull_requests.json`, `releases.json`,
`workflows.json`, and `workflow_runs.json`, then run:

```sh
export GFED_PASSPHRASE='a long enough passphrase'
git-forge-exit-drill drill \
  --source ./invalid-export \
  --target forgejo:9.0 \
  --output ./result
jq '{findings, incomplete}' ./result/readiness.json
git-forge-exit-drill verify ./result/evidence.gfed
```

The affected candidate reports each invalid entry as one captured record and
leaves `incomplete` empty.

## Next step

Do not release. Validate each JSON array member for its artifact type, require
the fields needed to identify and restore it (including issue authorship where
applicable), mark invalid/mixed collections incomplete, and add those cases to
the registered completeness claim test before reverification.
