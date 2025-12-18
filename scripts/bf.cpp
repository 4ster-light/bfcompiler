#include <fstream>
#include <iostream>
#include <stack>
#include <vector>
#include <string_view>
#include <array>

constexpr int MAX_PROG_SIZE = 30000;

void check_bounds(int ptr)
{
    if (ptr < 0 || ptr >= MAX_PROG_SIZE)
    {
        std::cerr << "Error: Memory access out of bounds at " << ptr << "\n";
        exit(EXIT_FAILURE);
    }
}

void interpretBF(std::string_view bf_code)
{
    std::array<unsigned char, MAX_PROG_SIZE> array{};
    int ptr = 0;
    size_t code_ptr = 0;
    std::stack<size_t> loop_stack;

    while (code_ptr < bf_code.size())
    {
        check_bounds(ptr);
        char instruction = bf_code[code_ptr];

        switch (instruction)
        {
            case '+': array[ptr]++; break;
            case '-': array[ptr]--; break;
            case '<': if (ptr > 0) ptr--; break;
            case '>': ptr++; break;
            case ',': array[ptr] = static_cast<unsigned char>(std::cin.get()); break;
            case '.': std::cout << static_cast<char>(array[ptr]) << std::flush; break;
            case '[':
                if (array[ptr] == 0)
                {
                    int balance = 1;
                    code_ptr++;
                    while (code_ptr < bf_code.size() && balance > 0)
                    {
                        if (bf_code[code_ptr] == '[') balance++;
                        else if (bf_code[code_ptr] == ']') balance--;
                        code_ptr++;
                    }
                    code_ptr--;
                }
                else loop_stack.push(code_ptr);
                break;
            case ']':
                if (array[ptr] != 0)
                    if (!loop_stack.empty())
                        code_ptr = loop_stack.top();
                else if (!loop_stack.empty())
                    loop_stack.pop();
                break;
            default: break;
        }

        code_ptr++;
    }
}

int main(int argc, char *argv[])
{
    if (argc != 2)
    {
        std::cerr << "Usage: " << argv[0] << " <filename>\n";
        return EXIT_FAILURE;
    }

	// Open the file
    std::ifstream file(argv[1], std::ios::binary | std::ios::ate);
    if (!file)
    {
        std::cerr << "Error: Could not open file '" << argv[1] << "'\n";
        return EXIT_FAILURE;
    }

	// Get file size
    auto file_size = file.tellg();
    file.seekg(0, std::ios::beg);

	// Read file content
    std::vector<char> bf_code(file_size);
    if (!file.read(bf_code.data(), file_size))
    {
        std::cerr << "Error: Could not read file '" << argv[1] << "'\n";
        return EXIT_FAILURE;
    }

	// Interpret the Brainfuck code
    interpretBF({bf_code.data(), bf_code.size()});
    return EXIT_SUCCESS;
}
