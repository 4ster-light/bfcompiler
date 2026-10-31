import "./style.css" with { type: "css" }
import init, { tape_size, WebInterpreter } from "./wasm/bf_web.js"

await init()

const HELLO_WORLD =
	"++++++++++[>+++++++>++++++++++>+++>+<<<<-]>++.>+.+++++++..+++.>++.<<+++++++++++++++.>.+++.------.--------.>+.>."
const ALPHABET = "++++++++[>++++++++<-]>+<++++++++++++++++++++++++++[>.+<-]"
const MAX_STEPS = 10_000_000
const PAGE_SIZE = 16
const decoder = new TextDecoder()
const encoder = new TextEncoder()

const examples: Record<string, { source: string; input: string }> = {
	hello: { source: HELLO_WORLD, input: "" },
	echo: { source: ",[.,]", input: "Hello, Brainfuck!" },
	alphabet: { source: ALPHABET, input: "" },
	blank: { source: "", input: "" }
}

function element<T extends HTMLElement>(id: string): T {
	const found = document.getElementById(id)
	if (!found) throw new Error(`Missing UI element: ${id}`)
	return found as T
}

const code = element<HTMLTextAreaElement>("code")
const input = element<HTMLTextAreaElement>("input")
const example = element<HTMLSelectElement>("example")
const output = element<HTMLElement>("output")
const runButton = element<HTMLButtonElement>("run")
const stepButton = element<HTMLButtonElement>("step")
const resetButton = element<HTMLButtonElement>("reset")
const copyButton = element<HTMLButtonElement>("copy")
const downloadButton = element<HTMLButtonElement>("download")
const statusBadge = element("status-badge")
const statusLabel = element("status-label")
const stepCount = element("step-count")
const elapsedLabel = element("elapsed")
const memoryTape = element("memory-tape")
const pointerCell = element("pointer-cell")
const memoryPrevious = element<HTMLButtonElement>("memory-previous")
const memoryNext = element<HTMLButtonElement>("memory-next")
const toast = element("toast")
const announcer = element("announcer")
const themeToggle = element<HTMLButtonElement>("theme-toggle")
const themeLabel = element("theme-label")
const themeColor = document.querySelector<HTMLMetaElement>(
	'meta[name="theme-color"]'
)

type Theme = "dark" | "light"

let machine: WebInterpreter | null = null
let state: "ready" | "running" | "paused" | "finished" | "error" = "ready"
let timer: ReturnType<typeof setTimeout> | undefined
let runGeneration = 0
let memoryStart = 0
let elapsedMs = 0
let stepLimit = MAX_STEPS
let batchStartedAt = 0
let toastTimer: ReturnType<typeof setTimeout> | undefined
const memorySize = tape_size()

function notify(message: string): void {
	toast.textContent = message
	toast.classList.add("visible")
	clearTimeout(toastTimer)
	toastTimer = setTimeout(() => toast.classList.remove("visible"), 2400)
}

function announce(message: string): void {
	announcer.textContent = ""
	requestAnimationFrame(() => {
		announcer.textContent = message
	})
}

function applyTheme(theme: Theme, persist = false): void {
	document.documentElement.dataset.theme = theme
	themeLabel.textContent = theme === "dark" ? "Light" : "Dark"
	const action =
		theme === "dark" ? "Switch to light theme" : "Switch to dark theme"
	themeToggle.setAttribute("aria-label", action)
	themeToggle.title = action
	if (themeColor) themeColor.content = theme === "dark" ? "#101713" : "#f5f4ee"

	if (persist) {
		try {
			localStorage.setItem("bf-theme", theme)
		} catch {
			// The theme still applies for this page even when storage is unavailable.
		}
		announce(`${theme === "dark" ? "Dark" : "Light"} theme enabled.`)
	}
}

function setState(next: typeof state): void {
	state = next
	const labels = {
		ready: "Ready",
		running: "Running",
		paused: "Paused",
		finished: "Finished",
		error: "Error"
	}
	statusLabel.textContent = labels[next]
	statusBadge.className = `status-badge ${next}`
	stepButton.disabled = next === "running"
	runButton.classList.toggle("is-running", next === "running")

	if (next === "running") {
		runButton.innerHTML =
			'<svg aria-hidden="true" viewBox="0 0 24 24"><path d="M8 5v14M16 5v14"/></svg><span>Pause</span>'
		runButton.title = "Pause execution (Escape)"
	} else {
		const label = next === "paused" ? "Continue" : "Run program"
		runButton.innerHTML = `<svg aria-hidden="true" viewBox="0 0 24 24"><path class="run-icon" d="m8 5 11 7-11 7V5Z"/></svg><span>${label}</span>`
		runButton.title = "Run program (Ctrl+Enter)"
	}
}

function renderOutput(bytes: Uint8Array, running = false): void {
	output.classList.remove("error-output")
	if (bytes.length === 0) {
		output.innerHTML = running
			? '<span class="output-placeholder">Program is running…</span>'
			: '<span class="output-placeholder">Program output will appear here.<br />Choose <strong>Run program</strong> to execute the current code.</span>'
	} else {
		output.textContent = decoder.decode(bytes)
	}
	copyButton.disabled = bytes.length === 0
	downloadButton.disabled = bytes.length === 0
}

function renderStats(steps = 0): void {
	stepCount.textContent = `${steps.toLocaleString()} instruction${steps === 1 ? "" : "s"} executed`
	elapsedLabel.textContent =
		elapsedMs < 1000
			? `${Math.round(elapsedMs)} ms`
			: `${(elapsedMs / 1000).toFixed(2)} s`
}

function renderMemory(): void {
	const pointer = machine?.pointer ?? 0
	const values =
		machine?.memory(memoryStart, PAGE_SIZE) ?? new Uint8Array(PAGE_SIZE)
	const fragment = document.createDocumentFragment()

	for (let index = 0; index < PAGE_SIZE; index += 1) {
		const address = memoryStart + index
		const value = values[index] ?? 0
		const cell = document.createElement("li")
		cell.className = `memory-cell${address === pointer ? " current" : ""}${value !== 0 ? " nonzero" : ""}`
		cell.setAttribute(
			"aria-label",
			`Cell ${address}: ${value}${address === pointer ? ", current pointer" : ""}`
		)
		cell.innerHTML = `<span class="cell-index">${String(address).padStart(4, "0")}</span><span class="cell-value">${value}</span>`
		fragment.append(cell)
	}

	memoryTape.replaceChildren(fragment)
	pointerCell.textContent = `cell ${pointer}`
	memoryPrevious.disabled = memoryStart === 0
	memoryNext.disabled = memoryStart + PAGE_SIZE >= memorySize
}

function renderMachine(running = false): void {
	if (!machine) return
	renderOutput(machine.output, running)
	renderMemory()
	renderStats(Number(machine.steps))
}

function updateEditorMeta(): void {
	const lines = code.value.split("\n")
	element("line-numbers").textContent = lines
		.map((_, index) => index + 1)
		.join("\n")
	element("instruction-count").textContent = String(
		(code.value.match(/[><+\-.,[\]]/g) ?? []).length
	)
	const beforeCursor = code.value.slice(0, code.selectionStart)
	const cursorLines = beforeCursor.split("\n")
	element("cursor-position").textContent =
		`Ln ${cursorLines.length}, Col ${(cursorLines.at(-1)?.length ?? 0) + 1}`
}

function stopTimer(): void {
	clearTimeout(timer)
	timer = undefined
	if (state === "running" && batchStartedAt) {
		elapsedMs += performance.now() - batchStartedAt
		batchStartedAt = 0
	}
}

function releaseMachine(): void {
	machine?.free()
	machine = null
}

function resetExecution(announceReset = false): void {
	runGeneration += 1
	stopTimer()
	releaseMachine()
	elapsedMs = 0
	stepLimit = MAX_STEPS
	memoryStart = 0
	setState("ready")
	renderOutput(new Uint8Array())
	renderStats()
	renderMemory()
	if (announceReset)
		announce("Execution reset. Program and input are unchanged.")
}

function createMachine(): boolean {
	try {
		releaseMachine()
		machine = new WebInterpreter(code.value, encoder.encode(input.value))
		memoryStart = Math.floor(machine.pointer / PAGE_SIZE) * PAGE_SIZE
		elapsedMs = 0
		output.classList.remove("error-output")
		return true
	} catch (error) {
		showError(error)
		return false
	}
}

function showError(error: unknown): void {
	stopTimer()
	const message = error instanceof Error ? error.message : String(error)
	setState("error")
	output.classList.add("error-output")
	output.textContent = `Could not run this program\n\n${message}`
	copyButton.disabled = true
	downloadButton.disabled = true
	renderMemory()
	renderStats(machine ? Number(machine.steps) : 0)
	announce(`Program error: ${message}`)
}

function complete(): void {
	stopTimer()
	setState("finished")
	renderMachine()
	announce(
		`Program finished. ${Number(machine?.steps ?? 0).toLocaleString()} instructions executed.`
	)
}

function runBatch(generation: number): void {
	if (!machine || generation !== runGeneration || state !== "running") return

	try {
		const status = machine.run(50_000)
		if (status === "finished") {
			complete()
			return
		}

		if (Number(machine.steps) >= stepLimit) {
			stepLimit += MAX_STEPS
			stopTimer()
			setState("paused")
			renderMachine()
			notify("Instruction limit reached. Continue to run another batch.")
			announce("Program paused at the instruction limit.")
			return
		}

		renderMachine(true)
		timer = setTimeout(() => runBatch(generation), 0)
	} catch (error) {
		showError(error)
	}
}

function startRunning(): void {
	if (state === "running") {
		stopTimer()
		setState("paused")
		renderMachine()
		announce("Program paused.")
		return
	}

	if (!machine || state === "finished" || state === "error") {
		resetExecution()
		if (!createMachine()) return
	}

	setState("running")
	batchStartedAt = performance.now()
	const generation = ++runGeneration
	runBatch(generation)
}

function stepOnce(): void {
	if (state === "running") return
	if (!machine || state === "finished" || state === "error") {
		resetExecution()
		if (!createMachine()) return
	}

	const currentMachine = machine
	if (!currentMachine) return
	const started = performance.now()
	try {
		const status = currentMachine.step()
		elapsedMs += performance.now() - started
		setState(status === "finished" ? "finished" : "paused")
		renderMachine()
		announce(
			status === "finished"
				? "Program finished."
				: `Stepped. Pointer at cell ${currentMachine.pointer}.`
		)
	} catch (error) {
		showError(error)
	}
}

function loadExample(name: string): void {
	const selected = examples[name]
	if (!selected) return
	resetExecution()
	code.value = selected.source
	input.value = selected.input
	updateEditorMeta()
	updateInputCount()
	code.focus()
}

function updateInputCount(): void {
	element("input-count").textContent =
		`${encoder.encode(input.value).length} bytes`
}

function outputBlob(): Blob {
	const bytes = machine?.output ?? new Uint8Array()
	return new Blob([bytes.slice().buffer as ArrayBuffer], {
		type: "application/octet-stream"
	})
}

function downloadBlob(blob: Blob, filename: string): void {
	const url = URL.createObjectURL(blob)
	const link = document.createElement("a")
	link.href = url
	link.download = filename
	link.click()
	setTimeout(() => URL.revokeObjectURL(url), 1000)
}

runButton.addEventListener("click", startRunning)
stepButton.addEventListener("click", stepOnce)
resetButton.addEventListener("click", () => resetExecution(true))
example.addEventListener("change", () => loadExample(example.value))
code.addEventListener("input", () => {
	resetExecution()
	updateEditorMeta()
})
code.addEventListener("click", updateEditorMeta)
code.addEventListener("keyup", updateEditorMeta)
code.addEventListener("select", updateEditorMeta)
code.addEventListener("scroll", () => {
	element("line-numbers").scrollTop = code.scrollTop
})
input.addEventListener("input", () => {
	resetExecution()
	updateInputCount()
})

code.addEventListener("keydown", (event) => {
	if (event.key === "Tab") {
		event.preventDefault()
		code.setRangeText("  ", code.selectionStart, code.selectionEnd, "end")
		code.dispatchEvent(new Event("input"))
	}
})

document.addEventListener("keydown", (event) => {
	if ((event.ctrlKey || event.metaKey) && event.key === "Enter") {
		event.preventDefault()
		if (event.shiftKey) stepOnce()
		else startRunning()
	} else if (event.key === "Escape" && state === "running") {
		event.preventDefault()
		startRunning()
	}
})

document
	.querySelectorAll<HTMLButtonElement>("[data-command]")
	.forEach((button) => {
		button.addEventListener("click", () => {
			const command = button.dataset.command ?? ""
			code.setRangeText(command, code.selectionStart, code.selectionEnd, "end")
			code.dispatchEvent(new Event("input"))
			code.focus()
		})
	})

themeToggle.addEventListener("click", () => {
	applyTheme(
		document.documentElement.dataset.theme === "light" ? "dark" : "light",
		true
	)
})

document.getElementById("save")?.addEventListener("click", () => {
	downloadBlob(
		new Blob([code.value], { type: "text/plain;charset=utf-8" }),
		"main.bf"
	)
	notify("Program saved as main.bf")
})

copyButton.addEventListener("click", async () => {
	try {
		await navigator.clipboard.writeText(decoder.decode(machine?.output))
		notify("Output copied to clipboard")
	} catch {
		notify("Could not access the clipboard. Select the output to copy it.")
		output.focus()
	}
})

downloadButton.addEventListener("click", () => {
	downloadBlob(outputBlob(), "output.txt")
	notify("Output downloaded")
})

memoryPrevious.addEventListener("click", () => {
	memoryStart = Math.max(0, memoryStart - PAGE_SIZE)
	renderMemory()
})
memoryNext.addEventListener("click", () => {
	memoryStart = Math.min(memorySize - PAGE_SIZE, memoryStart + PAGE_SIZE)
	renderMemory()
})
element("jump-pointer").addEventListener("click", () => {
	memoryStart = Math.floor((machine?.pointer ?? 0) / PAGE_SIZE) * PAGE_SIZE
	renderMemory()
})

applyTheme(
	document.documentElement.dataset.theme === "light" ? "light" : "dark"
)
code.value = HELLO_WORLD
updateEditorMeta()
updateInputCount()
resetExecution()
renderMemory()
