// Exercise each configuration label published in the reference inventory.
const fs = require('node:fs');
const path = require('node:path');
const {base, chromium, outputPath} = require('./showcase-browser-env.cjs');
const {previewState, tick} = require('./showcase-panel-review.cjs');
const dir = outputPath('example-review');
const inventory = fs.readdirSync(path.join(dir,'upstream')).map(slug=>{
  const page=JSON.parse(fs.readFileSync(path.join(dir,'upstream',slug,'index.json'),'utf8'));
  return {...page,examples:page.examples.map(e=>({...e,controls:e.panelKind==='selectors'?[]:e.fields}))};
});
const groups = new Set(['Charts','Dates','Extensions']);

async function operate(panel, label) {
  const role = kind => panel.getByRole(kind,{name:label,exact:true});
  if (label === 'Color' || label === 'Stroke color') {
    const name = label === 'Color' ? 'Use Red' : 'Stroke color: use Red';
    await panel.getByRole('button',{name,exact:true}).first().click();
    return name;
  }
  for (const kind of ['switch','checkbox','textbox','slider','combobox','radiogroup']) {
    const control = role(kind);
    if (!(await control.count())) continue;
    if (kind === 'switch' || kind === 'checkbox') await control.click();
    if (kind === 'textbox') await control.fill('Review '+label);
    if (kind === 'slider') {
      const end = await control.evaluate(el => Number(el.getAttribute('aria-valuenow') ?? el.value) >= Number(el.getAttribute('aria-valuemax') ?? el.max));
      await control.press(end ? 'Home' : 'End');
    }
    if (kind === 'combobox') {
      const other = await control.evaluate(el=>[...el.options].find(o=>!o.disabled && o.value && o.value!==el.value)?.value);
      await control.selectOption(other);
    }
    if (kind === 'radiogroup') await control.locator('input:not(:checked)').first().check();
    return kind;
  }
  throw Error('No named control for '+label);
}

async function observe(page, preview, label) {
  if (label === 'With tooltip') {
    const point = preview.locator('.sapid-plot-point').first();
    await point.hover({force:true});
    await tick(page);
    return preview.locator('.sapid-plot-tooltip').isVisible();
  }
  if (label === 'Drag free') {
    const slide = preview.locator('.sapid-carousel-slide').filter({hasText:'1'}).first();
    const box = await slide.boundingBox();
    const x = box.x + box.width/2, y = box.y + box.height/3;
    await page.mouse.move(x,y);
    await page.mouse.down();
    await page.mouse.move(x-35,y,{steps:8});
    await page.mouse.up();
    await tick(page);
    return Math.round((await slide.boundingBox()).x-box.x);
  }
  return previewState(preview);
}

async function run(page, entry) {
  const panels=[];
  await page.goto(base+'/showcase.html?widget='+entry.slug);
  await page.waitForFunction(()=>window.ready === true, undefined, {timeout:45000});
  for (const official of entry.examples.filter(e=>e.controls.length)) {
    const demo = official.title === 'Usage' ? page.locator('.site-demo').first() :
      page.locator('.site-component-detail').filter({has:page.getByRole('heading',{name:official.title,exact:true})})
        .filter({has:page.locator('.site-playground-controls')}).first();
    const controls = demo.locator('.site-playground-controls').first();
    const preview = demo.locator('.site-playground-preview').first();
    const panel={slug:entry.slug,title:official.title,controls:{}};
    panels.push(panel);
    for (const label of new Set(official.controls)) {
      try {
        await controls.scrollIntoViewIfNeeded();
        await tick(page);
        const before = await observe(page, preview,label);
        const action = await operate(controls,label);
        await tick(page);
        const after = await observe(page,preview,label);
        panel.controls[label]={action,changed:before!==after};
        if (['Drag free','With tooltip'].includes(label)) Object.assign(panel.controls[label],{before,after});
      } catch (err) { panel.controls[label]={error:err.message.split('\n')[0]}; }
    }
    const failed=Object.entries(panel.controls).filter(([,r])=>!r.changed).map(([k,v])=>k+' '+(v.error||'unchanged'));
    console.log(entry.slug,official.title,failed.join(', ')||'PASS');
  }
  return panels;
}

(async()=>{
  const browser=await chromium.launch();
  const requested=process.argv.slice(2);
  const queue=inventory.filter(e=>groups.has(e.group)&&e.examples.some(x=>x.controls.length)&&(!requested.length||requested.includes(e.slug)));
  const panels=[],errors=[];
  try{
    await Promise.all(Array.from({length:3},async()=>{
      const page=await browser.newPage({viewport:{width:1336,height:980},reducedMotion:'reduce'});
      page.setDefaultTimeout(2200);
      while(queue.length){
        const entry=queue.shift();
        const onError=e=>errors.push({slug:entry.slug,error:e.message});
        page.on('pageerror',onError);
        try {panels.push(...await run(page,entry));}catch(e){onError(e);}
        page.off('pageerror',onError);
      }
      await page.close();
    }));
  }finally{await browser.close();}
  const file=requested.length?'reference-controls-subset.json':'reference-controls.json';
  fs.writeFileSync(path.join(dir,file),JSON.stringify({base,panels,errors},null,1));
})().catch(e=>{console.error(e);process.exitCode=1;});
