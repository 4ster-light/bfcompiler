import AxeBuilder from "@axe-core/playwright"
import { expect, test } from "@playwright/test"

test.beforeEach(async ({ page }) => {
	await page.goto("/")
	await expect(page.getByRole("button", { name: "Run program" })).toBeVisible()
})

test("runs the hello world example and updates output and memory", async ({
	page
}) => {
	await page.getByRole("button", { name: "Run program" }).click()
	await expect(page.locator("#status-label")).toHaveText("Finished")
	await expect(page.locator("#output")).toHaveText("Hello World!\n")
	await expect(page.locator("#step-count")).not.toHaveText(
		"0 instructions executed"
	)
	await expect(page.locator("#copy")).toBeEnabled()
})

test("passes UTF-8 input to an echo program", async ({ page }) => {
	await page.locator("#example").selectOption("echo")
	await page.locator("#input").fill("héllo")
	await page.getByRole("button", { name: "Run program" }).click()
	await expect(page.locator("#output")).toHaveText("héllo")
})

test("single stepping is resumable and updates the pointer", async ({
	page
}) => {
	await page.locator("#example").selectOption("blank")
	await page.locator("#code").fill("+++.")
	await page.getByRole("button", { name: "Step", exact: true }).click()
	await expect(page.locator("#status-label")).toHaveText("Paused")
	await expect(page.locator("#step-count")).toHaveText("1 instruction executed")
	await expect(page.locator(".memory-cell.current .cell-value")).toHaveText("1")
	await page.getByRole("button", { name: "Step", exact: true }).click()
	await expect(page.locator(".memory-cell.current .cell-value")).toHaveText("2")
})

test("runs the alphabet example", async ({ page }) => {
	await page.locator("#example").selectOption("alphabet")
	await page.getByRole("button", { name: "Run program" }).click()
	await expect(page.locator("#output")).toHaveText("ABCDEFGHIJKLMNOPQRSTUVWXYZ")
})

test("can pause an infinite program without blocking the page", async ({
	page
}) => {
	await page.locator("#code").fill("+[]")
	await page.getByRole("button", { name: "Run program" }).click()
	await expect(
		page.getByRole("button", { name: "Pause", exact: true })
	).toBeVisible()
	await page.getByRole("button", { name: "Pause", exact: true }).click()
	await expect(page.locator("#status-label")).toHaveText("Paused")
	await expect(page.getByRole("button", { name: "Continue" })).toBeEnabled()
})

test("reports malformed programs without throwing a page error", async ({
	page
}) => {
	const errors: string[] = []
	page.on("pageerror", (error) => errors.push(error.message))
	await page.locator("#code").fill("+++[")
	await page.getByRole("button", { name: "Run program" }).click()
	await expect(page.locator("#status-label")).toHaveText("Error")
	await expect(page.locator("#output")).toContainText("Mismatched bracket")
	expect(errors).toEqual([])
})

test("supports the run keyboard shortcut and reset", async ({ page }) => {
	await page.locator("#code").focus()
	await page.keyboard.press("Control+Enter")
	await expect(page.locator("#status-label")).toHaveText("Finished")
	await page.getByRole("button", { name: "Reset" }).click()
	await expect(page.locator("#status-label")).toHaveText("Ready")
	await expect(page.locator("#output")).toContainText(
		"Program output will appear here."
	)
	await expect(page.locator("#code")).not.toBeEmpty()
})

test("defaults to dark and remembers a theme choice", async ({ page }) => {
	await expect(page.locator("html")).toHaveAttribute("data-theme", "dark")
	await page.getByRole("button", { name: "Switch to light theme" }).click()
	await expect(page.locator("html")).toHaveAttribute("data-theme", "light")
	await expect(
		page.getByRole("button", { name: "Switch to dark theme" })
	).toBeVisible()
	await page.reload()
	await expect(page.locator("html")).toHaveAttribute("data-theme", "light")
	await page.getByRole("button", { name: "Switch to dark theme" }).click()
	await expect(page.locator("html")).toHaveAttribute("data-theme", "dark")
})

test("has no automatically detectable accessibility violations in either theme", async ({
	page
}) => {
	const audit = () =>
		new AxeBuilder({ page })
			.withTags(["wcag2a", "wcag2aa", "wcag21aa"])
			.analyze()

	expect((await audit()).violations).toEqual([])
	await page.getByRole("button", { name: "Switch to light theme" }).click()
	expect((await audit()).violations).toEqual([])
	await page.getByRole("button", { name: "Run program" }).click()
	await expect(page.locator("#status-label")).toHaveText("Finished")
	expect((await audit()).violations).toEqual([])
	await page.getByRole("button", { name: "Switch to dark theme" }).click()
	expect((await audit()).violations).toEqual([])
})
