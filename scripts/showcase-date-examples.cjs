const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const {base, chromium, outputPath} = require('./showcase-browser-env.cjs');

const examples = {
  'time-picker': ['with-seconds','12-hour-format','change-am-pm-labels','with-dropdown','control-dropdown-opened-state','hours-minutes-seconds-step','clearable','disabled','read-only'],
  'time-grid': ['min-and-max-time','disable-specific-controls','disabled'],
  'mini-calendar': ['number-of-days','getdayprops','min-and-max-dates'],
  'time-input': ['with-seconds','disabled-state'],
  'date-input': ['disabled-state','date-parser','min-and-max-date'],
  'time-value': ['with-seconds'],
};
const tick = page => page.evaluate(() => new Promise(resolve => requestAnimationFrame(() => requestAnimationFrame(resolve))));

async function check(page, demo, slug, name) {
  const inputs = demo.locator('input:visible');
  const buttons = demo.getByRole('button');
  if (name.startsWith('disabled')) {
    const controls = slug === 'time-grid' ? buttons : inputs;
    assert(await controls.count());
    for (const control of await controls.all()) assert(await control.isDisabled());
    return;
  }
  if (slug === 'time-picker') {
    if (name === 'read-only') {
      assert(await inputs.first().evaluate(e => e.readOnly || e.getAttribute('aria-readonly') === 'true'));
    } else if (name === 'clearable') {
      assert(await inputs.first().inputValue());
      const clear = demo.getByRole('button', {name:'Clear time', exact:true});
      const buttonBox = await clear.boundingBox();
      const iconBox = await clear.locator('svg').boundingBox();
      assert(buttonBox && iconBox);
      assert.equal(buttonBox.width, 28);
      assert.equal(buttonBox.height, 28);
      for (const [axis, dimension] of [['x', 'width'], ['y', 'height']]) {
        assert(Math.abs(iconBox[dimension] - 19.6) < 0.1);
        const center = box => box[axis] + box[dimension] / 2;
        assert(Math.abs(center(buttonBox) - center(iconBox)) < 0.5);
      }
      await clear.click();
      await tick(page);
      assert.equal(await inputs.first().inputValue(), '');
    } else if (name === 'with-dropdown' || name === 'control-dropdown-opened-state') {
      const trigger = demo.getByRole('button',{name:'Choose time',exact:true});
      if (name === 'control-dropdown-opened-state') {
        const box = await trigger.boundingBox();
        const graphic = trigger.locator('svg');
        const glyphBox = await graphic.boundingBox();
        assert(box && glyphBox);
        assert.deepEqual([box.width, box.height, glyphBox.width, glyphBox.height], [28, 28, 18, 18]);
        assert.match(await graphic.locator('path').first().getAttribute('d'), /C/i);
      }
      await trigger.click();
      await tick(page);
      assert(await demo.locator('[aria-expanded=true]').count());
      await page.keyboard.press('Escape');
    } else if (name.includes('hour-format') || name === 'change-am-pm-labels') {
      const period = demo.getByRole('button',{name:'AM/PM'});
      const before = await period.textContent();
      if (name === 'change-am-pm-labels') assert.match(before,/पूर्वाह्न|अपराह्न/);
      await period.click();
      await tick(page);
      assert.notEqual(await period.textContent(),before);
    } else {
      const minute = demo.getByRole('spinbutton',{name:'Minute',exact:true});
      await minute.fill('10'); await minute.press('ArrowUp'); await tick(page);
      assert.equal(Number(await minute.inputValue()), name.endsWith('step') ? 15 : 11);
      assert(await demo.getByRole('spinbutton',{name:'Second',exact:true}).isVisible());
    }
  } else if (slug === 'time-input') {
    await inputs.first().fill('14:18:37'); await inputs.first().press('Tab');
    assert.equal(await inputs.first().inputValue(),'14:18:37');
  } else if (slug === 'time-grid') {
    const unavailable = name === 'min-and-max-time' ? ['09:00','19:00'] : ['10:45','11:00','11:30'];
    for (const time of unavailable) assert(await demo.getByRole('button',{name:time,exact:true}).isDisabled());
    const slot = demo.getByRole('button',{name:name === 'min-and-max-time' ? '12:00':'09:15',exact:true});
    await slot.click(); await tick(page);
    assert(await slot.evaluate(el => el.getAttribute('aria-pressed')==='true' || el.closest('[aria-selected=true]') !== null));
  } else if (slug === 'mini-calendar') {
    if (name === 'number-of-days') assert.equal(await buttons.count(), 7);
    if (name === 'getdayprops') assert.equal(await demo.getByRole('button',{name:/Saturday,/}).evaluate(e=>getComputedStyle(e).color),'rgb(224, 49, 49)');
    await demo.getByRole('button',{name:/Wednesday,/}).first().click(); await tick(page);
    assert(await demo.locator('[aria-pressed=true],[aria-selected=true]').count());
    if (name === 'min-and-max-dates') {
      await demo.getByRole('button',{name:'Next range'}).click(); await tick(page);
      assert(await demo.getByRole('button',{name:/April 2[5-9]/}).first().isDisabled() || await demo.getByRole('button',{name:'Next range'}).isDisabled());
    }
  } else if (slug === 'date-input') {
    const field = inputs.first();
    await field.fill(name==='date-parser'?'WW2':'2025-04-18'); await field.press('Enter'); await tick(page);
    assert.equal(await field.inputValue(), name==='date-parser'?'1939-09-01':'2025-04-18');
    if (name==='min-and-max-date') {
      await field.fill('2027-04-18'); await field.press('Enter'); await tick(page);
      assert(await field.getAttribute('aria-invalid')==='true' || (await field.inputValue())==='2025-04-18');
    }
  } else {
    assert.deepEqual(await demo.locator('.sapid-time-value').allTextContents(),['18:45:34','06:45:34 PM']);
  }
}

(async () => {
  const root = outputPath('example-review/date-direct'); fs.mkdirSync(root, {recursive:true});
  const browser = await chromium.launch(), results=[];
  const page = await browser.newPage({viewport:{width:1280,height:960},reducedMotion:'reduce'});
  page.setDefaultTimeout(4500);
  try {
    for (const [slug,names] of Object.entries(examples)) {
      await page.goto(base+'/showcase.html?widget='+slug, {waitUntil:'domcontentloaded'});
      await page.waitForFunction(()=>window.ready);
      for (const name of names) {
        const id = 'reference-'+slug+'-'+name;
        const demo = page.locator('.site-component-detail').filter({has:page.locator('#'+id)}).locator('.site-demo').first();
        const record={slug,id};
        try {
          await demo.screenshot({path:path.join(root,id+'-before.png')});
          await check(page,demo,slug,name);
          await demo.screenshot({path:path.join(root,id+'-after.png')}); record.passed=true;
        } catch(error) { record.error=String(error); }
        results.push(record);console.log(id,record.passed||record.error.split('\n')[0]);
      }
    }
  } finally {await browser.close();fs.writeFileSync(path.join(root,'checks.json'),JSON.stringify({base,results},null,2));}
  assert(results.every(r=>r.passed));
})().catch(e=>{console.error(e);process.exitCode=1;});
