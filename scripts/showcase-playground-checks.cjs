const assert = require('node:assert/strict');
const { assertPlaygroundLayout } = require('./showcase-browser-env.cjs');
const box = (locator) => locator.boundingBox();
const near = (a, b) => assert(Math.abs(a - b) < 1, `${a} != ${b}`);

async function checkAccordion({ page, primary, width, tick, key, detail }) {
  await assertPlaygroundLayout(primary, width);
  const trigger = primary.getByRole('button', {
    name: 'Apples',
    exact: true,
  });
  const triggerBox = await trigger.boundingBox();
  const rootBox = await primary.locator('.sapid-accordion').boundingBox();
  assert(triggerBox.width >= rootBox.width - 2, `${key}: full-width heading`);
  assert.equal(await primary.locator('.sapid-disclosure-chevron').count(), 3);
  await trigger.click();
  await tick();
  assert.equal(await trigger.getAttribute('aria-expanded'), 'false');
  await trigger.click();
  await tick();
  assert.equal(await trigger.getAttribute('aria-expanded'), 'true');
  await trigger.focus();
  await page.keyboard.press('End');
  await tick();
  assert(
    await primary
      .getByRole('button', { name: 'Broccoli', exact: true })
      .evaluate((element) => element === document.activeElement),
  );
  const returns = primary.getByRole('button', { name: 'Bananas' });
  await returns.click();
  await tick();
  const arrow = returns.locator('.sapid-disclosure-chevron');
  const beforeMove = await arrow.boundingBox();
  await primary.getByRole('radio', { name: 'Left', exact: true }).check();
  await tick();
  assert((await arrow.boundingBox()).x < beforeMove.x - 30);
  await primary.getByRole('slider', { name: 'Chevron icon size' }).press('End');
  await tick();
  assert(Math.abs((await arrow.boundingBox()).width - 25) < 1 / 32);
  const variant = primary.getByRole('combobox', { name: 'Variant' });
  for (const choice of ['unstyled', 'filled', 'separated', 'contained']) {
    await variant.selectOption(choice);
    await tick();
    assert.equal(await returns.getAttribute('aria-expanded'), 'true');
  }
  await primary.getByRole('slider', { name: 'Radius', exact: true }).press('End');
  await tick();
  assert.equal(
    await primary
      .locator('.sapid-accordion-item')
      .first()
      .evaluate((e) => getComputedStyle(e).borderTopLeftRadius),
    '12px',
  );
  const path = arrow.locator('path').first(),
    expandedPath = await path.getAttribute('d');
  await primary.getByRole('switch', { name: 'Disable chevron rotation' }).check();
  await tick();
  const fixedPath = await path.getAttribute('d');
  assert.notEqual(fixedPath, expandedPath);
  await returns.click();
  await tick();
  assert.equal(await path.getAttribute('d'), fixedPath);
  if (!detail) return;
  const disabled = detail('Disabled items');
  assert(await disabled.getByRole('button', { name: 'Bananas', exact: true }).isDisabled());
  await disabled.getByRole('button', { name: 'Broccoli', exact: true }).click();
  await tick();
  assert.equal(await disabled.getByRole('button', { name: 'Broccoli', exact: true }).getAttribute('aria-expanded'), 'true');
  assert.equal(await disabled.getByRole('button', { name: 'Apples', exact: true }).getAttribute('aria-expanded'), 'false');
  const lifetime = detail('Retain or retire panel content');
  const retained = lifetime.getByRole('textbox', { name: 'Retained draft', exact: true });
  await retained.fill('Retained note');
  await retained.evaluate((element) => {
    window.retainedAccordion = element;
  });
  await lifetime.getByRole('button', { name: 'Retained draft', exact: true }).click();
  await tick();
  assert.equal(await retained.isVisible(), false);
  await lifetime.getByRole('button', { name: 'Retained draft', exact: true }).click();
  await tick();
  assert.equal(await retained.inputValue(), 'Retained note');
  assert(await retained.evaluate((element) => element === window.retainedAccordion));
  const unmounted = lifetime.getByRole('textbox', { name: 'Unmounted draft', exact: true });
  await unmounted.fill('Discarded note');
  await lifetime.getByRole('button', { name: 'Unmounted draft', exact: true }).click();
  await tick();
  assert.equal(await unmounted.count(), 0);
  await lifetime.getByRole('button', { name: 'Unmounted draft', exact: true }).click();
  await tick();
  assert.equal(await unmounted.inputValue(), '');
  const dynamic = detail('Keyed sections');
  const shipping = dynamic.getByRole('button', { name: 'shipping', exact: true });
  await shipping.evaluate((element) => {
    window.keyedAccordion = element;
  });
  await dynamic.getByRole('button', { name: 'Add account section', exact: true }).click();
  await tick();
  assert(await dynamic.getByRole('button', { name: 'account', exact: true }).isVisible());
  assert(await shipping.evaluate((element) => element === window.keyedAccordion));
}

async function checkStack({ primary, width, tick, detail }) {
  await assertPlaygroundLayout(primary, width);
  const preview = primary.locator('.site-stack-example'),
    cards = preview.getByRole('button');
  const alignment = primary.getByRole('combobox', { name: 'Align' });
  const distribution = primary.getByRole('combobox', { name: 'Justify' });
  const stretch = await box(cards.first());
  await alignment.selectOption('end');
  await tick();
  const right = await box(cards.first());
  assert(right.x > stretch.x + 10 && right.width < stretch.width);
  await distribution.selectOption('start');
  await tick();
  const top = await box(cards.first());
  await distribution.selectOption('end');
  await tick();
  assert((await box(cards.first())).y > top.y + 10);
  await distribution.selectOption('start');
  await tick();
  const spacing = async () =>
    (await box(cards.nth(1))).y - (await box(cards.first())).y - (await box(cards.first())).height;
  const initialGap = await spacing();
  await primary.getByRole('slider', { name: 'Gap' }).press('End');
  await tick();
  assert((await spacing()) > initialGap);
  if (!detail) return;
  const section = detail('Alignment, distribution and long content'),
    stack = section.locator('.docs-stack-distribution');
  await stack.evaluate((e) => {
    window.stackElement = e;
  });
  await section
    .getByRole('checkbox', { name: 'Distribute through the available height', exact: true })
    .check();
  await tick();
  const rect = await box(stack),
    last = await box(stack.getByText('Deliver', { exact: true }));
  near(rect.height, 240);
  near(last.y + last.height, rect.y + rect.height);
  near((await box(stack.getByText('Deliver', { exact: true }))).x, rect.x);
  assert.equal(await stack.evaluate((e) => getComputedStyle(e).justifyContent), 'space-between');
  assert(await stack.evaluate((e) => e === window.stackElement));
}

async function checkBadge({ primary, width, tick, detail }) {
  await assertPlaygroundLayout(primary, width);
  const badge = primary.locator('.site-playground-preview .sapid-badge');
  const variant = primary.getByRole('combobox', { name: 'Variant' });
  const css = (prop) => badge.evaluate((el, p) => getComputedStyle(el)[p], prop);
  const colorEditor = primary.getByRole('textbox', { name: 'Custom color' });
  await primary.getByRole('button', { name: 'Custom color', exact: true }).click();
  await colorEditor.click();
  const palette = primary.getByRole('dialog', { name: 'Choose color' });
  await palette.waitFor();
  const hue = palette.getByRole('slider', { name: 'Hue', exact: true });
  await hue.scrollIntoViewIfNeeded();
  const track = await hue.boundingBox();
  const thumb = await hue.locator('.sapid-slider-thumb').boundingBox();
  const mouse = primary.page().mouse;
  await mouse.move(thumb.x + thumb.width / 2, thumb.y + thumb.height / 2);
  await mouse.down();
  try {
    for (const fraction of [0.25, 0.65, 0.85]) {
      await mouse.move(track.x + track.width * fraction, track.y + track.height / 2, { steps: 5 });
      await tick();
      assert(Math.abs(Number(await hue.getAttribute('aria-valuenow')) - fraction * 360) < 3);
    }
  } finally {
    await mouse.up();
  }
  await hue.press('Escape');
  await palette.waitFor({ state: 'hidden' });
  await colorEditor.fill('#663399');
  await tick();
  assert.equal(await css('backgroundColor'), 'rgb(102, 51, 153)');
  const initialSize = await css('fontSize');
  await primary.getByRole('slider', { name: 'Size', exact: true }).press('End');
  await tick();
  assert(parseFloat(await css('fontSize')) > parseFloat(initialSize));
  await primary.getByRole('slider', { name: 'Radius', exact: true }).press('Home');
  await tick();
  assert.equal(await css('borderTopLeftRadius'), '0px');
  for (const [option, suffix] of [
    ['light', 'soft'],
    ['outline', 'outline'],
    ['dot', 'dot'],
    ['transparent', 'transparent'],
    ['default', 'default'],
    ['white', 'white'],
    ['gradient', 'gradient'],
  ]) {
    await variant.selectOption(option);
    await tick();
    assert(await badge.evaluate((el, s) => el.classList.contains('sapid-badge-' + s), suffix));
    if (option === 'dot') assert.equal(await badge.locator('svg').count(), 1);
  }
  await primary.getByRole('button', { name: 'Gradient from: use Teal' }).click();
  await primary.getByRole('button', { name: 'Gradient to: use Orange' }).click();
  await primary.getByRole('slider', { name: 'Gradient degree' }).press('End');
  await tick();
  assert.match(await css('backgroundImage'), /360deg.*18, 184, 134.*253, 126, 20/);
  await variant.selectOption('filled');
  await tick();
  assert.equal(await css('backgroundColor'), 'rgb(102, 51, 153)');
  if (!detail) return;
  assert.equal(await detail('badge-variants').locator('.sapid-badge').count(), 4);
  assert.equal(await detail('badge-sizes').locator('.sapid-badge').count(), 6);
  const sections = detail('badge-sections'),
    label = sections.locator('.sapid-badge-content'),
    icon = sections.locator('svg');
  await label.evaluate((e) => (window.badgeLabel = e));
  await icon.evaluate((e) => (window.badgeIcon = e));
  await sections.getByRole('checkbox', { name: 'Ready to publish', exact: true }).check();
  await tick();
  assert.equal(await label.textContent(), 'Ready to publish');
  assert(await label.evaluate((e) => e === window.badgeLabel));
  assert(await icon.evaluate((e) => e === window.badgeIcon));
  const actions = detail('badge-actions');
  await actions.getByRole('button', { name: 'Review comments', exact: true }).click();
  await tick();
  assert(await actions.getByText('Comments marked as reviewed.', { exact: true }).isVisible());
}

module.exports = { checkAccordion, checkBadge, checkStack };
