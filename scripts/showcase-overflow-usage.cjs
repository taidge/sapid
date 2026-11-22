const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const { base, chromium, outputPath } = require('./showcase-browser-env.cjs');

const fruits = [
  'APPLE', 'BANANA', 'CHERRY', 'DATE', 'ELDERBERRY', 'FIG', 'GRAPE', 'HONEYDEW',
  'INDIAN FIG', 'JACKFRUIT', 'KIWI', 'LEMON', 'MANGO', 'NECTARINE', 'ORANGE', 'PAPAYA',
];

async function settle(page) {
  await page.evaluate(() => new Promise(resolve => {
    let frames = 0;
    function tick() { if (++frames === 6) resolve(); else requestAnimationFrame(tick); }
    requestAnimationFrame(tick);
  }));
}

async function read(resizer) {
  return resizer.evaluate(host => {
    const line = host.querySelector('.sapid-overflow-line');
    const strip = line.querySelector('.sapid-overflow-strip');
    const label = el => el.innerText.trim().replace(/\s+/g, ' ').toUpperCase();
    const box = el => {
      const r = el.getBoundingClientRect();
      return { left: r.left, right: r.right, centerY: (r.top + r.bottom) / 2, width: r.width };
    };
    const painted = el => {
      const rect = el.getBoundingClientRect();
      return rect.width > 0 && rect.height > 0 && el.checkVisibility({ checkOpacity: true, checkVisibilityCSS: true });
    };
    const items = [...strip.querySelectorAll(':scope > .sapid-overflow-item')].filter(painted).map(el => {
      const badge = el.querySelector('.sapid-badge') || el;
      return { label: label(badge), ...box(badge) };
    });
    const moreBox = line.querySelector('.sapid-overflow-more');
    const more = moreBox && painted(moreBox) ? moreBox.querySelector('.sapid-badge,button') || moreBox : null;
    return {
      width: host.getBoundingClientRect().width, resize: getComputedStyle(host).resize,
      line: box(line), strip: box(strip), gap: parseFloat(getComputedStyle(line).columnGap),
      scrollOverflow: Math.max(0, host.scrollWidth - host.clientWidth, line.scrollWidth - line.clientWidth),
      items, more: more && painted(more) ? { label: label(more), ...box(more) } : null,
    };
  });
}

function assertLayout(state, widths) {
  const { items, more, line, strip, gap } = state;
  assert(['horizontal', 'both', 'inline'].includes(state.resize), 'Usage must have a working horizontal resize handle');
  assert.equal(gap, 4, 'The item and overflow badges use a 4px gap');
  assert.deepEqual(items.map(item => item.label), fruits.slice(0, items.length), 'Visible items must be an ordered fruit prefix');
  if (items.length === fruits.length) assert.equal(more, null, 'Hide the overflow badge when every item fits');
  else {
    assert(more, 'Missing overflow count');
    assert.equal(more.label, '+' + (fruits.length - items.length) + ' MORE');
  }

  const visible = more ? [...items, more] : items;
  assert(visible.length > 0, 'At least the count badge should be painted');
  for (const [index, badge] of visible.entries()) {
    assert(Math.abs(badge.centerY - visible[0].centerY) <= 1, badge.label + ' wraps to another row');
    assert(badge.left >= line.left - 1 && badge.right <= line.right + 1, badge.label + ' extends outside the list');
    if (index < items.length) assert(badge.right <= strip.right + 1, badge.label + ' is clipped in the item strip');
    if (index > 0) assert(Math.abs(badge.left - visible[index - 1].right - gap) <= 1, badge.label + ' gap/overlap');
  }
  assert(state.scrollOverflow <= 1, 'OverflowList has a horizontal scrollbar');
  if (widths) {
    // Independent oracle: widths are measured with all 16 real badges visible in
    // the wide containing block. Reserve the count badge only when it is needed.
    const available = line.width + .5;
    const allWidth = widths.reduce((sum, width) => sum + width, gap * (widths.length - 1));
    if (allWidth <= available) assert.equal(items.length, widths.length, 'All fruit fits without a count badge');
    else {
      assert(more, 'A full row wider than the container needs an overflow badge');
      let expected = 0, used = more.width;
      while (expected < widths.length && used + gap + widths[expected] <= available) {
        used += gap + widths[expected++];
      }
      assert.equal(items.length, expected, 'Visible fruit prefix does not fill the available width');
    }
  }
}

async function dragHandle(page, resizer, requested) {
  await resizer.scrollIntoViewIfNeeded();
  const box = await resizer.boundingBox();
  const right = box.x + box.width - 3;
  const bottom = box.y + box.height - 3;
  await page.mouse.move(right, bottom);
  await page.mouse.down();
  try {
    await page.mouse.move(right + requested - box.width, bottom, { steps: 12 });
  } finally {
    await page.mouse.up();
  }
  await settle(page);
  const moved = await resizer.boundingBox();
  assert(Math.abs(moved.width - requested) <= 2, `Native drag to ${requested}px produced ${moved.width}px`);
}

async function dragTo(page, resizer, requested) {
  await dragHandle(page, resizer, requested);
  return read(resizer);
}

async function runWidth(browser, viewportWidth, directory, measuredWidths) {
  const wideFixture = !measuredWidths;
  const name = wideFixture ? 'wide-host' : String(viewportWidth);
  const page = await browser.newPage({ viewport: { width: viewportWidth, height: 980 }, reducedMotion: 'reduce' });
  const errors = [];
  page.on('pageerror', error => errors.push(error.message));
  try {
    await page.goto(new URL('showcase.html?widget=overflow-list', base + '/').href);
    await page.waitForFunction(() => window.ready === true);
    await page.evaluate(() => document.fonts.ready);
    const resizer = page.locator('.site-doc-content .site-preview .site-demo').first().locator('.site-overflow-resizer');
    await resizer.waitFor({ state: 'visible' });
    if (wideFixture) {
      // The normal documentation column is intentionally capped. Give the same
      // mounted widget a wide parent to exercise the zero-overflow boundary.
      await resizer.evaluate(el => {
        Object.assign(el.parentElement.style, {
          position: 'fixed', left: '16px', top: '80px', width: '1550px',
          maxWidth: 'none', boxSizing: 'border-box', zIndex: '1000',
          background: getComputedStyle(document.body).backgroundColor,
        });
      });
      await settle(page);
      await dragTo(page, resizer, 1450);
    }
    await resizer.scrollIntoViewIfNeeded();
    await settle(page);
    const first = await read(resizer);
    if (wideFixture) {
      assert.deepEqual(first.items.map(item => item.label), fruits, 'Wide host should expose all sixteen real badges');
      measuredWidths = first.items.map(item => item.width);
    }
    const records = [];
    async function record(step, state) {
      await resizer.screenshot({ path: path.join(directory, name + '-' + step + '.png') });
      assertLayout(state, measuredWidths);
      records.push({ step, ...state });
    }
    await record('initial', first);
    const medium = Math.min(260, Math.floor(first.width * .75));
    const narrow = Math.min(155, Math.floor(medium * .65));
    const midState = await dragTo(page, resizer, medium);
    await record('medium', midState);
    const smallState = await dragTo(page, resizer, narrow);
    await record('narrow', smallState);
    assert(first.items.length > midState.items.length && midState.items.length > smallState.items.length,
      'Shrinking the real handle twice must remove fruit at both widths');
    const last = await dragTo(page, resizer, first.width);
    await record('restored', last);
    assert.deepEqual(last.items.map(item => item.label), first.items.map(item => item.label));
    assert.equal(last.more?.label, first.more?.label, 'Dragging back should restore the same count');
    assert.deepEqual(errors, []);
    const asset = await page.locator('link[href$="site.css"]').getAttribute('href');
    return { name, asset, viewportWidth, measuredWidths, records };
  } finally {
    await page.close();
  }
}

async function captureReference(browser, result, directory) {
  const page = await browser.newPage({ viewport: { width: result.viewportWidth, height: 980 } });
  try {
    await page.goto(new URL('core/overflow-list/', process.env.SAPID_MANTINE_BASE + '/').href);
    await page.evaluate(() => document.fonts.ready);
    const host = page.locator('#mdx .m_a3c6e060').first().locator('[style*="resize"]').first();
    const original = await host.boundingBox();
    const states = [];
    for (const record of result.records) {
      if (record.step !== 'initial') {
        await dragHandle(page, host, record.step === 'restored' ? original.width : record.width);
      }
      await settle(page);
      const labels = await host.locator('.mantine-OverflowList-root > .mantine-Badge-root')
        .evaluateAll(nodes => nodes.map(node => node.innerText.trim().toUpperCase()));
      const width = (await host.boundingBox()).width;
      await host.screenshot({ path: path.join(directory, 'mantine-' + result.name + '-' + record.step + '.png') });
      if (Math.abs(width - record.width) <= 1) {
        const expected = record.items.map(item => item.label);
        if (record.more) expected.push(record.more.label);
        assert.deepEqual(labels, expected, `Mantine and Sapid visible text at ${width}px`);
      }
      states.push({ step: record.step, width, labels });
    }
    return states;
  } finally {
    await page.close();
  }
}

(async () => {
  const directory = outputPath('example-review/overflow-usage-regression');
  fs.mkdirSync(directory, { recursive: true });
  const browser = await chromium.launch();
  try {
    const wide = await runWidth(browser, 1680, directory);
    const results = [wide];
    for (const width of [1336, 390]) results.push(await runWidth(browser, width, directory, wide.measuredWidths));
    if (process.env.SAPID_MANTINE_BASE) {
      for (const result of results.slice(1)) result.reference = await captureReference(browser, result, directory);
    }
    fs.writeFileSync(path.join(directory, 'measurements.json'), JSON.stringify({ base, results }, null, 2));
    console.log('OverflowList drag, count, restoration and full fit passed at 1336px, 390px and in a wide host.');
  } finally {
    await browser.close();
  }
})().catch(error => { console.error(error); process.exitCode = 1; });
