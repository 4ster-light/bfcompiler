package scripts

import java.io.File
import java.io.IOException
import kotlin.system.exitProcess

private const val MAX_PROG_SIZE = 30000

class MemoryOutOfBoundsException : Exception("Memory access out of bounds")
class UnmatchedBracketException : Exception("Unmatched bracket")

private fun checkBounds(ptr: Int, array: ByteArray) {
    if (ptr < 0 || ptr >= array.size) throw MemoryOutOfBoundsException()
}

private fun findMatchingBrackets(code: String): Map<Int, Int> {
    val brackets = mutableMapOf<Int, Int>()
    val stack = ArrayDeque<Int>()

    code.forEachIndexed { index, char ->
        when (char) {
            '[' -> stack.addLast(index)
            ']' -> {
                val open = stack.removeLastOrNull() ?: throw UnmatchedBracketException()
                brackets[open] = index
                brackets[index] = open
            }
        }
    }

    if (stack.isNotEmpty()) throw UnmatchedBracketException()
    return brackets
}

private fun interpretBF(code: String) {
    val array = ByteArray(MAX_PROG_SIZE)
    val brackets = findMatchingBrackets(code)
    var ptr = 0
    var codePtr = 0

    while (codePtr < code.length) {
        checkBounds(ptr, array)
        when (code[codePtr]) {
            '+' -> array[ptr]++
            '-' -> array[ptr]--
            '<' -> ptr = maxOf(0, ptr - 1)
            '>' -> ptr++
            ',' -> {
                val input = System.`in`.read()
                array[ptr] = if (input == -1) 0 else input.toByte()
            }
            '.' -> print(array[ptr].toInt().toChar())
            '[' -> if (array[ptr] == 0.toByte()) codePtr = brackets[codePtr]!!
            ']' -> if (array[ptr] != 0.toByte()) codePtr = brackets[codePtr]!!
        }
        codePtr++
    }
}

fun main(args: Array<String>) {
    if (args.size != 1) {
        System.err.println("Usage: kotlinc BrainfuckInterpreter <filename>")
        exitProcess(1)
    }

    try {
        val code = File(args[0]).readText()
        interpretBF(code)
    } catch (e: IOException) {
        System.err.println("Error: Could not open or read file '${args[0]}': ${e.message}")
        exitProcess(1)
    } catch (e: Exception) {
        System.err.println("Error: ${e.message}")
        exitProcess(1)
    }
}
