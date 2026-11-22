// Exercise configurators in their containing example and observe the preview separately.
const fs = require('node:fs');
const {createHash} = require('node:crypto');
const {base, chromium, outputPath, showcaseSlugs} = require('./showcase-browser-env.cjs');

const tick = page => page.evaluate(() => new Promise(resolve =>
  requestAnimationFrame(() => requestAnimationFrame(() => requestAnimationFrame(resolve)))));
const hash = value => createHash('sha256').update(value).digest('hex');
const selector = 'input:not([type=hidden]),select,textarea,button,[role=slider],[role=radio],[role=switch]';

async function previewState(preview) {
  return hash(await preview.evaluate(root => {
    const origin = root.getBoundingClientRect();
    return JSON.stringify([...root.querySelectorAll('*')].map(el => {
      const style = getComputedStyle(el);
      const box = el.getBoundingClientRect();
      return [el.tagName, el.children.length ? null : el.textContent,
        el.value, el.checked, el.disabled, el.getAttribute('aria-checked'),
        el.getAttribute('aria-pressed'), el.getAttribute('aria-invalid'), el.getAttribute('aria-label'),
        el.getAttribute('d'), el.getAttribute('fill'), el.getAttribute('stroke'), style.strokeWidth, style.fill,
        style.backgroundImage, style.boxShadow, style.pointerEvents, el.inert,
        el.getAttribute('placeholder'), style.color, style.backgroundColor, style.borderColor,
        style.borderRadius, style.opacity, style.fontSize, style.fontWeight, style.display,
        style.visibility, style.transform, style.filter, style.maskImage,
        ...[box.x-origin.x,box.y-origin.y,box.width,box.height].map(x=>Math.round(x*10)/10)];
    }));
  }));
}

async function describe(control) {
  return control.evaluate(el => {
    const group = el.closest('[role=radiogroup]');
    const associated = el.labels?.[0]?.textContent || el.closest('label')?.textContent;
    const panel = el.closest('.site-playground-controls');
    let field = [...panel.children].find(child => child.contains(el));
    while (field?.matches('.sapid-stack') && field.children.length &&
      [...field.children].every(child => child.matches('.sapid-stack,.sapid-field,.sapid-radio-group,.sapid-switch,.sapid-checkbox'))) {
      field = [...field.children].find(child => child.contains(el));
    }
    return {label:el.getAttribute('aria-label') || associated?.trim() || el.textContent?.trim() ||
        el.getAttribute('name') || el.getAttribute('placeholder') || '',
      field:field?.innerText.trim().split(/\n| · /)[0].trim() || '',
      group:group?.getAttribute('aria-label') || '', type:el.getAttribute('role') ||
        (el.tagName==='INPUT' ? el.type : el.tagName.toLowerCase()),
      value:el.value, selected:el.checked || el.getAttribute('aria-checked')==='true' ||
        el.getAttribute('aria-pressed')==='true', options:el.options ?
        [...el.options].filter(o=>!o.disabled && o.value!==el.value).map(o=>o.value):[]};
  });
}

async function change(control, info) {
  switch(info.type) {
    case 'select':
    case 'combobox':
      if (!info.options.length) return false;
      await control.selectOption(info.options[0]); break;
    case 'range':
    case 'slider': {
      const atMax = await control.evaluate(el => Number(el.getAttribute('aria-valuenow') ?? el.value) >=
        Number(el.getAttribute('aria-valuemax') ?? el.max));
      await control.press(atMax ? 'Home' : 'End'); break;
    }
    case 'text': case 'search': case 'textarea': case 'textbox':
      await control.fill(info.value?.startsWith('#') ? '#ef36c4' : 'Preview check'); break;
    case 'color': await control.fill('#ef36c4'); break;
    case 'number': case 'spinbutton': await control.press('ArrowUp'); break;
    default: await control.click();
  }
  return true;
}

async function inspect(page,slug) {
  const result={slug,base,sections:[],errors:[]};
  const error = e => result.errors.push(e.message);
  page.on('pageerror',error);
  try {
    await page.goto(base+'/showcase.html?widget='+slug,{timeout:45000});
    await page.waitForFunction(()=>window.ready===true, undefined, {timeout:30000});
    result.asset=await page.locator('link[href*="assets-"][href$="site.css"]').first().getAttribute('href');
    const demos=page.locator('.site-doc-content .site-preview .site-demo');
    if (!(await demos.count())) throw Error('No rendered Usage example');
    for (const demo of await demos.all()) {
      const heading=await demo.evaluate(el=>{
        const title=el.closest('.site-component-detail')?.querySelector('h2,h3');
        return {title:title?.textContent || 'Usage', anchor:title?.id || 'usage'};
      });
      const panels=demo.locator('.site-playground-controls');
      const section={...heading,panels:await panels.count(),controls:[]};
      result.sections.push(section);
      for(const panel of await panels.all()) {
        const preview=panel.locator('..').locator(':scope > .site-playground-preview');
        const controls=panel.locator(selector);
        // Use live locators: changing variants can replace the preview's controls.
        for(let index=0;index<await controls.count();index++) {
          const control=controls.nth(index);
          if(!await control.isVisible() || !await control.isEnabled()) continue;
          const info=await describe(control);
          if(info.selected && ['radio','button'].includes(info.type)) continue;
          const record={label:info.label,group:info.group,field:info.field,type:info.type};
          section.controls.push(record);
          try {
            await control.scrollIntoViewIfNeeded();
            await tick(page);
            const before=await previewState(preview);
            if(!await change(control,info)){record.outcome='no alternate value';continue;}
            await tick(page);
            record.outcome=(await previewState(preview))!==before ? 'preview changed' : 'needs contextual check';
          } catch(e) {record.outcome='error';record.error=e.message.split('\n')[0];}
        }
      }
    }
  } catch(e) {result.errors.push(e.message.split('\n')[0]);}
  finally {page.off('pageerror',error);}
  fs.writeFileSync(outputPath('example-review/interactions/'+slug+'.json'),JSON.stringify(result,null,1));
  console.log(slug,result.sections.reduce((sum,s)=>sum+s.controls.length,0),result.errors.length);
}

module.exports = {previewState, tick};

if (require.main === module) (async()=>{
  fs.mkdirSync(outputPath('example-review/interactions'),{recursive:true});
  const requested=process.argv.slice(2);
  const queue=requested.length ? requested : showcaseSlugs();
  const browser=await chromium.launch();
  try{
    await Promise.all(Array.from({length:4},async()=>{
      const page=await browser.newPage({viewport:{width:1336,height:980},reducedMotion:'reduce'});
      page.setDefaultTimeout(2500);
      while(queue.length) await inspect(page,queue.shift());
      await page.close();
    }));
  } finally {await browser.close();}
})().catch(error=>{console.error(error);process.exitCode=1;});
