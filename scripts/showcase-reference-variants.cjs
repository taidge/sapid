const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const {base, chromium, outputPath} = require('./showcase-browser-env.cjs');

const groups = {
  'agenda-view': ['all-day-and-multi-day-events','date-formats','custom-event-rendering','event-click'],
  'time-picker': ['min-and-max-values','control-dropdown-opened-state','time-presets','time-presets-groups',
    'time-presets-range','close-dropdown-on-preset-select','dropdown-position','dropdown-width','paste-events'],
};
const settle = page => page.evaluate(() => new Promise(done => requestAnimationFrame(() => requestAnimationFrame(done))));

async function checkAgenda(page, demo, name) {
  const rows = demo.locator('.sapid-agenda-event button');
  if (name === 'all-day-and-multi-day-events') {
    assert.equal(await rows.count(), 5);
    assert.match(await rows.filter({hasText:'Company Holiday'}).textContent(), /All day/);
    assert.match(await rows.filter({hasText:'Sales Conference'}).textContent(), /2026-11-24.*2026-11-26/);
    await rows.filter({hasText:'Sales Conference'}).click();
  } else if (name === 'date-formats') {
    assert.match(await demo.locator('.sapid-schedule-window').textContent(), /Nov 22.*Nov 28/);
    assert.deepEqual(await demo.locator('.sapid-agenda-heading').allTextContents(), ['Sun, 22 Nov','Mon, 23 Nov','Tue, 24 Nov']);
    await demo.getByRole('button',{name:'Next schedule window'}).click();
    await demo.locator('.sapid-schedule-window').filter({hasText:/Nov 29.*Dec 5/}).waitFor();
  } else if (name === 'custom-event-rendering') {
    assert.deepEqual(await demo.locator('.site-agenda-custom-category').allTextContents(), ['meeting','workshop','dev','event']);
    const row = rows.filter({hasText:'Room A'});
    assert.match(await row.textContent(), /09:00.*09:30/);
    await row.click();
  } else {
    await rows.filter({hasText:'Design Workshop'}).click();
    const dialog = page.getByRole('dialog',{name:'Design Workshop'});
    await dialog.waitFor();
    assert.match(await dialog.textContent(), /Collaborative design session/);
    assert.match(await dialog.textContent(), /Creative Space/);
    return dialog;
  }
  await settle(page);
  return demo;
}

async function checkTime(page, demo, name) {
  const hour = demo.locator('.sapid-time-hour');
  if (name==='min-and-max-values') {
    assert.equal(await hour.count(),2);
    await hour.first().fill('13'); await hour.first().press('Tab');
    assert.equal(await hour.first().inputValue(),'13'); return;
  }
  if (name==='paste-events') {
    await hour.focus();
    await page.context().grantPermissions(['clipboard-read','clipboard-write'], {origin:new URL(base).origin});
    await page.evaluate(() => navigator.clipboard.writeText('05:23:14PM'));
    await hour.press('Control+v');
    await settle(page);
    assert.deepEqual(await demo.getByRole('spinbutton').evaluateAll(es=>es.map(e=>e.value)),['05','23','14']);return;
  }
  const trigger=demo.locator('.sapid-time-picker-preset-trigger');
  await trigger.click();await settle(page);assert.equal(await trigger.getAttribute('aria-expanded'),'true');
  const choices=demo.locator('.sapid-time-picker-preset:visible');
  const preset={'time-presets':'15:45','time-presets-groups':'17:30','time-presets-range':'10:30','close-dropdown-on-preset-select':'12:30'}[name];
  if (preset) {
    if (name==='time-presets-groups')assert.deepEqual(await demo.locator('.sapid-time-picker-preset-group-label').allTextContents(),['Morning','Afternoon','Evening']);
    if (name==='time-presets-range')assert.equal(await choices.count(),9);
    await choices.filter({hasText:preset}).click();await settle(page);
    assert.equal((await hour.inputValue()).padStart(2,'0'),preset.slice(0,2));
    assert.equal(await trigger.getAttribute('aria-expanded'),name==='close-dropdown-on-preset-select'?'false':'true');
  }
  await page.keyboard.press('Escape');await settle(page);
  assert.equal(await trigger.getAttribute('aria-expanded'),'false');
}


(async()=>{
  const folder=outputPath('example-review/reference-variants');fs.mkdirSync(folder,{recursive:true});
  const browser=await chromium.launch(),page=await browser.newPage({viewport:{width:1280,height:960},reducedMotion:'reduce'});
  page.setDefaultTimeout(5000);const results=[],errors=[];page.on('pageerror',err=>errors.push(String(err)));
  try {
    for(const [slug,names] of Object.entries(groups)) {
      await page.goto(base+'/showcase.html?widget='+slug,{waitUntil:'domcontentloaded'});await page.waitForFunction(()=>window.ready,undefined,{timeout:60000});
      for(const name of names){const id=`reference-${slug}-${name}`;const demo=page.locator('.site-component-detail').filter({has:page.locator('#'+id)}).locator('.site-demo').first();const row={slug,id};
        try {
          await demo.screenshot({path:path.join(folder,id+'-before.png')});
          let result=demo;
          if(slug==='agenda-view') result=await checkAgenda(page,demo,name);
          else if(slug==='time-picker') await checkTime(page,demo,name);
          await result.screenshot({path:path.join(folder,id+'-after.png')});
          if(slug==='agenda-view' && name==='event-click') {
            await page.keyboard.press('Escape');
            await result.waitFor({state:'hidden'});
          }
          row.passed=true;
        }
        catch(error){row.error=String(error);}results.push(row);console.log(id,row.passed||row.error.split('\n')[0]);
      }
    }
  } finally {await browser.close();fs.writeFileSync(path.join(folder,'checks.json'),JSON.stringify({base,results,errors},null,2));}
  assert(results.every(row=>row.passed));assert.deepEqual(errors,[]);
})().catch(e=>{console.error(e);process.exitCode=1;});
