// Capture each rendered example separately so a working primary demo cannot mask
// a missing configurator or an overflowing detail example further down the page.
const fs = require('node:fs');
const path = require('node:path');
const {base, chromium, outputPath} = require('./showcase-browser-env.cjs');

const source = fs.readFileSync(path.join(__dirname, '../examples/web/src/showcase_catalog.rs'), 'utf8');
const pages = [...source.matchAll(/slug: "([^"]+)".*?name: "([^"]+)".*?group: "([^"]+)"/g)]
  .map(([,slug,name,group]) => ({slug,name,group}));
const mode = process.argv[2];
if (!['local','upstream'].includes(mode)) throw Error('Specify local or upstream');
const viewportWidth = Number(process.env.SAPID_REVIEW_WIDTH || 1336);
if (!Number.isInteger(viewportWidth) || viewportWidth < 280 || viewportWidth > 3840) throw Error('Invalid review viewport width');
const reviewFolder = viewportWidth === 1336 ? 'example-review' : 'example-review/viewport-' + viewportWidth;
const requested = new Set(process.argv.slice(3));
const work = pages.filter(p => !requested.size || requested.has(p.slug));
const packagePaths = {Charts:'charts', Dates:'dates', Extensions:'x'};
const aliases = {'notifications-system':'x/notifications', 'navigation-progress':'x/nprogress',
  'rich-text-editor':'x/tiptap', 'modals-manager':'x/modals', 'code-highlight':'code-highlight/code-highlight',
  'json-viewer':'code-highlight/json-viewer'};
function urlFor({slug,group}) {
  if (mode === 'local') return base + '/showcase.html?widget=' + slug;
  const schedule = group === 'Dates' && /schedule|view$|events/.test(slug);
  const route = aliases[slug] || (schedule ? 'schedule' : packagePaths[group] || 'core') + '/' + slug;
  return 'https://mantine.dev/' + route + '/';
}

function locateExamples(upstream) {
  const query = (node, selector) => [...node.querySelectorAll(selector)];
  const getBox = node => {
    const {x,y,width,height} = node.getBoundingClientRect();
    return [x,y,width,height].map(n => Math.round(n*10)/10);
  };
  let examples;
  if (upstream) {
    const mdx = document.querySelector('#mdx');
    if (!mdx) throw Error('Mantine document body missing');
    // Padded and unpadded examples share the same preview class.
    const padded = mdx.querySelector('[data-with-padding]');
    const previewSelector = padded ? '.' + CSS.escape(padded.classList[0]) : '.m_a3c6e060';
    examples = query(mdx,previewSelector).map(preview => {
      const wrapper = preview.parentElement;
      const sibling = [...wrapper.children].filter(el => el !== preview);
      const stylePanel = sibling.find(el => [...el.querySelectorAll('p')].some(p => p.textContent==='Component Styles API'));
      const panel = stylePanel || sibling.find(el => el.querySelector('.mantine-SimpleGrid-root'));
      const heading = query(mdx,'[data-heading]').filter(el =>
        el.compareDocumentPosition(preview) & Node.DOCUMENT_POSITION_FOLLOWING).at(-1);
      return {root: panel ? wrapper : preview, preview, panel, panelKind:stylePanel ? 'selectors' : (panel ? 'properties' : null), title:heading?.dataset.heading || 'Overview',anchor:heading?.id};
    });
  } else {
    examples = query(document,'.site-doc-content .site-preview .site-demo').map((root,i) => ({
      root,preview:root.querySelector('.site-playground-preview,.site-styles-preview') || root,
      panel:root.querySelector('.site-playground-controls,.site-styles-parts'),
      panelKind:root.querySelector('.site-styles-parts') ? 'selectors' : null,
      title:root.closest('.site-component-detail')?.querySelector('h2,h3')?.textContent || (i ? 'Example' : 'Usage'),
      anchor:root.closest('.site-component-detail')?.querySelector('h2,h3')?.id || 'usage'
    }));
  }
  return examples.map(({root,preview,panel,panelKind,title,anchor},index) => {
    root.dataset.sapidReviewExample = index;
    let fields = [];
    if (panel && panelKind !== 'selectors') {
      function localFieldNames(control) {
        const children = [...control.children];
        if (control.matches('.sapid-stack') && children.length &&
            children.every(child => child.matches('.sapid-stack,.sapid-field,.sapid-radio-group,.sapid-switch,.sapid-checkbox'))) {
          return children.flatMap(localFieldNames);
        }
        return [control.innerText.trim().split(/\n| · /)[0].trim()];
      }
      fields = upstream
        ? query(panel,'.mantine-InputWrapper-label,.mantine-Switch-label,.mantine-Checkbox-label').map(el=>el.textContent.trim())
        : [...panel.children].flatMap(localFieldNames);
      fields = fields.filter(Boolean);
    }
    const geometry = query(preview,'input:not([type=hidden]),button,select,textarea,[role=slider],svg').filter(el=> {
      const r=el.getBoundingClientRect();return r.width && r.height;
    }).map(el=>({tag:el.tagName.toLowerCase(),cls:el.getAttribute('class'),label:el.getAttribute('aria-label'),
      box:getBox(el),parent:getBox(el.parentElement)}));
    const icons = query(preview, 'button, [role=button]').flatMap(button => {
      if (button.innerText.trim() || button.querySelectorAll('svg').length !== 1) return [];
      const icon = button.querySelector('svg');
      const [bx,by,bw,bh] = getBox(button), [ix,iy,iw,ih] = getBox(icon);
      if (!bw || !bh || !iw || !ih) return [];
      const round = n => Math.round(n*10)/10;
      return [{name:button.getAttribute('aria-label') || button.title || '', cls:button.className,
        button:[bw,bh],icon:[iw,ih],offset:[round(ix+iw/2-bx-bw/2),round(iy+ih/2-by-bh/2)]}];
    });
    const fieldIcons = [];
    const round = n => Math.round(n*10)/10;
    const fieldsWithIcons = query(preview,
      'input:not([type=hidden]):not([type=checkbox]):not([type=radio]),select,[role=combobox],button[aria-haspopup]');
    const glyphs = query(preview, 'svg').map(node => node.getBoundingClientRect()).filter(box =>
      box.width >= 4 && box.width <= 42 && box.height >= 4 && box.height <= 42);
    for (const field of fieldsWithIcons) {
      const box = field.getBoundingClientRect();
      if (box.width < 40 || box.height < 14 || box.height > 80) continue;
      const label = field.getAttribute('aria-label') || field.labels?.[0]?.textContent.trim() || '';
      const record = (side,width,height,inset,dy) => fieldIcons.push({label,side,
        size:[round(width),round(height)],inset:round(inset),dy:round(dy)});
      for (const glyph of glyphs) {
        const dy = glyph.y+glyph.height/2-box.y-box.height/2;
        const left = glyph.left-box.left, right = box.right-glyph.right;
        if (Math.abs(dy)>10 || Math.min(left,right)<-1 || Math.min(left,right)>Math.min(48,box.width/3)) continue;
        record(left < right ? 'left':'right',glyph.width,glyph.height,Math.min(left,right),dy);
      }
      const style=getComputedStyle(field);
      const size=style.backgroundSize.match(/^([\d.]+)px\s+([\d.]+)px$/);
      const position=style.backgroundPosition.match(/^(left|right)\s+([\d.]+)px\s+50%$/);
      if (style.backgroundImage.includes('svg') && size && position) {
        const side=position[1], border=style.backgroundOrigin==='border-box' ? 0 :
          parseFloat(side==='left' ? style.borderLeftWidth : style.borderRightWidth);
        record(side,+size[1],+size[2],+position[2]+border,0);
      }
    }
    const selectors = panelKind === 'selectors' ? query(panel, 'button').map(b=>({name:b.querySelector('p,.site-style-part-name')?.textContent, description:b.querySelector('p:nth-child(2),.site-style-part-description')?.textContent})).filter(s=>s.name) : [];
    return {index,title,anchor,panel:!!panel,panelKind:panelKind || (panel ? 'properties' : null),fields,selectors,icons,fieldIcons,
      previewOverflow:Math.max(0,preview.scrollWidth-preview.clientWidth),box:getBox(root),geometry};
  });
}

async function capture(context, item) {
  const dir = outputPath(reviewFolder + '/' + mode + '/' + item.slug);
  fs.mkdirSync(dir,{recursive:true});
  const page = await context.newPage();
  const record = {...item, url:urlFor(item), examples:[], errors:[]};
  page.on('pageerror', error => record.errors.push({message:error.message.split('\n')[0]}));
  try {
    const source = mode === 'upstream' && process.env.SAPID_MANTINE_BASE ?
      new URL(new URL(record.url).pathname,process.env.SAPID_MANTINE_BASE).href : record.url;
    record.renderedUrl=source;
    let response;
    for (let attempt=0; attempt<4; attempt++) {
      response = await page.goto(source,{waitUntil:'domcontentloaded',timeout:45000});
      if (response?.status() !== 429 || attempt === 3) break;
      const retry = response.headers()['retry-after'];
      const seconds = retry && Number.isFinite(+retry) ? +retry : 60;
      await new Promise(resolve=>setTimeout(resolve,Math.max(60,seconds)*1000));
    }
    if (!response?.ok()) throw Error('HTTP ' + response?.status());
    if (mode === 'local') await page.waitForFunction(()=>window.ready === true, undefined, {timeout:60000});
    else await page.locator('#mdx').waitFor();
    await page.evaluate(async () => {
      await document.fonts.ready;
      await new Promise(resolve => requestAnimationFrame(() => requestAnimationFrame(resolve)));
    });
    record.asset = await page.locator('link[href*="assets-"][href$="site.css"]').first().getAttribute('href', {timeout:1000}).catch(() => null);
    record.examples = await page.evaluate(locateExamples,mode === 'upstream');
    record.viewportWidth = viewportWidth;
    record.pageOverflow = await page.evaluate(() => Math.max(0, document.documentElement.scrollWidth - innerWidth));
    if (mode === 'local' && !record.examples.length) throw Error('No local example rendered');
    for (const demo of record.examples) {
      const locator = page.locator('[data-sapid-review-example="'+demo.index+'"]');
      const filename = String(demo.index+1).padStart(2,'0') + '.png';
      try {
        await locator.screenshot({path:path.join(dir,filename),timeout:20000,animations:'disabled'});
        demo.image=filename;
      } catch(e) {record.errors.push({example:demo.index,message:e.message.split('\n')[0]});}
    }
  } catch(e) {record.errors.push({message:e.message.split('\n')[0]});}
  finally {await page.close();}
  fs.writeFileSync(path.join(dir,'index.json'),JSON.stringify(record,null,1));
  console.log(item.slug,record.examples.length,record.errors.length);
  return record.errors.length;
}

(async()=>{
  const browser=await chromium.launch();
  try {
    const context=await browser.newContext({viewport:{width:viewportWidth,height:980},reducedMotion:'reduce'});
    const queue=work.slice();
    const remote = mode === 'upstream' && !process.env.SAPID_MANTINE_BASE;
    await Promise.all(Array.from({length:remote ? 2 : 4},async()=>{
      while(queue.length) {
        if (await capture(context,queue.shift())) process.exitCode=1;
        if (remote) await new Promise(resolve=>setTimeout(resolve,1200));
      }
    }));
  } finally {await browser.close();}
})().catch(err=>{console.error(err);process.exitCode=1;});
