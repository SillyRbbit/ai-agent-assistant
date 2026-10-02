import { chromium } from "playwright";
import assert from "node:assert/strict";
import { mkdir } from "node:fs/promises";
const output = process.argv[2];
if (!output) throw Error("External evidence directory required");
await mkdir(output, { recursive: true });
const browser = await chromium.launch({ headless: true });
try {
  for (const width of [1280, 760]) {
    const page = await browser.newPage({ viewport: { width, height: 1000 } });
    await page.goto("http://127.0.0.1:4175/scripts/browser/knowledge.html");
    const notes = page.getByRole("complementary", { name: "Knowledge items" });
    async function create(title, content) {
      await page.getByRole("button", { name: "New Markdown note", exact: true }).click();
      await page.getByRole("textbox", { name: "Note title", exact: true }).fill(title);
      await page.getByRole("textbox", { name: "Markdown content", exact: true }).fill(content);
      await page.getByRole("button", { name: "Save note version", exact: true }).click();
      await page.getByRole("heading", { name: title, level: 2, exact: true }).waitFor();
    }
    await create("Runbook", "# Recovery\n\n1. Inspect.\n2. Record.");
    await create("Incident", "# Incident\n\n- [x] Reviewed\n\n[[Runbook|Recovery steps]]");
    await page
      .getByRole("button", { name: "Fit linked notes", exact: true })
      .click({ timeout: 5000 });
    assert.equal(await page.locator(".react-flow__node").count(), 2);
    for (const label of ["Zoom In", "Zoom Out", "Fit View"]) {
      const icon = await page
        .getByRole("button", { name: label, exact: true })
        .locator("svg")
        .boundingBox();
      assert.ok(
        icon && icon.width >= 10 && icon.height >= 10,
        `${label} icon must remain visibly sized at ${width}px`,
      );
    }
    assert.equal(
      await page.locator(".knowledge-markdown").getByText("[x]", { exact: true }).count(),
      0,
    );
    await page.getByRole("button", { name: "Reset knowledge viewport", exact: true }).click();
    await page.getByRole("button", { name: "Fit linked notes", exact: true }).click();
    await page.getByRole("button", { name: "Zoom Out", exact: true }).click();
    await page.getByRole("button", { name: "Recovery steps", exact: true }).click();
    await page
      .getByRole("region", { name: "Backlinks", exact: true })
      .getByRole("button", { name: "Incident", exact: true })
      .waitFor();
    await page.getByRole("button", { name: "Edit local copy", exact: true }).click();
    await page.getByRole("textbox", { name: "Note title", exact: true }).fill("Renamed runbook");
    await page.getByRole("button", { name: "Save note version", exact: true }).click();
    await notes.getByRole("button", { name: /^Incident/ }).click();
    await page.getByRole("button", { name: "Recovery steps", exact: true }).click();
    await page.getByRole("heading", { name: "Renamed runbook", level: 2, exact: true }).waitFor();
    await page
      .getByRole("navigation", { name: "Linked notes", exact: true })
      .getByRole("button", { name: "Incident", exact: true })
      .click();
    await page.screenshot({ path: `${output}/connected-${width}.png`, fullPage: true });
    await page.getByRole("button", { name: "Edit local copy", exact: true }).click();
    await page.getByRole("textbox", { name: "Markdown content", exact: true }).fill("UNSAVED");

    await notes.getByRole("button", { name: /^Renamed runbook/ }).click();
    assert.equal(
      await page.getByRole("textbox", { name: "Markdown content", exact: true }).inputValue(),
      "UNSAVED",
    );
    await page.getByRole("button", { name: "Keep editing", exact: true }).click();
    await page.getByRole("button", { name: "Cancel edit", exact: true }).click();
    await page.getByRole("button", { name: "Discard edits and continue", exact: true }).click();
    await page.close();
  }
  console.log(
    "Connected Knowledge browser: Markdown, measured graph controls, backlinks, rename binding, graph navigation and unsaved protection passed at 1280/760.",
  );
} finally {
  await browser.close();
}
