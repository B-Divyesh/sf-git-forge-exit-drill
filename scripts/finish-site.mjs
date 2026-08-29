import { access, chmod, copyFile, mkdir, readFile, writeFile } from 'node:fs/promises';

const origin = 'https://git-forge-exit-drill.sociobot.in';
const home = await readFile('dist/site/index.html', 'utf8');
const routes = {
  '/demo': {
    title: 'Demo — Git Forge Exit Drill',
    description: 'See a complete GitHub move check with bundled sample data.',
  },
  '/privacy': {
    title: 'Privacy — Git Forge Exit Drill',
    description: 'Learn what the local CLI reads, stores, and sends.',
  },
  '/terms': {
    title: 'Terms — Git Forge Exit Drill',
    description: 'Read the terms for Git Forge Exit Drill and Team Pack.',
  },
  '/404': {
    title: 'Page not found — Git Forge Exit Drill',
    description: 'Return to the Git Forge Exit Drill home page.',
  },
};

function routeDocument(route, { title, description }) {
  return home
    .replace(/<title>[^<]*<\/title>/, `<title>${title}</title>`)
    .replace(/(<meta name="description" content=")[^"]*(" \/>)/, `$1${description}$2`)
    .replace(/(<link rel="canonical" href=")[^"]*(" \/>)/, `$1${origin}${route === '/404' ? '/404' : route}$2`)
    .replace(/(<meta property="og:title" content=")[^"]*(" \/>)/, `$1${title}$2`)
    .replace(/(<meta property="og:description" content=")[^"]*(" \/>)/, `$1${description}$2`)
    .replace(/(<meta property="og:url" content=")[^"]*(" \/>)/, `$1${origin}${route === '/404' ? '/404' : route}$2`)
    .replace(/(<meta name="twitter:title" content=")[^"]*(" \/>)/, `$1${title}$2`)
    .replace(/(<meta name="twitter:description" content=")[^"]*(" \/>)/, `$1${description}$2`);
}

for (const [route, metadata] of Object.entries(routes)) {
  const document = routeDocument(route, metadata);
  if (route === '/404') {
    await writeFile('dist/site/404.html', document);
  } else {
    const directory = `dist/site${route}`;
    await mkdir(directory, { recursive: true });
    await writeFile(`${directory}/index.html`, document);
  }
}

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
