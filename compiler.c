#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <stdbool.h>

#define MAX_PROG_SIZE 30000
#define MAX_CODE_SIZE 300000

void checkBounds(unsigned char *ptr, unsigned char *array) {
    if (ptr < array || ptr >= array + MAX_PROG_SIZE) {
        fprintf(stderr, "Memory access out of bounds\n");
        exit(1);
    }
}

void compileBrainfuck(const char *bfCode, char *output) {
    int dataPtr = 0;
    int bracketCount = 0;
    char *outPtr = output;

    // Start of the C function
    outPtr += sprintf(outPtr, "#include <stdio.h>\n#include <stdlib.h>\nint main() {\nunsigned char array[%d] = {0};\nunsigned char *ptr = array;\n", MAX_PROG_SIZE);

    // Bounds checking for pointer movement
    outPtr += sprintf(outPtr, "void checkBounds() {\nif (ptr < array || ptr >= array + %d) {\nfprintf(stderr, \"Memory access out of bounds\\n\");\nexit(1);\n}\n}\n", MAX_PROG_SIZE);

    for (int i = 0; bfCode[i]; i++) {
        switch(bfCode[i]) {
            case '>':
                outPtr += sprintf(outPtr, "++ptr; checkBounds(); \n");
                break;
            case '<':
                outPtr += sprintf(outPtr, "--ptr; checkBounds(); \n");
                break;
            case '+':
                outPtr += sprintf(outPtr, "++*ptr;\n");
                break;
            case '-':
                outPtr += sprintf(outPtr, "--*ptr;\n");
                break;
            case '.':
                outPtr += sprintf(outPtr, "putchar(*ptr);\n");
                break;
            case ',':
                outPtr += sprintf(outPtr, "*ptr=getchar();\n");
                break;
            case '[':
                outPtr += sprintf(outPtr, "while (*ptr) {\n");
                bracketCount++;
                break;
            case ']':
                if (bracketCount > 0) {
                    outPtr += sprintf(outPtr, "}\n");
                    bracketCount--;
                }
                break;
            default: 
                break;  // Ignore other characters as they are comments
        }
    }

    outPtr += sprintf(outPtr, "return 0;\n}\n");
}

void interpretBrainfuck(const char *bfCode) {
    unsigned char array[MAX_PROG_SIZE] = {0};
    unsigned char *ptr = array;
    const char *codePtr = bfCode;
    int loopStack[MAX_PROG_SIZE];
    int loopStackPtr = 0;

    while (*codePtr) {
        checkBounds(ptr, array);  // Check bounds before any operation

        switch (*codePtr) {
            case '>': ptr++; break;
            case '<': ptr--; break;
            case '+': ++(*ptr); break;
            case '-': --(*ptr); break;
            case '.': putchar(*ptr); break;
            case ',': *ptr = getchar(); break;
            case '[':
                if (loopStackPtr >= MAX_PROG_SIZE) {
                    fprintf(stderr, "Loop stack overflow\n");
                    exit(1);
                }
                if (*ptr == 0) {
                    int depth = 1;
                    while (depth > 0) {
                        codePtr++;
                        if (*codePtr == '[') depth++;
                        else if (*codePtr == ']') depth--;
                    }
                } else {
                    loopStack[loopStackPtr++] = (int)(codePtr - bfCode);
                }
                break;
            case ']':
                if (loopStackPtr <= 0) {
                    fprintf(stderr, "Loop stack underflow\n");
                    exit(1);
                }
                if (*ptr != 0) {
                    codePtr = bfCode + loopStack[loopStackPtr - 1];
                } else {
                    loopStackPtr--;
                }
                break;
        }
        codePtr++;
    }
}

int main(int argc, char **argv) {
    bool compileMode = false;

    // Check if --compile flag is provided
    if (argc > 2 && strcmp(argv[argc - 1], "--compile") == 0) {
        compileMode = true;
        argc--;  // Remove the --compile flag from argument count
    }

    // Check if the correct number of arguments is provided
    if (argc != 2) {
        printf("Usage: %s <brainfuck_file> [--compile]\n", argv[0]);
        return 1;
    }

    // Open the Brainfuck file
    FILE *file = fopen(argv[1], "r");
    if (!file) {
        perror("Error opening file");
        return 1;
    }

    // Read the Brainfuck code
    char bfCode[MAX_PROG_SIZE];
    fread(bfCode, 1, MAX_PROG_SIZE - 1, file);
    bfCode[MAX_PROG_SIZE - 1] = '\0';  // Ensure null-termination
    fclose(file);

    if (compileMode) {
        char output[MAX_CODE_SIZE];
        compileBrainfuck(bfCode, output);

        FILE *outFile = fopen("output.c", "w");
        if (!outFile) {
            perror("Error creating output file");
            return 1;
        }
        fprintf(outFile, "%s", output);
        fclose(outFile);

        printf("Brainfuck code has been compiled to output.c\n");
    } else {
        interpretBrainfuck(bfCode);
    }

    return 0;
}
