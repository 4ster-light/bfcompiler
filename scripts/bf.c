#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <stdbool.h>
#include <string.h>
#include <assert.h>

#define MAX_PROG_SIZE 30000
static_assert(MAX_PROG_SIZE > 0, "MAX_PROG_SIZE must be positive");

void interpretBF(const char *const bf_code)
{
	uint8_t array[MAX_PROG_SIZE] = {0};
	size_t ptr = 0;
	size_t code_ptr = 0;
	size_t loop_stack[MAX_PROG_SIZE];
	size_t stack_ptr = 0;

	while (bf_code[code_ptr] != '\0')
	{
		if (ptr >= MAX_PROG_SIZE)
		{
			fprintf(stderr, "Error: Memory access out of bounds at %zu\n", ptr);
			exit(EXIT_FAILURE);
		}

		switch (bf_code[code_ptr])
		{
		case '+': array[ptr]++; break;
		case '-': array[ptr]--; break;
		case '<': if (ptr > 0) ptr--; break;
		case '>': ptr++; break;
		case ',': array[ptr] = (uint8_t)fgetc(stdin); break;
		case '.': fputc(array[ptr], stdout); fflush(stdout); break;
		case '[':
			if (array[ptr] == 0)
			{
				int balance = 1;
				code_ptr++;
				while (bf_code[code_ptr] != '\0' && balance > 0)
				{
					if (bf_code[code_ptr] == '[')
						balance++;
					else if (bf_code[code_ptr] == ']')
						balance--;
					code_ptr++;
				}
				code_ptr--;
			}
			else loop_stack[stack_ptr++] = code_ptr;
			break;
		case ']':
			if (array[ptr] != 0) code_ptr = loop_stack[stack_ptr - 1];
			else stack_ptr--;
			break;
		default: break; // Ignore unknown commands
		}

		code_ptr++;
	}
}

int main(int argc, char *argv[])
{
	if (argc != 2)
	{
		fprintf(stderr, "Usage: %s <filename>\n", argv[0]);
		return EXIT_FAILURE;
	}

	FILE *file = fopen(argv[1], "r");
	if (!file)
	{
		perror("Error opening file");
		return EXIT_FAILURE;
	}

	// Determine file size
	fseek(file, 0, SEEK_END);
	long file_size = ftell(file);
	if (file_size <= 0)
	{
		fprintf(stderr, "Error: Empty file\n");
		fclose(file);
		return EXIT_FAILURE;
	}
	rewind(file);

	// Allocate for file content
	char *bf_code = malloc(file_size + 1);
	if (!bf_code)
	{
		perror("Memory allocation failed");
		fclose(file);
		return EXIT_FAILURE;
	}

	// Read the file into bf_code
	if (fread(bf_code, 1, file_size, file) != (size_t)file_size)
	{
		perror("Error reading file");
		free(bf_code);
		fclose(file);
		return EXIT_FAILURE;
	}
	bf_code[file_size] = '\0';
	fclose(file);

	// Interpret the Brainfuck code
	interpretBF(bf_code);
	free(bf_code);
	return EXIT_SUCCESS;
}
