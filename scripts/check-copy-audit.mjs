import { spawn } from 'node:child_process';
import { readFile, writeFile } from 'node:fs/promises';
import { once } from 'node:events';
import { createServer } from 'node:net';
import { chromium } from '@playwright/test';

const auditPath = '.factory/copy-audit.md';
const banned = ['leverage', 'seamless', 'effortless', 'robust', 'powerful', 'intuitive', 'reimagine', 'supercharge', 'unlock', 'delightful', 'journey', 'ecosystem', 'ai-powered'];
const write = process.argv.includes('--write');

function words(text) {
  return text.replace(/[`*_~]/g, '').replace(/[^\p{L}\p{N}.$+/'-]+/gu, ' ').trim().split(/\s+/).filter(Boolean);
}

function splitCopy(text) {
  const normalized = text.replace(/\s+/g, ' ').trim();
  return normalized ? normalized.split(/(?<=[.!?])\s+(?=[A-Z0-9])/u).filter(Boolean) : [];
}

function flag(text) {
  const count = words(text).length;
  const lower = text.toLowerCase();
  const found = banned.filter((word) => new RegExp(`(?:^|[^a-z])${word.replace('-', '[- ]')}(?:$|[^a-z])`, 'u').test(lower));
  const reasons = [];
  if (count > 22) reasons.push(`over 22 words (${count})`);
  if (found.length) reasons.push(`banned: ${found.join(', ')}`);
  return reasons.join('; ');
}

function markdown(text) {
  return text.replace(/\|/g, '\\|').replace(/\n/g, ' ');
}

async function unusedPort() {
  const server = createServer();
  server.listen(0, '127.0.0.1');
  await once(server, 'listening');
  const address = server.address();
  if (!address || typeof address === 'string') throw new Error('could not reserve a local audit port');
  const { port } = address;
  server.close();
  return port;
}

async function waitForServer(url, child) {
  for (let attempt = 0; attempt < 80; attempt += 1) {
    if (child.exitCode !== null) throw new Error(`Vite exited before copy audit server was ready (${child.exitCode})`);
    try {
      if ((await fetch(url)).ok) return;
    } catch {
      // Vite is still starting.
    }
    await new Promise((resolve) => setTimeout(resolve, 100));
  }
  throw new Error('timed out waiting for the copy audit server');
}

async function renderedCopy() {
  const port = await unusedPort();
  const child = spawn(process.execPath, ['node_modules/vite/bin/vite.js', '--config', 'site/vite.config.ts', '--host', '127.0.0.1', '--port', String(port), '--strictPort'], { stdio: 'ignore' });
  const origin = `http://127.0.0.1:${port}`;
  let browser;
  try {
    await waitForServer(origin, child);
    browser = await chromium.launch({ headless: true });
    const page = await browser.newPage();
    const result = [];
    for (const route of ['/', '/demo', '/privacy', '/terms', '/missing-evidence']) {
      await page.goto(`${origin}${route}`, { waitUntil: 'networkidle' });
      if (route === '/') await page.getByRole('button', { name: 'Enter Team Pack license' }).click();
      const units = await page.locator('h1, h2, h3, p, a, button, label, li, figcaption, code, pre, img[alt], [aria-label]').evaluateAll((elements) => {
        const visible = (element) => {
          const style = window.getComputedStyle(element);
          return style.display !== 'none' && style.visibility !== 'hidden' && !element.closest('[hidden]');
        };
        const rows = [];
        for (const element of elements) {
          if (!visible(element)) continue;
          const tag = element.tagName.toLowerCase();
          const text = tag === 'img' ? element.getAttribute('alt') : element.textContent;
          if (text?.trim()) rows.push({ kind: tag, text: text.trim() });
          const label = element.getAttribute('aria-label');
          if (label?.trim()) rows.push({ kind: 'accessible label', text: label.trim() });
        }
        return rows;
      });
      for (const unit of units) for (const text of splitCopy(unit.text)) result.push({ surface: route, kind: unit.kind, text });
    }
    return result;
  } finally {
    await browser?.close();
    child.kill('SIGTERM');
    await once(child, 'exit').catch(() => undefined);
  }
}

function readmeCopy(source) {
  const withoutCode = source.replace(/```[\s\S]*?```/g, '');
  const rows = [];
  let paragraph = [];
  const flush = () => {
    for (const text of splitCopy(paragraph.join(' ').replace(/\[([^\]]+)\]\([^)]*\)/g, '$1'))) rows.push({ surface: 'README', kind: 'prose', text });
    paragraph = [];
  };
  for (const line of withoutCode.split('\n')) {
    const trimmed = line.trim();
    if (!trimmed) flush();
    else if (/^#{1,6}\s/.test(trimmed)) {
      flush();
      rows.push({ surface: 'README', kind: 'heading', text: trimmed.replace(/^#{1,6}\s+/, '') });
    } else if (/^-\s+/.test(trimmed)) {
      flush();
      rows.push({ surface: 'README', kind: 'list item', text: trimmed.replace(/^-\s+/, '') });
    } else paragraph.push(trimmed);
  }
  flush();
  return rows;
}

function auditDocument(rows) {
  const unique = [...new Map(rows.map((row) => [`${row.surface}\u0000${row.kind}\u0000${row.text}`, row])).values()].sort((a, b) => a.surface.localeCompare(b.surface) || a.kind.localeCompare(b.kind) || a.text.localeCompare(b.text));
  const failures = unique.filter((row) => flag(row.text));
  const table = unique.map((row) => `| ${row.surface} | ${row.kind} | ${markdown(row.text)} | ${words(row.text).length} | ${flag(row.text) || 'pass'} |`).join('\n');
  return `# Copy audit

Generated by \`npm run audit:copy\` from rendered accessible text on every public route and README prose. It opens the router at /, /demo, /privacy, /terms, and an unknown route. The home route also opens the license form. Code blocks and URLs are excluded from README prose.

A word is a whitespace-delimited token after punctuation is removed. Hyphenated terms, versions, prices, and commands count as one word. The audit fails for reader-facing text over 22 words or for banned marketing words: ${banned.join(', ')}.

## Result

${failures.length ? `FAIL — ${failures.length} unit(s) need revision.` : 'PASS — every audited unit is within the plain-words limit.'}

## Units

| Surface | Type | Copy | Words | Result |
| --- | --- | --- | ---: | --- |
${table}
`;
}

const [rendered, readme] = await Promise.all([renderedCopy(), readFile('README.md', 'utf8')]);
const rows = [...rendered, ...readmeCopy(readme)];
const output = auditDocument(rows);
const failures = rows.filter((row) => flag(row.text));
if (write) await writeFile(auditPath, output);
else if (await readFile(auditPath, 'utf8') !== output) throw new Error(`${auditPath} is stale; run npm run audit:copy:write`);
if (failures.length) throw new Error(`copy audit found ${failures.length} violation(s); see ${auditPath}`);
