# Brainfuck compiler in C

This a simple implementation of a Brainfuck compiler that transpiles the brainfuck code to C code so it can later be compiled to an executable by any C compiler.

The only argument it recieves is the program in brainfuck (filename) and an optional --compile flag that decides wether the output will be the interpretation of the program or an "output.c" file the user can later compile to an executable.

```bash
# Compile the compiler (pun intended) first
gcc compiler.c -o compiler
# Either interpret a file
./compiler test.bf # Output: Hello World!
# Or compile it and run it
./compiler test.bf --compile # Output: output.c
gcc output.c -o test
./test2 # Output: Hello World!
```
