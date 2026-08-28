import { copyFile, mkdir, chmod } from 'node:fs/promises';

await mkdir('dist/site/downloads', { recursive: true });
const destination = 'dist/site/downloads/git-forge-exit-drill-linux-x86_64';
await copyFile('target/release/git-forge-exit-drill', destination);
await chmod(destination, 0o755);
