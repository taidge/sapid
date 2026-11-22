const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const { base, chromium, outputPath } = require('./showcase-browser-env.cjs');

const variants = {
  'day-view': ['slot-label-interval', 'slot-label-format', 'header-format'],
  'week-view': ['slot-label-interval', 'slot-label-format', 'weekday-format', 'week-label-format'],
};

async function verifyTimeline(page, preview, slug, variant) {
  const weekly = slug === 'week-view';
  const viewport = preview.locator('.sapid-schedule-viewport');
  async function fittedColumns() {
    await page.waitForFunction(([node, weekly]) => {
      const grid = node.querySelector('.sapid-schedule-timeline');
      return grid && Math.abs(grid.getBoundingClientRect().width -
        Math.max(node.getBoundingClientRect().width, 80 + (weekly ? 7 * 81 : 28))) <= 1;
    }, [await viewport.elementHandle(), weekly]);
  }
  await fittedColumns();
  const dimensions = await viewport.evaluate(node => [node.clientHeight, node.scrollHeight]);
  assert(Math.abs(dimensions[1]-dimensions[0]) <= 1, 'All time slots fit the vertical preview');
  await page.setViewportSize({width:390,height:980});
  await fittedColumns();
  await page.setViewportSize({width:1336,height:980});
  await fittedColumns();
  const gutter = await preview.locator('.sapid-schedule-time-label').allTextContents();
  const header = preview.locator('.sapid-schedule-header');
  if (variant === 'slot-label-interval') {
    assert(gutter.includes(weekly ? '08:30' : '08:15'));
    assert.equal(gutter.filter(value => /^\d\d:\d\d$/.test(value)).length, weekly ? 20 : 40);
  }
  if (variant === 'slot-label-format') {
    assert(gutter.includes('8:00 AM'));
    assert(gutter.includes('5:00 PM'));
  }
  if (variant === 'weekday-format') {
    const days = await preview.locator('[role=columnheader]').allTextContents();
    for (const day of ['Mo', 'Tu', 'We', 'Th', 'Fr', 'Sa', 'Su']) assert(days.includes(day), day);
  }
  if (variant === 'header-format') assert.equal((await header.innerText()).trim(), 'Sunday, November 22');
  if (variant === 'week-label-format') assert.match(await header.innerText(), /November 16, 2026.*November 22, 2026/s);

  const slot = preview.locator('.sapid-schedule-slot').first();
  assert.equal((await slot.innerText()).trim(), '');
  assert.match(await slot.getAttribute('aria-label'), /08:00/);
  await slot.click();
  await page.waitForFunction(el => el.closest('[role=gridcell]').getAttribute('aria-selected') === 'true',
    await slot.elementHandle());
  const oldHeading = await header.textContent();
  await header.getByRole('button', {name: 'Next schedule window'}).click();
  await page.waitForFunction(([node, before]) => node.textContent !== before,
    [await header.elementHandle(), oldHeading]);
  await header.getByRole('button', {name: 'Previous schedule window'}).click();
  await page.waitForFunction(([node, before]) => node.textContent === before,
    [await header.elementHandle(), oldHeading]);
}

(async () => {
  const directory = outputPath('example-review/timeline-label-actions');
  fs.mkdirSync(directory, {recursive: true});
  const browser = await chromium.launch();
  const results = [], errors = [];
  try {
    const page = await browser.newPage({viewport: {width: 1336, height: 980}, reducedMotion: 'reduce'});
    page.setDefaultTimeout(5000);
    page.on('pageerror', error => errors.push(String(error)));
    for (const [slug, options] of Object.entries(variants)) {
      await page.goto(base + '/showcase.html?widget=' + slug);
      await page.waitForFunction(() => window.ready, undefined, {timeout: 45000});
      for (const variant of options) {
        const id = 'reference-' + slug + '-' + variant;
        const preview = page.locator('.site-component-detail').filter({has: page.locator('#' + id)}).locator('.site-demo');
        const row = {slug, id};
        try {
          await preview.screenshot({path:path.join(directory,id+'-before.png')});
          await verifyTimeline(page, preview, slug, variant);
          await preview.screenshot({path:path.join(directory,id+'-after.png')});
          row.passed = true;
        } catch (error) { row.error = String(error); }
        results.push(row);
        console.log(id, row.passed || row.error);
      }
    }
  } finally {
    await browser.close();
    fs.writeFileSync(path.join(directory,'checks.json'), JSON.stringify({base, results, errors},null,2));
  }
  assert(results.every(row => row.passed));
  assert.equal(errors.length,0);
})().catch(error => {console.error(error);process.exitCode=1});
