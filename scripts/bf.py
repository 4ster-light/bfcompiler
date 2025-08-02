import sys


class MemoryOutOfBounds(Exception):
    pass


class UnmatchedBracket(Exception):
    pass


def check_bounds(ptr: int, array: list[int]):
    if ptr < 0 or ptr >= len(array):
        raise MemoryOutOfBounds("Memory access out of bounds")


def find_matching_brackets(code: str) -> dict[int, int]:
    brackets: dict[int, int] = {}
    stack: list[int] = []
    for i, char in enumerate(code):
        if char == "[":
            stack.append(i)
        elif char == "]":
            if not stack:
                raise UnmatchedBracket("Unmatched closing bracket")
            open_pos = stack.pop()
            brackets[open_pos] = i
            brackets[i] = open_pos
    if stack:
        raise UnmatchedBracket("Unmatched opening bracket")
    return brackets


def interpret_bf(code: str):
    max_prog_size = 30000
    array = [0] * max_prog_size
    ptr = 0
    code_ptr = 0
    brackets = find_matching_brackets(code)

    while code_ptr < len(code):
        check_bounds(ptr, array)

        char = code[code_ptr]
        if char == "+":
            array[ptr] = (array[ptr] + 1) % 256
        elif char == "-":
            array[ptr] = (array[ptr] - 1) % 256
        elif char == "<":
            ptr = max(0, ptr - 1)
        elif char == ">":
            ptr += 1
        elif char == ",":
            array[ptr] = ord(sys.stdin.read(1) or chr(0))
        elif char == ".":
            print(chr(array[ptr]), end="", flush=True)
        elif char == "[":
            if array[ptr] == 0:
                code_ptr = brackets[code_ptr]
        elif char == "]":
            if array[ptr] != 0:
                code_ptr = brackets[code_ptr]

        code_ptr += 1


if __name__ == "__main__":
    if len(sys.argv) != 2:
        print(f"Usage: {sys.argv[0]} <filename>", file=sys.stderr)
        sys.exit(1)

    filename = sys.argv[1]
    try:
        with open(filename, "r") as f:
            code = f.read()
            interpret_bf(code)
    except FileNotFoundError:
        print(f"Error: Could not open file '{filename}'", file=sys.stderr)
    except MemoryOutOfBounds as e:
        print(f"Error: {e}", file=sys.stderr)
    except UnmatchedBracket as e:
        print(f"Error: {e}", file=sys.stderr)
