import { chromium } from "playwright";
import assert from "node:assert/strict";
/* global document */
import { mkdir } from "node:fs/promises";
const output = process.argv[2];
if (!output) throw Error("Pass an external screenshot directory");
await mkdir(output, { recursive: true });
const browser = await chromium.launch({ headless: true });
try {
  for (const [label, width, height] of [
    ["laptop", 1280, 800],
    ["desktop", 1600, 1000],
    ["wide", 2200, 1200],
  ]) {
    const page = await browser.newPage({ viewport: { width, height } });
    await page.goto(
      "http://127.0.0.1:4173/scripts/browser/operational.html" + (label === "wide" ? "?long" : ""),
    );
    const hiddenCanvas = await page.addStyleTag({
      content: ".operational-canvas { width:0 !important; height:0 !important; }",
    });
    await page.getByRole("button", { name: "Command Center", exact: true }).click();
    await page
      .getByText("Waiting for graph canvas and node measurements.", { exact: true })
      .waitFor();
    assert.equal(
      await page.getByRole("button", { name: "Fit visible", exact: true }).isEnabled(),
      false,
    );
    await hiddenCanvas.evaluate((el) => el.remove());
    await page.getByRole("button", { name: "Fit visible", exact: true }).waitFor();
    await page.getByRole("button", { name: "Fit visible", exact: true }).isEnabled();
    await page.waitForFunction(() => {
      const canvas = document.querySelector(".operational-canvas")?.getBoundingClientRect();
      const nodes = [...document.querySelectorAll(".react-flow__node")].map((n) =>
        n.getBoundingClientRect(),
      );
      return (
        canvas &&
        nodes.length === 10 &&
        nodes.every(
          (n) =>
            n.left >= canvas.left &&
            n.right <= canvas.right &&
            n.top >= canvas.top &&
            n.bottom <= canvas.bottom,
        )
      );
    });
    await page.screenshot({ path: `${output}/${label}-roster.png`, fullPage: true });
    for (const [i, route] of ["research", "engineering", "operations", "workflow"].entries()) {
      await page.getByRole("button", { name: "Selected collaboration run", exact: true }).click();
      await page.getByRole("combobox", { name: "Room", exact: true }).selectOption(String(i + 1));
      await page
        .getByRole("combobox", { name: "Run", exact: true })
        .selectOption(`room-${i + 1}/run-1`);
      await page.getByRole("button", { name: "Fit visible", exact: true }).click();
      assert.equal(await page.locator(".operational-card--bot").count(), 9);
      assert.equal(
        await page.locator(".operational-card--stage").count(),
        ["engineering", "operations"].includes(route) ? 5 : 4,
      );
      assert.equal(await page.getByText("PRIVATE SENTINEL DO NOT PROJECT").count(), 0);
      await page.locator(".operational-card--stage").first().click();
      await page.getByRole("button", { name: "Open in room", exact: true }).click();
      await page.getByRole("heading", { name: `Synthetic ${route}`, exact: true }).waitFor();
      assert.equal(await page.locator(":focus").getAttribute("id"), `room-${i + 1}/run-1/stage/0`);
      await page.getByRole("button", { name: "Inspect run in Command Center" }).first().click();
      await page.getByRole("button", { name: "Fit visible", exact: true }).click();
    }
    await page
      .getByRole("textbox", { name: "Search nickname or canonical role" })
      .fill("no-matching-bot");
    await page
      .getByText("No matching nodes. Clear filters to restore the graph.", { exact: true })
      .waitFor();
    await page.getByRole("button", { name: "Clear filters", exact: true }).click();
    await page
      .getByRole("combobox", { name: "Participant", exact: true })
      .selectOption("personal-assistant");
    assert.equal(await page.locator(".operational-card--stage").count(), 2);
    await page.getByRole("button", { name: "Clear filters", exact: true }).click();
    await page.getByRole("combobox", { name: "Domain", exact: true }).selectOption("engineering");
    assert.equal(await page.locator(".operational-card--bot").count(), 2);
    await page.getByRole("button", { name: "Clear filters", exact: true }).click();
    await page
      .getByRole("region", {
        name: "Operational graph; arrows select, Enter focuses, Escape clears",
      })
      .press("Home");
    assert.equal(
      await page.locator(".react-flow__node.selected").getAttribute("data-id"),
      "AgentOrchestrator",
    );
    await page
      .getByRole("region", {
        name: "Operational graph; arrows select, Enter focuses, Escape clears",
      })
      .press("End");
    const inspectorToggle = page.getByRole("button", {
      name: "Show workspace inspector",
      exact: true,
    });
    if (await inspectorToggle.count()) await inspectorToggle.click();
    await page.getByRole("button", { name: "Fit visible", exact: true }).click();
    assert.equal(await page.getByText(/You can then press delete to remove it/).count(), 0);
    assert.equal(
      await page
        .getByText("Read-only information flow. Edges cannot be edited or deleted.")
        .count(),
      1,
    );
    assert.ok(
      await page.evaluate(() => {
        const main = document.querySelector(".application-main").getBoundingClientRect();
        const canvas = document.querySelector(".operational-canvas").getBoundingClientRect();
        return (
          canvas.right <= main.right &&
          canvas.right <=
            document.querySelector(".application-inspector").getBoundingClientRect().left &&
          [
            ...document.querySelectorAll(
              ".operational-controls button, .operational-controls select",
            ),
          ].every((el) => el.getBoundingClientRect().right <= main.right)
        );
      }),
      "Inspector must not cover the canvas or controls",
    );
    await page.screenshot({ path: `${output}/${label}-run.png`, fullPage: true });
    await page.close();
  }
  const page = await browser.newPage({ viewport: { width: 1440, height: 1000 } });
  await page.goto("http://127.0.0.1:4173/scripts/browser/operational.html?active");
  await page.getByRole("button", { name: "Command Center", exact: true }).click();
  await page.getByRole("button", { name: "Selected collaboration run", exact: true }).click();
  await page.getByRole("combobox", { name: "Room", exact: true }).selectOption("1");
  await page.getByRole("combobox", { name: "Run", exact: true }).selectOption("room-1/run-1");
  await page.getByRole("button", { name: "Fit visible", exact: true }).click();
  await page.getByRole("button", { name: "Zoom in", exact: true }).click();
  await page.locator(".operational-card--stage").first().click();
  await page
    .getByRole("heading", { name: "Provisional output — not a validated result" })
    .waitFor();
  const viewport = page.locator(".react-flow__viewport");
  const beforePan = await viewport.getAttribute("style");
  const box = await page.locator(".operational-canvas").boundingBox();
  assert.ok(box);
  await page.mouse.move(box.x + 25, box.y + 25);
  await page.mouse.down();
  await page.mouse.move(box.x + 70, box.y + 70, { steps: 5 });
  await page.mouse.up();
  assert.notEqual(await viewport.getAttribute("style"), beforePan);
  const beforeWheel = await viewport.getAttribute("style");
  await page.mouse.wheel(0, -100);
  await page.waitForFunction(
    (expected) =>
      document.querySelector(".react-flow__viewport")?.getAttribute("style") !== expected,
    beforeWheel,
  );
  // Finish wheel interaction before exercising a subsequent independent snapshot.
  await page.getByRole("button", { name: "Zoom in", exact: true }).click();
  const manual = await viewport.getAttribute("style");
  await page.getByRole("button", { name: "Advance synthetic snapshot", exact: true }).click();
  await page.locator('.operational-card--stage[data-status="cancelled"]').first().waitFor();
  assert.equal(await viewport.getAttribute("style"), manual, "snapshot reset manual viewport");
  await page.getByRole("button", { name: "Open in room", exact: true }).click();
  await page.getByRole("button", { name: "Inspect run in Command Center" }).first().click();
  assert.equal(
    await page.getByRole("combobox", { name: "Run", exact: true }).inputValue(),
    "room-1/run-1",
  );
  await page.getByRole("button", { name: "Fit visible", exact: true }).waitFor();
  await page.waitForFunction(
    (expected) =>
      document.querySelector(".react-flow__viewport")?.getAttribute("style") === expected,
    manual,
  );
  await page.getByRole("button", { name: "Toggle unavailable snapshots", exact: true }).click();
  await page.getByRole("button", { name: "Refresh snapshots", exact: true }).click();
  await page.getByText(/Last accepted data; current state unknown/).waitFor();
  await page.getByRole("button", { name: "Toggle unavailable snapshots", exact: true }).click();
  await page.getByRole("button", { name: "Remove synthetic rooms", exact: true }).click();
  await page.getByRole("button", { name: "Refresh snapshots", exact: true }).click();
  await page.getByText(/A deleted room or missing run is never replaced/).waitFor();
  await page.close();
  console.log(
    "Real Chromium: all four routes, initial fit, filters, keyboard, stage navigation, polling, cancellation projection, stale/deleted data, viewport preservation and screenshots passed",
  );
} finally {
  await browser.close();
}
