class MemoryOutOfBounds extends Error {
  constructor() {
    super("Memory access out of bounds")
  }
}

class UnmatchedBracket extends Error {
  constructor(message: string) {
    super(message)
  }
}

const MAX_PROG_SIZE = 30_000

function checkBounds(ptr: number, array: Uint8Array): void {
  if (ptr < 0 || ptr >= array.length) throw new MemoryOutOfBounds()
}

function findMatchingBrackets(code: string): Map<number, number> {
  const brackets = new Map<number, number>()
  const stack: number[] = []

  for (const [i, char] of code.split("").entries()) {
    if (char === "[") stack.push(i)
    else if (char === "]") {
      if (stack.length === 0) {
        throw new UnmatchedBracket("Unmatched closing bracket")
      }
      const openPos = stack.pop()!
      brackets.set(openPos, i)
      brackets.set(i, openPos)
    }
  }

  if (stack.length > 0) throw new UnmatchedBracket("Unmatched opening bracket")

  return brackets
}

async function interpretBF(code: string): Promise<void> {
  const array = new Uint8Array(MAX_PROG_SIZE)
  let ptr = 0
  let codePtr = 0
  const brackets = findMatchingBrackets(code)

  while (codePtr < code.length) {
    checkBounds(ptr, array)

    const char = code[codePtr]
    switch (char) {
      case "+":
        array[ptr]++
        break
      case "-":
        array[ptr]--
        break
      case "<":
        ptr = Math.max(0, ptr - 1)
        break
      case ">":
        ptr++
        break
      case ",": {
        const input = new Uint8Array(1)
        await Deno.stdin.read(input).then((n) => {
          array[ptr] = n === null ? 0 : input[0]
        })
        break
      }
      case ".":
        await Deno.stdout.write(new Uint8Array([array[ptr]]))
        break
      case "[":
        if (array[ptr] === 0) codePtr = brackets.get(codePtr)!
        break
      case "]":
        if (array[ptr] !== 0) codePtr = brackets.get(codePtr)!
        break
    }

    codePtr++
  }
}

const args = Deno.args
if (args.length !== 1) {
  console.error(`Usage: deno --allow-read bf.ts <filename>`)
  Deno.exit(1)
}

await Deno.readTextFile(args[0]).then(async (code) => await interpretBF(code))
  .catch((e) => {
    console.error(`Error: ${e.message}`)
    Deno.exit(1)
  })
