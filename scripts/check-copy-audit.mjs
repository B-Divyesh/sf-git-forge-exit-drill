import { createHash } from 'node:crypto';
import { readFile } from 'node:fs/promises';

const audit = await readFile('.factory/copy-audit.md', 'utf8');
for (const file of ['site/src/main.ts', 'README.md']) {
  const hash = createHash('sha256').update(await readFile(file)).digest('hex');
  if (!audit.includes(`${file}: ${hash}`)) {
    throw new Error(`${file} changed; regenerate .factory/copy-audit.md before release`);
  }
}
