#include <fstream>
#include <iostream>
#include <stack>
#include <vector>

const int MAX_PROG_SIZE = 30000;

void check_bounds(int ptr)
{
  if (ptr < 0 || ptr >= MAX_PROG_SIZE)
  {
    std::cerr << "Error: Memory access out of bounds at " << ptr << std::endl;
    exit(EXIT_FAILURE);
  }
}

void interpretBF(const std::vector<char> &bf_code)
{
  std::vector<unsigned char> array(MAX_PROG_SIZE, 0);
  int ptr = 0;
  int code_ptr = 0;
  std::stack<int> loop_stack;

  while (code_ptr < bf_code.size())
  {
    check_bounds(ptr);
    char instruction = bf_code[code_ptr];

    switch (instruction)
    {
    case '+':
      array[ptr]++;
      break;
    case '-':
      array[ptr]--;
      break;
    case '<':
      if (ptr > 0)
        ptr--;
      break;
    case '>':
      ptr++;
      break;
    case ',':
      array[ptr] = std::cin.get();
      break;
    case '.':
      std::cout << static_cast<char>(array[ptr]) << std::flush;
      break;
    case '[':
      if (array[ptr] == 0)
      {
        int balance = 1;
        code_ptr++;
        while (code_ptr < bf_code.size() && balance > 0)
        {
          if (bf_code[code_ptr] == '[')
            balance++;
          else if (bf_code[code_ptr] == ']')
            balance--;

          code_ptr++;
        }
        code_ptr--;
      }
      else
        loop_stack.push(code_ptr);

      break;
    case ']':
      if (array[ptr] != 0)
        code_ptr = loop_stack.top();
      else
        loop_stack.pop();

      break;
    default:
      break;
    }
    code_ptr++;
  }
}

int main(int argc, char *argv[])
{
  if (argc != 2)
  {
    std::cerr << "Usage: " << argv[0] << " <filename>" << std::endl;
    return EXIT_FAILURE;
  }

  const char *filename = argv[1];
  std::ifstream file(filename, std::ios::binary | std::ios::ate);
  if (!file.is_open())
  {
    std::cerr << "Error: Could not open file '" << filename << "'" << std::endl;
    return EXIT_FAILURE;
  }

  std::streamsize file_size = file.tellg();
  file.seekg(0, std::ios::beg);

  std::vector<char> bf_code(file_size);
  if (!file.read(bf_code.data(), file_size))
  {
    std::cerr << "Error: Could not read file '" << filename << "'" << std::endl;
    return EXIT_FAILURE;
  }
  file.close();

  interpretBF(bf_code);

  return EXIT_SUCCESS;
}
