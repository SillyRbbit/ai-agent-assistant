import { chromium } from "playwright";
import assert from "node:assert/strict";
import { mkdir } from "node:fs/promises";
/* global document */
const output = process.argv[2];
if (!output) throw Error("External evidence directory required");
await mkdir(output, { recursive: true });
const browser = await chromium.launch({ headless: true });
try {
  for (const collapsed of [false, true]) {
    const page = await browser.newPage({ viewport: { width: 1280, height: 960 } });
    // Existing synthetic fixture mounts the actual App shell. Diagnostics is
    // unavailable here; live native read/export is checked separately.
    await page.goto("http://127.0.0.1:4175/scripts/browser/knowledge.html?shell");
    await page
      .getByRole("navigation", { name: "Primary navigation" })
      .getByRole("button", { name: "Settings", exact: true })
      .click();
    await page.getByRole("heading", { name: "Local diagnostics", exact: true }).waitFor();
    if (collapsed)
      await page.getByRole("button", { name: "Collapse navigation", exact: true }).click();
    for (const width of [1280, 961, 960, 959, 760, 1280]) {
      await page.setViewportSize({ width, height: 960 });
      for (const inspector of [false, true]) {
        if (inspector)
          await page.getByRole("button", { name: "Show workspace inspector", exact: true }).click();
        await page.waitForFunction(() =>
          document
            .querySelector(".application-sidebar")
            .getAnimations()
            .every((a) => a.playState !== "running"),
        );
        const panel = await page.locator(".local-diagnostics").boundingBox();
        const sidebar = await page.locator(".application-sidebar").boundingBox();
        assert.ok(panel.x >= sidebar.x + sidebar.width - 1, `navigation overlap at ${width}`);
        assert.ok(panel.x + panel.width <= width + 1, `horizontal overflow at ${width}`);
        for (const name of [
          "Refresh diagnostics",
          "Copy sanitized summary",
          "Export diagnostics",
        ]) {
          const control = page.getByRole("button", { name, exact: true });
          await control.scrollIntoViewIfNeeded();
          const box = await control.boundingBox();
          assert.ok(box.x >= sidebar.x + sidebar.width - 1 && box.x + box.width <= width, name);
        }
        if (inspector) {
          const box = await page.locator(".application-inspector").boundingBox();
          assert.ok(panel.y >= box.y + box.height - 1, "inspector overlap");
          await page.getByRole("button", { name: "Hide workspace inspector", exact: true }).click();
        }
      }
      await page
        .getByRole("heading", { name: "Local diagnostics", exact: true })
        .scrollIntoViewIfNeeded();
      await page.screenshot({
        path: `${output}/${collapsed ? "collapsed" : "expanded"}-${width}.png`,
      });
    }
    await page.close();
  }
  console.log("Diagnostics actual-shell layout: 24 combinations passed.");
} finally {
  await browser.close();
}
