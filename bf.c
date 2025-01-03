#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <stdbool.h>
#include <unistd.h>

#define MAX_PROG_SIZE 30000
#define MAX_CODE_SIZE 300000

void checkBounds(unsigned char *ptr, unsigned char *array)
{
    if (ptr < array || ptr >= array + MAX_PROG_SIZE)
    {
        fprintf(stderr, "Memory access out of bounds\n");
        exit(1);
    }
}

void compileBrainfuck(const char *bfCode, char *output)
{
    int dataPtr = 0;
    char *outPtr = output;

    outPtr += sprintf(outPtr, "#include <stdio.h>\n#include <stdlib.h>\n");

    outPtr += sprintf(outPtr, "void checkBounds(unsigned char *ptr, unsigned char *array) {if (ptr < array || ptr >= array + %d) {fprintf(stderr, \"Memory access out of bounds\\n\");exit(1);}}\n", MAX_PROG_SIZE);

    outPtr += sprintf(outPtr, "int main() {unsigned char array[%d] = {0};unsigned char *ptr = array;", MAX_PROG_SIZE);

    for (int i = 0; bfCode[i]; i++)
    {
        switch (bfCode[i])
        {
        case '>':
            outPtr += sprintf(outPtr, "++ptr;checkBounds(ptr, array);");
            break;
        case '<':
            outPtr += sprintf(outPtr, "--ptr;checkBounds(ptr, array);");
            break;
        case '+':
            outPtr += sprintf(outPtr, "++*ptr;");
            break;
        case '-':
            outPtr += sprintf(outPtr, "--*ptr;");
            break;
        case '.':
            outPtr += sprintf(outPtr, "putchar(*ptr);");
            break;
        case ',':
            outPtr += sprintf(outPtr, "*ptr=getchar();");
            break;
        case '[':
            outPtr += sprintf(outPtr, "while(*ptr) {");
            break;
        case ']':
            outPtr += sprintf(outPtr, "}");
            break;
        default:
            break;
        }
    }

    outPtr += sprintf(outPtr, "return 0;}");
}

void interpretBrainfuck(const char *bfCode)
{
    unsigned char array[MAX_PROG_SIZE] = {0};
    unsigned char *ptr = array;
    const char *codePtr = bfCode;
    int loopStack[MAX_PROG_SIZE];
    int loopStackPtr = 0;

    while (*codePtr)
    {
        checkBounds(ptr, array);

        switch (*codePtr)
        {
        case '>':
            ptr++;
            break;
        case '<':
            ptr--;
            break;
        case '+':
            ++(*ptr);
            break;
        case '-':
            --(*ptr);
            break;
        case '.':
            putchar(*ptr);
            break;
        case ',':
            *ptr = getchar();
            break;
        case '[':
            if (loopStackPtr >= MAX_PROG_SIZE)
            {
                fprintf(stderr, "Loop stack overflow\n");
                exit(1);
            }
            if (*ptr == 0)
            {
                int depth = 1;
                while (depth > 0)
                {
                    codePtr++;
                    if (*codePtr == '[')
                        depth++;
                    else if (*codePtr == ']')
                        depth--;
                }
            }
            else
            {
                loopStack[loopStackPtr++] = (int)(codePtr - bfCode);
            }
            break;
        case ']':
            if (loopStackPtr <= 0)
            {
                fprintf(stderr, "Loop stack underflow\n");
                exit(1);
            }
            if (*ptr != 0)
            {
                codePtr = bfCode + loopStack[loopStackPtr - 1];
            }
            else
            {
                loopStackPtr--;
            }
            break;
        }
        codePtr++;
    }
}

int main(int argc, char **argv)
{
    if (argc < 3)
    {
        fprintf(stderr, "Usage: %s [run|build] <brainfuck_file> [-s|--save]\n", argv[0]);
        return 1;
    }

    bool saveOutput = false;
    bool buildMode = false;

    if (argc > 3)
    {
        if (strcmp(argv[3], "-s") == 0 || strcmp(argv[3], "--save") == 0)
        {
            saveOutput = true;
        }
        else
        {
            fprintf(stderr, "Unknown option: %s\n", argv[3]);
            return 1;
        }
    }

    if (strcmp(argv[1], "build") == 0)
    {
        buildMode = true;
    }
    else if (strcmp(argv[1], "run") != 0)
    {
        fprintf(stderr, "Invalid mode: %s\n", argv[1]);
        return 1;
    }

    FILE *file = fopen(argv[2], "r");
    if (!file)
    {
        perror("Error opening file");
        return 1;
    }

    // Read the Brainfuck code
    char bfCode[MAX_PROG_SIZE];
    fread(bfCode, 1, MAX_PROG_SIZE - 1, file);
    bfCode[MAX_PROG_SIZE - 1] = '\0'; // Ensure null-termination
    fclose(file);

    if (buildMode)
    {
        char output[MAX_CODE_SIZE];
        compileBrainfuck(bfCode, output);

        FILE *outFile = fopen("output.c", "w");
        if (!outFile)
        {
            perror("Error creating output file");
            return 1;
        }
        fprintf(outFile, "%s", output);
        fclose(outFile);

        // Try compiling with Clang first, if not available, use GCC
        char compileCommand[256];
        snprintf(compileCommand, sizeof(compileCommand), "clang -o output output.c || gcc -o output output.c");
        int compileResult = system(compileCommand);

        if (compileResult != 0)
        {
            fprintf(stderr, "Compilation failed\n");
            if (!saveOutput)
            {
                unlink("output.c"); // Remove the temporary C file if compilation failed
            }
            return 1;
        }

        if (!saveOutput)
        {
            unlink("output.c"); // Remove the temporary C file if not saving
        }
    }
    else
    {
        interpretBrainfuck(bfCode);
    }

    return 0;
}
