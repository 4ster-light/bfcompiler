const int MAX_PROG_SIZE = 30000;

class MemoryOutOfBoundsException : Exception
{
  public MemoryOutOfBoundsException() : base("Memory access out of bounds") { }
}

class UnmatchedBracketException : Exception
{
  public UnmatchedBracketException() : base("Unmatched bracket") { }
}

void CheckBounds(int ptr, byte[] array)
{
  if (ptr < 0 || ptr >= array.Length)
    throw new MemoryOutOfBoundsException();
}

Dictionary<int, int> FindMatchingBrackets(string code)
{
  var brackets = new Dictionary<int, int>();
  var stack = new Stack<int>();

  for (int i = 0; i < code.Length; i++)
    if (code[i] == '[')
      stack.Push(i);
    else if (code[i] == ']')
    {
      if (stack.Count == 0)
        throw new UnmatchedBracketException();

      int openPos = stack.Pop();
      brackets[openPos] = i;
      brackets[i] = openPos;
    }

  if (stack.Count > 0)
    throw new UnmatchedBracketException();

  return brackets;
}

void InterpretBF(string code)
{
  byte[] array = new byte[MAX_PROG_SIZE];
  var brackets = FindMatchingBrackets(code);
  int ptr = 0;
  int codePtr = 0;

  while (codePtr < code.Length)
  {
    CheckBounds(ptr, array);
    char instruction = code[codePtr];

    if (instruction == '+')
      array[ptr]++;
    else if (instruction == '-')
      array[ptr]--;
    else if (instruction == '<')
      ptr = Math.Max(0, ptr - 1);
    else if (instruction == '>')
      ptr++;
    else if (instruction == ',')
    {
      int input = Console.Read();
      array[ptr] = input == -1 ? (byte)0 : (byte)input;
    }
    else if (instruction == '.')
      Console.Write((char)array[ptr]);
    else if (instruction == '[')
    {
      if (array[ptr] == 0) codePtr = brackets[codePtr];
    }
    else if (instruction == ']')
    {
      if (array[ptr] != 0) codePtr = brackets[codePtr];
    }

    codePtr++;
  }
}

if (Args.Count != 1)
{
  Console.Error.WriteLine("Usage: dotnet script bf.csx <filename>");
  Environment.Exit(1);
}

try
{
  string code = File.ReadAllText(Args[0]);
  InterpretBF(code);
}
catch (FileNotFoundException)
{
  Console.Error.WriteLine($"Error: Could not open file '{Args[0]}'");
  Environment.Exit(1);
}
catch (Exception e)
{
  Console.Error.WriteLine($"Error: {e.Message}");
  Environment.Exit(1);
}
