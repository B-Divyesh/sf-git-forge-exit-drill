# Demo contract

- Site URL: `/demo`, `?demo=1`, or `https://git-forge-exit-drill.sociobot.in/demo`.
- CLI command: `git-forge-exit-drill demo`.
- Sample: `examples/atlas-notes-export/` contains a realistic manifest, issue and pull-request excerpts, releases, workflow runs, and a license inventory. The CLI creates a small valid bare Git mirror from its bundled Atlas Notes source before inventorying it, so the demo proves real Git object capture.
- Site reset: choose **Reset demo**. This removes and recreates the `demo:gfed:started` namespace only.
- Site exit: choose **Start for real**. This removes the demo namespace before opening install instructions.
- CLI reset: run `git-forge-exit-drill demo` again for a new temporary directory. With `--output`, choose a new or empty directory; a non-empty output is refused and never removed.
- CLI demo output: `result/evidence.gfed`, `result/readiness.md`, and `result/readiness.json` under the printed temporary path.
- CLI demo passphrase: `demo-only-passphrase`. It protects sample data only and must never be reused.
- Isolation: site demo state uses keys prefixed with `demo:`. The CLI copies bundled bytes into a new temporary directory and reads no workspace export.
- Network: neither demo requires an account. The CLI demo makes no network request.
