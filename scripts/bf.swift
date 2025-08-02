import Foundation

enum InterpreterError: Error {
    case memoryOutOfBounds
    case unmatchedBracket
}

let maxProgSize = 30000

func checkBounds(_ ptr: Int, _ array: [Int]) throws {
    if ptr < 0 || ptr >= array.count {
        throw InterpreterError.memoryOutOfBounds
    }
}

func findMatchingBrackets(_ code: String) throws -> [Int] {
    var brackets = Array(repeating: -1, count: code.count)
    var stack: [Int] = []

    for (i, char) in code.enumerated() {
        switch char {
        case "[":
            stack.append(i)
        case "]":
            if let openPos = stack.popLast() {
                brackets[openPos] = i
                brackets[i] = openPos
            } else {
                throw InterpreterError.unmatchedBracket
            }
        default:
            break
        }
    }

    if !stack.isEmpty {
        throw InterpreterError.unmatchedBracket
    }

    return brackets
}

func interpretBF(_ code: String) throws {
    var array = Array(repeating: 0, count: maxProgSize)
    let brackets = try findMatchingBrackets(code)
    var ptr = 0
    var codePtr = code.startIndex

    while codePtr < code.endIndex {
        try checkBounds(ptr, array)

        switch code[codePtr] {
        case "+":
            array[ptr] += 1
        case "-":
            array[ptr] -= 1
        case "<":
            ptr -= 1
        case ">":
            ptr += 1
        case ",":
            if let input = readLine() {
                array[ptr] = Int(input.unicodeScalars.first!.value)
            }
        case ".":
            print(Character(UnicodeScalar(array[ptr])!), terminator: "")
            fflush(stdout)
        case "[":
            if array[ptr] == 0 {
                codePtr = code.index(
                    code.startIndex, offsetBy: brackets[codePtr.utf16Offset(in: code)] + 1)
                continue
            }
        case "]":
            if array[ptr] != 0 {
                codePtr = code.index(
                    code.startIndex, offsetBy: brackets[codePtr.utf16Offset(in: code)])
                continue
            }
        default:
            break
        }

        codePtr = code.index(after: codePtr)
    }
}

do {
    guard CommandLine.arguments.count > 1 else {
        print("Usage: \(CommandLine.arguments[0]) <filename>")
        exit(1)
    }

    try interpretBF(try String(contentsOfFile: CommandLine.arguments[1], encoding: .utf8))
} catch InterpreterError.memoryOutOfBounds {
    print("Error: Memory access out of bounds")
} catch InterpreterError.unmatchedBracket {
    print("Error: Unmatched bracket")
} catch {
    print("Error: \(error)")
}
