const int MaxProgSize = 30000;

static void CheckBounds(int ptr, int[] array)
{
    if (ptr < 0 || ptr >= array.Length)
        throw new IndexOutOfRangeException("Memory access out of bounds");
}

static Dictionary<int, int> FindMatchingBrackets(string code)
{
    var stack = new Stack<int>();
    var brackets = new Dictionary<int, int>();
    for (int i = 0; i < code.Length; i++)
    {
        if (code[i] == '[') stack.Push(i);
        else if (code[i] == ']')
        {
            if (stack.Count == 0) throw new Exception("Unmatched bracket");
            int open = stack.Pop();
            brackets[open] = i;
            brackets[i] = open;
        }
    }
    if (stack.Count > 0) throw new Exception("Unmatched bracket");
    return brackets;
}

static void Interpret(string code)
{
    var array = new int[MaxProgSize];
    var brackets = FindMatchingBrackets(code);
    int ptr = 0, codePtr = 0;
    while (codePtr < code.Length)
    {
        CheckBounds(ptr, array);
        switch (code[codePtr])
        {
            case '+': array[ptr]++; break;
            case '-': array[ptr]--; break;
            case '>': ptr++; break;
            case '<': ptr--; break;
            case '.': Console.Write((char)array[ptr]); break;
            case ',': array[ptr] = Console.Read(); break;
            case '[': if (array[ptr] == 0) codePtr = brackets[codePtr]; break;
            case ']': if (array[ptr] != 0) codePtr = brackets[codePtr] - 1; break;
        }
        codePtr++;
    }
}

if (args.Length < 1)
{
    Console.Error.WriteLine("Usage: dotnet run <filename>");
    return;
}
try
{
    Interpret(File.ReadAllText(args[0]));
}
catch (Exception ex)
{
    Console.Error.WriteLine($"Error: {ex.Message}");
}
