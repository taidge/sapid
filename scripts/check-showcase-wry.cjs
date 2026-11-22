// Windows acceptance against the actual WebView2 that hosts the native Rust App.
const assert = require('node:assert/strict');
const { execFileSync, spawn } = require('node:child_process');
const fs = require('node:fs');
const net = require('node:net');
const path = require('node:path');
const { setTimeout: delay } = require('node:timers/promises');
const { buildRoot, chromium, outputPath } = require('./showcase-browser-env.cjs');
const { checkAccordion, checkBadge, checkStack } = require('./showcase-playground-checks.cjs');

const executable = process.env.SAPID_WRY_EXE || path.join(buildRoot, 'debug', 'showcase_wry.exe');
const checks = { accordion: checkAccordion, badge: checkBadge, stack: checkStack };

async function localPort() {
  const server = net.createServer();
  await new Promise((resolve, reject) =>
    server.once('error', reject).listen(0, '127.0.0.1', resolve),
  );
  const port = server.address().port;
  await new Promise((resolve) => server.close(resolve));
  return port;
}

// Exercise a local control when the example supplies one. The result records
// the observed state separately, since controlled widgets can reject a change.
async function exercise(primary, tick) {
  const selector = [
    'input:not([type=hidden]):not([type=file]):not([readonly])',
    'select',
    'textarea:not([readonly])',
    '[role=slider]',
    '[contenteditable=true]',
    'button',
  ].join(',');
  const controls = primary.locator(selector);
  for (let index = 0; index < (await controls.count()); index++) {
    const item = controls.nth(index);
    if (!(await item.isVisible()) || !(await item.isEnabled())) continue;
    const info = await item.evaluate((el) => ({
      tag: el.tagName,
      role: el.getAttribute('role'),
      type: el.type,
      label:
        el.getAttribute('aria-label') ||
        el.labels?.[0]?.textContent ||
        el.textContent.trim().slice(0, 80),
      editable: el.isContentEditable,
      external:
        el.matches('[aria-readonly=true], [data-gly-file], [data-gly-form-action=submit]') ||
        !!el.closest('[data-gly-file-mode]') ||
        /choose.*file|upload|browse.*file|pick.*screen|choose.*image|attach|clipboard|copy |download/i.test(
          el.getAttribute('aria-label') || el.textContent,
        ),
    }));
    if (info.external || (!info.label && info.tag === 'BUTTON')) continue;
    const snapshot = () =>
      primary.evaluate((el) =>
        JSON.stringify([
          el.innerText,
          ...[
            ...el.querySelectorAll(
              '[aria-valuenow],[aria-selected],[aria-expanded],input,select,textarea,[aria-pressed]',
            ),
          ].map((node) => [
            node.value,
            node.checked,
            node.getAttribute('aria-valuenow'),
            node.getAttribute('aria-expanded'),
            node.getAttribute('aria-selected'),
            node.getAttribute('aria-pressed'),
          ]),
        ]),
      );
    const before = await snapshot();
    let action;
    if (info.tag === 'SELECT') {
      const options = await item
        .locator('option')
        .evaluateAll((els) =>
          els
            .filter((el) => !el.disabled && !el.selected && el.hasAttribute('data-gly-option'))
            .map((el) => el.value),
        );
      if (!options.length) continue;
      await item.selectOption(options[0]);
      action = 'select';
    } else if (info.role === 'slider') {
      await item.press('End');
      action = 'range';
    } else if (
      info.editable ||
      info.tag === 'TEXTAREA' ||
      (info.tag === 'INPUT' &&
        ['text', 'search', 'tel', 'password', 'email', 'url', 'number'].includes(info.type))
    ) {
      await item.focus();
      await item.press(info.type === 'number' ? '3' : 'A');
      action = 'edit';
    } else if (info.tag === 'BUTTON' || ['radio', 'checkbox'].includes(info.type)) {
      await item.click({ timeout: 3000 });
      action = 'click';
    } else continue;
    await tick();
    return { action, control: info.label, observedChange: before !== (await snapshot()) };
  }
  return { action: 'display' };
}

async function inspect(slug, usageOnly, width, profile) {
  const port = await localPort();
  const endpoint = 'http://127.0.0.1:' + port;
  const child = spawn(executable, [...(usageOnly ? ['--usage'] : []), slug], {
    windowsHide: true,
    env: {
      ...process.env,
      WEBVIEW2_USER_DATA_FOLDER: profile,
      WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS:
        '--remote-debugging-address=127.0.0.1 --remote-debugging-port=' + port,
    },
  });
  const exited = new Promise((resolve) => child.once('exit', resolve));
  let diagnostics = '',
    browser;
  child.stderr.on('data', (chunk) => {
    diagnostics = (diagnostics + chunk).slice(-8192);
  });
  try {
    const ready = async () => {
      const deadline = Date.now() + 60000;
      while (Date.now() < deadline) {
        if (child.exitCode !== null) throw Error('Native application exited ' + child.exitCode);
        try {
          if ((await fetch(endpoint + '/json/version')).ok) return;
        } catch {}
        await delay(100);
      }
      throw Error('Timed out waiting for WebView2');
    };
    await ready();
    browser = await chromium.connectOverCDP(endpoint);
    const context = browser.contexts()[0];
    const page = context.pages()[0] || (await context.waitForEvent('page'));
    page.setDefaultTimeout(15000);
    const errors = [];
    page.on('pageerror', (error) => errors.push(error.message));
    await page.setViewportSize({ width, height: 980 });
    const primary = page.locator('.site-preview .site-demo').first();
    await primary.waitFor();
    const tick = () =>
      page.evaluate(async () => {
        for (let i = 0; i < 8; i++) await new Promise(requestAnimationFrame);
      });
    await tick();
    const heading = page.getByRole('heading', { level: 1 }).first();
    const title = usageOnly ? slug : await heading.innerText();
    const examples = page.locator('.site-preview .site-demo');
    const boxes = await examples.evaluateAll((elements) =>
      elements.map((element) => {
        const box = element.getBoundingClientRect();
        return { width: box.width, height: box.height, descendants: element.childElementCount };
      }),
    );
    assert(
      boxes.length && boxes.every((box) => box.width > 0 && box.height > 0 && box.descendants > 0),
      'One or more showcase examples rendered empty',
    );
    let probe;
    if (checks[slug]) {
      const detail = usageOnly
        ? undefined
        : (name) =>
            page.locator('.site-component-detail').filter({
              has:
                slug === 'badge'
                  ? page.locator('#' + name)
                  : page.getByRole('heading', { name, exact: true }),
            });
      await checks[slug]({ page, primary, width, key: slug, tick, detail });
      probe = { action: 'asserted playground and detail interactions' };
    } else {
      probe = await exercise(primary, tick);
    }
    await primary.screenshot({
      path: outputPath('wry-previews/' + slug + '-' + width + '.png'),
      timeout: 10000,
    });
    if (!usageOnly && slug === 'stack' && width === 1280) {
      await page
        .locator('a.site-nav-link')
        .filter({ hasText: /^Badge$/ })
        .first()
        .click();
      await page.getByRole('heading', { level: 1, name: 'Badge', exact: true }).waitFor();
      await page.locator('.site-toc-link[href="#composition"]').click();
      await tick();
      assert(
        await page.locator('#composition').evaluate((el) => {
          const box = el.getBoundingClientRect();
          return box.bottom > 0 && box.top < innerHeight;
        }),
        'The fragment did not scroll its heading into view',
      );
      await page.getByRole('link', { name: 'Switch to dark theme' }).click();
      await page.locator('.sapid-site.site-dark').waitFor();
    }
    await tick();
    assert.deepEqual(errors, []);
    return { slug, title, width, examples: boxes.length, interactions: probe, passed: true };
  } catch (error) {
    // A bridge fault closes the window before Playwright receives stderr.
    await Promise.race([exited, delay(180)]);
    return { slug, width, passed: false, error: diagnostics.trim() || error.message };
  } finally {
    if (browser) await browser.close().catch(() => {});
    if (child.exitCode === null) child.kill();
    await exited;
  }
}

async function main() {
  if (process.platform !== 'win32') throw Error('This WebView2 acceptance driver requires Windows');
  const args = process.argv.slice(2);
  const usageOnly = args[0] === '--usage';
  if (usageOnly) args.shift();
  const known = execFileSync(executable, ['--list'], { encoding: 'utf8' }).trim().split(/\r?\n/);
  const slugs = args.length ? args : known;
  for (const slug of slugs) assert(known.includes(slug), 'Unknown widget: ' + slug);
  fs.mkdirSync(outputPath('wry-previews'), { recursive: true });
  const profile = fs.mkdtempSync(path.join(buildRoot, 'wry-profile-'));
  const report = outputPath('showcase-wry-' + (usageOnly ? 'usage' : 'pages') + '.jsonl');
  const output = fs.openSync(report, 'w');
  let failures = 0;
  try {
    for (const slug of slugs) {
      for (const width of checks[slug] ? [1280, 390] : [1280]) {
        const result = await inspect(slug, usageOnly, width, profile);
        fs.writeSync(output, JSON.stringify(result) + '\n');
        if (!result.passed) {
          failures++;
          console.error(slug, width, result.error);
        }
      }
    }
  } finally {
    fs.closeSync(output);
    try {
      await fs.promises.rm(profile, {
        recursive: true,
        force: true,
        maxRetries: 10,
        retryDelay: 350,
      });
    } catch (error) {
      if (!['EPERM', 'EBUSY', 'ENOTEMPTY'].includes(error.code)) throw error;
      console.warn('WebView2 still holds its temporary test profile at', profile);
    }
  }
  console.log(slugs.length + ' widgets checked, ' + failures + ' failed, ' + report);
  if (failures) process.exitCode = 1;
}
main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
