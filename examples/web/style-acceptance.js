async (hostPage) => {
  const context = await hostPage.context().browser().newContext();
  const reports = [], errors = [];
  const assert = (value, message) => { if (!value) throw new Error(message); };
  try {
    const page = await context.newPage();
    page.on('pageerror', error => errors.push(error.message));
    const response = await page.request.get(`${process.env.SAPID_STYLE_BASE || 'http://127.0.0.1:8794'}/sapid.css`);
    assert(response.ok(), 'shared Sapid stylesheet was not served');
    const css = await response.text();
    // Test-host markup deliberately excludes showcase.css, whose global button
    // cursor/focus rules would conceal a regression in the library stylesheet.
    await page.setContent('<!doctype html><html><head></head><body><main id="fixture"></main></body></html>');
    await page.addStyleTag({ content: css });
    for (const hue of ['blue', 'green']) for (const scheme of ['light', 'dark']) {
      const mode = `${hue}-${scheme}`;
      await page.locator('#fixture').evaluate((root, mode) => {
        root.className = `sapid-theme-${mode}`;
        root.innerHTML = '<button id="before">Before</button><button id="focused" class="sapid-button">Keyboard target</button>';
        root.style.padding = '12px';
        root.style.background = 'var(--sp-surface)';
      }, mode);
      await page.locator('#before').focus();
      await page.keyboard.press('Tab');
      const focus = await page.locator('#focused').evaluate(element => {
        const style = getComputedStyle(element);
        const rgb = value => {
          const probe = document.createElement('span');
          probe.style.color = value;
          element.append(probe);
          const channels = getComputedStyle(probe).color.match(/[\d.]+/g).slice(0, 3).map(Number);
          probe.remove();
          return channels;
        };
        const luminance = channels => channels.map(channel => {
          const value = channel / 255;
          return value <= 0.04045 ? value / 12.92 : ((value + 0.055) / 1.055) ** 2.4;
        }).reduce((sum, value, index) => sum + value * [0.2126, 0.7152, 0.0722][index], 0);
        const ring = luminance(rgb(style.outlineColor));
        const contrasts = ['canvas', 'surface', 'input'].map(token => {
          const background = luminance(rgb(style.getPropertyValue(`--sp-${token}`)));
          return (Math.max(ring, background) + 0.05) / (Math.min(ring, background) + 0.05);
        });
        return { active: document.activeElement === element, visible: element.matches(':focus-visible'),
          outline: style.outlineStyle, width: style.outlineWidth, contrasts };
      });
      assert(focus.active && focus.visible && focus.outline === 'solid' && focus.width === '2px',
        `${mode}: real Tab did not show the shared focus ring: ${JSON.stringify(focus)}`);
      assert(focus.contrasts.every(value => value >= 3), `${mode}: focus contrast below 3:1: ${focus.contrasts}`);
      await page.locator('#fixture').screenshot({ path: `${process.env.SAPID_STYLE_OUTPUT || 'target/verification/sapid'}/focus-${mode}.png` });
      reports.push({ suite: `style-focus-${mode}`, status: 'passed', contrasts: focus.contrasts });
      for (const inputClass of ['sapid-text-field', 'sapid-combobox', 'sapid-tags-input', 'sapid-pills-input']) {
        await page.locator('#fixture').evaluate((root, inputClass) => {
          root.innerHTML = '<button id="before">Before</button>';
          const input = document.createElement('input');
          input.id = 'editor'; input.className = inputClass;
          input.style.cssText = 'border:1px solid var(--sp-border);background:var(--sp-input)';
          root.append(input);
        }, inputClass);
        const editor = page.locator('#editor');
        for (const invalid of [false, true]) {
          await page.locator('#before').focus();
          await editor.evaluate((node, invalid) => {
            node.toggleAttribute('data-gly-invalid', invalid);
            node.style.borderColor = invalid ? 'var(--sp-danger)' : 'var(--sp-border)';
            node.style.color = invalid ? 'var(--sp-danger)' : 'var(--sp-input-focus)';
          }, invalid);
          await page.keyboard.press('Tab');
          const focused = await editor.evaluate(async node => {
            await Promise.all(node.getAnimations().map(animation => animation.finished));
            const style = getComputedStyle(node);
            return node.matches(':focus-visible') && style.outlineStyle === 'none'
              && style.borderColor === style.color && style.borderTopWidth === '1px'
              && style.boxShadow.includes(style.borderColor) && style.boxShadow.includes('0px 0px 0px 1px inset');
          });
          assert(focused, `${mode}/${inputClass}: expected a single thickened ${invalid ? 'error' : 'focus'} border`);
        }
        reports.push({ suite: `style-input-focus-${inputClass}-${mode}`, status: 'passed' });
      }
    }
    const triggerClasses = ['sapid-accordion-trigger', 'sapid-tab', 'sapid-collapsible-trigger',
      'sapid-dialog-trigger', 'sapid-alert-dialog-trigger', 'sapid-floating-dialog-trigger', 'sapid-drawer-trigger', 'sapid-popover-trigger',
      'sapid-command-trigger', 'sapid-hint-trigger', 'sapid-menu-trigger', 'sapid-menu-submenu-trigger',
      'sapid-navigation-menu-trigger', 'sapid-sidebar-trigger', 'sapid-spoiler-trigger', 'sapid-stepper-trigger'];
    const isolation = await page.locator('#fixture').evaluate((root, classes) => {
      root.replaceChildren();
      const cursor = (className, disabled = false, unstyledParent = false, ariaDisabled = false, parentClass = '') => {
        const wrapper = document.createElement('div');
        wrapper.className = `${parentClass} ${unstyledParent ? 'sapid-unstyled' : ''}`;
        const element = document.createElement('button');
        element.className = className;
        element.disabled = disabled;
        if (ariaDisabled) element.setAttribute('aria-disabled', 'true');
        wrapper.append(element); root.append(wrapper);
        return getComputedStyle(element).cursor;
      };
      const foreign = document.createElement('div');
      foreign.className = 'application-menu-trigger'; root.append(foreign);
      return { foreign: getComputedStyle(foreign).cursor, native: cursor(''), nativeDisabled: cursor('', true),
        triggers: classes.map(name => ({ name,
        enabled: cursor(name), disabled: cursor(name, true), ownUnstyled: cursor(`${name} sapid-unstyled`),
        parentUnstyled: cursor(name, false, true), ariaDisabled: cursor(name, false, false, true) })),
        menubar: { enabled: cursor('sapid-menu-trigger', false, false, false, 'sapid-menubar'),
          disabled: cursor('sapid-menu-trigger', true, false, false, 'sapid-menubar'),
          unstyled: cursor('sapid-menu-trigger', false, true, false, 'sapid-menubar'),
          ariaDisabled: cursor('sapid-menu-trigger', false, false, true, 'sapid-menubar') } };
    }, triggerClasses);
    assert(isolation.foreign === 'auto', `Sapid styled a host application's trigger: ${JSON.stringify(isolation)}`);
    for (const trigger of isolation.triggers) {
      assert(trigger.enabled === 'pointer' && trigger.disabled === isolation.nativeDisabled
        && trigger.ownUnstyled === isolation.native && trigger.parentUnstyled === isolation.native
        && trigger.ariaDisabled === isolation.native,
        `trigger cursor ignored disabled/unstyled: ${JSON.stringify(trigger)}`);
    }
    assert(isolation.menubar.enabled === 'pointer'
      && isolation.menubar.disabled === isolation.nativeDisabled
      && isolation.menubar.unstyled === isolation.native
      && isolation.menubar.ariaDisabled === isolation.native,
      `menubar descendant cursor ignored disabled/unstyled: ${JSON.stringify(isolation.menubar)}`);
    reports.push({ suite: 'style-trigger-isolation', status: 'passed' });
    assert(errors.length === 0, JSON.stringify(errors));
    return { reports, errors };
  } finally {
    await context.close();
  }
}
