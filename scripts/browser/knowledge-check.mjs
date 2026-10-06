import { chromium } from "playwright";
import assert from "node:assert/strict";
import { mkdir } from "node:fs/promises";
/* global document, window, getComputedStyle */
const output = process.argv[2];
const portOption = process.argv.find((arg) => arg.startsWith("--port="));
const port = portOption?.slice(7) ?? "4175";
if (!/^\d{4,5}$/.test(port) || Number(port) > 65535) throw Error("Invalid local QA port");
const baseUrl = `http://127.0.0.1:${port}`;
if (!output) throw Error("External screenshot directory required");
await mkdir(output, { recursive: true });
const botsOnly = process.argv.includes("--bots-only");
const conversationsOnly = process.argv.includes("--conversations-only");
const browser = await chromium.launch({ headless: true });
try {
  if (!process.argv.includes("--shell-only") && !botsOnly && !conversationsOnly) {
    for (const [name, width] of [
      ["desktop", 1280],
      ["narrow", 760],
    ]) {
      const page = await browser.newPage({ viewport: { width, height: 900 } });
      await page.goto(`${baseUrl}/scripts/browser/knowledge.html`);
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
      await page
        .getByRole("complementary", { name: "Knowledge items" })
        .getByRole("button", { name: /Reusable synthetic draft/ })
        .click();
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
      await page.goto(`${baseUrl}/scripts/browser/knowledge.html?${mode}`);
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
  for (const expanded of botsOnly || conversationsOnly ? [] : [true, false]) {
    const page = await browser.newPage({ viewport: { width: 1600, height: 1000 } });
    await page.goto(`${baseUrl}/scripts/browser/knowledge.html?shell`);
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
  if (!botsOnly && !conversationsOnly) {
    console.log(
      "Knowledge actual-shell inspector matrix passed; retained standalone scenarios: empty/import/search/selection/draft/export-error/loading/long Unicode at 1280/760 widths",
    );
  }
  // Bots uses the same real App and immutable synthetic profiles. Never Save,
  // prepare a conversation, or execute a workflow in this layout regression.
  for (const expanded of conversationsOnly ? [] : [true, false]) {
    const page = await browser.newPage({ viewport: { width: 1600, height: 1000 } });
    await page.goto(`${baseUrl}/scripts/browser/knowledge.html?shell`);
    // Exercise the public Tauri invoke boundary only inside the isolated mock.
    // Rejected writes are synthetic calls; this browser has no native authority.
    const boundary = await page.evaluate(async () => {
      const { invoke } = await import("/node_modules/@tauri-apps/api/core.js");
      const readiness = await invoke("list_agent_connections");
      const before = JSON.stringify(await invoke("list_agent_preferences"));
      const blocked = [
        "save_agent_preferences",
        "clear_agent_note",
        "restore_agent_defaults",
        "discover_agent_models",
        "start_agent_conversation",
        "send_agent_message",
        "start_collaboration",
        "cancel_collaboration",
        "unknown_layout_command",
      ];
      const rejected = [];
      for (const command of blocked) {
        try {
          await invoke(command);
        } catch {
          rejected.push(command);
        }
      }
      return {
        readiness,
        blocked,
        rejected,
        preserved: before === JSON.stringify(await invoke("list_agent_preferences")),
      };
    });
    assert.deepEqual(
      boundary.readiness,
      ["simulation", "openai_api", "codex", "anthropic_api", "lm_studio", "ollama"].map(
        (connection) => ({
          connection,
          status: connection === "simulation" ? "ready" : "blocked",
          message: "Synthetic layout fixture only; execution is unavailable.",
        }),
      ),
    );
    assert.deepEqual(boundary.rejected, boundary.blocked);
    assert.equal(boundary.preserved, true);
    const nav = page.getByRole("navigation", { name: "Primary navigation" });
    const stateName = expanded ? "expanded" : "collapsed";
    if (!expanded) {
      await page.getByRole("button", { name: "Collapse navigation", exact: true }).click();
    }
    async function checkShell() {
      await page.waitForFunction(() => {
        const side = document.querySelector(".application-sidebar");
        return !!side && side.getAnimations().every((a) => a.playState !== "running");
      });
      assert.equal(
        await page
          .getByRole("button", {
            name: expanded ? "Collapse navigation" : "Expand navigation",
            exact: true,
          })
          .count(),
        1,
        "Bots resizing/routing must retain navigation state",
      );
      const geometry = await page.evaluate(() => {
        const main = document.querySelector(".application-main");
        const side = document.querySelector(".application-sidebar");
        const content = document.querySelector(".application-content");
        if (!main || !side || !content) return null;
        const m = main.getBoundingClientRect();
        const n = side.getBoundingClientRect();
        return {
          mainLeft: m.left,
          sideRight: n.right,
          mainHeight: m.height,
          contentScroll: content.scrollWidth,
          contentWidth: content.clientWidth,
          documentWidth: document.documentElement.scrollWidth,
          windowWidth: window.innerWidth,
        };
      });
      const separated =
        geometry !== null &&
        geometry.mainLeft >= geometry.sideRight - 1 &&
        geometry.mainHeight > 100 &&
        geometry.contentScroll <= geometry.contentWidth + 1 &&
        geometry.documentWidth <= geometry.windowWidth;
      if (!separated) {
        console.log(
          "Overflow diagnostic",
          await page.evaluate(() => {
            const content = document.querySelector(".application-content");
            const rect = content.getBoundingClientRect();
            return {
              scrollLeft: content.scrollLeft,
              content: rect.toJSON(),
              overflowing: [...content.querySelectorAll("*")]
                .map((el) => {
                  const box = el.getBoundingClientRect();
                  return {
                    tag: el.tagName,
                    className: el.className,
                    parent: el.parentElement?.className,
                    text: el.textContent?.slice(0, 100),
                    right: box.right,
                    left: box.left,
                    width: box.width,
                    scroll: el.scrollWidth,
                    client: el.clientWidth,
                  };
                })
                .filter(
                  (v) =>
                    v.width > 0 &&
                    (v.right > rect.left + content.clientWidth + 1 || v.left < rect.left - 1),
                )
                .slice(0, 20),
            };
          }),
        );
        await page.screenshot({
          path: `${output}/failure-${page.viewportSize()?.width}-${stateName}.png`,
        });
      }
      assert.equal(
        separated,
        true,
        `Bots/sidebar separation and overflow ${stateName}: ${JSON.stringify(geometry)}`,
      );
    }
    async function reachable(locator) {
      await locator.scrollIntoViewIfNeeded();
      assert.equal(
        await locator.evaluate((el) => {
          const box = el.getBoundingClientRect();
          const main = document.querySelector(".application-main")?.getBoundingClientRect();
          if (!main || box.left < main.left - 1 || box.right > main.right + 1) return false;
          const y = box.top + box.height / 2;
          return (
            y >= main.top &&
            y <= main.bottom &&
            [box.left + 2, box.left + box.width / 2, box.right - 2].every((x) => {
              const hit = document.elementFromPoint(x, y);
              return !!hit && (hit === el || el.contains(hit));
            })
          );
        }),
        true,
        `Bots reachable ${stateName}: ${await locator.textContent()}`,
      );
    }
    for (const [step, [width, height]] of [
      [1600, 1000],
      [961, 1000],
      [960, 1000],
      [959, 1000],
      [761, 521],
      [760, 520],
      [761, 521],
      [959, 1000],
      [960, 1000],
      [961, 1000],
      [1600, 1000],
    ].entries()) {
      await page.setViewportSize({ width, height });
      for (const name of ["Knowledge", "Collaboration", "Bots"]) {
        await nav.getByRole("button", { name, exact: true }).click();
        await checkShell();
      }
      await reachable(page.getByRole("heading", { name: "Bots", exact: true }));
      const choices = page.getByRole("navigation", { name: "Agent selection" }).getByRole("button");
      await choices.nth(8).waitFor({ state: "visible" });
      assert.equal(await choices.count(), 9);
      for (const choice of await choices.all()) {
        await reachable(choice);
        await choice.click();
        assert.equal(await choice.getAttribute("aria-current"), "true");
        await reachable(page.locator("#agent-settings-title"));
        await reachable(
          page.getByLabel("Nickname (optional, 48 characters; blank uses canonical role)", {
            exact: true,
          }),
        );
        await reachable(page.getByRole("combobox", { name: /^Avatar/ }));
        const preview = page.locator(".bot-preview");
        const label = preview.locator(":scope > span:last-child");
        await reachable(label);
        const previewGeometry = await preview.evaluate((el) => {
          const row = el.getBoundingClientRect();
          const label = el.lastElementChild;
          const text = label.getBoundingClientRect();
          const presentation = el.querySelector(".bot-mascot-presentation--large");
          const space = el.querySelector(".bot-mascot-space");
          const box = space.getBoundingClientRect();
          const art = presentation.getBoundingClientRect();
          return {
            textFits:
              text.left >= row.left - 1 &&
              text.right <= row.right + 1 &&
              label.scrollWidth <= label.clientWidth + 1,
            artworkFits: art.left >= row.left - 1 && art.right <= row.right + 1,
            canvas: [box.width, box.height],
            padding: ["paddingTop", "paddingRight", "paddingBottom", "paddingLeft"].map(
              (key) => getComputedStyle(presentation)[key],
            ),
            margin: ["marginTop", "marginRight", "marginBottom", "marginLeft"].map(
              (key) => getComputedStyle(space)[key],
            ),
          };
        });
        assert.equal(previewGeometry.textFits, true, "Full preview label remains within its row");
        assert.equal(previewGeometry.artworkFits, true, "Unshrunk mascot remains within preview");
        assert.deepEqual(previewGeometry.canvas, [144, 144]);
        assert.deepEqual(previewGeometry.padding, ["16px", "16px", "16px", "16px"]);
        assert.deepEqual(previewGeometry.margin, ["16px", "16px", "16px", "16px"]);
        await reachable(preview.getByRole("button", { name: "Playful spin", exact: true }));
        await reachable(
          preview.getByText("Decorative avatar · not connection or work status", { exact: true }),
        );
        await checkShell();

        await reachable(page.getByRole("combobox", { name: /^Connection/ }));
        await reachable(page.getByRole("button", { name: "Save settings and note", exact: true }));
        assert.equal(
          await page
            .getByRole("button", { name: "Save settings and note", exact: true })
            .isEnabled(),
          false,
        );
      }
      // Reachability probes exercise the real content scroll owner, including
      // the settings at the bottom; a visible center alone cannot pass overlap.
      assert.equal(
        await page.locator(".application-content").evaluate((el) => el.scrollTop > 0),
        true,
      );
      await page.getByRole("button", { name: "Show workspace inspector", exact: true }).click();
      const inspector = page.locator(".application-inspector");
      assert.equal(await inspector.isVisible(), true);
      assert.equal(
        await inspector.evaluate((el) => {
          const i = el.getBoundingClientRect();
          const m = document.querySelector(".application-main")?.getBoundingClientRect();
          const n = document.querySelector(".application-sidebar")?.getBoundingClientRect();
          return (
            !!m && !!n && i.left >= n.right - 1 && (i.bottom <= m.top + 1 || i.left >= m.right - 1)
          );
        }),
        true,
        "inspector remains separate from Bots and navigation",
      );
      await page.getByRole("button", { name: "Close workspace inspector", exact: true }).focus();
      await page.keyboard.press("Escape");
      await page.waitForFunction(
        () =>
          document.activeElement ===
          document.querySelector('button[aria-label="Show workspace inspector"]'),
      );
      assert.equal(await inspector.isVisible(), false);
      await page.getByRole("button", { name: "Show workspace inspector", exact: true }).click();
      await page.getByRole("button", { name: "Close workspace inspector", exact: true }).click();
      assert.equal(await inspector.isVisible(), false);
      await checkShell();
      await reachable(page.getByRole("heading", { name: "Bots", exact: true }));
      await page.screenshot({
        path: `${output}/bots-${String(step)}-${String(width)}-${stateName}.png`,
      });
    }
    await page.close();
  }
  if (!conversationsOnly)
    console.log(
      "Bots actual-App layout passed: 22 resize/state cases, nine choices/settings, route transitions, scrolling, overflow and inspector close/Escape",
    );
  // Conversations reuses the existing actual-App fixture; no Send or execution.
  for (const expanded of conversationsOnly ? [true, false] : []) {
    const page = await browser.newPage({ viewport: { width: 1600, height: 1000 } });
    await page.goto(`${baseUrl}/scripts/browser/knowledge.html?shell`);
    const nav = page.getByRole("navigation", { name: "Primary navigation" });
    const stateName = expanded ? "expanded" : "collapsed";
    if (!expanded)
      await page.getByRole("button", { name: "Collapse navigation", exact: true }).click();
    async function separated() {
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
      );
      assert.equal(
        await page.evaluate(() => {
          const side = document.querySelector(".application-sidebar").getBoundingClientRect();
          const main = document.querySelector(".application-main").getBoundingClientRect();
          const content = document.querySelector(".application-content");
          return (
            main.left >= side.right - 1 &&
            main.height > 100 &&
            content.scrollWidth <= content.clientWidth + 1 &&
            document.documentElement.scrollWidth <= window.innerWidth
          );
        }),
        true,
        `Conversations separated/overflow ${stateName} ${page.viewportSize().width}`,
      );
    }
    async function reachable(locator) {
      await locator.scrollIntoViewIfNeeded();
      assert.equal(
        await locator.evaluate((el) => {
          const b = el.getBoundingClientRect();
          const main = document.querySelector(".application-main").getBoundingClientRect();
          if (b.left < main.left - 1 || b.right > main.right + 1) return false;
          return [b.left + 2, b.left + b.width / 2, b.right - 2].every((x) => {
            const y = b.top + b.height / 2,
              hit = document.elementFromPoint(x, y);
            return y >= main.top && y <= main.bottom && !!hit && (hit === el || el.contains(hit));
          });
        }),
        true,
        `Conversations reachable ${stateName}: ${await locator.textContent()}`,
      );
    }
    async function wheelBoundary(locator, down) {
      if (
        await locator.evaluate(
          (el, toBottom) =>
            toBottom ? el.scrollTop + el.clientHeight >= el.scrollHeight - 1 : el.scrollTop === 0,
          down,
        )
      )
        return;
      const box = await locator.boundingBox();
      assert.ok(box);
      await page.mouse.move(box.x + box.width / 2, box.y + box.height / 2);
      await locator.evaluate((el) => {
        el.dataset.qaScrollSettled = "false";
        el.addEventListener(
          "scrollend",
          () => {
            el.dataset.qaScrollSettled = "true";
          },
          { once: true },
        );
      });
      await page.mouse.wheel(0, down ? 2000 : -2000);
      await page.waitForFunction(
        ({ el, toBottom }) =>
          el.dataset.qaScrollSettled === "true" &&
          (toBottom
            ? el.scrollTop > 0 && el.scrollTop + el.clientHeight >= el.scrollHeight - 1
            : el.scrollTop === 0),
        { el: await locator.elementHandle(), toBottom: down },
      );
    }
    async function sidebarReachability() {
      const list = page.locator('[data-scroll-region="primary-navigation-scroll"]');
      const workspace = page.locator(".application-content");
      const workspaceBefore = await workspace.evaluate((el) => el.scrollTop);
      const scrollable = await list.evaluate((el) => el.scrollHeight > el.clientHeight + 1);
      const box = await list.boundingBox();
      assert.ok(box && box.height > 0);
      await page.mouse.move(box.x + box.width / 2, box.y + box.height / 2);
      if (scrollable) {
        await wheelBoundary(list, false);
        await wheelBoundary(list, true);
      }
      async function navHit(label) {
        const control = nav.getByRole("button", { name: label, exact: true });
        try {
          await page.waitForFunction(
            (el) => {
              const b = el.getBoundingClientRect(),
                list = el.closest("ul").getBoundingClientRect();
              const hit = document.elementFromPoint(b.left + b.width / 2, b.top + b.height / 2);
              return (
                b.top >= list.top - 1 &&
                b.bottom <= list.bottom + 1 &&
                b.top >= 0 &&
                b.bottom <= window.innerHeight &&
                !!hit &&
                el.contains(hit)
              );
            },
            await control.elementHandle(),
          );
        } catch (error) {
          console.log(
            JSON.stringify(
              await control.evaluate((el) => {
                const list = el.closest("ul"),
                  b = el.getBoundingClientRect();
                const hit = document.elementFromPoint(b.left + b.width / 2, b.top + b.height / 2);
                return {
                  name: el.getAttribute("aria-label"),
                  button: b.toJSON(),
                  list: list.getBoundingClientRect().toJSON(),
                  scrollTop: list.scrollTop,
                  clientHeight: list.clientHeight,
                  scrollHeight: list.scrollHeight,
                  hit: hit?.tagName,
                  active: document.activeElement?.getAttribute("aria-label"),
                  viewport: [window.innerWidth, window.innerHeight],
                };
              }),
            ),
          );
          await page.screenshot({ path: `${output}/sidebar-failure.png` });
          throw error;
        }
      }
      await navHit("Settings");
      if (!expanded) {
        const settings = nav.getByRole("button", { name: "Settings", exact: true });
        await settings.hover();
        const tip = page.locator("#navigation-tooltip-settings");
        await tip.waitFor({ state: "visible" });
        assert.equal(
          await tip.evaluate((el) => {
            const b = el.getBoundingClientRect(),
              side = document.querySelector(".application-sidebar").getBoundingClientRect();
            return (
              !el.closest("ul") &&
              b.left >= side.right &&
              b.right <= window.innerWidth &&
              b.top >= 0 &&
              b.bottom <= window.innerHeight &&
              getComputedStyle(el).visibility === "visible"
            );
          }),
          true,
          "Tooltip must escape the list without viewport clipping",
        );
        await page.screenshot({ path: `${output}/sidebar-tooltip-${step}-${stateName}.png` });
        await page.keyboard.press("Escape");
        await tip.waitFor({ state: "hidden" });
      }
      if (scrollable) await wheelBoundary(list, false);
      await navHit("Collaboration");
      assert.equal(
        await workspace.evaluate((el) => el.scrollTop),
        workspaceBefore,
        "Sidebar wheel must not move workspace",
      );
      // Use native tab order, never programmatic list scrolling, to reach every route.
      await nav.getByRole("button", { name: "Collaboration", exact: true }).focus();
      const names = await nav
        .getByRole("button")
        .evaluateAll((els) => els.map((el) => el.getAttribute("aria-label")));
      for (const name of names.slice(1)) {
        await page.keyboard.press("Tab");
        assert.equal(
          await page.evaluate(() => document.activeElement?.getAttribute("aria-label")),
          name,
        );
        await navHit(name);
        if (!expanded) {
          const id = await nav
            .getByRole("button", { name, exact: true })
            .getAttribute("aria-describedby");
          await page.locator(`#${id}[data-open="true"]`).waitFor({ state: "visible" });
        }
      }
      for (const name of names.slice(0, -1).reverse()) {
        await page.keyboard.press("Shift+Tab");
        assert.equal(
          await page.evaluate(() => document.activeElement?.getAttribute("aria-label")),
          name,
        );
        await navHit(name);
        if (!expanded) {
          const id = await nav
            .getByRole("button", { name, exact: true })
            .getAttribute("aria-describedby");
          await page.locator(`#${id}[data-open="true"]`).waitFor({ state: "visible" });
        }
      }
      const listBefore = await list.evaluate((el) => el.scrollTop);
      await wheelBoundary(workspace, false);
      if (await workspace.evaluate((el) => el.scrollHeight > el.clientHeight + 1)) {
        await wheelBoundary(workspace, true);
      }
      assert.equal(
        await list.evaluate((el) => el.scrollTop),
        listBefore,
        "Workspace wheel must not move navigation",
      );
    }
    let step = 0;
    for (const [width, height] of [
      [1600, 1000],
      [961, 1000],
      [960, 1000],
      [959, 1000],
      [840, 562],
      [1600, 520],
      [760, 520],
      [595, 520],
      [595, 853],
      [760, 520],
      [959, 1000],
      [960, 1000],
      [961, 1000],
      [1600, 1000],
    ]) {
      step++;
      await page.setViewportSize({ width, height });
      for (const route of ["Knowledge", "Collaboration", "Bots", "Conversations"]) {
        await nav.getByRole("button", { name: route, exact: true }).click();
        await separated();
      }
      await reachable(page.getByRole("heading", { name: "Conversations", exact: true }));
      await reachable(page.getByRole("combobox", { name: /^Conversation mode/ }));
      assert.equal(
        await page.getByRole("combobox", { name: /^Conversation mode/ }).inputValue(),
        "mock",
      );
      await reachable(page.getByLabel("Assistant request", { exact: true }));
      await reachable(page.getByRole("button", { name: "Send", exact: true }));
      assert.equal(
        await page.getByRole("button", { name: "Send", exact: true }).isEnabled(),
        false,
      );
      assert.equal(await page.getByLabel("Assistant request", { exact: true }).inputValue(), "");
      await reachable(
        page.getByText("Local mock only · no model or tool execution", { exact: true }),
      );
      assert.equal(
        await page
          .locator(".application-content")
          .evaluate(
            (el) =>
              el.scrollHeight <= el.clientHeight + 1 || getComputedStyle(el).overflowY === "auto",
          ),
        true,
      );
      await sidebarReachability();
      await page.screenshot({ path: `${output}/conversations-${step}-${width}-${stateName}.png` });
    }
    await page.close();
  }
  if (conversationsOnly)
    console.log(
      "Conversations actual-App: 28 resize/state cases; real sidebar/workspace wheel, keyboard, tooltip reachability;, route transitions, heading/mode/composer/disclosure hit-testing, scrolling, no overflow; no input or execution",
    );
} finally {
  await browser.close();
}
