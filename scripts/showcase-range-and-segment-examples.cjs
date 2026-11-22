const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const {chromium,base,outputPath} = require('./showcase-browser-env.cjs');
const selectors = {'date-picker':'.sapid-calendar-day','month-picker':'.sapid-period-button','year-picker':'.sapid-period-button'};
async function pick(page,demo,slug,variant) {
  const cells=demo.locator(selectors[slug]+':enabled');
  const header=demo.locator(slug==='date-picker'?'.sapid-calendar-header':'.sapid-period-header').first();
  const before=await header.textContent();
  await cells.nth(7).click();
  if(variant==='controlled-date') {
    await page.waitForFunction(([node,old])=>node.textContent!==old,[await header.elementHandle(),before]);
    const next=await header.textContent();
    await cells.nth(8).click();
    await page.evaluate(()=>new Promise(resolve=>requestAnimationFrame(()=>requestAnimationFrame(resolve))));
    assert.equal(await header.textContent(),next);
  } else {
    await cells.nth(7).click();
    await cells.nth(9).click();
    await page.waitForFunction(control => {
      const state = control.getAttribute('aria-selected') || control.getAttribute('aria-pressed')
        || control.closest('[role=gridcell]')?.getAttribute('aria-selected');
      return state === 'false';
    }, await cells.nth(7).elementHandle());
  }
}
async function tooltip(demo) {
  const plots=demo.locator('.sapid-plot');
  assert.equal(await plots.count(),2);
  for(let i=0;i<2;i++) {
    const plot=plots.nth(i);
    await plot.locator('.sapid-plot-point').first().hover();
    const tip=plot.locator('.sapid-plot-tooltip');
    await tip.waitFor();
    const text=await tip.textContent();
    assert.match(text,/USA.*400/);
    assert.equal(text.includes('India'),i===1);
    assert.equal(text.includes('Japan'),i===1);
    assert.equal(text.includes('Other'),i===1);
  }
}
(async()=>{
  const root=outputPath('example-review/range-and-segment-actions');
  fs.mkdirSync(root,{recursive:true});const browser=await chromium.launch();const page=await browser.newPage({viewport:{width:1336,height:980}});
  const rows=[],errors=[];page.on('pageerror',e=>errors.push(String(e)));page.setDefaultTimeout(4500);
  try {
    for(const slug of [...Object.keys(selectors),'pie-chart','donut-chart','funnel-chart']) {
      await page.goto(base+'/showcase.html?widget='+slug);await page.waitForFunction(()=>window.ready,undefined,{timeout:45000});
      for(const variant of (selectors[slug]?['single-date-in-range','controlled-date']:['tooltip-data-source'])) {
        const id='reference-'+slug+'-'+variant;
        const demo=page.locator('.site-component-detail').filter({has:page.locator('#'+id)}).locator('.site-demo');
        let row={slug,id};
        try {
          await demo.screenshot({path:path.join(root,id+'-before.png')});
          if(selectors[slug])await pick(page,demo,slug,variant);else await tooltip(demo);
          await demo.screenshot({path:path.join(root,id+'-after.png')});row.passed=true;
        } catch(e){row.error=String(e);}
        rows.push(row);console.log(id,row.passed||row.error);
      }
    }
  } finally {await browser.close();fs.writeFileSync(path.join(root,'checks.json'),JSON.stringify({base,results:rows,errors},null,2));}
  assert(rows.every(r=>r.passed));assert.equal(errors.length,0);
})().catch(e=>{console.error(e);process.exitCode=1;});
