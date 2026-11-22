// Inspect the computed outline on the rendered part and the displayed CSS example.
const fs = require('node:fs');
const path = require('node:path');
const {base, chromium, outputPath} = require('./showcase-browser-env.cjs');
const {tick} = require('./showcase-panel-review.cjs');
const root = outputPath('example-review');
const requested = new Set(process.argv.slice(2));
async function outlined(preview, selector) {
  return preview.evaluate((el, expression) => expression.split(',').some(part => {
    const pseudo = part.match(/::(after|before)$/)?.[0];
    const query = part.replace(/::(after|before)$/,'').trim();
    return [...el.querySelectorAll(query)].some(node => {
      const rect=node.getBoundingClientRect();
      const css=getComputedStyle(node,pseudo);
      return rect.width>0 && rect.height>0 && css.outlineStyle==='solid' && css.outlineWidth==='2px' &&
        css.outlineColor==='rgb(254, 13, 69)';
    });
  }),selector);
}
(async()=>{
  const browser=await chromium.launch(), records=[];
  try{
    for(const slug of fs.readdirSync(path.join(root,'upstream'))) {
      if(requested.size&&!requested.has(slug))continue;
      const reference=JSON.parse(fs.readFileSync(path.join(root,'upstream',slug,'index.json'),'utf8'));
      const source=reference.examples.filter(e=>e.panelKind==='selectors');
      if(!source.length) continue;
      const page=await browser.newPage({viewport:{width:1336,height:980},reducedMotion:'reduce'});
      page.setDefaultTimeout(3000);
      try{
        await page.goto(base+'/showcase.html?widget='+slug,{timeout:45000});
        await page.waitForFunction(()=>window.ready===true,undefined,{timeout:45000});
        const explorers=page.locator('.site-styles-columns');
        for(const [index,ref] of source.entries()){
          const columns=explorers.nth(index), preview=columns.locator('.site-styles-preview'), rows=columns.locator('.site-styles-parts button');
          const code=columns.locator('..').locator('.site-styles-css');
          const record={slug,index:ref.index+1,expected:ref.selectors.map(s=>s.name),controls:[]};
          records.push(record);
          for(const part of ref.selectors) {
            const result={name:part.name};record.controls.push(result);
            try{
              const button=rows.filter({has:page.locator('.site-style-part-name',{hasText:new RegExp('^'+part.name+'$')})}).first();
              await button.hover();await tick(page);
              const css=(await code.innerText()).split('{')[0].trim();
              result.selector=css; result.hover=await outlined(preview,css);
              await page.mouse.move(0,0);await button.focus();await tick(page);
              result.focus=await outlined(preview,css);
              await button.blur();
            }catch(e){result.error=e.message.split('\n')[0];}
          }
          console.log(slug,record.controls.filter(c=>c.hover&&c.focus).length+'/'+record.expected.length);
        }
      }catch(e){records.push({slug,error:e.message.split('\n')[0]});}
      await page.close();
    }
  }finally{await browser.close();}
  const suffix=requested.size?'-'+[...requested].join('-'):'';
  fs.writeFileSync(path.join(root,'selector-checks'+suffix+'.json'),JSON.stringify({base,records},null,1));
})().catch(e=>{console.error(e);process.exitCode=1;});
