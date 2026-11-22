// Render every registered widget, including additions after the original audit.
const assert=require('node:assert/strict'),fs=require('node:fs'),path=require('node:path');
const {assertDocHeadingWeights,chromium,base,writeReport,showcaseSlugs}=require('./showcase-browser-env.cjs');
(async()=>{
 const slugs=showcaseSlugs();
 const original=JSON.parse(fs.readFileSync(path.join(__dirname,'../docs/shared-page-remediation.json'),'utf8'));
 for(const {slug} of original.pages)assert(slugs.includes(slug),slug);
 const browser=await chromium.launch(),page=await browser.newPage(),report={browser:browser.version(),cases:[],errors:[]};let current;
 page.on('pageerror',e=>report.errors.push({page:current,error:e.message}));page.on('console',m=>{if(m.type()==='error')report.errors.push({page:current,error:m.text()});});
 const visit=async url=>{for(let attempt=0;;attempt++){try{await page.goto(url);return;}catch(error){if(attempt>=2||!String(error).includes('ERR_NO_BUFFER_SPACE'))throw error;(report.navigationRetries??=[]).push({page:current,attempt,error:String(error)});await page.waitForTimeout(250*(attempt+1));}}};
 try{for(const [width,scheme]of [[1280,'light'],[390,'dark']])for(const slug of slugs){current=slug+'-'+width+'-'+scheme;await page.setViewportSize({width,height:1000});await visit(base+'/showcase.html?widget='+slug+'&theme=blue&scheme='+scheme);await page.waitForFunction(slug=>window.ready===true&&document.documentElement.getAttribute('data-sapid-widget')===slug,slug,{timeout:30000});await page.evaluate(()=>new Promise(resolve=>requestAnimationFrame(()=>requestAnimationFrame(resolve))));
  const state=await page.evaluate(()=>({asset:document.documentElement.innerHTML.match(/assets-[a-f0-9]{20}/)?.[0],overflow:document.documentElement.scrollWidth-innerWidth,preview:!!document.querySelector('.site-preview .site-demo')}));assert(state.asset);if(report.asset)assert.equal(state.asset,report.asset);report.asset=state.asset;assert(state.preview,current);await assertDocHeadingWeights(page);assert(state.overflow<=1,current+' page overflow '+state.overflow);report.cases.push({key:current,passed:true});console.log('PASS '+current);writeReport('all-pages-current.json',report);
 }assert.equal(report.cases.length,slugs.length*2);assert.deepEqual(report.errors,[]);}catch(error){report.failure=error.stack;throw error;}finally{writeReport('all-pages-current.json',report);await browser.close();}
})().catch(error=>{console.error(error);process.exitCode=1;});
