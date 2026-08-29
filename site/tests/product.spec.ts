import AxeBuilder from '@axe-core/playwright';
import { expect, test } from '@playwright/test';
import { execFile as execFileCallback } from 'node:child_process';
import { createServer } from 'node:http';
import { mkdir, mkdtemp, readFile, writeFile } from 'node:fs/promises';
import { createHash } from 'node:crypto';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { promisify } from 'node:util';

const execFile = promisify(execFileCallback);
const binary = join(process.cwd(), 'target/debug/git-forge-exit-drill');
const sample = join(process.cwd(), 'examples/atlas-notes-export');

async function createValidMirror(source: string) {
  await execFile('git', ['clone', '--mirror', '--quiet', process.cwd(), join(source, 'mirror.git')]);
}

test('@claim:demo-private sample demo is immediate and same-origin only', async ({ page }) => {
  const origins = new Set<string>();
  page.on('request', (request) => origins.add(new URL(request.url()).origin));
  await page.goto('/');
  await page.evaluate(() => localStorage.setItem('sb_license:git-forge-exit-drill', 'real-data-must-not-be-read'));
  origins.clear();
  await page.goto('/?demo=1');
  await expect(page).toHaveURL(/\/demo$/);
  await expect(page.getByRole('heading', { level: 1, name: 'See a complete exit drill' })).toBeVisible();
  await expect(page.getByText('Outcome: BLOCKED')).toBeVisible();
  await expect(page.getByText('Demo — sample data, nothing is saved')).toBeVisible();
  expect([...origins]).toEqual(['http://127.0.0.1:4173']);
  expect(await page.evaluate(() => localStorage.getItem('demo:gfed:started'))).toBeTruthy();
  expect(await page.evaluate(() => localStorage.getItem('sb_license:git-forge-exit-drill'))).toBe('real-data-must-not-be-read');
});

test('@claim:no-telemetry the site demo makes no third-party requests', async ({ page }) => {
  const origins = new Set<string>();
  page.on('request', (request) => origins.add(new URL(request.url()).origin));
  await page.goto('/demo');
  await expect(page.getByRole('heading', { level: 1, name: 'See a complete exit drill' })).toBeVisible();
  expect([...origins]).toEqual(['http://127.0.0.1:4173']);
});

test('@claim:recorded-cli the demo transcript matches the bundled CLI drill', async ({ page }) => {
  const { stdout } = await execFile(binary, ['demo']);
  await page.goto('/demo');
  for (const line of [
    'Demo — sample data. No workspace files were read.',
    'Repository: acme-labs/atlas-notes',
    'Target: Forgejo 9.0',
    'Outcome: BLOCKED',
    'Demo archive passphrase: demo-only-passphrase',
    'Choose a new output directory to run this demo again.',
  ]) {
    expect(stdout).toContain(line);
    await expect(page.getByText(line, { exact: true })).toBeVisible();
  }
});

test('@claim:evidence-complete captured counts require valid exported records', async () => {
  const root = await mkdtemp(join(tmpdir(), 'gfed-evidence-complete-'));
  const environment = { ...process.env, GFED_PASSPHRASE: 'browser claim passphrase' };

  const invalid = join(root, 'invalid');
  await mkdir(invalid);
  await writeFile(join(invalid, 'issues.json'), 'this is not json');
  const invalidOutput = join(root, 'invalid-output');
  await execFile(binary, ['drill', '--source', invalid, '--target', 'forgejo:9.0', '--output', invalidOutput], { env: environment });
  await execFile(binary, ['verify', join(invalidOutput, 'evidence.gfed')], { env: environment });
  const invalidReport = JSON.parse(await readFile(join(invalidOutput, 'readiness.json'), 'utf8'));
  expect(invalidReport.findings.find((finding: { artifact: string }) => finding.artifact === 'issues')).toMatchObject({
    captured: false,
    count: null,
    result: 'incomplete evidence',
  });

  const manifestOnly = join(root, 'manifest-only');
  await mkdir(manifestOnly);
  await writeFile(join(manifestOnly, 'manifest.json'), JSON.stringify({
    artifacts: { issues: 999, pull_requests: 888, releases: 777, actions_workflows: 666, actions_runs: 555 },
  }));
  const manifestOutput = join(root, 'manifest-output');
  await execFile(binary, ['drill', '--source', manifestOnly, '--target', 'forgejo:9.0', '--output', manifestOutput], { env: environment });
  await execFile(binary, ['verify', join(manifestOutput, 'evidence.gfed')], { env: environment });
  const manifestReport = JSON.parse(await readFile(join(manifestOutput, 'readiness.json'), 'utf8'));
  for (const artifact of ['issues', 'pull_requests', 'releases', 'actions_workflows', 'actions_runs']) {
    expect(manifestReport.findings.find((finding: { artifact: string }) => finding.artifact === artifact)).toMatchObject({
      captured: false,
      result: 'incomplete evidence',
    });
  }

  const structurallyInvalid = join(root, 'structurally-invalid');
  await mkdir(structurallyInvalid);
  await writeFile(join(structurallyInvalid, 'manifest.json'), JSON.stringify({
    artifacts: { issues: 1, pull_requests: 1, releases: 1, actions_workflows: 1, actions_runs: 1 },
  }));
  for (const name of ['issues.json', 'pull_requests.json', 'releases.json', 'workflows.json', 'workflow_runs.json']) {
    await writeFile(join(structurallyInvalid, name), '[null]');
  }
  await createValidMirror(structurallyInvalid);
  const structurallyInvalidOutput = join(root, 'structurally-invalid-output');
  await execFile(binary, ['drill', '--source', structurallyInvalid, '--target', 'forgejo:9.0', '--output', structurallyInvalidOutput], { env: environment });
  const structurallyInvalidReport = JSON.parse(await readFile(join(structurallyInvalidOutput, 'readiness.json'), 'utf8'));
  for (const artifact of ['issues', 'pull_requests', 'releases', 'actions_workflows', 'actions_runs']) {
    expect(structurallyInvalidReport.findings.find((finding: { artifact: string }) => finding.artifact === artifact)).toMatchObject({
      captured: false,
      count: null,
      result: 'incomplete evidence',
    });
    expect(structurallyInvalidReport.incomplete[artifact]).toContain('record 1 must be a JSON object');
  }

  const mixedRecords = join(root, 'mixed-records');
  await mkdir(mixedRecords);
  await writeFile(join(mixedRecords, 'manifest.json'), JSON.stringify({ artifacts: { issues: 1 } }));
  await writeFile(join(mixedRecords, 'issues.json'), JSON.stringify([
    { number: 81, title: 'Valid issue', author: 'mira' },
    null,
    7,
    { id: 5 },
    { title: 'Missing identity', author: 'mira' },
    { number: 82, title: 'Missing author' },
  ]));
  await createValidMirror(mixedRecords);
  const mixedOutput = join(root, 'mixed-output');
  await execFile(binary, ['drill', '--source', mixedRecords, '--target', 'forgejo:9.0', '--output', mixedOutput], { env: environment });
  const mixedReport = JSON.parse(await readFile(join(mixedOutput, 'readiness.json'), 'utf8'));
  expect(mixedReport.findings.find((finding: { artifact: string }) => finding.artifact === 'issues')).toMatchObject({
    captured: false,
    count: 1,
    result: 'incomplete evidence',
  });
  expect(mixedReport.incomplete.issues).toContain('record 2 must be a JSON object');

  const sampleOutput = join(root, 'sample-output');
  await execFile(binary, ['drill', '--source', sample, '--target', 'forgejo:9.0', '--output', sampleOutput], { env: environment });
  await execFile(binary, ['verify', join(sampleOutput, 'evidence.gfed')], { env: environment });
  const sampleReport = JSON.parse(await readFile(join(sampleOutput, 'readiness.json'), 'utf8'));
  for (const [artifact, count] of [['issues', 2], ['pull_requests', 2], ['releases', 1], ['release_assets', 2], ['actions_workflows', 1], ['actions_runs', 1]]) {
    expect(sampleReport.findings.find((finding: { artifact: string }) => finding.artifact === artifact)).toMatchObject({ captured: true, count });
  }

  const requests: string[] = [];
  let apiOrigin = '';
  const api = createServer((request, response) => {
    const requestUrl = new URL(request.url ?? '/', apiOrigin);
    requests.push(`${requestUrl.pathname}${requestUrl.search}`);
    response.setHeader('content-type', 'application/json');
    if (requestUrl.pathname === '/repos/acme/pagination') {
      response.end(JSON.stringify({ full_name: 'acme/pagination', private: true }));
      return;
    }
    if (requestUrl.pathname.endsWith('/issues')) {
      const page = Number(requestUrl.searchParams.get('page') ?? '1');
      if (page === 1) {
        response.setHeader('link', `<${apiOrigin}${requestUrl.pathname}?state=all&per_page=100&page=2>; rel="next"`);
      }
      const firstId = (page - 1) * 100;
      const records = page <= 2
        ? Array.from({ length: 100 }, (_, index) => ({ id: firstId + index + 1, title: `Issue ${firstId + index + 1}`, user: { login: 'mira' } }))
        : [];
      response.end(JSON.stringify(records));
      return;
    }
    if (requestUrl.pathname.endsWith('/actions/workflows')) {
      response.end(JSON.stringify({ total_count: 0, workflows: [] }));
      return;
    }
    if (requestUrl.pathname.endsWith('/actions/runs')) {
      const page = Number(requestUrl.searchParams.get('page') ?? '1');
      const count = page <= 100 ? 100 : page === 101 ? 1 : 0;
      const firstId = (page - 1) * 100;
      response.end(JSON.stringify({
        total_count: 10_001,
        workflow_runs: Array.from({ length: count }, (_, index) => ({
          id: firstId + index + 1,
          name: `Build ${firstId + index + 1}`,
          head_sha: `sha-${firstId + index + 1}`,
        })),
      }));
      return;
    }
    response.end('[]');
  });
  await new Promise<void>((resolve) => api.listen(0, '127.0.0.1', resolve));
  const address = api.address();
  if (!address || typeof address === 'string') throw new Error('pagination fixture did not start');
  apiOrigin = `http://127.0.0.1:${address.port}`;
  const apiOutput = join(root, 'api-output');
  try {
    await execFile(binary, ['drill', '--repo', 'acme/pagination', '--target', 'forgejo:9.0', '--output', apiOutput], {
      env: {
        ...environment,
        GITHUB_TOKEN: 'pagination-fixture-token',
        GFED_GITHUB_API_BASE: apiOrigin,
      },
    });
  } finally {
    api.close();
  }
  const apiReport = JSON.parse(await readFile(join(apiOutput, 'readiness.json'), 'utf8'));
  expect(apiReport.findings.find((finding: { artifact: string }) => finding.artifact === 'actions_runs')).toMatchObject({
    captured: true,
    count: 10_001,
  });
  expect(apiReport.incomplete.actions_runs).toBeUndefined();
  expect(apiReport.findings.find((finding: { artifact: string }) => finding.artifact === 'issues')).toMatchObject({
    captured: true,
    count: 200,
  });
  expect(requests).toContain('/repos/acme/pagination/actions/runs?per_page=100&page=101');
  expect(requests.some((request) => request.includes('/actions/runs?') && request.includes('page=102'))).toBe(false);
  expect(requests).toContain('/repos/acme/pagination/issues?state=all&per_page=100&page=2');
  expect(requests.some((request) => request.includes('/issues?') && request.includes('page=3'))).toBe(false);
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
  const report = JSON.parse(await readFile(join(output, 'readiness.json'), 'utf8'));
  expect(report.findings).toHaveLength(13);
  expect(report.findings.find((finding: { artifact: string }) => finding.artifact === 'issues').target_support).toBe('native');
});

test('@claim:source-read-only a local drill leaves its selected source unchanged', async () => {
  const root = await mkdtemp(join(tmpdir(), 'gfed-source-'));
  const output = join(root, 'result');
  const before = await readFile(join(sample, 'issues.json'));
  await execFile(binary, ['drill', '--source', sample, '--target', 'gitea:1.22', '--output', output], {
    env: { ...process.env, GFED_PASSPHRASE: 'browser claim passphrase' },
  });
  expect(await readFile(join(sample, 'issues.json'))).toEqual(before);
  expect(await readFile(join(output, 'readiness.json'), 'utf8')).toContain('acme-labs/atlas-notes');
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
  expect(stdout).toContain('Evidence files: 7');
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
  const report = JSON.parse(await readFile(join(root, 'readiness.json'), 'utf8'));
  expect(report.outcome).toBe('blocked');
  expect(report.findings.find((finding: { artifact: string }) => finding.artifact === 'git_repository')).toMatchObject({ captured: false, result: 'missing evidence' });
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
  const tenSources = Array.from({ length: 10 }, () => ['--source', sample]).flat();
  try {
    await execFile(binary, [
      'portfolio', ...tenSources, '--target', 'forgejo:9.0', '--output', join(root, 'portfolio'),
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
  expect(report).toContain('Repositories:** 10');
  expect(report.match(/acme-labs\/atlas-notes/g)).toHaveLength(10);

  const elevenSources = Array.from({ length: 11 }, () => ['--source', sample]).flat();
  await expect(execFile(binary, [
    'portfolio', ...elevenSources, '--target', 'forgejo:9.0', '--output', join(root, 'too-many'),
  ], {
    env: {
      ...process.env,
      GFED_PASSPHRASE: 'browser claim passphrase',
      GFED_LICENSE: 'test-license',
      GFED_BILLING_BASE: 'http://127.0.0.1:1',
      XDG_CONFIG_HOME: join(root, 'eleven-config'),
    },
  })).rejects.toMatchObject({
    code: 1,
    stderr: expect.stringContaining('portfolio accepts at most 10 export directories'),
  });
  await expect(readFile(join(root, 'too-many/portfolio.md'), 'utf8')).rejects.toThrow();
});

test('landing page has the required first screen and keyboard path', async ({ page }) => {
  await page.goto('/');
  await expect(page).toHaveTitle('Git Forge Exit Drill — test a GitHub move');
  await expect(page.locator('h1')).toHaveCount(1);
  await expect(page.getByRole('heading', { level: 1 })).toHaveText('Test your GitHub move before cutover');
  await expect(page.getByRole('link', { name: 'Try it with sample data' })).toBeVisible();
  await page.keyboard.press('Tab');
  await expect(page.getByRole('link', { name: 'Skip to main content' })).toBeFocused();
  await page.getByRole('link', { name: 'Try it with sample data' }).click();
  await expect(page).toHaveURL(/\/demo$/);
  await expect(page.locator('h1')).toBeFocused();
});

test('required first-screen content fits common desktop viewports', async ({ page }) => {
  for (const viewport of [{ width: 1280, height: 720 }, { width: 1366, height: 768 }, { width: 1440, height: 900 }]) {
    await page.setViewportSize(viewport);
    await page.goto('/');
    for (const locator of [
      page.getByRole('heading', { level: 1 }),
      page.locator('.lede'),
      page.getByRole('link', { name: 'Try it with sample data' }),
      page.locator('.hero-action p'),
      page.locator('.facts'),
    ]) {
      const box = await locator.boundingBox();
      expect(box, `${await locator.textContent()} has a layout box at ${viewport.width}x${viewport.height}`).not.toBeNull();
      expect(box!.y, `${await locator.textContent()} starts inside ${viewport.width}x${viewport.height}`).toBeGreaterThanOrEqual(0);
      expect(box!.y + box!.height, `${await locator.textContent()} fits inside ${viewport.width}x${viewport.height}`).toBeLessThanOrEqual(viewport.height);
    }
  }
});

test('desktop and 390px mobile render without page overflow or console errors', async ({ page }) => {
  const errors: string[] = [];
  page.on('console', (message) => {
    if (message.type() === 'error') errors.push(message.text());
  });
  await page.setViewportSize({ width: 1440, height: 900 });
  await page.goto('/');
  await expect(page.locator('h1')).toBeVisible();
  expect(await page.evaluate(() => document.documentElement.scrollWidth <= window.innerWidth)).toBe(true);
  await page.setViewportSize({ width: 390, height: 844 });
  await page.goto('/demo');
  await expect(page.locator('h1')).toBeVisible();
  expect(await page.evaluate(() => document.documentElement.scrollWidth <= window.innerWidth)).toBe(true);
  expect(await page.locator('.terminal pre').evaluate((element) => element.scrollWidth <= element.clientWidth)).toBe(true);
  expect(errors).toEqual([]);
});

test('390px interactive controls meet the 44px touch-target baseline', async ({ page }) => {
  await page.setViewportSize({ width: 390, height: 844 });
  for (const route of ['/', '/demo', '/privacy', '/terms']) {
    await page.goto(route);
    const controls = page.locator('a, button');
    for (let index = 0; index < await controls.count(); index += 1) {
      const control = controls.nth(index);
      if (!await control.isVisible()) continue;
      const box = await control.boundingBox();
      expect(box, `${route} control ${index} has a box`).not.toBeNull();
      expect(box!.width, `${route} control ${index} width`).toBeGreaterThanOrEqual(44);
      expect(box!.height, `${route} control ${index} height`).toBeGreaterThanOrEqual(44);
    }
  }
});

test('the demo accepts a service-worker update check and reloads offline after its first visit', async ({ page, context }) => {
  await page.goto('/demo');
  await page.evaluate(async () => {
    const registration = await navigator.serviceWorker.ready;
    await registration.update();
    if (!registration.active) throw new Error('service worker is not active after update check');
  });
  await page.reload();
  await context.setOffline(true);
  await page.reload();
  await expect(page.getByRole('heading', { level: 1, name: 'See a complete exit drill' })).toBeVisible();
  await context.setOffline(false);
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

test('@claim:license-browser-storage license return, restore, and removal use the required browser key', async ({ page }) => {
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

test('static deployment configuration reserves HTTP 404 for unknown routes', async () => {
  const config = JSON.parse(await readFile(join(process.cwd(), 'site/public/staticwebapp.config.json'), 'utf8'));
  expect(config.navigationFallback).toBeUndefined();
  expect(config.routes.filter((route: { rewrite?: string }) => ['/demo/index.html', '/privacy/index.html', '/terms/index.html'].includes(route.rewrite ?? '')).map((route: { route: string }) => route.route)).toEqual(['/demo', '/privacy', '/terms']);
  expect(config.responseOverrides['404']).toEqual({ rewrite: '/404.html' });
});

test('@claim:cli-demo-isolated CLI demo prints isolated output paths and preserves non-empty output', async () => {
  const root = await mkdtemp(join(tmpdir(), 'gfed-demo-isolated-'));
  const output = join(root, 'demo');
  const { stdout } = await execFile(binary, ['demo', '--output', output]);
  const report = stdout.match(/^Report:\s+(.+)$/m)?.[1];
  const archive = stdout.match(/^Encrypted evidence:\s+(.+)$/m)?.[1];
  expect(report).toBe(join(output, 'result', 'readiness.md'));
  expect(archive).toBe(join(output, 'result', 'evidence.gfed'));
  await expect(readFile(report!, 'utf8')).resolves.toContain('## Restore drill');
  const occupied = join(root, 'occupied');
  await mkdir(occupied);
  await writeFile(join(occupied, 'sentinel.txt'), 'keep this');
  await expect(execFile(binary, ['demo', '--output', occupied])).rejects.toMatchObject({ code: 1, stderr: expect.stringContaining('choose a new or empty directory') });
  await expect(readFile(join(occupied, 'sentinel.txt'), 'utf8')).resolves.toBe('keep this');
});

test('@claim:target-mappings named target versions have each published support state', async () => {
  const mapping = JSON.parse(await readFile(join(process.cwd(), 'mappings', 'targets.json'), 'utf8'));
  expect(mapping.targets.map((target: { label: string }) => target.label)).toEqual(['Forgejo 9.0', 'Gitea 1.22', 'GitLab 17.0']);
  for (const target of mapping.targets) {
    const states = new Set(Object.values(target.capabilities).map((item: any) => item.status));
    expect(states).toEqual(new Set(['native', 'manual', 'unsupported']));
  }
});

test('@claim:restore-checklist local reports contain generated restore steps', async () => {
  const root = await mkdtemp(join(tmpdir(), 'gfed-restore-checklist-'));
  await execFile(binary, ['drill', '--source', sample, '--target', 'forgejo:9.0', '--output', root], { env: { ...process.env, GFED_PASSPHRASE: 'browser claim passphrase' } });
  const report = await readFile(join(root, 'readiness.md'), 'utf8');
  expect(report).toContain('## Restore drill');
  expect(report).toContain('Restore into a disposable target project');
  expect(report).toContain('Compare the default branch and every tag');
  expect(report).toContain('Run one build from a pinned commit');
});

test('@claim:output-boundary local drill writes only its declared output artifacts', async () => {
  const root = await mkdtemp(join(tmpdir(), 'gfed-output-boundary-'));
  const source = join(root, 'source'); const output = join(root, 'output');
  await mkdir(source); await writeFile(join(source, 'notes.txt'), 'source sentinel'); await writeFile(join(source, 'issues.json'), '[]');
  await execFile(binary, ['drill', '--source', source, '--target', 'forgejo:9.0', '--output', output], { env: { ...process.env, GFED_PASSPHRASE: 'browser claim passphrase' } });
  expect(await readFile(join(source, 'notes.txt'), 'utf8')).toBe('source sentinel');
  for (const name of ['readiness.md', 'readiness.json', 'evidence.gfed']) await expect(readFile(join(output, name))).resolves.toBeTruthy();
});

test('@claim:linux-download production site output ships an executable versioned Linux binary', async () => {
  const download = join(process.cwd(), 'dist', 'site', 'downloads', 'git-forge-exit-drill-linux-x86_64');
  const { stdout } = await execFile(download, ['--version']);
  expect(stdout).toContain('git-forge-exit-drill 0.1.0');
});

test('@claim:billing-contract Team Pack checkout is active and shows the published one-time $39 offer', async ({ page }) => {
  await page.goto('/');
  await expect(page.getByText('A $39 one-time purchase adds the portfolio command and one consolidated readiness report.')).toBeVisible();
  await expect(page.getByRole('link', { name: /Buy Team Pack/ })).toHaveAttribute('href', 'https://api.sociobot.in/api/v1/products/git-forge-exit-drill/checkout');
  await page.goto('/terms');
  await expect(page.getByText('The Team Pack costs $39 once.')).toBeVisible();
  await expect(page.getByText('Sociobot handles payment, receipts, and refunds.')).toBeVisible();
  const checkout = await fetch('https://api.sociobot.in/api/v1/products/git-forge-exit-drill/checkout', { redirect: 'manual' });
  expect(checkout.status).toBe(303);
  const location = checkout.headers.get('location');
  expect(location).toMatch(/^https:\/\/checkout\.dodopayments\.com\/session\//);
  const receipt = await fetch(location!);
  const checkoutHtml = await receipt.text();
  expect(receipt.ok).toBe(true);
  expect(checkoutHtml).toContain('Git Forge Exit Drill');
  expect(checkoutHtml).toContain('$39.00');
  expect(checkoutHtml).toContain('One-time unlock');
});

test('@claim:archive-file-completeness archive lists every regular nested source file with its digest', async () => {
  const root = await mkdtemp(join(tmpdir(), 'gfed-archive-files-'));
  const source = join(root, 'source'); const output = join(root, 'output');
  await mkdir(join(source, 'nested'), { recursive: true });
  await writeFile(join(source, 'issues.json'), '[]'); await writeFile(join(source, 'empty.txt'), ''); await writeFile(join(source, 'nested', 'binary.bin'), Buffer.from([0, 1, 2, 255]));
  await execFile(binary, ['drill', '--source', source, '--target', 'forgejo:9.0', '--output', output], { env: { ...process.env, GFED_PASSPHRASE: 'browser claim passphrase' } });
  const { stdout } = await execFile(binary, ['--json', 'verify', join(output, 'evidence.gfed')], { env: { ...process.env, GFED_PASSPHRASE: 'browser claim passphrase' } });
  const verified = JSON.parse(stdout) as { evidence_files: Array<{ path: string; sha256: string }> };
  const sourceFiles = ['empty.txt', 'issues.json', 'nested/binary.bin'];
  const expected = await Promise.all(sourceFiles.map(async (path) => ({
    path,
    sha256: createHash('sha256').update(await readFile(join(source, path))).digest('hex'),
  })));
  expect(verified.evidence_files).toEqual(expected);
});

test('@claim:api-metadata-blocks-git API metadata reports Git history as missing and blocks readiness', async () => {
  const requests: string[] = [];
  const server = createServer((request, response) => { requests.push(request.url ?? ''); response.setHeader('content-type', 'application/json'); if (request.url === '/repos/acme/api-only') response.end(JSON.stringify({ full_name: 'acme/api-only' })); else if (request.url?.includes('/actions/workflows')) response.end(JSON.stringify({ workflows: [] })); else if (request.url?.includes('/actions/runs')) response.end(JSON.stringify({ workflow_runs: [] })); else response.end('[]'); });
  await new Promise<void>((resolve) => server.listen(0, '127.0.0.1', resolve));
  const address = server.address(); if (!address || typeof address === 'string') throw new Error('fixture did not start');
  const output = await mkdtemp(join(tmpdir(), 'gfed-api-blocked-'));
  try { await execFile(binary, ['drill', '--repo', 'acme/api-only', '--target', 'forgejo:9.0', '--output', output], { env: { ...process.env, GITHUB_TOKEN: 'api-fixture', GFED_PASSPHRASE: 'browser claim passphrase', GFED_GITHUB_API_BASE: `http://127.0.0.1:${address.port}` } }); } finally { server.close(); }
  const report = JSON.parse(await readFile(join(output, 'readiness.json'), 'utf8'));
  expect(report.outcome).toBe('blocked');
  expect(report.findings.find((finding: { artifact: string }) => finding.artifact === 'git_repository')).toMatchObject({ captured: false, result: 'missing evidence' });
  expect(report.unavailable.git_repository).toContain('metadata only');
  expect(requests.join('\n')).not.toContain('git/objects');
});

test('@claim:json-summary successful --json output is parseable and contains command paths', async () => {
  const output = await mkdtemp(join(tmpdir(), 'gfed-json-summary-'));
  const { stdout } = await execFile(binary, ['--json', 'drill', '--source', sample, '--target', 'forgejo:9.0', '--output', output], { env: { ...process.env, GFED_PASSPHRASE: 'browser claim passphrase' } });
  const result = JSON.parse(stdout);
  expect(result.repository).toBe('acme-labs/atlas-notes'); expect(result.markdown_report).toContain('readiness.md'); expect(result.evidence_archive).toContain('evidence.gfed');
  try {
    await execFile(binary, ['--json', 'drill', '--source', join(output, 'missing'), '--target', 'forgejo:9.0'], { env: { ...process.env, GFED_PASSPHRASE: 'browser claim passphrase' } });
    throw new Error('missing source unexpectedly succeeded');
  } catch (error) {
    const failure = error as { code?: number; stdout?: string };
    expect(failure.code).toBe(1);
    expect(JSON.parse(failure.stdout ?? '')).toMatchObject({ ok: false, error: expect.stringContaining('check --source and try again') });
  }
});

test('@claim:actionable-errors documented setup errors exit non-zero with one next step', async () => {
  const missing = await mkdtemp(join(tmpdir(), 'gfed-missing-source-'));
  await expect(execFile(binary, ['drill', '--source', join(missing, 'missing'), '--target', 'forgejo:9.0'], { env: { ...process.env, GFED_PASSPHRASE: 'browser claim passphrase' } })).rejects.toMatchObject({ code: 1, stderr: expect.stringContaining('check --source and try again') });
  await expect(execFile(binary, ['drill', '--source', sample, '--target', 'forgejo:9.0'], { env: { ...process.env, GFED_PASSPHRASE: 'short' } })).rejects.toMatchObject({ code: 1, stderr: expect.stringContaining('set a longer value and try again') });
});

test('@claim:cli-network-boundaries local work avoids the network while API and license checks use only configured origins', async () => {
  const localOutput = await mkdtemp(join(tmpdir(), 'gfed-network-local-'));
  await execFile(binary, ['drill', '--source', sample, '--target', 'forgejo:9.0', '--output', localOutput], { env: { ...process.env, GFED_PASSPHRASE: 'browser claim passphrase', HTTP_PROXY: 'http://127.0.0.1:1', HTTPS_PROXY: 'http://127.0.0.1:1', NO_PROXY: '' } });
  const apiCalls: string[] = [];
  const api = createServer((request, response) => {
    apiCalls.push(request.url ?? ''); response.setHeader('content-type', 'application/json');
    if (request.url === '/repos/acme/network') response.end(JSON.stringify({ full_name: 'acme/network' }));
    else if (request.url?.includes('/actions/workflows')) response.end(JSON.stringify({ workflows: [] }));
    else if (request.url?.includes('/actions/runs')) response.end(JSON.stringify({ workflow_runs: [] }));
    else response.end('[]');
  });
  await new Promise<void>((resolve) => api.listen(0, '127.0.0.1', resolve));
  const apiAddress = api.address(); if (!apiAddress || typeof apiAddress === 'string') throw new Error('API fixture did not start');
  const apiOutput = await mkdtemp(join(tmpdir(), 'gfed-network-api-'));
  try { await execFile(binary, ['drill', '--repo', 'acme/network', '--target', 'forgejo:9.0', '--output', apiOutput], { env: { ...process.env, GITHUB_TOKEN: 'network-fixture', GFED_PASSPHRASE: 'browser claim passphrase', GFED_GITHUB_API_BASE: `http://127.0.0.1:${apiAddress.port}` } }); } finally { api.close(); }
  expect(apiCalls).toContain('/repos/acme/network');
  expect(apiCalls.every((call) => call.startsWith('/repos/acme/network'))).toBe(true);
  const calls: string[] = []; const billing = createServer((request, response) => { calls.push(request.url ?? ''); response.setHeader('content-type', 'application/json'); response.end(JSON.stringify({ valid: true, reason: 'ok' })); });
  await new Promise<void>((resolve) => billing.listen(0, '127.0.0.1', resolve)); const address = billing.address(); if (!address || typeof address === 'string') throw new Error('billing fixture did not start');
  const root = await mkdtemp(join(tmpdir(), 'gfed-network-license-'));
  try { await execFile(binary, ['portfolio', '--source', sample, '--target', 'forgejo:9.0', '--output', join(root, 'portfolio')], { env: { ...process.env, GFED_PASSPHRASE: 'browser claim passphrase', GFED_LICENSE: 'fixture-license', GFED_BILLING_BASE: `http://127.0.0.1:${address.port}`, XDG_CONFIG_HOME: join(root, 'config') } }); } finally { billing.close(); }
  expect(calls).toEqual(['/api/v1/products/git-forge-exit-drill/verify?license=fixture-license']);
});

test('built deep-link documents have route-specific source metadata', async () => {
  for (const [path, title, description, canonical] of [['demo', 'Demo — Git Forge Exit Drill', 'See a complete GitHub move check with bundled sample data.', 'https://git-forge-exit-drill.sociobot.in/demo'], ['privacy', 'Privacy — Git Forge Exit Drill', 'Learn what the local CLI reads, stores, and sends.', 'https://git-forge-exit-drill.sociobot.in/privacy'], ['terms', 'Terms — Git Forge Exit Drill', 'Read the terms for Git Forge Exit Drill and Team Pack.', 'https://git-forge-exit-drill.sociobot.in/terms']]) {
    const html = await readFile(join(process.cwd(), 'dist', 'site', path, 'index.html'), 'utf8');
    expect(html).toContain(`<title>${title}</title>`); expect(html).toContain(`description" content="${description}"`); expect(html).toContain(`canonical" href="${canonical}"`); expect(html).toContain(`og:title" content="${title}"`); expect(html).toContain(`og:description" content="${description}"`); expect(html).toContain(`og:url" content="${canonical}"`); expect(html).toContain(`twitter:title" content="${title}"`); expect(html).toContain(`twitter:description" content="${description}"`);
  }
});

test('browser Back restores focus to the install heading', async ({ page }) => {
  await page.goto('/'); await page.getByRole('link', { name: 'Install' }).click(); await page.getByRole('link', { name: 'Try it with sample data' }).click(); await page.goBack(); await expect(page.locator('#install-title')).toBeFocused();
});
