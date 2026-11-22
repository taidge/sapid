const assert = require('node:assert/strict');
const fs = require('node:fs');
const { chromium, base, outputPath, writeReport } = require('./showcase-browser-env.cjs');

const primaryFields = ['text-input', 'textarea', 'password-input', 'number-input', 'file-input', 'mask-input', 'color-input'];
const secondaryFields = ['autocomplete', 'select', 'multi-select', 'tags-input', 'tree-select', 'pills-input', 'cascader', 'json-input', 'phone-input'];

function fieldSection(page, slug) {
  return secondaryFields.includes(slug)
    ? page.locator('.site-component-detail').filter({has:page.locator('#input-properties')}).locator('.site-field-configurator')
    : page.locator('.site-preview .site-field-configurator').first();
}
const settle=page=>page.evaluate(()=>new Promise(done=>requestAnimationFrame(()=>requestAnimationFrame(()=>requestAnimationFrame(done)))));
async function fieldAppearance(preview, slug) {
  return preview.locator('.sapid-field').first().evaluate((field,slug) => {
    const parts = [...field.querySelectorAll('*')].filter(node => node.getBoundingClientRect().height > 0);
    const shell = (slug==="phone-input" && field.querySelector('input[type="tel"],input[autocomplete="tel"]')) || parts.find(node => {
      const style = getComputedStyle(node);
      return parseFloat(style.borderTopWidth) > 0 && node.getBoundingClientRect().width > 180;
    });
    if (!shell) throw new Error('No visible bordered field surface');
    const style = getComputedStyle(shell);
    return { height:shell.getBoundingClientRect().height, radius:parseFloat(style.borderTopRightRadius),
      background:style.backgroundColor, border:style.borderTopColor };
  },slug);
}

async function verifyField(page, slug) {
  const section=fieldSection(page,slug);
  const controls=section.locator('.site-playground-controls'), preview=section.locator('.site-playground-preview');
  const description=controls.getByRole('textbox',{name:'Description',exact:true});
  const label=controls.getByRole('textbox',{name:'Label',exact:true});
  await label.fill('Shipping address');
  await description.fill('Sent to this address');
  await page.waitForFunction(() => [...document.querySelectorAll('.site-playground-preview .sapid-field-label')].some(el => el.textContent === 'Shipping address'));
  await settle(page);
  assert.equal((await preview.locator('.sapid-field-description').first().innerText()).trim(),'Sent to this address');
  const initial=await fieldAppearance(preview,slug);
  await controls.getByRole('slider',{name:'Size',exact:true}).press('End');
  await settle(page);
  const enlarged=await fieldAppearance(preview,slug);
  assert(enlarged.height>initial.height,slug+' Size must increase the rendered field height');
  await controls.getByRole('slider',{name:'Radius',exact:true}).press('End');
  await settle(page);
  assert((await fieldAppearance(preview,slug)).radius>initial.radius,slug+' Radius must round the actual field');
  await controls.getByRole('radio',{name:'Filled',exact:true}).check();
  await settle(page);
  assert.notEqual((await fieldAppearance(preview,slug)).background,initial.background,slug+' Filled must change the background');
  await controls.getByRole('radio',{name:'Default',exact:true}).check();
  await controls.getByRole('switch',{name:'With asterisk',exact:true}).click();
  await settle(page);
  const marker=await preview.locator('.sapid-field-label').first().evaluate(el=>getComputedStyle(el,'::after').content);
  assert(marker.includes('*'),slug+' required indicator must be visible');
  const error=controls.getByRole('textbox',{name:'Error',exact:true});
  await error.fill('Enter a valid value');
  await settle(page);
  assert.equal(await preview.locator('.sapid-field-error').first().innerText(),'Enter a valid value');
  await preview.locator('[aria-invalid=true]').first().waitFor({state:'attached'});
  assert.notEqual((await fieldAppearance(preview,slug)).border,initial.border,slug+' Error must change the real border');
}

(async()=>{
 const browser=await chromium.launch(), page=await browser.newPage({viewport:{width:1280,height:900}});
 const errors=[],results=[];page.on('pageerror',e=>errors.push(e.message));
 page.setDefaultTimeout(5000);
 fs.mkdirSync(outputPath('field-configurators'),{recursive:true});
 try {
   for (const slug of [...primaryFields,...secondaryFields]) {
     try {
       await page.goto(base+'/showcase.html?widget='+slug);
       await page.waitForFunction(()=>window.ready===true,undefined,{timeout:20000});
       const panel=fieldSection(page,slug);
       await panel.screenshot({path:outputPath('field-configurators/'+slug+'.png')});
       await verifyField(page,slug);
       results.push({slug,passed:true});
     } catch(error){results.push({slug,passed:false,failure:error.message});}
   }
 } finally {
    writeReport('field-configurators.json',{results,errors});
    await browser.close();
 }
 assert.deepEqual(errors,[]);assert.deepEqual(results.filter(result=>!result.passed),[]);
})().catch(error=>{console.error(error);process.exitCode=1});
