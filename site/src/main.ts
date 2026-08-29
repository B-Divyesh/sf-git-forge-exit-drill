import './style.css';

const PRODUCT = 'git-forge-exit-drill';
const BILLING = 'https://api.sociobot.in';
const app = document.querySelector<HTMLDivElement>('#app')!;

type Route = '/' | '/demo' | '/privacy' | '/terms' | '/404';

const terminalLines = [
  { kind: 'command', text: '$ git-forge-exit-drill demo' },
  { kind: 'muted', text: 'Demo — sample data. No workspace files were read.' },
  { kind: 'plain', text: 'Repository: acme-labs/atlas-notes' },
  { kind: 'plain', text: 'Target: Forgejo 9.0' },
  { kind: 'danger', text: 'Outcome: BLOCKED' },
  { kind: 'plain', text: 'Demo archive passphrase: demo-only-passphrase' },
  { kind: 'plain', text: 'Choose a new output directory to run this demo again.' },
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
    <div><a href="/privacy" data-link>Privacy</a><a href="/terms" data-link>Terms</a><a href="https://sociobot.in" rel="external">Built by Param Factory <span aria-label="(external)">↗</span></a></div>
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
        <p class="eyebrow">Git host migration check</p>
        <h1 id="page-title" tabindex="-1">Test your GitHub move before cutover</h1>
        <p class="lede">For small teams changing Git hosts, it finds missing repository history and build evidence before cutover.</p>
        <div class="hero-action"><a class="button primary" href="/demo" data-link>Try it with sample data</a><p>See a complete drill with no setup.</p></div>
        <ul class="facts" aria-label="Product facts">
          <li><span aria-hidden="true">01</span> Local drills need no network connection.</li>
          <li><span aria-hidden="true">02</span> Sample data stays in demo storage.</li>
          <li><span aria-hidden="true">03</span> One-repository drills are free.</li>
        </ul>
      </div>
      <figure class="hero-art">
        <img src="/geometry-exit-drill.webp" width="1100" height="733" alt="A GitHub repository-item graph maps to a checked target grid with unsupported items marked." fetchpriority="high" />
        <figcaption>The drill maps each GitHub repository item to the target and marks unsupported items.</figcaption>
      </figure>
    </section>

    <section class="preview ruled" aria-labelledby="preview-title">
      <div class="section-index">Sample result</div>
      <div><h2 id="preview-title">Sample drill results</h2><p>The sample repository has code, issues, releases, and build history. The drill counts an item only after it validates exported records.</p></div>
      ${terminal(true)}
    </section>

    <section class="how" aria-labelledby="how-title">
      <div class="section-index">How it works</div>
      <h2 id="how-title">Run one repeatable drill</h2>
      <ol class="steps">
        <li><span>01</span><div><h3>Inventory the source</h3><p>Read an extracted export or an authorized GitHub API repository.</p></div></li>
        <li><span>02</span><div><h3>Map the target</h3><p>Compare each repository item with GitLab, Gitea, or Forgejo version maps.</p></div></li>
        <li><span>03</span><div><h3>Plan the restore test</h3><p>Keep encrypted evidence and use the generated restore checklist.</p></div></li>
      </ol>
    </section>

    <section class="install ruled" id="install" aria-labelledby="install-title">
      <div class="section-index">Install</div>
      <div><h2 id="install-title" tabindex="-1">Start with the bundled sample</h2><p>Build from source, then run the sample shown on this page.</p></div>
      <div class="code-block"><code tabindex="-1" data-copy-code>cargo install --path .<br />git-forge-exit-drill demo</code><button class="copy-button" data-copy="cargo install --path .\ngit-forge-exit-drill demo">Copy commands</button><p class="copy-status" data-copy-feedback role="status" aria-live="polite"></p></div>
      <a class="text-link" href="/downloads/git-forge-exit-drill-linux-x86_64" download>Download Linux x86-64 binary <span aria-hidden="true">↓</span></a>
    </section>

    <section class="boundaries" aria-labelledby="boundaries-title">
      <div class="section-index">Limits</div>
      <div><h2 id="boundaries-title">Know what the CLI writes</h2><p>The CLI writes reports and an evidence archive to the output directory. It does not change your selected export.</p><a class="text-link" href="/privacy" data-link>Read the privacy details <span aria-hidden="true">→</span></a></div>
      <div class="boundary-list"><p><span>LOCAL</span> Output stays in your chosen directory</p><p><span>CHECK</span> Review the readiness report before cutover</p><p><span>NO</span> Sample demo telemetry</p></div>
    </section>

    <section class="pricing ruled" aria-labelledby="pricing-title">
      <div class="section-index">Team Pack</div>
      <div><h2 id="pricing-title">Check ten repositories together</h2><p>A $39 one-time purchase adds the portfolio command and one consolidated readiness report. The one-repository drill stays free.</p><ul><li>Up to ten local exports per run</li><li>One consolidated readiness report in Markdown</li></ul></div>
      <div class="purchase">
        <a class="button primary" href="${BILLING}/api/v1/products/${PRODUCT}/checkout">Buy Team Pack — $39 <span aria-label="(hosted checkout)">↗</span></a>
        <p>You buy from Sociobot through its hosted checkout.</p>
        <button class="button secondary" type="button" data-show-license-form>Enter Team Pack license</button>
        <form class="license-form" hidden>
          <label for="license-token">License token</label>
          <input id="license-token" name="license" type="password" autocomplete="off" required />
          <button class="button secondary" type="submit">Verify license</button>
        </form>
        <p class="license-status" role="status" aria-live="polite"></p>
        <section class="license-handoff" data-license-handoff hidden aria-labelledby="cli-license-title">
          <h3 id="cli-license-title">Use your license in the CLI</h3>
          <p>Copy this private token, then set it in the terminal that runs the portfolio command.</p>
          <label for="cli-license-token">Team Pack license token</label>
          <div class="license-token-control"><input id="cli-license-token" type="password" readonly autocomplete="off" spellcheck="false" /><button class="button secondary" type="button" data-show-license aria-pressed="false">Show license</button><button class="button secondary" type="button" data-copy-license>Copy license</button></div>
          <p>Set the token before running <code>portfolio</code>.</p>
          <div class="cli-command"><code>export GFED_LICENSE='paste-license-here'</code><button type="button" data-copy-license-command>Copy setup command</button></div>
          <p class="license-handoff-status" role="status" aria-live="polite"></p>
        </section>
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
    <p class="lede">This recording matches the bundled CLI and Atlas Notes sample export.</p>
    ${terminal()}
    <section class="demo-findings" aria-labelledby="finding-title">
      <div><p class="outcome"><span>BLOCKED</span> Cutover should wait</p><h2 id="finding-title">The report found critical risks</h2><p>Past Actions runs cannot become native Forgejo history. The report also asks for missing export evidence before cutover.</p></div>
      <ol><li><span>01</span>Save old build logs and artifact checksums.</li><li><span>02</span>Recreate secrets through the target’s secure process.</li><li><span>03</span>Run one build from a pinned commit.</li></ol>
    </section>
    <div class="demo-next"><a class="button primary" href="/#install" data-real>Run your own drill</a><p>Run the CLI against a local export.</p></div>
  </main>${footer()}`;
}

function privacy(): string {
  return `${header()}<main id="main" class="page-shell legal-page"><p class="eyebrow">Privacy</p><h1 id="page-title" tabindex="-1">Keep repository evidence private</h1>
    <p class="lede">Run the CLI on your computer. It writes evidence to your chosen local directory.</p>
    <h2>Data the CLI handles</h2><p>Local mode reads the export directory you choose. It writes reports and an encrypted archive to your chosen output directory.</p>
    <h2>Network requests</h2><p>Local drills make no network requests. API drills contact the GitHub API origin you configure. Portfolio license checks contact the Sociobot billing API.</p>
    <h2>License storage</h2><p>This site stores a pasted license and its last verdict in your browser. You can remove both items with the button below.</p>
    <button class="button secondary" type="button" data-clear-license>Remove saved license</button><p class="license-status" role="status" aria-live="polite"></p>
    <h2>Payments</h2><p>The buy link opens a hosted Sociobot checkout. Read its payment terms before you buy.</p>
    <h2>Contact</h2><p>Email <a href="mailto:privacy@sociobot.in">privacy@sociobot.in</a> with a privacy question.</p>
  </main>${footer()}`;
}

function terms(): string {
  return `${header()}<main id="main" class="page-shell legal-page"><p class="eyebrow">Terms</p><h1 id="page-title" tabindex="-1">Use the drill before you cut over</h1>
    <p class="lede">These terms cover the CLI, this site, and the Team Pack license.</p>
    <h2>Your responsibility</h2><p>Use only exports, repositories, and tokens you are allowed to access. Review every finding before changing a production Git host.</p>
    <h2>What the report means</h2><p>Capability maps are planning baselines for named target versions. Git-host settings and importers change. A report does not guarantee a complete migration.</p>
    <h2>Team Pack</h2><p>The Team Pack costs $39 once. It adds portfolio reports for up to ten repositories per run. The buy link opens hosted checkout terms.</p>
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

function render(moveFocus = true, restoreHash = false): void {
  const route = currentRoute();
  if (route !== '/demo') localStorage.removeItem('demo:gfed:started');
  if (route === '/demo') localStorage.setItem('demo:gfed:started', String(Date.now()));
  app.innerHTML = route === '/' ? home() : route === '/demo' ? demo() : route === '/privacy' ? privacy() : route === '/terms' ? terms() : notFound();
  document.title = metadata[route].title;
  document.querySelector<HTMLMetaElement>('meta[name="description"]')!.content = metadata[route].description;
  document.querySelector<HTMLLinkElement>('link[rel="canonical"]')!.href = `https://git-forge-exit-drill.sociobot.in${route === '/404' ? '/404' : route}`;
  bindActions();
  if (moveFocus) {
    const destination = restoreHash && window.location.hash
      ? document.querySelector<HTMLElement>(window.location.hash)
      : null;
    if (destination) {
      destination.scrollIntoView();
      const focusTarget = destination.matches('[tabindex]')
        ? destination
        : destination.querySelector<HTMLElement>('[tabindex], h1, h2, h3') ?? destination;
      requestAnimationFrame(() => requestAnimationFrame(() => {
        focusTarget.focus({ preventScroll: true });
        announce(focusTarget.textContent ?? 'Section loaded');
      }));
    } else {
      window.scrollTo({ top: 0 });
      document.querySelector<HTMLElement>('h1')?.focus({ preventScroll: true });
      announce(document.querySelector('h1')?.textContent ?? 'Page loaded');
    }
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
    document.querySelector<HTMLButtonElement>('[data-reset-demo]')?.focus();
    announce('Demo reset with fresh sample data');
  });
  document.querySelector<HTMLButtonElement>('[data-copy]')?.addEventListener('click', async (event) => {
    const button = event.currentTarget as HTMLButtonElement;
    const feedback = document.querySelector<HTMLElement>('[data-copy-feedback]');
    try {
      await navigator.clipboard.writeText(button.dataset.copy ?? '');
      button.textContent = 'Commands copied';
      if (feedback) feedback.textContent = '';
      announce('Install commands copied');
    } catch {
      selectText(document.querySelector<HTMLElement>('[data-copy-code]'));
      if (feedback) feedback.textContent = 'Clipboard access was denied. Select the commands above and copy them manually.';
      announce('Clipboard access was denied. Select the commands above and copy them manually.');
    }
  });
  document.querySelector<HTMLButtonElement>('[data-show-license-form]')?.addEventListener('click', () => {
    const form = document.querySelector<HTMLFormElement>('.license-form')!;
    form.hidden = false;
    form.querySelector<HTMLInputElement>('input')?.focus();
  });
  document.querySelector<HTMLFormElement>('.license-form')?.addEventListener('submit', submitLicense);
  document.querySelector<HTMLButtonElement>('[data-show-license]')?.addEventListener('click', toggleLicenseVisibility);
  document.querySelector<HTMLButtonElement>('[data-copy-license]')?.addEventListener('click', () => copyLicenseValue('token'));
  document.querySelector<HTMLButtonElement>('[data-copy-license-command]')?.addEventListener('click', () => copyLicenseValue('command'));
  document.querySelector<HTMLButtonElement>('[data-clear-license]')?.addEventListener('click', () => {
    localStorage.removeItem(`sb_license:${PRODUCT}`);
    localStorage.removeItem(`sb_license_cache:${PRODUCT}`);
    hideLicenseHandoff();
    setLicenseStatus('Saved license removed.');
  });
}

function navigate(event: Event): void {
  const link = event.currentTarget as HTMLAnchorElement;
  if (link.origin !== window.location.origin) return;
  event.preventDefault();
  const next = `${link.pathname}${link.search}${link.hash}`;
  history.pushState({}, '', next);
  render(true, Boolean(link.hash));
}

async function submitLicense(event: SubmitEvent): Promise<void> {
  event.preventDefault();
  const form = event.currentTarget as HTMLFormElement;
  const token = new FormData(form).get('license')?.toString().trim() ?? '';
  if (!token) return;
  localStorage.setItem(`sb_license:${PRODUCT}`, token);
  setLicenseStatus('Checking license…');
  await verifyLicense(token);
}

async function verifyLicense(token: string): Promise<void> {
  const cacheKey = `sb_license_cache:${PRODUCT}`;
  const cached = readLicenseCache(cacheKey);
  if (cached?.token === token && Date.now() - cached.checkedAt < 86_400_000) {
    setLicenseStatus(cached.valid ? 'Team Pack license active.' : 'License no longer active.');
    if (cached.valid) showLicenseHandoff(token);
    else hideLicenseHandoff();
    return;
  }
  try {
    const response = await fetch(`${BILLING}/api/v1/products/${PRODUCT}/verify?license=${encodeURIComponent(token)}`);
    const verdict = await response.json() as { valid: boolean };
    localStorage.setItem(cacheKey, JSON.stringify({ token, valid: verdict.valid, checkedAt: Date.now() }));
    setLicenseStatus(verdict.valid ? 'Team Pack license active.' : 'License no longer active.');
    if (verdict.valid) showLicenseHandoff(token);
    else hideLicenseHandoff();
  } catch {
    hideLicenseHandoff();
    setLicenseStatus('License check failed. Connect to the internet and try again.');
  }
}

type LicenseCache = { token: string; valid: boolean; checkedAt: number };

function readLicenseCache(cacheKey: string): LicenseCache | null {
  const raw = localStorage.getItem(cacheKey);
  if (!raw) return null;
  try {
    const parsed: unknown = JSON.parse(raw);
    if (
      typeof parsed === 'object' && parsed !== null
      && typeof (parsed as LicenseCache).token === 'string'
      && typeof (parsed as LicenseCache).valid === 'boolean'
      && Number.isFinite((parsed as LicenseCache).checkedAt)
    ) return parsed as LicenseCache;
  } catch {
    // An interrupted browser write is a cache miss, not a page error.
  }
  localStorage.removeItem(cacheKey);
  return null;
}

function showLicenseHandoff(token: string): void {
  const handoff = document.querySelector<HTMLElement>('[data-license-handoff]');
  const input = document.querySelector<HTMLInputElement>('#cli-license-token');
  const toggle = document.querySelector<HTMLButtonElement>('[data-show-license]');
  if (!handoff || !input) return;
  input.value = token;
  input.type = 'password';
  if (toggle) {
    toggle.textContent = 'Show license';
    toggle.setAttribute('aria-pressed', 'false');
  }
  handoff.hidden = false;
}

function hideLicenseHandoff(): void {
  const handoff = document.querySelector<HTMLElement>('[data-license-handoff]');
  const input = document.querySelector<HTMLInputElement>('#cli-license-token');
  const toggle = document.querySelector<HTMLButtonElement>('[data-show-license]');
  if (input) input.value = '';
  if (input) input.type = 'password';
  if (toggle) {
    toggle.textContent = 'Show license';
    toggle.setAttribute('aria-pressed', 'false');
  }
  if (handoff) handoff.hidden = true;
}

function toggleLicenseVisibility(): void {
  const input = document.querySelector<HTMLInputElement>('#cli-license-token');
  const toggle = document.querySelector<HTMLButtonElement>('[data-show-license]');
  if (!input || !toggle) return;
  const visible = input.type === 'text';
  input.type = visible ? 'password' : 'text';
  toggle.textContent = visible ? 'Show license' : 'Hide license';
  toggle.setAttribute('aria-pressed', String(!visible));
}

async function copyLicenseValue(kind: 'token' | 'command'): Promise<void> {
  const input = document.querySelector<HTMLInputElement>('#cli-license-token');
  const feedback = document.querySelector<HTMLElement>('.license-handoff-status');
  const token = input?.value ?? '';
  if (!token) return;
  const value = kind === 'token' ? token : `export GFED_LICENSE=${shellQuote(token)}`;
  try {
    await navigator.clipboard.writeText(value);
    if (feedback) feedback.textContent = kind === 'token' ? 'License copied. Keep it private.' : 'Setup command copied. Run it in your terminal.';
    announce(kind === 'token' ? 'Team Pack license copied' : 'CLI setup command copied');
  } catch {
    input?.focus();
    input?.select();
    if (feedback) feedback.textContent = 'Select the license token, copy it, then paste it in your terminal.';
  }
}

function selectText(element: HTMLElement | null): void {
  if (!element) return;
  const selection = window.getSelection();
  if (!selection) return;
  const range = document.createRange();
  range.selectNodeContents(element);
  selection.removeAllRanges();
  selection.addRange(range);
  element.focus();
}

function shellQuote(value: string): string {
  return `'${value.replace(/'/g, "'\\''")}'`;
}

function setLicenseStatus(message: string): void {
  const status = document.querySelector<HTMLElement>('.license-status');
  if (status) status.textContent = message;
}

function acceptReturnedLicense(): string | null {
  const params = new URLSearchParams(window.location.search);
  const token = params.get('license');
  if (!token) return null;
  localStorage.setItem(`sb_license:${PRODUCT}`, token);
  params.delete('license');
  history.replaceState({}, '', `${window.location.pathname}${params.size ? `?${params}` : ''}${window.location.hash}`);
  return token;
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

function normalizeDemoQuery(): void {
  const params = new URLSearchParams(window.location.search);
  if (window.location.pathname === '/' && params.get('demo') === '1') {
    params.delete('demo');
    history.replaceState({}, '', `/demo${params.size ? `?${params}` : ''}${window.location.hash}`);
  }
}

normalizeDemoQuery();
window.addEventListener('popstate', () => render(true, true));
render(false);
if (currentRoute() !== '/demo') {
  const savedLicense = acceptReturnedLicense() ?? localStorage.getItem(`sb_license:${PRODUCT}`);
  if (savedLicense) void verifyLicense(savedLicense);
}
if ('serviceWorker' in navigator && import.meta.env.PROD) window.addEventListener('load', () => navigator.serviceWorker.register('/sw.js'));
