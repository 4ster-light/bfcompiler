import process from "node:process"
import { defineConfig, devices } from "@playwright/test"

export default defineConfig({
	testDir: "./tests",
	fullyParallel: true,
	retries: 0,
	use: {
		baseURL: "http://127.0.0.1:5173",
		trace: "retain-on-failure",
		screenshot: "only-on-failure"
	},
	projects: [
		{
			name: "chromium",
			use: { ...devices["Desktop Chrome"] }
		},
		{
			name: "mobile",
			use: { ...devices["iPhone 13"], browserName: "chromium" }
		}
	],
	webServer: {
		command: "pnpm run dev",
		url: "http://127.0.0.1:5173",
		reuseExistingServer: !process.env.CI
	}
})
