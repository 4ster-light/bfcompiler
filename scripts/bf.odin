package main

import "core:fmt"
import "core:os"

Brainfuck_Error :: struct {
	code:    enum {
		None,
		Memory_Out_Of_Bounds,
		Unmatched_Bracket,
		File_Not_Found,
		Invalid_Input,
	},
	message: string,
}

MAX_PROG_SIZE: int : 30_000

check_bounds :: proc(ptr: int, array: [MAX_PROG_SIZE]u8) -> Brainfuck_Error {
	if ptr < 0 || ptr >= len(array) {
		return Brainfuck_Error {
			.Memory_Out_Of_Bounds,
			fmt.tprintf("Memory access out of bounds at ptr=%d", ptr),
		}
	}
	return Brainfuck_Error{.None, ""}
}

find_matching_brackets :: proc(code: string) -> (map[int]int, Brainfuck_Error) {
	brackets := make(map[int]int)
	stack := make([dynamic]int)
	defer delete(stack)

	for c, i in code {
		switch c {
		case '[':
			append(&stack, i)
		case ']':
			if len(stack) == 0 {
				delete(brackets)
				return nil, Brainfuck_Error {
					.Unmatched_Bracket,
					fmt.tprintf("Unmatched closing bracket at position %d", i),
				}
			}
			open_pos := pop(&stack)
			brackets[open_pos] = i
			brackets[i] = open_pos
		}
	}

	if len(stack) > 0 {
		delete(brackets)
		return nil, Brainfuck_Error {
			.Unmatched_Bracket,
			fmt.tprintf("Unmatched opening bracket at position %d", stack[len(stack) - 1]),
		}
	}

	return brackets, Brainfuck_Error{.None, ""}
}

interpret_bf :: proc(code: string) -> Brainfuck_Error {
	array: [MAX_PROG_SIZE]u8
	ptr: int
	code_ptr: int

	brackets, err := find_matching_brackets(code)
	if err.code != .None {
		return err
	}
	defer delete(brackets)

	for code_ptr < len(code) {
		if err := check_bounds(ptr, array); err.code != .None {
			return err
		}

		switch code[code_ptr] {
		case '+':
			array[ptr] += 1
		case '-':
			array[ptr] -= 1
		case '<':
			ptr -= 1
		case '>':
			ptr += 1
		case ',':
			buf: [1]u8
			n, read_err := os.read(os.stdin, buf[:])
			if read_err != 0 || n == 0 {
				return Brainfuck_Error{.Invalid_Input, "Failed to read input from stdin"}
			}
			array[ptr] = buf[0]
		case '.':
			fmt.print(rune(array[ptr]), flush = true)
		case '[':
			if array[ptr] == 0 {
				code_ptr = brackets[code_ptr]
			}
		case ']':
			if array[ptr] != 0 {
				code_ptr = brackets[code_ptr]
			}
		case:
			break
		}

		code_ptr += 1
	}

	return Brainfuck_Error{.None, ""}
}

main :: proc() {
	if len(os.args) != 2 {
		fmt.eprintf("Usage: %s <filename>", os.args[0])
		os.exit(1)
	}

	code_bytes, ok := os.read_entire_file(os.args[1])
	if !ok {
		fmt.eprintf("Error: Could not read file '%s'", os.args[1])
		os.exit(1)
	}
	defer delete(code_bytes)

	if err := interpret_bf(string(code_bytes)); err.code != .None {
		fmt.eprintf("Error: %s", err.message)
		os.exit(1)
	}
}
