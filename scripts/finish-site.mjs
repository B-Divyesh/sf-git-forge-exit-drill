import { access, chmod, copyFile, mkdir } from 'node:fs/promises';

await copyFile('dist/site/index.html', 'dist/site/404.html');

for (const candidate of ['target/release/git-forge-exit-drill', 'target/debug/git-forge-exit-drill']) {
  try {
    await access(candidate);
    await mkdir('dist/site/downloads', { recursive: true });
    const destination = 'dist/site/downloads/git-forge-exit-drill-linux-x86_64';
    await copyFile(candidate, destination);
    await chmod(destination, 0o755);
    break;
  } catch {
    // A site-only build may run before Rust is installed.
  }
}
