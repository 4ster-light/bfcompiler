# Brainfuck compiler in C

This a simple implementation of a Brainfuck compiler that transpiles the brainfuck code to C code so it can later be compiled to an executable by any C compiler.

The only argument it recieves is the program in brainfuck (filename) and an optional --compile flag that decides wether the output will be the interpretation of the program or an "output.c" file the user can later compile to an executable.

Compile the compiler (pun intended) first

```bash
gcc compiler.c -o compiler
```

Either interpret a file

```bash
./compiler test.bf # Output: Hello World!
```

Or compile it and run it

```bash
./compiler test.bf --compile # Output: output.c
gcc output.c -o test
./test # Output: Hello World!
```
