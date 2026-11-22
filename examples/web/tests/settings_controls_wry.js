document.addEventListener("DOMContentLoaded", async () => {
  const frame = () => new Promise(requestAnimationFrame);
  async function waitFor(predicate) {
    const deadline = performance.now() + 3000;
    while (performance.now() < deadline) {
      const value = predicate();
      if (value) return value;
      await frame();
    }
    throw Error("Timed out waiting for the WebView");
  }
  const report = result => document.querySelector("." + result).click();
  const select = await waitFor(() => document.querySelector("select.interval"));
  if (select.offsetHeight !== 34 || getComputedStyle(select).appearance !== "auto") {
    report("select-failed");
    return;
  }

  try {
    const marker = await waitFor(() => document.querySelector(".sapid-segment-indicator.is-positioned"));
    const track = marker.parentElement;
    const first = track.querySelector("input:checked").closest("label");
    const last = track.querySelector('input[value="dark"]');
    const start = first.getBoundingClientRect();
    const target = last.closest("label").getBoundingClientRect();
    const aligned = (box, label) => ["left", "top", "width", "height"].every(
      property => Math.abs(box[property] - label[property]) < 1,
    );
    if (!aligned(marker.getBoundingClientRect(), start)) throw Error("Initial indicator alignment");

    let moved = false;
    last.click();
    await waitFor(() => {
      const box = marker.getBoundingClientRect();
      moved ||= box.left > start.left + 1 && box.left < target.left - 1;
      return last.checked && aligned(box, target);
    });
    if (moved === matchMedia("(prefers-reduced-motion: reduce)").matches) {
      throw Error("Unexpected indicator movement");
    }
    report("test-passed");
  } catch (error) {
    console.error(error);
    report("indicator-failed");
  }
}, {once: true});
