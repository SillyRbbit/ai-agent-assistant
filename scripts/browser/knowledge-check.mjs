import { chromium } from "playwright";
import assert from "node:assert/strict";
import { mkdir } from "node:fs/promises";
/* global document, window */
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
  // Exercise the real shell without silently collapsing expanded navigation.
  // Reuse each page across breakpoint crossings to cover native resize ordering.
  for (const expanded of [true, false]) {
    const page = await browser.newPage({ viewport: { width: 1600, height: 1000 } });
    await page.goto("http://127.0.0.1:4175/scripts/browser/knowledge.html?shell");
    const nav = page.getByRole("navigation", { name: "Primary navigation" });
    const stateName = expanded ? "expanded" : "collapsed";
    if (!expanded) {
      await page.getByRole("button", { name: "Collapse navigation", exact: true }).click();
    }
    async function route(name) {
      // The collapsed navigation retains accessible route buttons; never change
      // the owner's navigation state to make a geometry assertion pass.
      await nav.getByRole("button", { name, exact: true }).click();
    }
    async function settled() {
      await page.waitForFunction(() => {
        const side = document.querySelector(".application-sidebar");
        return (
          !!side && side.getAnimations().every((animation) => animation.playState !== "running")
        );
      });
      assert.equal(
        await page
          .getByRole("button", {
            name: expanded ? "Collapse navigation" : "Expand navigation",
            exact: true,
          })
          .count(),
        1,
        "resizing and routing preserve navigation state",
      );
    }
    async function unobscured(locator) {
      await locator.scrollIntoViewIfNeeded();
      assert.equal(
        await locator.evaluate((el) => {
          const r = el.getBoundingClientRect();
          const main = document.querySelector(".application-main")?.getBoundingClientRect();
          if (!main || r.left < main.left - 1 || r.right > main.right + 1) return false;
          // Test both sides as well as the center: a wide control may have an
          // accessible center even though its left label is behind navigation.
          return [r.left + 2, r.left + r.width / 2, r.right - 2].every((x) => {
            const y = r.top + r.height / 2;
            const hit = document.elementFromPoint(x, y);
            return y >= main.top && y <= main.bottom && !!hit && (el === hit || el.contains(hit));
          });
        }),
        true,
        `reachable ${stateName}: ${await locator.textContent()}`,
      );
    }
    async function separated(open) {
      await settled();
      assert.equal(
        await page.evaluate((inspectorOpen) => {
          const m = document.querySelector(".application-main")?.getBoundingClientRect();
          const n = document.querySelector(".application-sidebar")?.getBoundingClientRect();
          const i = document.querySelector(".application-inspector")?.getBoundingClientRect();
          return (
            !!m &&
            !!n &&
            m.height > 100 &&
            m.left >= n.right - 1 &&
            (!inspectorOpen ||
              (!!i && i.left >= n.right - 1 && (i.bottom <= m.top + 1 || m.right <= i.left + 1))) &&
            document.documentElement.scrollWidth <= window.innerWidth
          );
        }, open),
        true,
        `navigation/workspace/inspector nonoverlap ${stateName} at ${page.viewportSize()?.width}`,
      );
    }
    for (const [width, height] of [
      [1600, 1000],
      [961, 1410],
      [960, 1410],
      [959, 1410],
      [760, 520],
      [959, 1410],
      [960, 1410],
      [961, 1410],
      [1600, 1000],
    ]) {
      await page.setViewportSize({ width, height });
      await route("Knowledge");
      if (
        await page.getByRole("button", { name: "Show workspace inspector", exact: true }).count()
      ) {
        await page.getByRole("button", { name: "Show workspace inspector", exact: true }).click();
      }
      await separated(true);
      await unobscured(page.getByRole("heading", { name: "Knowledge & Documents", exact: true }));
      await unobscured(page.getByRole("button", { name: "Import selected file", exact: true }));
      await page.screenshot({ path: `${output}/shell-${width}-${stateName}-knowledge.png` });
      await page.getByRole("button", { name: "Close workspace inspector", exact: true }).focus();
      await page.keyboard.press("Escape");
      assert.equal(await page.locator(".application-inspector").isVisible(), false);
      // The real shell restores focus on its next animation frame. Observe the
      // promised focus boundary rather than racing that callback.
      await page.waitForFunction(() => {
        const toggle = document.querySelector('button[aria-label="Show workspace inspector"]');
        return !!toggle && toggle === document.activeElement;
      });
      assert.equal(
        await page
          .getByRole("button", { name: "Show workspace inspector", exact: true })
          .evaluate((el) => el === document.activeElement),
        true,
      );
      await separated(false);
      await unobscured(page.getByRole("button", { name: "Import selected file", exact: true }));
      await page.getByRole("button", { name: "Show workspace inspector", exact: true }).click();
      await route("Collaboration");
      await page.getByRole("button", { name: "Synthetic layout room", exact: true }).click();
      await separated(true);
      await unobscured(page.getByRole("heading", { name: "Collaboration", exact: true }));
      await page.getByLabel("Objective", { exact: true }).fill("Synthetic layout only");
      const prepare = page.getByRole("button", {
        name: "Check readiness and review transmission",
        exact: true,
      });
      await unobscured(prepare);
      await prepare.click();
      await unobscured(page.getByText(/Each listed provider receives the objective/));
      const start = page.getByRole("button", { name: "Start simulation workflow", exact: true });
      await unobscured(start);
      assert.equal(await start.isEnabled(), false);
      await page.screenshot({ path: `${output}/shell-${width}-${stateName}-disclosure.png` });
      await page.getByRole("button", { name: "Close workspace inspector", exact: true }).click();
      await separated(false);
      await unobscured(start);
      await page.getByRole("button", { name: "Show workspace inspector", exact: true }).click();
      await route("Knowledge");
      await separated(true);
      if (width >= 1100) {
        await route("Command Center");
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
        await route("Collaboration");
        await separated(true);
      }
    }
    await page.close();
  }
  console.log(
    "Knowledge actual-shell inspector matrix passed; retained standalone scenarios: empty/import/search/selection/draft/export-error/loading/long Unicode at 1280/760 widths",
  );
} finally {
  await browser.close();
}
