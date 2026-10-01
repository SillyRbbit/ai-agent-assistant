import { chromium } from "playwright";
import assert from "node:assert/strict";
import { mkdir } from "node:fs/promises";
/* global document, window, getComputedStyle */
const output = process.argv[2];
if (!output) throw Error("External screenshot directory required");
await mkdir(output, { recursive: true });
const browser = await chromium.launch({ headless: true });
try {
  if (!process.argv.includes("--shell-only")) {
    for (const [name, width] of [
      ["desktop", 1280],
      ["narrow", 760],
    ]) {
      const page = await browser.newPage({ viewport: { width, height: 900 } });
      await page.goto("http://127.0.0.1:4175/scripts/browser/knowledge.html");
      await page.getByText("No library items yet.", { exact: true }).waitFor();
      await page.screenshot({ path: `${output}/${name}-empty.png`, fullPage: true });
      await page.getByRole("button", { name: "Import selected file" }).click();
      await page.getByRole("heading", { name: "Synthetic runbook 1" }).waitFor();
      assert.equal(await page.locator("img,iframe").count(), 0);
      assert.equal(
        await page.locator("pre").textContent(),
        "---\ntags: [operations]\n---\n# Incident runbook 界\nInvestigate synthetic service availability.\n[[Related note]] <script>never execute</script> ![remote](https://invalid.test/no)\n",
      );
      await page.getByLabel("Search local passages").fill("availability");
      await page.getByRole("button", { name: "Search", exact: true }).click();
      await page.getByText(/1 matching passages/).waitFor();
      await page.getByRole("button", { name: "Workflow source surface" }).click();
      await page.getByLabel("Find library passages").fill("availability");
      await page.getByRole("button", { name: "Find passages" }).click();
      await page.getByRole("button", { name: "Select K1_V1_P1", exact: true }).click();
      await page.getByText(/Selected: Synthetic runbook/).waitFor();
      await page.screenshot({ path: `${output}/${name}-sources.png`, fullPage: true });
      await page.getByRole("button", { name: "Draft surface" }).click();
      await page.getByLabel("Draft title").fill("Reusable synthetic draft");
      await page.getByRole("button", { name: "Save draft to Knowledge" }).click();
      await page.getByRole("button", { name: /Reusable synthetic draft/ }).click();
      await page.getByRole("button", { name: "Export Markdown" }).click();
      await page.getByRole("alert").filter({ hasText: "Nothing was overwritten" }).waitFor();
      await page.screenshot({ path: `${output}/${name}-draft-error.png`, fullPage: true });
      assert.equal(
        await page.evaluate(
          () => document.documentElement.scrollWidth <= document.documentElement.clientWidth,
        ),
        true,
      );
      await page.close();
    }
    for (const mode of ["loading", "error", "long"]) {
      const page = await browser.newPage({ viewport: { width: 760, height: 900 } });
      await page.goto(`http://127.0.0.1:4175/scripts/browser/knowledge.html?${mode}`);
      if (mode === "loading") await page.getByText("Loading local library…").waitFor();
      else if (mode === "error") await page.getByRole("alert").waitFor();
      else await page.getByRole("button", { name: /界.*imported/ }).click();
      await page.screenshot({ path: `${output}/${mode}.png`, fullPage: true });
      assert.equal(
        await page.evaluate(
          () => document.documentElement.scrollWidth <= document.documentElement.clientWidth,
        ),
        true,
      );
      await page.close();
    }
  }
  // Test CSS geometry at the real shell boundary, not isolated route components.
  for (const [width, height] of [
    [1600, 1000],
    [1440, 1000],
    [1280, 800],
    [760, 520],
  ]) {
    const page = await browser.newPage({ viewport: { width, height } });
    await page.goto("http://127.0.0.1:4175/scripts/browser/knowledge.html?shell");
    const nav = page.getByRole("navigation", { name: "Primary navigation" });
    // Narrow navigation itself is an intentional overlay. Close it after routing.
    async function route(name) {
      const expand = page.getByRole("button", { name: "Expand navigation", exact: true });
      if (await expand.count()) await expand.click();
      await nav.getByRole("button", { name, exact: true }).click();
      const collapse = page.getByRole("button", { name: "Collapse navigation", exact: true });
      if (await collapse.count()) await collapse.click();
      await page.waitForFunction(() => {
        const side = document.querySelector(".application-sidebar");
        return (
          !!side &&
          Math.abs(
            side.getBoundingClientRect().width -
              parseFloat(getComputedStyle(side).getPropertyValue("--app-sidebar-collapsed-width")),
          ) < 1
        );
      });
    }
    async function unobscured(locator) {
      await locator.scrollIntoViewIfNeeded();
      assert.equal(
        await locator.evaluate((el) => {
          const r = el.getBoundingClientRect();
          const x = r.left + r.width / 2,
            y = r.top + r.height / 2;
          const hit = document.elementFromPoint(x, y);
          const main = document.querySelector(".application-main")?.getBoundingClientRect();
          return (
            !!main &&
            r.left >= main.left - 1 &&
            r.right <= main.right + 1 &&
            y >= main.top &&
            y <= main.bottom &&
            !!hit &&
            (el === hit || el.contains(hit))
          );
        }),
        true,
        `reachable ${width}: ${await locator.textContent()}`,
      );
    }
    async function separated() {
      assert.equal(
        await page.evaluate(() => {
          const m = document.querySelector(".application-main")?.getBoundingClientRect();
          const i = document.querySelector(".application-inspector")?.getBoundingClientRect();
          return (
            !!m &&
            !!i &&
            m.height > 100 &&
            (i.bottom <= m.top + 1 || m.right <= i.left + 1) &&
            document.documentElement.scrollWidth <= window.innerWidth
          );
        }),
        true,
        `nonoverlap ${width}`,
      );
    }
    await route("Knowledge");
    await page.getByRole("button", { name: "Show workspace inspector", exact: true }).click();
    await separated();
    await unobscured(page.getByRole("button", { name: "Import selected file", exact: true }));
    await page.screenshot({ path: `${output}/shell-${width}-knowledge.png` });
    await page.getByRole("button", { name: "Close workspace inspector", exact: true }).focus();
    await page.keyboard.press("Escape");
    assert.equal(await page.locator(".application-inspector").isVisible(), false);
    assert.equal(
      await page
        .getByRole("button", { name: "Show workspace inspector", exact: true })
        .evaluate((el) => el === document.activeElement),
      true,
    );
    await page.getByRole("button", { name: "Show workspace inspector", exact: true }).click();
    await route("Collaboration");
    await page.getByRole("button", { name: "Synthetic layout room", exact: true }).click();
    await separated();
    await page.getByLabel("Objective", { exact: true }).fill("Synthetic layout only");
    const prepare = page.getByRole("button", {
      name: "Check readiness and review transmission",
      exact: true,
    });
    await unobscured(prepare);
    await prepare.click();
    const disclosure = page.getByText(/Each listed provider receives the objective/);
    await unobscured(disclosure);
    await page.screenshot({ path: `${output}/shell-${width}-disclosure.png` });
    const start = page.getByRole("button", { name: "Start simulation workflow", exact: true });
    await unobscured(start);
    assert.equal(await start.isEnabled(), false);
    await page.getByRole("button", { name: "Close workspace inspector", exact: true }).click();
    await unobscured(start);
    await page.getByRole("button", { name: "Show workspace inspector", exact: true }).click();
    await route("Knowledge");
    await separated();
    await route("Command Center");
    if (width >= 1100) {
      await page.waitForFunction(
        () => document.querySelectorAll(".react-flow__node").length === 10,
      );
      assert.equal(
        await page.evaluate(() => {
          const c = document.querySelector(".operational-canvas")?.getBoundingClientRect();
          const i = document.querySelector(".application-inspector")?.getBoundingClientRect();
          return !!c && !!i && c.right <= i.left + 1;
        }),
        true,
        "graph docking retained",
      );
    }
    await route("Collaboration");
    await separated();
    await page.close();
  }
  console.log(
    "Knowledge actual-shell inspector matrix passed; retained standalone scenarios: empty/import/search/selection/draft/export-error/loading/long Unicode at 1280/760 widths",
  );
} finally {
  await browser.close();
}
