package scripts;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.HashMap;
import java.util.Map;
import java.util.Stack;

class MemoryOutOfBoundsException extends Exception {
	public MemoryOutOfBoundsException() {
		super("Memory access out of bounds");
	}
}

class UnmatchedBracketException extends Exception {
	public UnmatchedBracketException() {
		super("Unmatched bracket");
	}
}

public class bf {
	private static final int MAX_PROG_SIZE = 30000;

	private static void checkBounds(int ptr, byte[] array) throws MemoryOutOfBoundsException {
		if (ptr < 0 || ptr >= array.length)
			throw new MemoryOutOfBoundsException();
	}

	private static Map<Integer, Integer> findMatchingBrackets(String code) throws UnmatchedBracketException {
		var brackets = new HashMap<Integer, Integer>();
		var stack = new Stack<Integer>();

		for (int i = 0; i < code.length(); i++) {
			if (code.charAt(i) == '[')
				stack.push(i);
			else if (code.charAt(i) == ']') {
				if (stack.isEmpty())
					throw new UnmatchedBracketException();

				int openPos = stack.pop();
				brackets.put(openPos, i);
				brackets.put(i, openPos);
			}
		}

		if (!stack.isEmpty())
			throw new UnmatchedBracketException();

		return brackets;
	}

	private static void interpretBF(String code)
			throws MemoryOutOfBoundsException, UnmatchedBracketException, IOException {
		byte[] array = new byte[MAX_PROG_SIZE];
		var brackets = findMatchingBrackets(code);
		int ptr = 0;
		int codePtr = 0;

		while (codePtr < code.length()) {
			checkBounds(ptr, array);
			char instruction = code.charAt(codePtr);

			if (instruction == '+')
				array[ptr]++;
			else if (instruction == '-')
				array[ptr]--;
			else if (instruction == '<')
				ptr = Math.max(0, ptr - 1);
			else if (instruction == '>')
				ptr++;
			else if (instruction == ',') {
				int input = System.in.read();
				array[ptr] = (byte) (input == -1 ? 0 : input);
			} else if (instruction == '.')
				System.out.print((char) array[ptr]);
			else if (instruction == '[') {
				if (array[ptr] == 0)
					codePtr = brackets.get(codePtr);
			} else if (instruction == ']') {
				if (array[ptr] != 0)
					codePtr = brackets.get(codePtr);
			}

			codePtr++;
		}
	}

	public static void main(String[] args) {
		if (args.length != 1) {
			System.err.println("Usage: java BrainfuckInterpreter <filename>");
			System.exit(1);
		}

		try {
			String code = Files.readString(Paths.get(args[0]));
			interpretBF(code);
		} catch (IOException e) {
			System.err.println("Error: Could not open or read file '" + args[0] + "': " + e.getMessage());
			System.exit(1);
		} catch (Exception e) {
			System.err.println("Error: " + e.getMessage());
			System.exit(1);
		}
	}
}
