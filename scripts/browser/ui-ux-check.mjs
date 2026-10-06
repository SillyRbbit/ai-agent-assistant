import { chromium } from "playwright";
import assert from "node:assert/strict";
import { mkdir, writeFile } from "node:fs/promises";

/* global document, window, requestAnimationFrame, getComputedStyle */
const output = process.argv[2];
const supplementalOnly = process.argv.includes("--supplemental-only");
const port = process.argv.find((arg) => arg.startsWith("--port="))?.slice(7) ?? "4192";
if (!output) throw Error("An external evidence directory is required");
if (!/^\d{4,5}$/.test(port) || Number(port) > 65535) throw Error("Invalid loopback QA port");
const origin = `http://127.0.0.1:${port}`;
const url = `${origin}/scripts/browser/knowledge.html`;
const routes = [
  ["Conversations", "conversations", "Conversations"],
  ["Bots", "bots", "Bots"],
  ["Knowledge", "knowledge", "Knowledge & Documents"],
  ["Collaboration", "collaboration", "Collaboration"],
  ["Command Center", "command-center", "Command Center"],
  ["Settings", "settings", "Settings"],
];
const sizes = [
  [1600, 1000],
  [1280, 900],
  [961, 1000],
  [960, 1000],
  [959, 1000],
  [840, 562],
  [760, 520],
  [840, 562],
  [959, 1000],
  [960, 1000],
  [961, 1000],
  [1280, 900],
  [1600, 1000],
];
await mkdir(output, { recursive: true });
const browser = await chromium.launch({ headless: true });
const cases = [];
const networkViolations = [];
const pageErrors = [];
let currentPage;
let boundaryEvidence = null;

async function newPage(viewport) {
  const page = await browser.newPage({ viewport, reducedMotion: "reduce" });
  currentPage = page;
  page.on("pageerror", (error) => pageErrors.push(error.message));
  await page.route("**/*", async (route) => {
    const request = route.request();
    if (new URL(request.url()).origin !== origin) {
      networkViolations.push({ origin: new URL(request.url()).origin, method: request.method() });
      await route.abort();
    } else await route.continue();
  });
  return page;
}

async function settle(page) {
  // Wait for actual transitions and stable geometry, never a fixed delay.
  await page.evaluate(async () => {
    await document.fonts.ready;
    await new Promise((resolve) => {
      let previous = "";
      let stableFrames = 0;
      function sample() {
        const elements = [".application-sidebar", ".application-main", ".application-inspector"]
          .map((selector) => document.querySelector(selector))
          .filter(Boolean);
        const geometry = JSON.stringify(elements.map((el) => el.getBoundingClientRect().toJSON()));
        const moving = elements.some((el) =>
          el.getAnimations().some((animation) => animation.playState === "running"),
        );
        stableFrames = !moving && geometry === previous ? stableFrames + 1 : 0;
        previous = geometry;
        if (stableFrames >= 2) resolve();
        else requestAnimationFrame(sample);
      }
      requestAnimationFrame(sample);
    });
  });
}

async function reachable(page, locator, label) {
  await locator.scrollIntoViewIfNeeded();
  await settle(page);
  const visible = await locator.evaluate((el) => {
    const box = el.getBoundingClientRect();
    const main = document.querySelector(".application-main")?.getBoundingClientRect();
    const y = box.top + box.height / 2;
    return (
      !!main &&
      box.width > 0 &&
      box.left >= main.left - 1 &&
      box.right <= main.right + 1 &&
      y >= main.top &&
      y <= main.bottom &&
      [box.left + 2, box.left + box.width / 2, box.right - 2].every((x) => {
        const hit = document.elementFromPoint(x, y);
        return !!hit && (hit === el || el.contains(hit));
      })
    );
  });
  assert.equal(visible, true, `Unobscured workspace control: ${label}`);
}

async function shellGeometry(page, expanded, inspectorOpen) {
  await settle(page);
  assert.equal(
    await page.locator(".application-sidebar").getAttribute("data-expanded"),
    String(expanded),
    "Resizing and route transitions retain navigation state",
  );
  const geometry = await page.evaluate(() => {
    const main = document.querySelector(".application-main");
    const side = document.querySelector(".application-sidebar");
    const content = document.querySelector(".application-content");
    const inspector = document.querySelector(".application-inspector");
    return {
      main: main.getBoundingClientRect().toJSON(),
      side: side.getBoundingClientRect().toJSON(),
      inspector: inspector.getBoundingClientRect().toJSON(),
      contentWidth: content.clientWidth,
      contentScroll: content.scrollWidth,
      documentWidth: document.documentElement.scrollWidth,
      viewportWidth: window.innerWidth,
    };
  });
  const { main, side, inspector } = geometry;
  assert.ok(main.left >= side.right - 1 && main.height >= 100, "Sidebar and workspace separate");
  assert.ok(
    geometry.contentScroll <= geometry.contentWidth + 1,
    "Workspace has no horizontal overflow",
  );
  assert.ok(
    geometry.documentWidth <= geometry.viewportWidth,
    "Document has no horizontal overflow",
  );
  if (inspectorOpen) {
    assert.ok(inspector.width > 0 && inspector.height > 0, "Inspector is visibly allocated");
    assert.ok(inspector.left >= side.right - 1, "Inspector never covers navigation");
    assert.ok(
      main.right <= inspector.left + 1 ||
        main.top >= inspector.bottom - 1 ||
        main.bottom <= inspector.top + 1,
      "Inspector occupies independent space without covering workspace",
    );
  }
  return geometry;
}

async function wheelBoundary(page, locator, bottom) {
  const reached = () =>
    locator.evaluate(
      (el, down) =>
        down ? el.scrollTop + el.clientHeight >= el.scrollHeight - 1 : el.scrollTop <= 1,
      bottom,
    );
  if (await reached()) return;
  const box = await locator.boundingBox();
  assert.ok(box && box.width > 0 && box.height > 0, "Scroll region has visible area");
  await page.mouse.move(box.x + box.width - 8, box.y + box.height / 2);
  await page.mouse.wheel(0, bottom ? 12000 : -12000);
  await page.waitForFunction(
    ({ element, down }) =>
      down
        ? element.scrollTop + element.clientHeight >= element.scrollHeight - 1
        : element.scrollTop <= 1,
    { element: await locator.elementHandle(), down: bottom },
  );
}

async function navigationReachability(page, expanded) {
  const nav = page.getByRole("navigation", { name: "Primary navigation" });
  const list = page.locator('[data-scroll-region="primary-navigation-scroll"]');
  const workspace = page.locator(".application-content");
  const before = await workspace.evaluate((el) => el.scrollTop);
  async function navHit(name) {
    assert.equal(
      await nav.getByRole("button", { name, exact: true }).evaluate((el) => {
        const box = el.getBoundingClientRect();
        const list = el.closest("ul").getBoundingClientRect();
        const hit = document.elementFromPoint(box.left + box.width / 2, box.top + box.height / 2);
        return (
          box.top >= list.top - 1 && box.bottom <= list.bottom + 1 && !!hit && el.contains(hit)
        );
      }),
      true,
      `Sidebar control reachable by wheel/keyboard: ${name}`,
    );
  }
  await wheelBoundary(page, list, true);
  await navHit("Settings");
  if (!expanded) {
    await nav.getByRole("button", { name: "Settings", exact: true }).hover();
    const tooltip = page.locator("#navigation-tooltip-settings");
    await tooltip.waitFor({ state: "visible" });
    assert.equal(
      await tooltip.evaluate((el) => {
        const box = el.getBoundingClientRect();
        const side = document.querySelector(".application-sidebar").getBoundingClientRect();
        return (
          !el.closest("ul") &&
          box.left >= side.right &&
          box.right <= window.innerWidth &&
          box.top >= 0 &&
          box.bottom <= window.innerHeight
        );
      }),
      true,
      "Collapsed tooltip escapes list clipping",
    );
    await page.keyboard.press("Escape");
    await tooltip.waitFor({ state: "hidden" });
  }
  await wheelBoundary(page, list, false);
  const names = await nav
    .getByRole("button")
    .evaluateAll((buttons) => buttons.map((button) => button.getAttribute("aria-label")));
  await navHit(names[0]);
  assert.equal(
    await workspace.evaluate((el) => el.scrollTop),
    before,
    "Sidebar wheel leaves workspace unchanged",
  );
  await nav.getByRole("button", { name: names[0], exact: true }).focus();
  for (const name of names.slice(1)) {
    await page.keyboard.press("Tab");
    assert.equal(
      await page.evaluate(() => document.activeElement?.getAttribute("aria-label")),
      name,
    );
    await navHit(name);
  }
  if (!expanded) {
    await page
      .locator("#navigation-tooltip-settings[data-open=true]")
      .waitFor({ state: "visible" });
    await page.keyboard.press("Escape");
    await page.locator("#navigation-tooltip-settings").waitFor({ state: "hidden" });
  }
  const listBefore = await list.evaluate((el) => el.scrollTop);
  await wheelBoundary(page, workspace, true);
  assert.equal(
    await list.evaluate((el) => el.scrollTop),
    listBefore,
    "Workspace wheel leaves navigation unchanged",
  );
  await reachable(page, page.getByLabel("Assistant request", { exact: true }), "empty composer");
  await reachable(
    page,
    page.getByText("Local mock only · no model or tool execution", { exact: true }),
    "mock disclosure",
  );
  await wheelBoundary(page, workspace, false);
  await wheelBoundary(page, list, false);
}

async function fixtureBoundary(page) {
  const evidence = await page.evaluate(async () => {
    const { invoke } = await import("/node_modules/@tauri-apps/api/core.js");
    const reads = ["list_agent_preferences", "list_collaboration_rooms", "list_knowledge"];
    const before = await Promise.all(reads.map((command) => invoke(command)));
    const rejected = [];
    const writes = [
      "save_agent_preferences",
      "clear_agent_note",
      "restore_agent_defaults",
      "discover_agent_models",
      "start_agent_conversation",
      "send_agent_message",
      "cancel_agent_conversation",
      "create_collaboration_room",
      "start_collaboration",
      "cancel_collaboration",
      "delete_collaboration_room",
      "save_knowledge",
      "import_knowledge",
      "remove_knowledge",
      "export_knowledge",
      "export_knowledge_draft",
      "export_diagnostics",
      "unknown_design_command",
    ];
    for (const command of writes) {
      try {
        await invoke(command);
      } catch {
        rejected.push(command);
      }
    }
    return {
      writes,
      rejected,
      unchanged:
        JSON.stringify(before) ===
        JSON.stringify(await Promise.all(reads.map((command) => invoke(command)))),
      readiness: await invoke("list_agent_connections"),
      profiles: before[0],
      diagnostics: await invoke("read_diagnostics"),
    };
  });
  assert.deepEqual(
    evidence.rejected,
    evidence.writes,
    "Actual-App fixture denies writes, discovery and execution",
  );
  assert.equal(evidence.unchanged, true);
  assert.equal(evidence.profiles.length, 9);
  assert.ok(
    evidence.profiles.every(
      (profile) =>
        profile.connection === "simulation" &&
        profile.memoryMode === "off" &&
        !profile.ownerInstructions &&
        !profile.note,
    ),
  );
  assert.deepEqual(
    evidence.readiness.map(({ connection, status }) => [connection, status]),
    [
      ["simulation", "ready"],
      ["openai_api", "blocked"],
      ["codex", "blocked"],
      ["anthropic_api", "blocked"],
      ["lm_studio", "blocked"],
      ["ollama", "blocked"],
    ],
  );
  assert.deepEqual(evidence.diagnostics, { available: true, events: [] });
  return evidence;
}

async function routeReady(page, label) {
  if (label === "Bots")
    await page
      .getByRole("navigation", { name: "Agent selection" })
      .getByRole("button")
      .nth(8)
      .waitFor();
  if (label === "Knowledge")
    await page
      .getByRole("complementary", { name: "Knowledge items" })
      .getByRole("button", { name: /^Workspace design notes/ })
      .click();
  if (label === "Collaboration") {
    await page.getByRole("button", { name: "Synthetic layout room", exact: true }).click();
    await page.getByRole("textbox", { name: /^Objective/ }).fill("Synthetic layout only");
    await page
      .getByRole("button", { name: "Check readiness and review transmission", exact: true })
      .click();
    await page.getByRole("button", { name: "Start simulation workflow", exact: true }).waitFor();
  }
  if (label === "Command Center")
    await page.waitForFunction(() => document.querySelectorAll(".react-flow__node").length === 10);
  if (label === "Settings")
    await page.getByText("No matching diagnostic events.", { exact: true }).waitFor();
}

async function routeControls(page, label) {
  const controls = {
    Conversations: [
      page.getByRole("combobox", { name: /^Conversation mode/ }),
      page.getByLabel("Assistant request", { exact: true }),
      page.getByRole("button", { name: "Send", exact: true }),
      page.getByText("Local mock only · no model or tool execution", { exact: true }),
    ],
    Bots: [
      page.getByLabel("Nickname (optional, 48 characters; blank uses canonical role)", {
        exact: true,
      }),
      page.getByRole("combobox", { name: /^Connection/ }),
      page.getByRole("button", { name: "Save settings and note", exact: true }),
    ],
    Knowledge: [
      page.getByRole("button", { name: "New Markdown note", exact: true }),
      page.getByRole("combobox", { name: "Version", exact: true }),
      page.getByRole("button", { name: "Edit local copy", exact: true }),
    ],
    Collaboration: [
      page.getByRole("combobox", { name: "Workflow", exact: true }),
      page.getByRole("textbox", { name: /^Objective/ }),
      page.getByText(/Each listed provider receives the objective/),
      page.getByRole("button", { name: "Start simulation workflow", exact: true }),
    ],
    "Command Center": [
      page.getByRole("combobox", { name: "Room", exact: true }),
      page.getByRole("textbox", { name: "Search nickname or canonical role", exact: true }),
      page.getByRole("toolbar", { name: "Operational graph viewport", exact: true }),
    ],
    Settings: [
      page.getByRole("checkbox", { name: "Enable bot animations", exact: true }),
      page.getByRole("combobox", { name: "Severity", exact: true }),
      page.getByRole("combobox", { name: "Provider/runtime", exact: true }),
      page.getByRole("combobox", { name: "Request/run ID", exact: true }),
      page.getByRole("button", { name: "Copy troubleshooting summary", exact: true }),
      page.getByRole("button", { name: "Refresh diagnostics", exact: true }),
      page.getByRole("button", { name: "Copy sanitized summary", exact: true }),
      page.getByRole("button", { name: "Export diagnostics", exact: true }),
    ],
  };
  for (const control of controls[label])
    await reachable(
      page,
      control,
      `${label}: ${(await control.getAttribute("aria-label")) ?? (await control.textContent())}`,
    );
  const selectors = page.locator(".application-content select:visible");
  assert.ok(await selectors.count(), `${label} exposes its expected selector controls`);
  for (const size of await selectors.evaluateAll((elements) =>
    elements.map((el) => ({
      height: el.getBoundingClientRect().height,
      label: el.closest("label")?.textContent?.trim() ?? el.getAttribute("aria-label"),
    })),
  ))
    assert.ok(size.height >= 38, `${label} selector minimum 38px: ${size.label}`);
  if (label === "Settings") {
    const diagnosticControls = page
      .locator(".local-diagnostics__controls")
      .locator("select, button");
    assert.equal(
      await diagnosticControls.count(),
      7,
      "All three diagnostic filters and four actions are present",
    );
    for (const size of await diagnosticControls.evaluateAll((elements) =>
      elements.map((el) => ({
        height: el.getBoundingClientRect().height,
        text: el.textContent?.trim(),
      })),
    ))
      assert.ok(size.height >= 38, `Diagnostic action/filter minimum 38px: ${size.text}`);
  }
  if (label === "Conversations") {
    assert.equal(await page.getByLabel("Assistant request", { exact: true }).inputValue(), "");
    assert.equal(await page.getByRole("button", { name: "Send", exact: true }).isEnabled(), false);
  }
  if (label === "Bots")
    assert.equal(
      await page.getByRole("button", { name: "Save settings and note", exact: true }).isEnabled(),
      false,
    );
  if (label === "Collaboration")
    assert.equal(
      await page
        .getByRole("button", { name: "Start simulation workflow", exact: true })
        .isEnabled(),
      false,
    );
}

try {
  if (!supplementalOnly) {
    const page = await newPage({ width: 1600, height: 1000 });
    await page.goto(`${url}?design`);
    await page.getByRole("heading", { name: "Conversations", exact: true }).waitFor();
    const boundary = await fixtureBoundary(page);
    boundaryEvidence = boundary;
    for (const expanded of [true, false]) {
      if (!expanded)
        await page.getByRole("button", { name: "Collapse navigation", exact: true }).click();
      for (const [index, [width, height]] of sizes.entries()) {
        await page.setViewportSize({ width, height });
        for (const [label, slug, heading] of routes) {
          const nav = page.getByRole("navigation", { name: "Primary navigation" });
          await nav.getByRole("button", { name: label, exact: true }).click();
          await routeReady(page, label);
          await reachable(page, page.getByRole("heading", { name: heading, exact: true }), heading);
          await routeControls(page, label);
          await shellGeometry(page, expanded, false);
          if (label === "Conversations") await navigationReachability(page, expanded);
          await page.getByRole("button", { name: "Show workspace inspector", exact: true }).click();
          await shellGeometry(page, expanded, true);
          await routeControls(page, label);
          const close = page.locator("[data-application-inspector-close=true]");
          await close.focus();
          await page.keyboard.press("Escape");
          await page.waitForFunction(
            () =>
              document.activeElement ===
              document.querySelector('button[aria-label="Show workspace inspector"]'),
          );
          assert.equal(await page.locator(".application-inspector").isVisible(), false);
          await shellGeometry(page, expanded, false);
          await wheelBoundary(page, page.locator(".application-content"), false);
          if (expanded && width === 1600 && label === "Command Center") {
            const visibleGraphHeight = await page.locator(".operational-canvas").evaluate((el) => {
              const graph = el.getBoundingClientRect();
              const main = document.querySelector(".application-content").getBoundingClientRect();
              return Math.max(
                0,
                Math.min(graph.bottom, main.bottom) - Math.max(graph.top, main.top),
              );
            });
            assert.ok(
              visibleGraphHeight >= 300,
              `Command Center graph is prominent at initial scroll: ${visibleGraphHeight}px visible`,
            );
          }
          if (expanded && index === 0) await page.screenshot({ path: `${output}/${slug}.png` });
          if (width === 760 && label === "Conversations")
            await page.screenshot({
              path: `${output}/compact-${expanded ? "expanded" : "collapsed"}.png`,
            });
          cases.push({
            route: label,
            width,
            height,
            expanded,
            direction: index < 7 ? "forward" : "reverse",
            inspector: "open/closed",
          });
        }
      }
    }
    assert.deepEqual(
      await fixtureBoundary(page),
      boundary,
      "Presentation interactions preserve all synthetic saved values",
    );
    await page.close();
  }

  // Saves below use only the pre-existing standalone in-memory injected client.
  // The actual App fixture above never accepts save or execution IPC.
  const knowledge = await newPage({ width: 1280, height: 900 });
  await knowledge.goto(url);
  await knowledge.getByRole("button", { name: "New Markdown note", exact: true }).click();
  await knowledge.getByLabel("Note title", { exact: true }).fill("Redesign synthetic note");
  await knowledge
    .getByRole("textbox", { name: /^Markdown content/ })
    .fill("# Review\nSynthetic version one.\n");
  await knowledge.getByRole("button", { name: "Save note version", exact: true }).click();
  await knowledge.getByRole("heading", { name: "Redesign synthetic note", exact: true }).waitFor();
  await knowledge.getByRole("button", { name: "Edit local copy", exact: true }).click();
  await knowledge
    .getByRole("textbox", { name: /^Markdown content/ })
    .fill("# Review\nSynthetic version one.\n\nQA: version two.\n");
  await knowledge.getByRole("button", { name: "Save note version", exact: true }).click();
  await knowledge.getByRole("combobox", { name: "Version", exact: true }).selectOption("1");
  await knowledge.getByText(/Historical version\. Link navigation is unavailable/).waitFor();
  assert.equal(await knowledge.getByText("QA: version two.", { exact: true }).count(), 0);
  await knowledge.getByRole("combobox", { name: "Version", exact: true }).selectOption("2");
  await knowledge.getByText("QA: version two.", { exact: true }).waitFor();
  await knowledge.getByRole("button", { name: "Edit local copy", exact: true }).click();
  await knowledge
    .getByRole("textbox", { name: /^Markdown content/ })
    .fill("Unsaved synthetic edit — discard this.");
  await knowledge.getByRole("button", { name: "Cancel edit", exact: true }).click();
  await knowledge.getByRole("alertdialog", { name: "Unsaved Knowledge edits" }).waitFor();
  await knowledge.getByRole("button", { name: "Discard edits and continue", exact: true }).click();
  await knowledge.getByText("QA: version two.", { exact: true }).waitFor();
  assert.equal(
    await knowledge
      .getByRole("combobox", { name: "Version", exact: true })
      .locator("option")
      .count(),
    2,
  );
  await knowledge.screenshot({ path: `${output}/knowledge-synthetic-versions.png` });
  await knowledge.getByLabel("Search local passages", { exact: true }).fill("version");
  await knowledge.getByRole("button", { name: "Search", exact: true }).click();
  const searchResults = knowledge.getByRole("region", { name: "Search results", exact: true });
  await searchResults
    .getByRole("status")
    .filter({ hasText: /^1 matching passages/ })
    .waitFor();
  assert.match(await searchResults.textContent(), /Redesign synthetic note · version 2/);
  for (const width of [1280, 760]) {
    await knowledge.setViewportSize({ width, height: 900 });
    await searchResults.scrollIntoViewIfNeeded();
    await settle(knowledge);
    assert.equal(
      await searchResults.evaluate((el) => {
        const result = el.getBoundingClientRect();
        const form = document.querySelector(".knowledge-search").getBoundingClientRect();
        return (
          el.previousElementSibling?.matches(".knowledge-search") &&
          result.top >= form.bottom &&
          result.left >= 0 &&
          result.right <= window.innerWidth &&
          document.documentElement.scrollWidth <= window.innerWidth &&
          el.scrollWidth <= el.clientWidth + 1
        );
      }),
      true,
      "Successful search results follow their form and remain contained",
    );
    await knowledge.screenshot({ path: `${output}/knowledge-search-${width}.png` });
  }
  await knowledge.close();

  const motion = await newPage({ width: 1280, height: 900 });
  await motion.goto(`${url}?design`);
  await motion
    .getByRole("navigation", { name: "Primary navigation" })
    .getByRole("button", { name: "Bots", exact: true })
    .click();
  await routeReady(motion, "Bots");
  const preview = motion.locator(".bot-preview .bot-mascot-space");
  const play = motion.getByRole("button", { name: "Playful spin", exact: true });
  await preview.scrollIntoViewIfNeeded();
  await motion.waitForFunction(
    () =>
      document.querySelector(".bot-preview .bot-mascot-space")?.getAttribute("data-motion") ===
      "static",
  );
  assert.equal(await play.isEnabled(), false, "Reduced motion disables the decorative action");
  assert.equal(await preview.evaluate((el) => el.getAnimations({ subtree: true }).length), 0);
  await motion.emulateMedia({ reducedMotion: "no-preference" });
  await motion.waitForFunction(
    () =>
      document.querySelector(".bot-preview .bot-mascot-space")?.getAttribute("data-motion") ===
      "idle",
  );
  await play.click();
  assert.equal(await preview.getAttribute("data-motion"), "playful");
  assert.equal(await play.isEnabled(), false, "Active finite reaction cannot queue another spin");
  assert.ok(
    await preview.evaluate((el) =>
      el
        .getAnimations({ subtree: true })
        .some(
          (animation) =>
            animation.animationName === "mascot-spin" && animation.playState === "running",
        ),
    ),
  );
  await motion.waitForFunction(
    () =>
      document.querySelector(".bot-preview .bot-mascot-space")?.getAttribute("data-motion") ===
      "idle",
  );
  assert.equal(await play.isEnabled(), true);
  await play.click();
  await motion.emulateMedia({ reducedMotion: "reduce" });
  await motion.waitForFunction(
    () =>
      document.querySelector(".bot-preview .bot-mascot-space")?.getAttribute("data-motion") ===
      "static",
  );
  assert.equal(
    await preview.evaluate((el) => el.getAnimations({ subtree: true }).length),
    0,
    "Reduced motion cancels finite motion and leaves no queued animation",
  );
  const themes = [];
  for (const colorScheme of ["light", "dark"]) {
    await motion.emulateMedia({ colorScheme });
    await settle(motion);
    const colors = await motion.evaluate(() => {
      const style = getComputedStyle(document.documentElement);
      return { background: style.backgroundColor, color: style.color, scheme: style.colorScheme };
    });
    assert.equal(colors.scheme, "dark", "Cortexa retains its supported dark presentation");
    assert.equal(colors.background, "rgb(16, 17, 22)");
    assert.equal(colors.color, "rgb(243, 243, 247)");
    themes.push({ osPreference: colorScheme, ...colors });
  }
  await motion.screenshot({ path: `${output}/reduced-motion-preview.png` });
  await motion.close();
  assert.deepEqual(networkViolations, [], "No requests outside the loopback fixture origin");
  assert.deepEqual(pageErrors, [], "No uncaught browser errors");
  await writeFile(
    `${output}/results.json`,
    JSON.stringify(
      {
        mode: supplementalOnly ? "supplemental" : "full",
        cases,
        boundary: boundaryEvidence,
        syntheticVersions: 2,
        searchWidths: [1280, 760],
        normalMotion: "finite spin returns idle; no queue",
        reducedMotion: "static; no CSS animations",
        themes,
        networkViolations,
        pageErrors,
      },
      null,
      2,
    ),
  );
  console.log(
    supplementalOnly
      ? "UI/UX supplemental PASS: standalone two-version/discard and search containment, finite normal motion, reduced-motion cancellation and dark presentation under both OS color preferences; layout matrix not repeated."
      : `UI/UX actual-App PASS: ${cases.length} route/resize/navigation cases, open/closed inspectors, wheel/keyboard reachability, read-only synthetic boundaries, standalone two-version/discard/search, motion and OS color-preference checks.`,
  );
} catch (error) {
  if (currentPage && !currentPage.isClosed()) {
    await currentPage.screenshot({ path: `${output}/failure.png` });
    await writeFile(
      `${output}/failure.json`,
      JSON.stringify({ cases, message: error.message, networkViolations, pageErrors }, null, 2),
    );
  }
  throw error;
} finally {
  await browser.close();
}
