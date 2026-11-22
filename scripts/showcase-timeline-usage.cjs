const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const { base, chromium, outputPath } = require('./showcase-browser-env.cjs');

const events = [
  { name: 'branch', title: /branch/i, time: '2 hours ago' },
  { name: 'commit', title: /commit/i, time: '52 minutes ago' },
  { name: 'pull request', title: /pull request/i, time: '34 minutes ago' },
  { name: 'review comment', title: /review/i, time: '12 minutes ago' },
];

function near(actual, expected, message, tolerance = 1) {
  assert(Math.abs(actual - expected) <= tolerance, `${message}: expected ${expected} ± ${tolerance}, got ${actual}`);
}

async function measureUsage(timeline) {
  return timeline.evaluate(root => {
    const origin = root.getBoundingClientRect();
    function box(node) {
      const r = node.getBoundingClientRect();
      return { x: r.left - origin.left, y: r.top - origin.top, w: r.width, h: r.height };
    }
    return Array.from(root.querySelectorAll('.sapid-timeline-item'), row => {
      const marker = row.querySelector('.sapid-timeline-marker');
      const icon = marker.querySelector('svg');
      const line = row.querySelector('.sapid-timeline-line');
      const part = name => {
        const el = row.querySelector('.sapid-timeline-' + name);
        return { ...box(el), text: el.textContent.trim(), lineHeight: parseFloat(getComputedStyle(el).lineHeight) };
      };
      return {
        marker: box(marker), radius: parseFloat(getComputedStyle(marker).borderTopLeftRadius),
        icon: icon ? box(icon) : null, iconCount: marker.querySelectorAll('svg').length,
        title: part('title'), description: part('description'), time: part('timestamp'),
        line: line ? { ...box(line), style: getComputedStyle(line).borderLeftStyle } : null,
      };
    });
  });
}

function checkLayout(rows) {
  assert.equal(rows.length, 4, 'Usage must render four events');
  for (const [i, row] of rows.entries()) {
    const { title, description, time, marker, icon, line } = row;
    const context = events[i].name;
    assert.match(title.text, events[i].title);
    assert.equal(time.text, events[i].time);
    assert(description.text.length > 0, context + ' missing description');
    near(marker.w, 24, context + ' marker width');
    near(marker.h, 24, context + ' marker height');
    assert(row.radius >= marker.w / 2, context + ' marker must be round');
    assert.equal(row.iconCount, 1, context + ' should have one painted SVG');
    near(icon.w, 13, context + ' icon width', .5);
    near(icon.h, 13, context + ' icon height', .5);
    near(icon.x + icon.w / 2, marker.x + marker.w / 2, context + ' horizontal icon centre', .5);
    near(icon.y + icon.h / 2, marker.y + marker.h / 2, context + ' vertical icon centre', .5);
    near(title.x - marker.x - marker.w / 2, 27, context + ' marker-to-title offset');
    near(title.y, marker.y, context + ' title top');
    near(title.h, 16, context + ' title height');
    near(description.x, title.x, context + ' description start');
    near(description.y - title.y - title.h, 5, context + ' title/description gap');
    near(description.lineHeight, 20.3, context + ' description line height');
    near(time.x, title.x, context + ' time start');
    near(time.y - description.y - description.h, 4, context + ' description/time gap');
    near(time.h, 16.8, context + ' timestamp height');
    if (i === rows.length - 1) {
      assert.equal(line, null, 'The last marker should have no outgoing line');
      continue;
    }
    assert(line, context + ' missing outgoing line');
    const next = rows[i + 1].marker;
    near(line.w, 2, context + ' connector thickness', .5);
    near(line.x + line.w / 2, marker.x + marker.w / 2, context + ' connector horizontal centre', 1);
    near(line.y, marker.y + marker.h, context + ' connector start', 1);
    near(line.y + line.h, next.y, context + ' connector end', 1);
    near(next.y - time.y - time.h, 32, context + ' spacing to the next event');
    assert.equal(line.style === 'dashed', i === 2, context + ' connector dash');
  }
}

async function readIconPixels(page, icon) {
  const png = await icon.screenshot({ scale: 'device', animations: 'disabled' });
  return page.evaluate(async base64 => {
    const image = new Image();
    image.src = 'data:image/png;base64,' + base64;
    await image.decode();
    const canvas = document.createElement('canvas');
    canvas.width = image.width;
    canvas.height = image.height;
    const ctx = canvas.getContext('2d');
    ctx.drawImage(image, 0, 0);
    const { data } = ctx.getImageData(0, 0, image.width, image.height);
    const colors = new Map();
    const pixels = [];
    for (let i = 0; i < data.length; i += 4) {
      const rgb = data[i] * 65536 + data[i + 1] * 256 + data[i + 2];
      pixels.push(rgb);
      colors.set(rgb, (colors.get(rgb) || 0) + 1);
    }
    const background = [...colors.keys()].sort((a, b) => colors.get(b) - colors.get(a))[0];
    function lum(rgb) {
      const [red, green, blue] = [rgb >> 16, (rgb >> 8) & 255, rgb & 255].map(n => {
        const s = n / 255;
        return s <= .04045 ? s / 12.92 : ((s + .055) / 1.055) ** 2.4;
      });
      return .2126 * red + .7152 * green + .0722 * blue;
    }
    const bg = lum(background);
    const contrast = pixels.map(rgb => {
      const l = lum(rgb);
      return (Math.max(l, bg) + .05) / (Math.min(l, bg) + .05);
    });
    const sorted = [...contrast].sort((a, b) => a - b);
    const foregroundContrast = sorted[Math.floor(sorted.length * .98)];
    let left = image.width, right = -1, top = image.height, bottom = -1, ink = 0;
    for (let i = 0; i < contrast.length; i++) {
      // Include antialiased edges when locating the visible drawing.
      if (contrast[i] < 1.45) continue;
      ink++;
      const x = i % image.width, y = Math.floor(i / image.width);
      left = Math.min(left, x);
      right = Math.max(right, x);
      top = Math.min(top, y);
      bottom = Math.max(bottom, y);
    }
    return {
      background, foregroundContrast, inkFraction: ink / pixels.length,
      horizontalExtent: (right - left + 1) / image.width,
      verticalExtent: (bottom - top + 1) / image.height,
      centreX: (left + right + 1) / (2 * image.width),
      centreY: (top + bottom + 1) / (2 * image.height),
    };
  }, png.toString('base64'));
}

async function checkIcons(page, usage) {
  const results = [];
  const geometries = new Set();
  for (const [i, event] of events.entries()) {
    const icon = usage.locator('.sapid-timeline-marker svg').nth(i);
    // Ignore paint colours and generated DOM ids when checking for repeated icons.
    const geometry = await icon.evaluate(svg => {
      const attributes = ['d', 'points', 'x', 'y', 'x1', 'y1', 'x2', 'y2',
        'r', 'rx', 'ry', 'cx', 'cy', 'width', 'height', 'transform', 'href'];
      const fromScreen = svg.getScreenCTM().inverse();
      return Array.from(svg.querySelectorAll('path,polygon,polyline,line,circle,rect,ellipse,use'))
        .filter(shape => shape.checkVisibility({ checkOpacity: true, checkVisibilityCSS: true }))
        .map(shape => {
          const m = fromScreen.multiply(shape.getScreenCTM());
          const style = getComputedStyle(shape);
          return [shape.localName, ...attributes.map(name => shape.getAttribute(name)),
            ...[m.a, m.b, m.c, m.d, m.e, m.f].map(n => n.toFixed(5)),
            style.fill !== 'none', style.stroke !== 'none', style.strokeWidth];
        });
    });
    assert(geometry.length, event.name + ' has no vector drawing');
    const signature = JSON.stringify(geometry);
    assert(!geometries.has(signature), event.name + ' repeats a previous event icon');
    geometries.add(signature);
    const pixels = await readIconPixels(page, icon);
    assert(pixels.foregroundContrast >= 3, event.name + ' painted icon contrast: ' + pixels.foregroundContrast);
    assert(pixels.inkFraction >= .04 && pixels.inkFraction <= .85, event.name + ' should have visible strokes and clear space');
    assert(pixels.horizontalExtent >= .25 && pixels.verticalExtent >= .25, event.name + ' drawing is too small to read');
    near(pixels.centreX, .5, event.name + ' visible ink horizontal centre', .18);
    near(pixels.centreY, .5, event.name + ' visible ink vertical centre', .18);
    results.push({ name: event.name, ...pixels });
  }
  return results;
}

async function changeProperty(props, control, key, selector, cssProperty, expected) {
  const slider = props.getByRole('slider', { name: control, exact: true });
  await slider.focus();
  await slider.press(key);
  await props.page().waitForFunction(({ root, selector, cssProperty, expected }) => {
    return Math.abs(parseFloat(getComputedStyle(root.querySelector(selector))[cssProperty]) - expected) < .1;
  }, { root: await props.elementHandle(), selector, cssProperty, expected });
}

async function verify(browser, width, directory) {
  const page = await browser.newPage({ viewport: { width, height: 980 }, deviceScaleFactor: 4, reducedMotion: 'reduce' });
  const errors = [];
  page.on('pageerror', e => errors.push(e.message));
  try {
    await page.goto(new URL('showcase.html?widget=timeline', base + '/').href);
    await page.waitForFunction(() => window.ready === true);
    await page.evaluate(() => document.fonts.ready);
    const usage = page.locator('.site-doc-content .site-preview .site-demo').first().locator('.sapid-timeline');
    const props = page.locator('.site-component-detail').filter({ has: page.locator('#timeline-props-play') });
    const before = await measureUsage(usage);
    await usage.screenshot({ path: path.join(directory, 'usage-' + width + '.png'), scale: 'css' });
    checkLayout(before);
    const icons = await checkIcons(page, usage);
    const preview = props.locator('.site-playground-preview');
    assert.equal(await preview.locator('.sapid-timeline-marker svg').count(), 0, 'Props configurator has plain markers');
    await changeProperty(props, 'Bullet size', 'End', '.sapid-timeline-marker', 'width', 40);
    await changeProperty(props, 'Line width', 'End', '.sapid-timeline-line', 'width', 6);
    await changeProperty(props, 'Radius', 'Home', '.sapid-timeline-marker', 'borderTopLeftRadius', 2);
    await preview.screenshot({ path: path.join(directory, 'props-' + width + '.png'), scale: 'css' });
    assert.deepEqual(await measureUsage(usage), before, 'Editing props must not move or restyle Usage');
    const afterIcons = await checkIcons(page, usage);
    for (const [i, icon] of icons.entries()) {
      assert.equal(afterIcons[i].background, icon.background, icon.name + ' marker colour changed with props');
      near(afterIcons[i].foregroundContrast, icon.foregroundContrast, icon.name + ' icon contrast changed with props', .25);
      near(afterIcons[i].inkFraction, icon.inkFraction, icon.name + ' icon paint area changed with props', .035);
    }
    assert.deepEqual(errors, [], 'Page errors');
    if (process.env.SAPID_MANTINE_BASE) {
      await page.goto(new URL('core/timeline/', process.env.SAPID_MANTINE_BASE + '/').href);
      const reference = page.locator('#mdx .mantine-Timeline-root').first();
      await reference.screenshot({ path: path.join(directory, 'mantine-' + width + '.png'), scale: 'css' });
    }
    return { width, rows: before, icons };
  } finally {
    await page.close();
  }
}

(async () => {
  const directory = outputPath('example-review/timeline-usage-regression');
  fs.mkdirSync(directory, { recursive: true });
  const browser = await chromium.launch();
  try {
    const results = [];
    for (const width of [1336, 390]) results.push(await verify(browser, width, directory));
    fs.writeFileSync(path.join(directory, 'measurements.json'), JSON.stringify({ base, results }, null, 2));
    console.log('Timeline Usage and props passed at 1336px and 390px.');
  } finally {
    await browser.close();
  }
})().catch(error => { console.error(error); process.exitCode = 1; });
