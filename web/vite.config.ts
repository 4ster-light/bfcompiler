import { defineConfig } from "vite"

export default defineConfig({
	server: {
		// Playwright's webServer health check uses 127.0.0.1.
		host: "127.0.0.1"
	},
	build: {
		target: "es2024",
		sourcemap: true
	}
})
