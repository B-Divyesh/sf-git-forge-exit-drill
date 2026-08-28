import AxeBuilder from '@axe-core/playwright';
import { expect, test } from '@playwright/test';
import { execFile as execFileCallback } from 'node:child_process';
import { createServer } from 'node:http';
import { mkdtemp, readFile } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { promisify } from 'node:util';

const execFile = promisify(execFileCallback);
const binary = join(process.cwd(), 'target/debug/git-forge-exit-drill');
const sample = join(process.cwd(), 'examples/atlas-notes-export');

test('@claim:demo-private sample demo is immediate and same-origin only', async ({ page }) => {
  const origins = new Set<string>();
  page.on('request', (request) => origins.add(new URL(request.url()).origin));
  await page.goto('/demo');
  await expect(page.getByRole('heading', { level: 1, name: 'See a complete exit drill' })).toBeVisible();
  await expect(page.getByText('Outcome: BLOCKED')).toBeVisible();
  await expect(page.getByText('Demo — sample data, nothing is saved')).toBeVisible();
  expect([...origins]).toEqual(['http://127.0.0.1:4173']);
  expect(await page.evaluate(() => localStorage.getItem('demo:gfed:started'))).toBeTruthy();
  expect(await page.evaluate(() => Object.keys(localStorage).filter((key) => !key.startsWith('demo:')))).toEqual([]);
});

test('@claim:free-single one-repository drill runs without a license or network', async () => {
  const root = await mkdtemp(join(tmpdir(), 'gfed-free-'));
  const output = join(root, 'result');
  const { stdout } = await execFile(binary, ['--json', 'drill', '--source', sample, '--target', 'gitea:1.22', '--output', output], {
    env: {
      ...process.env,
      GFED_PASSPHRASE: 'browser claim passphrase',
      HTTP_PROXY: 'http://127.0.0.1:1',
      HTTPS_PROXY: 'http://127.0.0.1:1',
      NO_PROXY: '',
    },
  });
  const result = JSON.parse(stdout);
  expect(result.repository).toBe('acme-labs/atlas-notes');
  expect(result.target).toBe('Gitea 1.22');
});

test('@claim:encrypted-evidence archive hides source text and verifies', async () => {
  const root = await mkdtemp(join(tmpdir(), 'gfed-encrypted-'));
  const output = join(root, 'result');
  const environment = { ...process.env, GFED_PASSPHRASE: 'browser claim passphrase' };
  await execFile(binary, ['drill', '--source', sample, '--target', 'gitlab:17.0', '--output', output], { env: environment });
  const archive = join(output, 'evidence.gfed');
  const raw = await readFile(archive);
  expect(raw.includes(Buffer.from('Keep author attribution'))).toBe(false);
  const { stdout } = await execFile(binary, ['verify', archive], { env: environment });
  expect(stdout).toContain('Archive verified');
  expect(stdout).toContain('Evidence files: 6');
});

test('@claim:token-private API token stays out of every output file', async () => {
  const token = 'github-secret-fixture-token';
  const server = createServer((request, response) => {
    response.writeHead(200, { 'content-type': 'application/json' });
    if (request.url?.includes('/actions/workflows')) {
      response.end(JSON.stringify({ total_count: 0, workflows: [] }));
    } else if (request.url?.includes('/actions/runs')) {
      response.end(JSON.stringify({ total_count: 0, workflow_runs: [] }));
    } else if (request.url === '/repos/acme/test') {
      response.end(JSON.stringify({ full_name: 'acme/test', private: true }));
    } else {
      response.end('[]');
    }
  });
  await new Promise<void>((resolve) => server.listen(0, '127.0.0.1', resolve));
  const address = server.address();
  if (!address || typeof address === 'string') throw new Error('mock server did not start');
  const root = await mkdtemp(join(tmpdir(), 'gfed-api-'));
  try {
    await execFile(binary, ['drill', '--repo', 'acme/test', '--target', 'forgejo:9.0', '--output', root], {
      env: {
        ...process.env,
        GITHUB_TOKEN: token,
        GFED_PASSPHRASE: 'browser claim passphrase',
        GFED_GITHUB_API_BASE: `http://127.0.0.1:${address.port}`,
      },
    });
  } finally {
    server.close();
  }
  for (const name of ['readiness.md', 'readiness.json', 'evidence.gfed']) {
    expect((await readFile(join(root, name))).includes(Buffer.from(token))).toBe(false);
  }
});

test('@claim:team-portfolio valid Team Pack license creates a ten-repository-capable report', async () => {
  const server = createServer((request, response) => {
    expect(request.url).toContain('/api/v1/products/git-forge-exit-drill/verify?license=test-license');
    response.writeHead(200, { 'content-type': 'application/json' });
    response.end(JSON.stringify({ valid: true, reason: 'ok', expires_at: null }));
  });
  await new Promise<void>((resolve) => server.listen(0, '127.0.0.1', resolve));
  const address = server.address();
  if (!address || typeof address === 'string') throw new Error('mock server did not start');
  const root = await mkdtemp(join(tmpdir(), 'gfed-team-'));
  try {
    await execFile(binary, [
      'portfolio', '--source', sample, '--source', sample, '--target', 'forgejo:9.0', '--output', join(root, 'portfolio'),
    ], {
      env: {
        ...process.env,
        GFED_PASSPHRASE: 'browser claim passphrase',
        GFED_LICENSE: 'test-license',
        GFED_BILLING_BASE: `http://127.0.0.1:${address.port}`,
        XDG_CONFIG_HOME: join(root, 'config'),
      },
    });
  } finally {
    server.close();
  }
  const report = await readFile(join(root, 'portfolio/portfolio.md'), 'utf8');
  expect(report).toContain('Repositories:** 2');
  expect(report.match(/acme-labs\/atlas-notes/g)).toHaveLength(2);
});

test('landing page has the required first screen and keyboard path', async ({ page }) => {
  await page.goto('/');
  await expect(page).toHaveTitle('Git Forge Exit Drill — test a GitHub move');
  await expect(page.locator('h1')).toHaveCount(1);
  await expect(page.getByRole('heading', { level: 1 })).toHaveText('Test your GitHub exit before cutover');
  await expect(page.getByRole('link', { name: 'Try it with sample data' })).toBeVisible();
  await page.keyboard.press('Tab');
  await expect(page.getByRole('link', { name: 'Skip to main content' })).toBeFocused();
  await page.getByRole('link', { name: 'Try it with sample data' }).click();
  await expect(page).toHaveURL(/\/demo$/);
  await expect(page.locator('h1')).toBeFocused();
});

for (const route of ['/', '/demo', '/privacy', '/terms']) {
  test(`${route} has no serious accessibility violations`, async ({ page }) => {
    await page.goto(route);
    await expect(page.locator('main')).toBeVisible();
    await expect(page.locator('h1')).toHaveCount(1);
    const results = await new AxeBuilder({ page }).analyze();
    expect(results.violations.filter((violation) => ['serious', 'critical'].includes(violation.impact ?? ''))).toEqual([]);
  });
}

test('license return, restore, and removal use the required browser key', async ({ page }) => {
  await page.route('https://api.sociobot.in/api/v1/products/git-forge-exit-drill/verify?license=returned-token', (route) => route.fulfill({ json: { valid: true, reason: 'ok', expires_at: null } }));
  await page.goto('/?license=returned-token');
  await expect.poll(() => page.evaluate(() => localStorage.getItem('sb_license:git-forge-exit-drill'))).toBe('returned-token');
  await expect(page).not.toHaveURL(/license=/);
  await page.goto('/privacy');
  await page.getByRole('button', { name: 'Remove saved license' }).click();
  await expect(page.getByText('Saved license removed.')).toBeVisible();
  expect(await page.evaluate(() => localStorage.getItem('sb_license:git-forge-exit-drill'))).toBeNull();
});

test('unknown routes show a styled way home', async ({ page }) => {
  await page.goto('/not-a-route');
  await expect(page).toHaveTitle('Page not found — Git Forge Exit Drill');
  await expect(page.getByRole('heading', { level: 1 })).toHaveText('This route has no evidence');
  await expect(page.getByRole('link', { name: 'Return home' })).toBeVisible();
});
