import './style.css';

const PRODUCT = 'git-forge-exit-drill';
const BILLING = 'https://api.sociobot.in';
const app = document.querySelector<HTMLDivElement>('#app')!;

type Route = '/' | '/demo' | '/privacy' | '/terms' | '/404';

const terminalLines = [
  { kind: 'command', text: '$ git-forge-exit-drill demo' },
  { kind: 'muted', text: 'Demo — sample data, nothing was read from your workspace.' },
  { kind: 'plain', text: 'Repository: acme-labs/atlas-notes' },
  { kind: 'plain', text: 'Target: Forgejo 9.0' },
  { kind: 'danger', text: 'Outcome: BLOCKED' },
  { kind: 'pass', text: '✓ Git repository · captured · native' },
  { kind: 'pass', text: '✓ Issues · 18 captured · native' },
  { kind: 'warn', text: '! Pull requests · 12 captured · restore test' },
  { kind: 'danger', text: '× Actions run history · target gap' },
  { kind: 'warn', text: '! Secrets · missing evidence' },
  { kind: 'plain', text: 'Encrypted evidence: …/result/evidence.gfed' },
  { kind: 'plain', text: 'Report: …/result/readiness.md' },
];

function header(): string {
  return `<header class="site-header">
    <a class="wordmark" href="/" data-link aria-label="Git Forge Exit Drill home"><span class="mark" aria-hidden="true"></span><span>EXIT/DRILL</span></a>
    <nav aria-label="Main navigation">
      <a href="/demo" data-link>Demo</a>
      <a href="/#install" data-link>Install</a>
      <a href="/privacy" data-link>Privacy</a>
    </nav>
  </header>`;
}

function footer(): string {
  return `<footer class="site-footer">
    <p>Test a GitHub move before cutover.</p>
    <div><a href="/privacy" data-link>Privacy</a><a href="/terms" data-link>Terms</a><a href="https://sociobot.in" rel="external">Built by Param Factory <span class="sr-only">(external)</span></a></div>
    <p class="build">v0.1.0 · build 2026.08.28</p>
  </footer>`;
}

function terminal(compact = false): string {
  return `<div class="terminal ${compact ? 'terminal-compact' : ''}" aria-label="Recorded terminal output from the bundled sample drill">
    <div class="terminal-bar"><span></span><span></span><span></span><b>sample / atlas-notes</b></div>
    <pre tabindex="0" aria-label="Terminal transcript">${terminalLines.map((line) => `<span class="line ${line.kind}">${escapeHtml(line.text)}</span>`).join('')}</pre>
  </div>`;
}

function home(): string {
  return `${header()}<main id="main">
    <section class="hero" aria-labelledby="page-title">
      <div class="hero-copy">
        <p class="eyebrow">Migration readiness / CLI</p>
        <h1 id="page-title" tabindex="-1">Test your GitHub exit before cutover</h1>
        <p class="lede">For small teams moving forges, it finds missing history and build evidence before Monday.</p>
        <div class="hero-action"><a class="button primary" href="/demo" data-link>Try it with sample data</a><p>See a complete drill with no setup.</p></div>
        <ul class="facts" aria-label="Product facts">
          <li><span aria-hidden="true">01</span> Local exports stay on your machine.</li>
          <li><span aria-hidden="true">02</span> No account is needed.</li>
          <li><span aria-hidden="true">03</span> One-repository drills are free.</li>
        </ul>
      </div>
      <figure class="hero-art">
        <img src="/geometry-exit-drill.webp" width="1100" height="733" alt="A tangled artifact graph crosses a boundary and becomes a checked geometric grid." fetchpriority="high" />
        <figcaption>Source artifacts cross the forge boundary. Gaps stay visible.</figcaption>
      </figure>
    </section>

    <section class="preview ruled" aria-labelledby="preview-title">
      <div class="section-index">FIELD TEST / 001</div>
      <div><h2 id="preview-title">See the gap before it becomes downtime</h2><p>The sample repository has code, issues, releases, and build history. Its report stops the move when evidence cannot cross.</p></div>
      ${terminal(true)}
    </section>

    <section class="how" aria-labelledby="how-title">
      <div class="section-index">METHOD / 003</div>
      <h2 id="how-title">Run one repeatable drill</h2>
      <ol class="steps">
        <li><span>01</span><div><h3>Inventory the source</h3><p>Read an extracted export or an authorized GitHub API repository.</p></div></li>
        <li><span>02</span><div><h3>Map the target</h3><p>Compare each artifact with a versioned GitLab, Gitea, or Forgejo map.</p></div></li>
        <li><span>03</span><div><h3>Prove the restore</h3><p>Keep encrypted evidence, then follow the generated restore checklist.</p></div></li>
      </ol>
    </section>

    <section class="install ruled" id="install" aria-labelledby="install-title">
      <div class="section-index">TERMINAL / START</div>
      <div><h2 id="install-title">Start with the bundled sample</h2><p>Build from source, then run the same sample used in this page.</p></div>
      <div class="code-block"><code>cargo install --path .<br />git-forge-exit-drill demo</code><button class="copy-button" data-copy="cargo install --path .\ngit-forge-exit-drill demo">Copy commands</button></div>
      <a class="text-link" href="/downloads/git-forge-exit-drill-linux-x86_64" download>Download Linux x86-64 binary <span aria-hidden="true">↓</span></a>
    </section>

    <section class="boundaries" aria-labelledby="boundaries-title">
      <div class="section-index">BOUNDARY / CLEAR</div>
      <div><h2 id="boundaries-title">Know what stays untouched</h2><p>The CLI does not cut over repositories, forward webhooks, or host a forge. It reads only the source you provide.</p><a class="text-link" href="/privacy" data-link>Read the privacy details <span aria-hidden="true">→</span></a></div>
      <div class="boundary-list"><p><span>NO</span> Automatic migration</p><p><span>NO</span> Background service</p><p><span>NO</span> Telemetry</p></div>
    </section>

    <section class="pricing ruled" aria-labelledby="pricing-title">
      <div class="section-index">TEAM PACK / $39</div>
      <div><h2 id="pricing-title">Check ten repositories together</h2><p>One $39 purchase adds the portfolio command and one ordered risk list. The complete one-repository drill stays free.</p><ul><li>Up to ten local exports per run</li><li>One consolidated Markdown report</li><li>License use on your own devices</li></ul></div>
      <div class="purchase">
        <a class="button primary" href="${BILLING}/api/v1/products/${PRODUCT}/checkout">Buy Team Pack — $39</a>
        <p>One-time purchase. Sociobot is the merchant of record.</p>
        <button class="button secondary" type="button" data-show-license>Have a license? Paste it</button>
        <form class="license-form" hidden>
          <label for="license-token">License token</label>
          <input id="license-token" name="license" type="password" autocomplete="off" required />
          <button class="button secondary" type="submit">Verify license</button>
          <p class="license-status" role="status" aria-live="polite"></p>
        </form>
        <p class="legal-links"><a href="/privacy" data-link>Privacy</a> · <a href="/terms" data-link>Terms</a></p>
      </div>
    </section>
  </main>${footer()}`;
}

function demo(): string {
  return `<div class="demo-banner" role="status"><span>Demo — sample data, nothing is saved</span><div><button type="button" data-reset-demo>Reset demo</button><a href="/#install" data-real>Start for real</a></div></div>
  ${header()}<main id="main" class="page-shell demo-page">
    <p class="eyebrow">Bundled sample / no setup</p>
    <h1 id="page-title" tabindex="-1">See a complete exit drill</h1>
    <p class="lede">This recording comes from the real CLI and its bundled Atlas Notes export.</p>
    ${terminal()}
    <section class="demo-findings" aria-labelledby="finding-title">
      <div><p class="outcome"><span>BLOCKED</span> Cutover should wait</p><h2 id="finding-title">The report found two critical risks</h2><p>Past Actions runs cannot become native Forgejo history. Secret values are also absent from GitHub exports.</p></div>
      <ol><li><span>01</span>Save old build logs and artifact checksums.</li><li><span>02</span>Recreate secrets through the target’s secure process.</li><li><span>03</span>Run one build from a pinned commit.</li></ol>
    </section>
    <div class="demo-next"><a class="button primary" href="/#install" data-real>Run your own drill</a><p>The CLI reads your export locally.</p></div>
  </main>${footer()}`;
}

function privacy(): string {
  return `${header()}<main id="main" class="page-shell legal-page"><p class="eyebrow">Policy / 2026.08.28</p><h1 id="page-title" tabindex="-1">Keep repository evidence private</h1>
    <p class="lede">The CLI works from your computer. This site does not receive repository exports.</p>
    <h2>Data the CLI handles</h2><p>Local mode reads the export directory you choose. It writes reports and an encrypted archive to your chosen output directory.</p>
    <h2>Network requests</h2><p>Local drills make no network requests. API drills contact GitHub with the token environment variable you name. Portfolio license checks contact the Sociobot billing API.</p>
    <h2>License storage</h2><p>This site stores a pasted license in your browser under <code>sb_license:${PRODUCT}</code>. It stores the last verdict for one day. You can remove both items with the button below.</p>
    <button class="button secondary" type="button" data-clear-license>Remove saved license</button><p class="license-status" role="status" aria-live="polite"></p>
    <h2>Payments</h2><p>Sociobot and Dodo handle checkout, receipts, refunds, and payment data. This site never receives card details.</p>
    <h2>Contact</h2><p>Email <a href="mailto:privacy@sociobot.in">privacy@sociobot.in</a> with a privacy question.</p>
  </main>${footer()}`;
}

function terms(): string {
  return `${header()}<main id="main" class="page-shell legal-page"><p class="eyebrow">Terms / 2026.08.28</p><h1 id="page-title" tabindex="-1">Use the drill before you cut over</h1>
    <p class="lede">These terms cover the CLI, this site, and the Team Pack license.</p>
    <h2>Your responsibility</h2><p>Use only exports, repositories, and tokens you are allowed to access. Review every finding before changing a production forge.</p>
    <h2>What the report means</h2><p>Capability maps are planning baselines for named target versions. Forge settings and importers change. A report does not guarantee a complete migration.</p>
    <h2>Team Pack</h2><p>The Team Pack costs $39 once. It adds portfolio reports for up to ten repositories per run. Sociobot is the merchant of record. Approved refunds revoke the license.</p>
    <h2>No warranty</h2><p>The software is provided under the MIT License without warranty. Keep independent backups before every migration or restore test.</p>
    <h2>Contact</h2><p>Email <a href="mailto:support@sociobot.in">support@sociobot.in</a> for purchase help.</p>
  </main>${footer()}`;
}

function notFound(): string {
  return `${header()}<main id="main" class="page-shell not-found"><p class="eyebrow">Route / missing</p><h1 id="page-title" tabindex="-1">This route has no evidence</h1><p class="lede">The page may have moved. Return to the drill start.</p><a class="button primary" href="/" data-link>Return home</a><div class="lost-node" aria-hidden="true"><span></span></div></main>${footer()}`;
}

function currentRoute(): Route {
  const path = window.location.pathname.replace(/\/$/, '') || '/';
  if (path === '/demo' || path === '/privacy' || path === '/terms') return path;
  return path === '/' ? '/' : '/404';
}

const metadata: Record<Route, { title: string; description: string }> = {
  '/': { title: 'Git Forge Exit Drill — test a GitHub move', description: 'Inventory a GitHub export, encrypt the evidence, and find migration gaps before cutover.' },
  '/demo': { title: 'Demo — Git Forge Exit Drill', description: 'See a complete migration readiness drill with bundled sample data.' },
  '/privacy': { title: 'Privacy — Git Forge Exit Drill', description: 'Learn what the local CLI reads, stores, and sends.' },
  '/terms': { title: 'Terms — Git Forge Exit Drill', description: 'Read the terms for Git Forge Exit Drill and its Team Pack.' },
  '/404': { title: 'Page not found — Git Forge Exit Drill', description: 'Return to the Git Forge Exit Drill home page.' },
};

function render(moveFocus = true): void {
  const route = currentRoute();
  if (route !== '/demo') localStorage.removeItem('demo:gfed:started');
  if (route === '/demo') localStorage.setItem('demo:gfed:started', String(Date.now()));
  app.innerHTML = route === '/' ? home() : route === '/demo' ? demo() : route === '/privacy' ? privacy() : route === '/terms' ? terms() : notFound();
  document.title = metadata[route].title;
  document.querySelector<HTMLMetaElement>('meta[name="description"]')!.content = metadata[route].description;
  document.querySelector<HTMLLinkElement>('link[rel="canonical"]')!.href = `https://git-forge-exit-drill.sociobot.in${route === '/404' ? '/404' : route}`;
  bindActions();
  if (moveFocus) {
    window.scrollTo({ top: 0 });
    document.querySelector<HTMLElement>('h1')?.focus({ preventScroll: true });
    announce(document.querySelector('h1')?.textContent ?? 'Page loaded');
  }
}

function bindActions(): void {
  document.querySelectorAll<HTMLAnchorElement>('[data-link]').forEach((link) => link.addEventListener('click', navigate));
  document.querySelectorAll<HTMLAnchorElement>('[data-real]').forEach((link) => link.addEventListener('click', (event) => {
    localStorage.removeItem('demo:gfed:started');
    navigate(event);
  }));
  document.querySelector<HTMLButtonElement>('[data-reset-demo]')?.addEventListener('click', () => {
    localStorage.removeItem('demo:gfed:started');
    render(false);
    announce('Demo reset with fresh sample data');
  });
  document.querySelector<HTMLButtonElement>('[data-copy]')?.addEventListener('click', async (event) => {
    const button = event.currentTarget as HTMLButtonElement;
    await navigator.clipboard.writeText(button.dataset.copy ?? '');
    button.textContent = 'Commands copied';
    announce('Install commands copied');
  });
  document.querySelector<HTMLButtonElement>('[data-show-license]')?.addEventListener('click', () => {
    const form = document.querySelector<HTMLFormElement>('.license-form')!;
    form.hidden = false;
    form.querySelector<HTMLInputElement>('input')?.focus();
  });
  document.querySelector<HTMLFormElement>('.license-form')?.addEventListener('submit', submitLicense);
  document.querySelector<HTMLButtonElement>('[data-clear-license]')?.addEventListener('click', () => {
    localStorage.removeItem(`sb_license:${PRODUCT}`);
    localStorage.removeItem(`sb_license_cache:${PRODUCT}`);
    setLicenseStatus('Saved license removed.');
  });
}

function navigate(event: Event): void {
  const link = event.currentTarget as HTMLAnchorElement;
  if (link.origin !== window.location.origin) return;
  event.preventDefault();
  const next = `${link.pathname}${link.search}${link.hash}`;
  history.pushState({}, '', next);
  render();
  if (link.hash) requestAnimationFrame(() => document.querySelector(link.hash)?.scrollIntoView());
}

async function submitLicense(event: SubmitEvent): Promise<void> {
  event.preventDefault();
  const form = event.currentTarget as HTMLFormElement;
  const token = new FormData(form).get('license')?.toString().trim() ?? '';
  if (!token) return;
  localStorage.setItem(`sb_license:${PRODUCT}`, token);
  setLicenseStatus('Checking license…');
  await verifyLicense(token, true);
}

async function verifyLicense(token: string, force = false): Promise<void> {
  const cacheKey = `sb_license_cache:${PRODUCT}`;
  const cached = JSON.parse(localStorage.getItem(cacheKey) || 'null') as { valid: boolean; checkedAt: number } | null;
  if (!force && cached && Date.now() - cached.checkedAt < 86_400_000) {
    setLicenseStatus(cached.valid ? 'Team Pack license active.' : 'License no longer active.');
    return;
  }
  try {
    const response = await fetch(`${BILLING}/api/v1/products/${PRODUCT}/verify?license=${encodeURIComponent(token)}`);
    const verdict = await response.json() as { valid: boolean };
    localStorage.setItem(cacheKey, JSON.stringify({ valid: verdict.valid, checkedAt: Date.now() }));
    setLicenseStatus(verdict.valid ? 'Team Pack license active.' : 'License no longer active.');
  } catch {
    setLicenseStatus('License check failed. Connect to the internet and try again.');
  }
}

function setLicenseStatus(message: string): void {
  const status = document.querySelector<HTMLElement>('.license-status');
  if (status) status.textContent = message;
}

function acceptReturnedLicense(): void {
  const params = new URLSearchParams(window.location.search);
  const token = params.get('license');
  if (!token) return;
  localStorage.setItem(`sb_license:${PRODUCT}`, token);
  params.delete('license');
  history.replaceState({}, '', `${window.location.pathname}${params.size ? `?${params}` : ''}${window.location.hash}`);
  void verifyLicense(token, true);
}

function announce(message: string): void {
  let region = document.querySelector<HTMLDivElement>('#route-status');
  if (!region) {
    region = document.createElement('div');
    region.id = 'route-status';
    region.className = 'sr-only';
    region.setAttribute('aria-live', 'polite');
    document.body.append(region);
  }
  region.textContent = message;
}

function escapeHtml(value: string): string {
  return value.replace(/[&<>"']/g, (character) => ({ '&': '&amp;', '<': '&lt;', '>': '&gt;', '"': '&quot;', "'": '&#039;' })[character]!);
}

window.addEventListener('popstate', () => render());
render(false);
acceptReturnedLicense();
const savedLicense = localStorage.getItem(`sb_license:${PRODUCT}`);
if (savedLicense) void verifyLicense(savedLicense);
if ('serviceWorker' in navigator && import.meta.env.PROD) window.addEventListener('load', () => navigator.serviceWorker.register('/sw.js'));
