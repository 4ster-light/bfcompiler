#include <stdio.h>
#include <stdlib.h>

#define MAX_PROG_SIZE 30000

void check_bounds(int ptr) {
  if (ptr < 0 || ptr >= MAX_PROG_SIZE) {
    fprintf(stderr, "Error: Memory access out of bounds at %d\n", ptr);
    exit(EXIT_FAILURE);
  }
}

void interpretBF(const char *bf_code) {
  unsigned char array[MAX_PROG_SIZE] = {0};
  int ptr = 0;
  int code_ptr = 0;
  int loop_stack[MAX_PROG_SIZE];
  int stack_ptr = 0;

  while (bf_code[code_ptr] != '\0') {
    check_bounds(ptr);
    char instruction = bf_code[code_ptr];

    switch (instruction) {
    case '+':
      array[ptr]++;
      break;
    case '-':
      array[ptr]--;
      break;
    case '<':
      if (ptr > 0) {
        ptr--;
      }
      break;
    case '>':
      ptr++;
      break;
    case ',':
      array[ptr] = getchar();
      break;
    case '.':
      putchar(array[ptr]);
      fflush(stdout);
      break;
    case '[':
      if (array[ptr] == 0) {
        int balance = 1;
        code_ptr++;
        while (bf_code[code_ptr] != '\0' && balance > 0) {
          if (bf_code[code_ptr] == '[') {
            balance++;
          } else if (bf_code[code_ptr] == ']') {
            balance--;
          }
          code_ptr++;
        }
        code_ptr--;
      } else {
        loop_stack[stack_ptr++] = code_ptr;
      }
      break;
    case ']':
      if (array[ptr] != 0) {
        code_ptr = loop_stack[stack_ptr - 1];
      } else {
        stack_ptr--;
      }
      break;
    default:
      break;
    }
    code_ptr++;
  }
}

int main(int argc, char *argv[]) {
  if (argc != 2) {
    fprintf(stderr, "Usage: %s <filename>\n", argv[0]);
    return EXIT_FAILURE;
  }

  const char *filename = argv[1];
  FILE *file = fopen(filename, "r");
  if (!file) {
    fprintf(stderr, "Error: Could not open file '%s'\n", filename);
    return EXIT_FAILURE;
  }

  fseek(file, 0, SEEK_END);
  long file_size = ftell(file);
  fseek(file, 0, SEEK_SET);

  char *bf_code = (char *)malloc(file_size + 1);
  if (!bf_code) {
    fprintf(stderr, "Error: Memory allocation failed\n");
    fclose(file);
    return EXIT_FAILURE;
  }
  fread(bf_code, 1, file_size, file);
  bf_code[file_size] = '\0';
  fclose(file);

  interpretBF(bf_code);

  free(bf_code);
  return EXIT_SUCCESS;
}
