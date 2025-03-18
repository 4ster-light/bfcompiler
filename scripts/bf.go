package main

import (
	"bufio"
	"fmt"
	"io"
	"os"
)

const MAX_PROG_SIZE int = 30000

func check_bounds(ptr int, array []byte) {
	if ptr >= len(array) {
		panic("Memory access out of bounds")
	}
}

func interpret_bf(bfCode string) {
	array := make([]byte, MAX_PROG_SIZE)
	ptr := 0
	codePtr := 0
	loopStack := []int{}

	for codePtr < len(bfCode) {
		check_bounds(ptr, array)
		char := bfCode[codePtr]

		switch char {
		case '+':
			array[ptr]++
		case '-':
			array[ptr]--
		case '<':
			if ptr > 0 {
				ptr--
			}
		case '>':
			ptr++
		case ',':
			reader := bufio.NewReader(os.Stdin)
			input, _ := reader.ReadByte()
			array[ptr] = input
		case '.':
			fmt.Printf("%c", array[ptr])
		case '[':
			if array[ptr] == 0 {
				balance := 1
				codePtr++
				for codePtr < len(bfCode) && balance > 0 {
					currentChar := bfCode[codePtr]
					if currentChar == '[' {
						balance++
					} else if currentChar == ']' {
						balance--
					}
					codePtr++
				}
				codePtr--
			} else {
				loopStack = append(loopStack, codePtr)
			}
		case ']':
			if array[ptr] != 0 {
				codePtr = loopStack[len(loopStack)-1]
			} else {
				loopStack = loopStack[:len(loopStack)-1]
			}
		}

		codePtr++
	}
}

func main() {
	if len(os.Args) < 2 {
		fmt.Println("Usage: go run bf.go <filename>")
		return
	}

	filename := os.Args[1]
	file, err := os.Open(filename)
	if err != nil {
		fmt.Printf("Could not open file: %s\n", filename)
		return
	}
	defer file.Close()

	bfCode, err := io.ReadAll(file)
	if err != nil {
		fmt.Println("Error reading file")
		return
	}

	interpret_bf(string(bfCode))
}
