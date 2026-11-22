const { checkBadge } = require('./showcase-playground-checks.cjs');
const assert = require('node:assert/strict'), fs = require('node:fs');
const {assertDocHeadingWeights,matrix, chromium, base, outputPath, writeReport } = require('./showcase-browser-env.cjs');
(async () => {
 const browser=await chromium.launch(), context=await browser.newContext(), page=await context.newPage();
 page.setDefaultTimeout(10000);
 const report={browser:browser.version(),pages:[],errors:[]};
 page.on('pageerror',e=>report.errors.push(e.message));
 page.on('console',m=>{if(m.type()==='error')report.errors.push(m.text());});
 const counts={avatar:11,badge:9,card:6,'color-swatch':4,'data-list':4,image:4,indicator:8,kbd:4,'number-formatter':3,'overflow-list':7,'rolling-number':3,spoiler:4,'theme-icon':5};
 const tick=()=>page.evaluate(()=>new Promise(resolve=>{let n=0;const f=()=>++n===8?resolve():requestAnimationFrame(f);requestAnimationFrame(f);}));
 const detail=id=>page.locator('.site-component-detail').filter({has:page.locator(`#${id}`)});
 const box=async l=>{const r=await l.boundingBox();assert(r);return r;};
 const focus=async l=>assert(await l.evaluate(e=>e===document.activeElement));
 const loaded=async l=>{await l.waitFor({state:'visible'});await page.waitForFunction(e=>e.complete&&e.naturalWidth>0,await l.elementHandle(),{timeout:10000});};
 const shot=async(l,name)=>l.screenshot({path:outputPath(`data-display-remediation/${name}.png`)});
 fs.mkdirSync(outputPath('data-display-remediation'),{recursive:true});
 try {
 for(const theme of (process.env.SAPID_DATA_DISPLAY_THEMES||matrix.themes.join(',')).split(',')) for(const scheme of (process.env.SAPID_DATA_DISPLAY_SCHEMES||matrix.schemes.join(',')).split(',')) for(const width of (process.env.SAPID_DATA_DISPLAY_WIDTHS||'1280,390').split(',').map(Number)) for(const slug of (process.env.SAPID_DATA_DISPLAY_PAGES||Object.keys(counts).join(',')).split(',')) {
  const key=`${slug}-${theme}-${scheme}-${width}`;
  try {
   await page.setViewportSize({width,height:1100});await page.emulateMedia({reducedMotion:'no-preference'});
   await page.goto(`${base}/showcase.html?widget=${slug}&theme=${theme}&scheme=${scheme}`);
   await page.waitForFunction(()=>window.ready===true,undefined,{timeout:30000});await tick();
   const asset=await page.evaluate(()=>document.documentElement.innerHTML.match(/assets-[a-f0-9]{20}/)?.[0]);assert(asset);if(report.asset)assert.equal(asset,report.asset);report.asset=asset;
   assert.equal(await page.locator('.site-component-detail').count(),counts[slug]);
   await assertDocHeadingWeights(page);
   const primary=page.locator('.site-preview .site-demo').first();
   if(slug==='avatar') {
    await loaded(primary.locator('img'));assert.equal(await primary.locator('.sapid-avatar').getAttribute('aria-label'),'Ava Patel');assert(!await primary.locator('.sapid-avatar-fallback').isVisible());
    const sizes=detail('avatar-sizes').locator('.sapid-avatar');assert.equal(await sizes.count(),10);for(let i=0;i<10;i++){const r=await box(sizes.nth(i));assert.equal(r.width,[24,30,36,42,50][i%5]);assert.equal(r.height,r.width);}
    const group=detail('avatar-group');assert.equal(await group.locator('.sapid-avatar').count(),4);assert(await group.getByText('+2',{exact:true}).isVisible());
    const recovery=detail('avatar-recovery'),image=recovery.locator('img'),icon=recovery.locator('.sapid-avatar-fallback');await loaded(image);await image.evaluate(e=>window.avatarImage=e);await icon.evaluate(e=>window.avatarFallback=e);await recovery.getByRole('button',{name:'Use unavailable avatar',exact:true}).click();await tick();assert(await icon.isVisible());assert.equal(await icon.evaluate(e=>e.tagName.toLowerCase()),'svg');await recovery.getByRole('button',{name:'Restore avatar photo',exact:true}).click();await loaded(image);await tick();assert(!await icon.isVisible());assert(await image.evaluate(e=>e===window.avatarImage));assert(await icon.evaluate(e=>e===window.avatarFallback));
   } else if(slug==='badge') {
          await checkBadge({ primary, width, tick, detail });
   } else if(slug==='card') {
    await loaded(primary.getByRole('img'));const image=await box(primary.locator('img'));assert.equal(image.height,190);assert(image.width<=420);await primary.getByRole('button',{name:'Save trip idea',exact:true}).click();await tick();assert(await primary.getByText('Added to your local trip ideas.',{exact:true}).isVisible());await primary.getByRole('button',{name:'Trip idea saved',exact:true}).click();await tick();assert(await primary.getByText('Save this idea for later.',{exact:true}).isVisible());
    const content=detail('card-content'),heading=content.getByRole('heading',{level:3});await heading.evaluate(e=>window.cardHeading=e);await content.getByRole('button',{name:'Toggle project details',exact:true}).click();await tick();assert(await content.locator('.sapid-card-body').getByText(/The design system brings together accessible components/).isVisible());assert(await heading.evaluate(e=>e===window.cardHeading));
   } else if(slug==='color-swatch') {
    await primary.getByRole('button',{name:'Lake blue',exact:true}).click();await tick();assert.equal(await primary.getByRole('button',{name:'Lake blue',exact:true}).getAttribute('aria-pressed'),'true');assert.equal(await primary.getByRole('button',{name:'Forest green',exact:true}).getAttribute('aria-pressed'),'false');assert(await primary.getByText('Selected color: rgba(35, 115, 210, 255)',{exact:true}).isVisible());
    assert.equal(await detail('swatch-alpha').locator('.sapid-color-swatch').count(),4);assert.equal(await detail('swatch-sizes').locator('.sapid-color-swatch').count(),6);
    const control=detail('swatch-controlled'),swatch=control.getByRole('button',{name:'Controlled lake blue',exact:true});await swatch.click();await tick();assert.equal(await swatch.getAttribute('aria-pressed'),'false');await control.getByRole('checkbox',{name:'Accept swatch selection',exact:true}).check();await swatch.focus();await page.keyboard.press('Space');await tick();assert.equal(await swatch.getAttribute('aria-pressed'),'true');assert(await control.getByRole('button',{name:'Unavailable gray',exact:true}).isDisabled());
   } else if(slug==='data-list') {
    const list=primary.locator('dl');assert.equal(await list.count(),1);assert.equal(await list.locator(':scope > dt').count(),4);assert.equal(await list.locator(':scope > dd').count(),4);const term=await box(list.locator('dt').first()),definition=await box(list.locator('dd').first());assert(Math.abs(definition.x-term.x)<2);assert(definition.y>=term.y+term.height);
    const lists=detail('data-orientation').locator('dl'),vertical=lists.nth(1),t=await box(vertical.locator('dt').first()),d=await box(vertical.locator('dd').first());assert(Math.abs(t.x-d.x)<2);assert(d.y>=t.y+t.height);
    const content=detail('data-content');await content.getByRole('button',{name:'Show delivery instructions',exact:true}).click();await tick();assert(await content.getByText('Ask reception to notify Ava when the package arrives.',{exact:true}).isVisible());
   } else if(slug==='image') {
    const img=primary.getByRole('img');await loaded(img);assert.equal(await img.evaluate(e=>getComputedStyle(e).objectFit),'cover');assert.equal((await box(img)).height,230);assert.equal(await img.getAttribute('alt'),'An alpine village beside a turquoise lake and high mountains');
    const fits=detail('image-fit').locator('img');for(let i=0;i<2;i++)await loaded(fits.nth(i));assert.equal(await fits.nth(0).evaluate(e=>getComputedStyle(e).objectFit),'cover');assert.equal(await fits.nth(1).evaluate(e=>getComputedStyle(e).objectFit),'contain');await shot(detail('image-fit'),`fit-${key}`);
    const dimensions=detail('image-dimensions').locator('img').first();await loaded(dimensions);const r=await box(dimensions);assert(Math.abs(r.width/r.height-1.5)<0.02);
    const fallback=detail('image-fallback'),photo=fallback.getByRole('img');await loaded(photo);await photo.evaluate(e=>window.fallbackPhoto=e);await fallback.getByRole('button',{name:'Load unavailable photo',exact:true}).click();await page.waitForFunction(()=>window.fallbackPhoto.currentSrc.includes('alpine-lake-photo.png')&&window.fallbackPhoto.complete&&window.fallbackPhoto.naturalWidth>0);await fallback.getByRole('button',{name:'Restore original photo',exact:true}).click();await tick();await loaded(photo);assert(await photo.evaluate(e=>e===window.fallbackPhoto));assert.equal(await detail('image-alternative').locator('img').getAttribute('alt'),'');
   } else if(slug==='indicator') {
    const demoMarker=primary.locator('.sapid-indicator-marker');assert.equal(await primary.locator('.sapid-avatar img').count(),1);await loaded(primary.locator('.sapid-avatar img'));await primary.getByRole('switch',{name:'Disabled',exact:true}).check();await tick();assert(!await demoMarker.isVisible());await primary.getByRole('switch',{name:'Disabled',exact:true}).uncheck();await tick();assert(await demoMarker.isVisible());
    assert.equal(await detail('indicator-positions').locator('.sapid-indicator-marker').count(),9);
    const inline=detail('indicator-pointer'), marker=inline.locator('.sapid-indicator-marker');
    assert.equal(await marker.innerText(),'New');
    assert.equal(await marker.evaluate(e=>getComputedStyle(e).pointerEvents),'none');
    await loaded(inline.locator('.sapid-avatar img'));
    assert.deepEqual(await detail('indicator-max-value').locator('.sapid-indicator-marker').allTextContents(), ['50','99+','999+']);
    assert.equal(await detail('indicator-zero').locator('.sapid-indicator-marker:visible').count(),1);
   } else if(slug==='kbd') {
    assert.equal(await primary.locator('kbd').count(),2);assert.equal(await primary.getByRole('button').count(),0);const platforms=detail('kbd-platforms');for(const label of ['Windows','Command','Super'])assert(await platforms.locator('kbd').filter({hasText:new RegExp('^'+label+'$')}).isVisible());const editor=detail('kbd-sizes').getByRole('textbox',{name:'Try normal typing',exact:true});await editor.fill('Ctrl K is ordinary text here');await page.keyboard.press('Home');await page.keyboard.press('ArrowRight');await page.keyboard.type('!');assert.equal(await editor.inputValue(),'C!trl K is ordinary text here');await focus(editor);
   } else if(slug==='number-formatter') {
    assert.equal(await primary.locator('.sapid-number-formatter').textContent(),'$ 1,000,000');
    assert.deepEqual(await detail('number-locales').locator('.sapid-number-formatter').allTextContents(),['1,000,000','1.000.000']);
    assert.deepEqual(await detail('number-affixes').locator('.sapid-number-formatter').allTextContents(),['$ 100','100 RUB']);
    assert.equal(await detail('number-precision').locator('.sapid-number-formatter').textContent(),'1.67');
   } else if(slug==='overflow-list') {
    const resizing=detail('overflow-width');await resizing.getByRole('checkbox',{name:'Use narrow navigation',exact:true}).check();await tick();const trigger=resizing.getByRole('button',{name:/^More \(/});await trigger.click();await tick();const settings=resizing.getByRole('button',{name:'Settings',exact:true});assert(await settings.isVisible());await settings.focus();await page.keyboard.press('Enter');await tick();assert(await resizing.getByText('Selected workspace section: Settings',{exact:true}).isVisible());await page.keyboard.press('Escape');await tick();
    const control=detail('overflow-controlled'),more=control.getByRole('button',{name:/^More \(/});await more.click();await tick();assert.equal(await more.getAttribute('aria-expanded'),'false');await control.getByRole('checkbox',{name:'Allow more tags popup',exact:true}).check();await more.click();await tick();assert.equal(await more.getAttribute('aria-expanded'),'true');assert(await control.getByText('Accessibility',{exact:true}).isVisible());await page.keyboard.press('Escape');await tick();
    const retained=detail('overflow-retained'),input=retained.getByRole('textbox',{name:'Retained project note',exact:true});await input.fill('Keep the project note');await input.evaluate(e=>window.retainedNote=e);await retained.getByRole('checkbox',{name:'Move note into More',exact:true}).check();await tick();await retained.getByRole('button',{name:/^More \(/}).click();await tick();assert.equal(await input.inputValue(),'Keep the project note');assert(await input.evaluate(e=>e===window.retainedNote));await input.focus();await page.keyboard.press('End');await page.keyboard.type('!');assert.equal(await input.inputValue(),'Keep the project note!');await shot(retained,`editing-${key}`);await page.keyboard.press('Escape');await tick();await retained.getByRole('checkbox',{name:'Move note into More',exact:true}).uncheck();await tick();assert.equal(await input.inputValue(),'Keep the project note!');assert(await input.evaluate(e=>e===window.retainedNote));
   } else if(slug==='rolling-number') {
    const value=primary.locator('.sapid-visually-hidden');assert.equal(await value.textContent(),'128');const reel=primary.locator('.sapid-rolling-number-reel').first();await reel.evaluate(e=>window.countReel=e);await primary.getByRole('button',{name:'Add one project',exact:true}).click();await tick();assert.equal(await value.textContent(),'129');assert(await reel.evaluate(e=>e===window.countReel));await primary.getByRole('button',{name:'Remove one project',exact:true}).click();await tick();assert.equal(await value.textContent(),'128');const line=(await box(primary.locator('.sapid-rolling-number-digit').first())).height;assert(Math.abs(line-58.9)<0.1);
    const changes=detail('rolling-digits');for(const next of ['100','1000','-12']){await changes.getByRole('button',{name:`Set ${next}`,exact:true}).click();await tick();assert.equal(await changes.locator('.sapid-visually-hidden').textContent(),next==='1000'?'1,000':next);}
    await primary.getByRole('checkbox',{name:'Animate project count',exact:true}).uncheck();await tick();assert(await primary.locator('.sapid-rolling-number-reel').evaluateAll(es=>es.every((e,i)=>Math.abs(new DOMMatrixReadOnly(getComputedStyle(e).transform).m42+[58.9,117.8,471.2][i])<0.1)));
    await page.emulateMedia({reducedMotion:'reduce'});await primary.getByRole('button',{name:'Add one project',exact:true}).click();await tick();assert.equal(await value.textContent(),'129');assert(await primary.locator('.sapid-rolling-number-reel').evaluateAll(es=>es.every(e=>getComputedStyle(e).transitionDuration==='0s')));
   } else if(slug==='spoiler') {
    const trigger=primary.getByRole('button',{name:'Read the full walking guide',exact:true});
    const viewport=primary.locator('.sapid-spoiler-viewport');
    const text=viewport.locator('.sapid-spoiler-text');
    await loaded(viewport.getByRole('img',{name:'The lakeside village below the alpine peaks'}));
    await text.evaluate(e=>window.guideText=e);
    assert.equal(await trigger.getAttribute('aria-expanded'),'false');
    const before=(await box(viewport)).height;
    await trigger.click();
    await tick();
    assert.equal(await primary.getByRole('button',{name:'Show a shorter guide',exact:true}).getAttribute('aria-expanded'),'true');
    assert((await box(viewport)).height>=before+100,'the full guide reveals several more lines');
    assert(await text.evaluate(e=>e===window.guideText));
    await page.keyboard.press('Space');
    await tick();
    assert.equal(await trigger.getAttribute('aria-expanded'),'false');
    await viewport.evaluate((el,target)=>new Promise((resolve,reject)=>{const deadline=performance.now()+2000;function check(){if(Math.abs(el.getBoundingClientRect().height-target)<0.5)return resolve();if(performance.now()>deadline)return reject(Error('Spoiler did not settle at collapsed height'));requestAnimationFrame(check)}check()}),before);
    await focus(trigger);
    assert.equal(await detail('spoiler-threshold').locator('.site-preview').getByRole('button').count(),1);const control=detail('spoiler-controlled'),expand=control.getByRole('button',{name:'Read more about the route',exact:true});await expand.click();await tick();assert.equal(await expand.getAttribute('aria-expanded'),'false');await control.getByRole('checkbox',{name:'Allow guide expansion',exact:true}).check();await expand.click();await tick();assert(await control.getByRole('button',{name:'Collapse route details',exact:true}).isVisible());const changing=detail('spoiler-changes');assert.equal(await changing.locator('.site-preview').getByRole('button').count(),0);await changing.getByRole('checkbox',{name:'Show extended route notes',exact:true}).check();await tick();assert(await changing.getByRole('button',{name:'Show more',exact:true}).isVisible());
   } else if(slug==='theme-icon') {
    const icon=primary.locator('.sapid-theme-icon');
    await primary.getByRole('button',{name:'Use Yellow',exact:true}).click();
    await tick();
    assert.equal(await icon.evaluate(e=>getComputedStyle(e).backgroundColor),'rgb(250, 176, 5)');
    await primary.getByRole('combobox',{name:'Variant'}).selectOption('outline');
    await tick();
    assert.equal(await icon.evaluate(e=>getComputedStyle(e).borderTopStyle),'solid');
    const gradient=detail('theme-icon-gradient'), tile=gradient.locator('.sapid-theme-icon');
    const initialGradient=await tile.evaluate(e=>getComputedStyle(e).backgroundImage);
    await gradient.getByRole('button', { name: 'Gradient to: use Orange' }).click();
    await gradient.getByRole('slider',{name:'Gradient degree'}).press('End');
    await tick();
    const changed=await tile.evaluate(e=>getComputedStyle(e).backgroundImage);
    assert.notEqual(changed,initialGradient);
    assert(changed.includes('360deg')&&changed.includes('253, 126, 20'));
    assert.equal(await primary.locator('.site-playground-preview').getByRole('button').count(),0);assert.equal(await primary.locator('.sapid-theme-icon svg').count(),1);const sizes=detail('theme-icon-sizes').locator('.sapid-theme-icon');assert.equal(await sizes.count(),8);for(let i=0;i<5;i++){const r=await box(sizes.nth(i));assert.equal(r.width,[24,30,36,42,50][i]);}for(let i=0;i<3;i++)assert.equal((await box(sizes.nth(i+5).locator('svg'))).width,[12,20,28][i]);const actions=detail('theme-icon-actions');await actions.getByRole('button',{name:'Open project information',exact:true}).focus();await page.keyboard.press('Enter');await tick();assert(await actions.getByText('Project information: this workspace is shared with your design team.',{exact:true}).isVisible());
   }
   assert(await page.evaluate(()=>document.documentElement.scrollWidth<=innerWidth+1));
   await primary.scrollIntoViewIfNeeded();await shot(primary,key);
   report.pages.push({key,passed:true});writeReport('data-display-remediation.json',report);console.log(`PASS ${key}`);
  } catch(error) {
   report.pages.push({key,passed:false,failure:error.stack});writeReport('data-display-remediation.json',report);console.error(`FAIL ${key}: ${error.stack}`);await page.screenshot({path:outputPath(`data-display-remediation/failure-${key}.png`)});
  }
 }
 assert.equal(report.pages.filter(p=>!p.passed).length,0);assert.deepEqual(report.errors,[]);
 } finally {writeReport('data-display-remediation.json',report);await browser.close();}
})().catch(error=>{console.error(error);process.exitCode=1;});
