const fs = require('node:fs');
const path = require('node:path');
const {base, chromium, outputPath} = require('./showcase-browser-env.cjs');

const directory = outputPath('example-review');
const output = path.join(directory, 'panel-preview-bounds');
const reference = process.env.SAPID_MANTINE_BASE || 'https://mantine.dev';
const read = file => JSON.parse(fs.readFileSync(file, 'utf8'));

async function captureComponent(context, slug, examples) {
  const [official, local] = await Promise.all([context.newPage(), context.newPage()]);
  try {
    const upstream = read(path.join(directory, 'upstream', slug, 'index.json'));
    const route = new URL(new URL(upstream.url).pathname, reference);
    await Promise.all([
      official.goto(route.href),
      local.goto(base + '/showcase.html?widget=' + slug),
    ]);
    await local.waitForFunction(() => window.ready);
    await Promise.all([local, official].map(page => page.evaluate(() => document.fonts.ready)));
    const localAsset = await local.locator('link[href*="assets-"][href$="site.css"]').first().getAttribute('href');
    const captures = [];
    for (const {ordinal, section} of examples) {
      const upstreamPreview = official.locator('#mdx .m_a3c6e060').nth(ordinal - 1);
      const detail = local.locator('.site-component-detail').filter({has: local.locator('[id="' + section + '"]')});
      const parent = section === 'usage' && !await detail.count()
        ? local.locator('.site-doc-content .site-preview').first()
        : detail;
      const currentPreview = parent.locator('.site-playground-preview,.site-styles-preview').first();
      const stem = slug + '-' + ordinal;
      await Promise.all([
        upstreamPreview.screenshot({path: path.join(output, stem + '-mantine.png'), animations: 'disabled'}),
        currentPreview.screenshot({path: path.join(output, stem + '-local.png'), animations: 'disabled'}),
      ]);
      captures.push({slug, ordinal, section, asset: localAsset});
    }
    return captures;
  } finally {
    await Promise.all([local.close(), official.close()]);
  }
}

(async () => {
  const components = new Map();
  for (const example of read(path.join(directory, 'examples.json'))) {
    if (!example.panel || !example.section) continue;
    if (!components.has(example.component)) components.set(example.component, []);
    components.get(example.component).push(example);
  }
  fs.mkdirSync(output, {recursive: true});
  const browser = await chromium.launch();
  const context = await browser.newContext({viewport: {width: 1336, height: 980}, reducedMotion: 'reduce'});
  const queue = [...components];
  const captures = [], errors = [];
  try {
    await Promise.all(Array.from({length: 4}, async () => {
      while (queue.length) {
        const [slug, examples] = queue.shift();
        try {
          captures.push(...await captureComponent(context, slug, examples));
        } catch (error) {
          errors.push({slug, error: String(error).split('\n')[0]});
        }
      }
    }));
  } finally {
    await browser.close();
    fs.writeFileSync(path.join(output, 'capture.json'), JSON.stringify({base, reference, captures, errors}, null, 2));
  }
  console.log('Paired panel previews:', captures.length, 'errors:', errors.length);
  if (errors.length || !captures.length) process.exitCode = 1;
})().catch(error => { console.error(error); process.exitCode = 1; });
