const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const { base, chromium, outputPath } = require('./showcase-browser-env.cjs');

const reference = process.env.SAPID_MANTINE_BASE || 'https://mantine.dev';
const destination = outputPath('example-review/responsive-native-select');
const widths = [1440, 1336, 1280, 1024, 390];

async function sample(page, width, local) {
  const site = local ? 'local' : 'mantine';
  await page.setViewportSize({ width, height: 960 });
  await page.goto(local ? base + '/showcase.html?widget=native-select' : reference + '/core/native-select/', {waitUntil: 'domcontentloaded'});
  if (local) await page.waitForFunction(() => window.ready === true);
  const select = page.locator(local ? '.site-preview select' : '#mdx .mantine-NativeSelect-input').first();
  await select.scrollIntoViewIfNeeded();
  const result = await select.evaluate((input, local) => {
    const form = local ? input.closest('.site-configurator') : input.closest('.m_a3c6e060').parentElement;
    const box = input.getBoundingClientRect();
    const parent = form.getBoundingClientRect();
    let glyph;
    if (local) {
      const style = getComputedStyle(input);
      const pixels = style.backgroundSize.match(/([\d.]+)px\s+([\d.]+)px/);
      const inset = style.backgroundPosition.match(/^right\s+([\d.]+)px\s+50%$/);
      if (!pixels || !inset || !style.backgroundImage.includes('svg')) throw Error('Expected centered NativeSelect SVG');
      // CSS positions this image inside the select border.
      const border = style.backgroundOrigin === 'border-box' ? 0 : parseFloat(style.borderRightWidth);
      glyph = { width: +pixels[1], height: +pixels[2], right: +inset[1] + border, centerY: 0 };
    } else {
      const arrow = input.parentElement.querySelector('.mantine-ComboboxChevron-chevron').getBoundingClientRect();
      glyph = { width: arrow.width, height: arrow.height, right: box.right - arrow.right,
        centerY: arrow.y + arrow.height / 2 - box.y - box.height / 2 };
    }
    const panel = local ? form.querySelector('.site-playground-controls') : [...form.children].find(el => !el.contains(input));
    const fields = panel ? [...panel.querySelectorAll('input:not([type=hidden])')].filter(el => el.getBoundingClientRect().width > 0).map(el => {
      const r = el.getBoundingClientRect();
      return { type: el.type, label: el.getAttribute('aria-label') || el.labels?.[0]?.textContent.trim() || '',
        top: Math.round(r.top - parent.top), height: Math.round(r.height), radius: getComputedStyle(el).borderRadius };
    }) : [];
    return { frameX: parent.x, frameWidth: parent.width, frameHeight: parent.height, fields, inputX: box.x, width: box.width,
      height: box.height, inputY: box.top - parent.top, value: input.selectedOptions[0].textContent, glyph };
  }, local);
  const form = local ? page.locator('.site-configurator').first() : page.locator('#mdx .m_a3c6e060').first().locator('..');
  await form.screenshot({ path: path.join(destination, site + '-' + width + '.png'), animations: 'disabled' });
  const tabs = page.locator(local ? '.site-doc-tabs' : '.mantine-Tabs-list').first();
  const docs = tabs.getByRole(local ? 'button' : 'tab', {name:'Documentation', exact:true});
  result.docsTab = await docs.evaluate((button, local) => {
    const style = getComputedStyle(button);
    const graphic = local ? getComputedStyle(button,'::before') : getComputedStyle(button.querySelector('svg'));
    return {
      height: button.getBoundingClientRect().height,
      icon: graphic.display === 'none' ? 0 : parseFloat(graphic.width),
      inset: parseFloat(style.paddingLeft) + parseFloat(style.borderLeftWidth),
      font: style.fontSize,
      radius: style.borderRadius,
      hasGraphic: !local || graphic.maskImage !== 'none',
    };
  }, local);
  await tabs.screenshot({ path: path.join(destination, site + '-tabs-' + width + '.png'), animations:'disabled' });
  return result;
}

(async () => {
  fs.mkdirSync(destination, {recursive:true});
  const browser = await chromium.launch();
  const results = [];
  try {
    const page = await browser.newPage({ reducedMotion: 'reduce' });
    for (const viewport of widths) {
      const official = await sample(page, viewport, false);
      const current = await sample(page, viewport, true);
      const deviations = [];
      for (const key of ['frameX','frameWidth','frameHeight','inputX','inputY','width','height']) {
        if (Math.abs(official[key] - current[key]) > 1) deviations.push(key);
      }
      for (const key of ['width','height','right','centerY']) {
        if (Math.abs(official.glyph[key] - current.glyph[key]) > 1) deviations.push('glyph.' + key);
      }
      for (const field of official.fields.filter(f => f.type === 'text')) {
        const rendered = current.fields.find(f => f.type === 'text' && f.label === field.label);
        if (!rendered || rendered.height !== field.height || Math.abs(rendered.top - field.top) > 2 || rendered.radius !== field.radius) deviations.push(field.label);
      }
      if (official.value !== current.value) deviations.push('value');
      for (const property of Object.keys(official.docsTab)) {
        if (official.docsTab[property] !== current.docsTab[property]) deviations.push('docsTab.' + property);
      }
      results.push({viewport, official, current, deviations});
    }
    fs.writeFileSync(path.join(destination, 'measurements.json'), JSON.stringify({base, reference, results}, null, 2));
    const sections = results.map(({viewport, official, current, deviations}) => {
      const figure = (site, title, sample) => {
        const file = site + '-' + viewport + '.png';
        const {width, height, right, centerY} = sample.glyph;
        const tabs = site + '-tabs-' + viewport + '.png';
        return `<figure><figcaption>${title} · 箭头 ${width} × ${height}px，右边距 ${right}px，垂直偏移 ${centerY}px · <a href="${tabs}">文档页签</a></figcaption><a href="${file}"><img src="${file}" alt="${title} NativeSelect"></a></figure>`;
      };
      return `<section><h2>${viewport}px</h2><div>${figure('mantine', 'Mantine', official)}${figure('local', 'Sapid', current)}</div>${deviations.length ? '<p>超出容差的项目：' + deviations.join('、') + '</p>' : ''}</section>`;
    });
    fs.writeFileSync(path.join(destination, 'comparison.html'), `<!doctype html><html lang="zh-CN"><meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1"><title>NativeSelect 逐宽度对照</title>
<style>body{font:15px/1.5 system-ui;margin:24px}section div{display:grid;grid-template-columns:repeat(2,minmax(0,1fr));gap:16px}figure{margin:0}img{width:100%;height:auto}figcaption{margin-bottom:6px}@media(max-width:700px){section div{grid-template-columns:1fr}}</style>
<h1>NativeSelect 逐宽度对照</h1><a href="measurements.json">查看输入框、箭头及配置栏测量</a>${sections.join('')}</html>`);
    for (const row of results) assert.deepEqual(row.deviations, [], String(row.viewport));
    console.log('NativeSelect preview, field and chevron matched at ' + widths.join(', '));
  } finally {
    await browser.close();
  }
})().catch(error => { console.error(error); process.exitCode = 1; });
