const assert=require('node:assert/strict'),fs=require('node:fs');
const {assertDocHeadingWeights,matrix,chromium,base,outputPath,writeReport}=require('./showcase-browser-env.cjs');
const chartPages=['area-chart','bar-chart','line-chart','composite-chart'];
const plots=['candlestick-chart','donut-chart','pie-chart','funnel-chart','radar-chart','scatter-chart','bubble-chart','radial-bar-chart','sparkline','bars-list','bullet-chart','gauge-chart','waffle-chart','treemap','sunburst-chart','sankey-chart','matrix-chart'];
(async()=>{
 const browser=await chromium.launch(),page=await browser.newPage();page.setDefaultTimeout(7000);
 const report={browser:browser.version(),pages:[],errors:[]};page.on('pageerror',e=>report.errors.push(e.message));page.on('console',m=>{if(m.type()==='error')report.errors.push(m.text());});
 fs.mkdirSync(outputPath('charts-remediation'),{recursive:true});
 const tick=()=>page.evaluate(()=>new Promise(resolve=>{let n=0;const f=()=>++n===5?resolve():requestAnimationFrame(f);requestAnimationFrame(f);}));
 const click=async l=>{await l.click();await tick();};
 const rect=async l=>{const r=await l.boundingBox();assert(r);return r;};
 const section=(slug,part)=>page.locator('.site-component-detail').filter({has:page.locator(`#chart-${slug}-${part}`)}).locator('.site-demo').first();
 const remember=async(l,k)=>l.evaluate((e,k)=>window[k]=e,k),retained=async(l,k)=>assert(await l.evaluate((e,k)=>e===window[k],k));
 try{
 for(const theme of (process.env.SAPID_CHART_THEMES||matrix.themes.join(',')).split(','))for(const scheme of (process.env.SAPID_CHART_SCHEMES||matrix.schemes.join(',')).split(','))for(const width of (process.env.SAPID_CHART_WIDTHS||'1280,390').split(',').map(Number))for(const slug of (process.env.SAPID_CHART_PAGES||[...chartPages,...plots].join(',')).split(',')){
 const key=`${slug}-${theme}-${scheme}-${width}`;
 try{
 await page.setViewportSize({width,height:1100});await page.goto(`${base}/showcase.html?widget=${slug}&theme=${theme}&scheme=${scheme}`);await page.waitForFunction(()=>window.ready===true,undefined,{timeout:30000});await tick();
 const asset=await page.evaluate(()=>document.documentElement.innerHTML.match(/assets-[a-f0-9]{20}/)?.[0]);assert(asset);if(report.asset)assert.equal(asset,report.asset);report.asset=asset;
 await assertDocHeadingWeights(page);assert(await page.evaluate(()=>document.documentElement.scrollWidth<=innerWidth+1));
 const cartesian=chartPages.includes(slug),primary=page.locator('.site-preview .site-demo').first(),frameSelector=cartesian?'.sapid-chart-plot':'.sapid-plot-drawing',pointSelector=cartesian?'.sapid-chart-point':'.sapid-plot-point',legendSelector=cartesian?'.sapid-chart-legend-item':'.sapid-plot-legend-item',detailSelector=cartesian?'.sapid-chart-point-detail':'.sapid-plot-detail';
 const expected=({'area-chart':32,'bar-chart':26,'line-chart':27,'composite-chart':27,'donut-chart':15,'pie-chart':13,'funnel-chart':12,'radar-chart':6,'radial-bar-chart':7,'bubble-chart':6,'sunburst-chart':10,'waffle-chart':8,'sankey-chart':9,'treemap':8,'scatter-chart':5,'matrix-chart':10,'bars-list':5,'bullet-chart':10,'gauge-chart':10})[slug] || 4;assert.equal(await page.locator('.site-component-detail').count(),expected);assert.equal(await primary.locator('.site-chart-card').count(),0);
 const frame=primary.locator(frameSelector),b=await rect(frame),container=await rect(primary);assert(b.width<=container.width+1&&b.width>=(['sparkline','donut-chart','pie-chart','gauge-chart'].includes(slug)?145:220));if(slug==='bullet-chart'){assert(Math.abs(b.height-b.width*120/800)<2)}else{assert(b.height>=(slug==='sparkline'?35:65))};
 await remember(frame,'primaryChartFrame');
 if(slug==='sparkline'){
  assert(b.width<=321&&b.height<=81);assert.equal(await primary.locator(legendSelector).count(),0);assert.equal(await primary.getByRole('button',{name:'Next data point',exact:true}).count(),0);assert(await primary.getByLabel('Weekly values: 4, 7, 5, 9, 8, 12, 15',{exact:true}).count());
 }else if(slug==='candlestick-chart'){
  const labels=primary.locator('.sapid-plot-label');
  const initial=await labels.count();assert(initial>0);
  await primary.getByLabel('With xaxis',{exact:true}).uncheck();await tick();
  assert((await labels.count())<initial);
  await primary.getByLabel('With xaxis',{exact:true}).check();await tick();
  assert.equal(await labels.count(),initial);
 }else if(cartesian){
  const preview = primary.locator('.site-playground-preview');
  const bounds = await rect(preview);
  assert(b.width >= bounds.width - 75 && b.width <= bounds.width);
  const panel = primary.locator('.site-playground-controls');
  const paths = () => frame.locator('path').evaluateAll(nodes => nodes.map(node => node.getAttribute('d')).join(' '));
  if(slug!=='bar-chart'){
   const lines=await paths();
   await panel.getByLabel('Curve type',{exact:true}).selectOption('step');await tick();
   assert.notEqual(await paths(),lines);
  }
  for (const axis of ['x','y']){
   const control=panel.getByLabel('With '+axis+'axis',{exact:true});
   const before=await paths();await control.uncheck();await tick();assert.notEqual(await paths(),before);
   await control.check();await tick();
  }
  const dataSection=section(slug,'series');
  assert(await dataSection.getByRole('table').count()>0);
 }else{
  const detailed=section(slug,'data');
  assert(await detailed.getByRole('table').count()>0);assert(await primary.locator(pointSelector).count()>0);
  const next=detailed.getByRole('button',{name:'Next data point',exact:true});assert((await rect(next)).height<=31);await click(next);assert.notEqual((await detailed.locator(detailSelector).textContent()).trim(),'No data point selected');
  await click(detailed.getByRole('button',{name:'Clear data point',exact:true}));assert.equal((await detailed.locator(detailSelector).textContent()).trim(),'No data point selected');
  const legend=detailed.locator(legendSelector);if(await legend.count()){
   const item=legend.first();await remember(item,'primaryLegend');assert.equal(await item.getAttribute('aria-pressed'),'true');assert((await rect(item)).height<=31);const name=await item.getAttribute('aria-label')||await item.textContent();assert(name.includes('Hide'));await click(item);assert.equal(await item.getAttribute('aria-pressed'),'false');assert((await item.textContent()).includes('Show'));await retained(item,'primaryLegend');await click(item);assert.equal(await item.getAttribute('aria-pressed'),'true');
  }
 }
 const controlled=section(slug,'controlled'),readout=controlled.locator(detailSelector);await click(controlled.getByRole('button',{name:'Next data point',exact:true}));assert.equal((await readout.textContent()).trim(),'No data point selected');await controlled.getByLabel('Accept chart changes',{exact:true}).check();await tick();await click(controlled.getByRole('button',{name:'Next data point',exact:true}));assert.notEqual((await readout.textContent()).trim(),'No data point selected');
 const recovery=section(slug,'recovery'),recoveryFrame=recovery.locator(frameSelector);await remember(recoveryFrame,'recoveryFrame');const pointCount=await recovery.locator(pointSelector).count(),tables=await recovery.getByRole('table').allTextContents();await click(recovery.getByRole('button',{name:'Reject invalid update',exact:true}));assert.equal(await recovery.locator(pointSelector).count(),pointCount);assert.deepEqual(await recovery.getByRole('table').allTextContents(),tables);assert((await recovery.textContent()).includes('Rejected update:'));await click(recovery.getByRole('button',{name:'Show empty data',exact:true}));assert.equal(await recovery.locator(pointSelector).count(),0);assert((await recovery.textContent()).includes(cartesian?'No visible chart data':'No visible plot data'));await click(recovery.getByRole('button',{name:'Restore chart data',exact:true}));assert.equal(await recovery.locator(pointSelector).count(),pointCount);await retained(recoveryFrame,'recoveryFrame');
 if(cartesian){
  const window=section(slug,'window'),before=await window.locator(pointSelector).count(),rows=await window.getByRole('row').count();await window.getByLabel('Show March through May',{exact:true}).check();await tick();assert(await window.locator(pointSelector).count()<before);assert.equal(await window.getByRole('row').count(),rows);assert((await window.locator('.sapid-chart-clipping').textContent()).includes('clipped'));
  const missing=section(slug,'series');assert((await missing.getByRole('table').textContent()).includes('No value'));assert((await missing.getByRole('table').textContent()).includes('0'));
 }else{
  const data=section(slug,'data');await click(data.getByRole('button',{name:'Next data point',exact:true}));const reading=await data.locator(detailSelector).textContent();
  if(slug==='candlestick-chart')for(const field of ['open','high','low','close'])assert(reading.includes(field));
  if(slug==='scatter-chart'||slug==='bubble-chart'){for(const field of ['x=','y=','size='])assert(reading.includes(field));assert.equal(await primary.locator('.sapid-plot-label').count(),12);}
  if(slug==='treemap'||slug==='sunburst-chart'){await click(data.getByRole('button',{name:'First child',exact:true}));assert.notEqual(await data.locator(detailSelector).textContent(),reading);await click(data.getByRole('button',{name:'Parent node',exact:true}));assert.equal(await data.locator(detailSelector).textContent(),reading);}
  const presentation=section(slug,'presentation');
  if(slug==='matrix-chart'){assert.deepEqual(await primary.locator('.sapid-plot-label').allTextContents(),['James','Mary','Robert','Linda','Michael','Sarah','David','Emma']);assert.equal(await primary.locator('.sapid-plot-layer path').count(),8*31);assert.equal(await primary.locator('.sapid-plot-scale-legend').innerText(),'');}
  if(slug==='donut-chart'){assert.equal(await primary.locator('.sapid-plot-point').count(),4);const center=presentation.getByText('Completion',{exact:true}),c=await rect(center),f=await rect(presentation.locator(frameSelector));assert(Math.abs(c.x+c.width/2-f.x-f.width/2)<2);assert(c.width<f.width/2);}
  if(slug==='waffle-chart'){const cells=async scope=>scope.locator('.sapid-plot-layer path').evaluateAll(es=>es.reduce((sum,e)=>sum+(e.getAttribute('d').match(/M/g)||[]).length,0));assert.equal(await cells(primary),100);assert.equal(await cells(presentation),20);await click(presentation.getByRole('button',{name:'Next data point',exact:true}));assert((await presentation.locator(detailSelector).textContent()).includes('of 20 cells'));}
  if(slug==='bubble-chart'){const maximum=await presentation.locator('.sapid-plot-layer path').evaluateAll(es=>Math.max(...es.map(e=>e.getBBox()).filter(b=>Math.abs(b.width-b.height)<0.01).map(b=>b.width)));assert(Math.abs(maximum-64)<0.1);}
 }
 await page.setViewportSize({width:width===390?1280:390,height:1100});await tick();await retained(frame,'primaryChartFrame');await page.setViewportSize({width,height:1100});await tick();await primary.scrollIntoViewIfNeeded();await primary.screenshot({path:outputPath(`charts-remediation/${key}.png`)});assert(await page.evaluate(()=>document.documentElement.scrollWidth<=innerWidth+1));
 report.pages.push({key,passed:true});console.log(`PASS ${key}`);
 }catch(error){report.pages.push({key,passed:false,failure:error.stack});console.error(`FAIL ${key}: ${error.stack}`);await page.screenshot({path:outputPath(`charts-remediation/failure-${key}.png`)});}
 writeReport('charts-remediation.json',report);
 }
 assert.equal(report.pages.filter(p=>!p.passed).length,0);assert.deepEqual(report.errors,[]);
 }finally{writeReport('charts-remediation.json',report);await browser.close();}
})().catch(e=>{console.error(e);process.exitCode=1;});
