using System;
using System.Collections.Generic;
using System.IO;

internal static class Program
{
    private const int MAX_PROG_SIZE = 30000;

    private class MemoryOutOfBoundsException : Exception
    {
        public MemoryOutOfBoundsException() : base("Memory access out of bounds") { }
    }

    private class UnmatchedBracketException : Exception
    {
        public UnmatchedBracketException() : base("Unmatched bracket") { }
    }

    private static void CheckBounds(int ptr, byte[] array)
    {
        if (ptr < 0 || ptr >= array.Length)
            throw new MemoryOutOfBoundsException();

    }

    private static Dictionary<int, int> FindMatchingBrackets(string code)
    {
        var brackets = new Dictionary<int, int>();
        var stack = new Stack<int>();

        for (int i = 0; i < code.Length; i++)
        {
            if (code[i] == '[') stack.Push(i);
            else if (code[i] == ']')
            {
                if (stack.Count == 0)
                    throw new UnmatchedBracketException();

                int openPos = stack.Pop();
                brackets[openPos] = i;
                brackets[i] = openPos;
            }
        }

        if (stack.Count > 0)
            throw new UnmatchedBracketException();

        return brackets;
    }

    private static void InterpretBF(string code)
    {
        byte[] array = new byte[MAX_PROG_SIZE];
        var brackets = FindMatchingBrackets(code);
        int ptr = 0;
        int codePtr = 0;

        while (codePtr < code.Length)
        {
            CheckBounds(ptr, array);

            switch (code[codePtr])
            {
                case '+':
                    array[ptr]++;
                    break;
                case '-':
                    array[ptr]--;
                    break;
                case '<':
                    ptr = Math.Max(0, ptr - 1);
                    break;
                case '>':
                    ptr++;
                    break;
                case ',':
                    int input = Console.Read();
                    array[ptr] = input == -1 ? (byte)0 : (byte)input;
                    break;
                case '.':
                    Console.Write((char)array[ptr]);
                    break;
                case '[':
                    if (array[ptr] == 0)
                    {
                        codePtr = brackets[codePtr];
                    }
                    break;
                case ']':
                    if (array[ptr] != 0)
                    {
                        codePtr = brackets[codePtr];
                    }
                    break;
            }

            codePtr++;
        }
    }

    private static void Main(string[] args)
    {
        if (args.Length != 1)
        {
            Console.Error.WriteLine($"Usage: csi {Environment.GetCommandLineArgs()[0]} <filename>");
            Environment.Exit(1);
        }

        try
        {
            string code = File.ReadAllText(args[0]);
            InterpretBF(code);
        }
        catch (FileNotFoundException)
        {
            Console.Error.WriteLine($"Error: Could not open file '{args[0]}'");
            Environment.Exit(1);
        }
        catch (MemoryOutOfBoundsException e)
        {
            Console.Error.WriteLine($"Error: {e.Message}");
            Environment.Exit(1);
        }
        catch (UnmatchedBracketException e)
        {
            Console.Error.WriteLine($"Error: {e.Message}");
            Environment.Exit(1);
        }
        catch (Exception e)
        {
            Console.Error.WriteLine($"Error: {e.Message}");
            Environment.Exit(1);
        }
    }
}
