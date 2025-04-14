package main

import "core:fmt"
import "core:os"

Brainfuck_Error :: enum {
	None,
	Memory_Out_Of_Bounds,
	Unmatched_Bracket,
	File_Not_Found,
}

MAX_PROG_SIZE : int : 30_000

check_bounds :: proc(ptr: int, array: [MAX_PROG_SIZE]u8) {
	if ptr < 0 || ptr >= len(array) {
		panic("Memory access out of bounds")
	}
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
				return nil, .Unmatched_Bracket
			}
			open_pos := pop(&stack)
			brackets[open_pos] = i
			brackets[i] = open_pos
		}
	}

	if len(stack) > 0 {
		delete(brackets)
		return nil, .Unmatched_Bracket
	}

	return brackets, .None
}

interpret_bf :: proc(code: string) -> Brainfuck_Error {
	array: [MAX_PROG_SIZE]u8
	ptr: int
	code_ptr: int
	brackets, err := find_matching_brackets(code)
	if err != .None {
		return err
	}
	defer delete(brackets)

	for code_ptr < len(code) {
		check_bounds(ptr, array)

		switch code[code_ptr] {
		case '+':
			array[ptr] += 1
		case '-':
			array[ptr] -= 1
		case '<':
			ptr = max(0, ptr - 1)
		case '>':
			ptr += 1
		case ',':
			array[ptr] = 0
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
		}

		code_ptr += 1
	}

	return .None
}

main :: proc() {
	if len(os.args) != 2 {
		fmt.eprintf("Usage: %s <filename>\n", os.args[0])
		os.exit(1)
	}

	filename := os.args[1]
	code, ok := os.read_entire_file(filename)
	if !ok {
		fmt.eprintf("Error: Could not read file '%s'\n", filename)
		os.exit(1)
	}
	defer delete(code)

	err := interpret_bf(string(code))
	switch err {
	case .Memory_Out_Of_Bounds:
		fmt.eprintf("Error: Memory access out of bounds\n")
	case .Unmatched_Bracket:
		fmt.eprintf("Error: Unmatched bracket\n")
	case .File_Not_Found:
		fmt.eprintf("Error: File not found\n")
	case .None:
		os.exit(0)
	}
}
