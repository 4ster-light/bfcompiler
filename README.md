# Brainfuck compiler in C

This a simple implementation of a Brainfuck compiler that transpiles the brainfuck code to C code so it can later be compiled to an executable by any C compiler.

The only argument it recieves is the program in brainfuck (filename) and an optional --compile flag that decides wether the output will be the interpretation of the program or an "output.c" file the user can later compile to an executable.

The followind usage guide will all use Clang since it is my preferred choice, but if you use GCC it'll work exactly the same, just replace it in any command and the rest of it remains the same

## Usage

Compile the program

```bash
clang compiler.c -o bf
```

Either interpret a file

```bash
./bf run test.bf # Output: Hello World!
```

Or compile it and run it

```bash
./bf build test.bf --compile # Output: output (executable)
./output # Output: Hello World!
```

If you want to save the generated intermediate code to a C file, use the --save or -s flag

```bash
./bf build test.bf --save # Output: output.c (C file)
```

And you can now compile it manually if you wish to do so

```bash
clang output.c -o output
./output # Output: Hello World!
```

## License

GNU General Public License v3.0
