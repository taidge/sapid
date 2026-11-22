// Shared, environment-driven paths for browser acceptance scripts.
const fs = require('node:fs');
const path = require('node:path');
const os = require('node:os');
const { execFileSync } = require('node:child_process');

const buildRootValue = process.env.SAPID_BUILD_ROOT || JSON.parse(execFileSync('cargo',
  ['metadata', '--no-deps', '--format-version', '1'],
  { cwd: path.join(__dirname, '..', 'examples', 'web'), encoding: 'utf8' })).target_directory;
const buildRoot = path.resolve(buildRootValue);
const outputRoot = path.resolve(process.env.SAPID_BROWSER_OUTPUT || buildRoot);
for (const directory of [buildRoot, outputRoot]) {
  const normalized = directory.replaceAll('\\', '/').toLowerCase();
  if (normalized === 'd:/buildcache' || normalized.startsWith('d:/buildcache/') || normalized.includes('/.codex/build-targets')) {
    throw new Error('Browser artifacts cannot use a prohibited legacy build directory');
  }
}
const tempDir = os.tmpdir();
fs.mkdirSync(outputRoot, { recursive: true });

const base = process.env.SAPID_SHOWCASE_BASE || 'http://127.0.0.1:8795';
const acceptanceMode = process.env.SAPID_SHOWCASE_MODE || 'full';
if (!['quick', 'full'].includes(acceptanceMode)) throw Error(`Unknown showcase mode: ${acceptanceMode}`);
// Both responsive widths and all behavior assertions remain in quick runs.
// The shared site suite still covers palette switching, dark mode and SSR.
const matrix = { themes: acceptanceMode === 'quick' ? ['blue'] : ['blue', 'green'],
  schemes: acceptanceMode === 'quick' ? ['light'] : ['light', 'dark'] };

function outputPath(name) {
  const resolved = path.resolve(outputRoot, name);
  const relative = path.relative(outputRoot, resolved);
  if (relative.startsWith('..') || path.isAbsolute(relative)) {
    throw new Error(`Browser artifact must stay under ${outputRoot}: ${resolved}`);
  }
  return resolved;
}

function writeReport(name, report) {
  fs.writeFileSync(outputPath(name), JSON.stringify(report, null, 2));
}

async function assertPlaygroundLayout(primary, width) {
  const preview = await primary.locator('.site-playground-preview').boundingBox();
  const controls = await primary.locator('.site-playground-controls').boundingBox();
  const assert = require('node:assert/strict');
  assert(preview && controls, 'visible preview and controls');
  if (width > 600) {
    assert(controls.x >= preview.x + preview.width - 1 && Math.abs(controls.y - preview.y) < 1, 'desktop controls beside preview');
  } else {
    assert(controls.y >= preview.y + preview.height - 1 && Math.abs(controls.x - preview.x) < 1, 'phone controls below preview');
  }
}

module.exports = { assertPlaygroundLayout, base, buildRoot, outputPath, outputRoot, tempDir, writeReport, matrix,
  get chromium() { return require(process.env.SAPID_PLAYWRIGHT || 'playwright').chromium; } };
