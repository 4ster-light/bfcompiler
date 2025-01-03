# Brainfuck compiler

This a simple implementation of a Brainfuck compiler, there are both C and Rust versions of it, each of them transpiles the bf code into their own language to then be further compiled into a binary.

## Rust version

You need to have Rust installed, which is usually done via curl to get Rust up (further instructions in the Rust home page), or at least rustc rust compiler with the std lib.

## Usage

Compile the program

```bash
rustc bf.rs -o bf
```

Either interpret a file

```bash
./bf run hello.bf # Output: Hello World!
```

Or compile it and run it

```bash
./bf build hello.bf # Output: output (executable)
./output # Output: Hello World!
```

If you want to save the generated intermediate code to a C file, use the --save or -s flag

```bash
./bf build hello.bf --save # Output: output.rs (C file)
```

And you can now compile it manually if you wish to do so

```bash
rustc output.rs -o hello
./hello # Output: Hello World!
```

## C vesrion

The followind usage guide will all use Clang since it is my preferred choice, but if you use GCC it'll work exactly the same, just replace it in any command and the rest of it remains the same

> [!IMPORTANT]
> Some minimal Linux distributions don't come with Clang nor Gcc by default (like Alpine for example), so you'll have to install either manually if you want to use this compiler.

## Usage

Compile the program

```bash
clang bf.c -o bf
```

Either interpret a file

```bash
./bf run hello.bf # Output: Hello World!
```

Or compile it and run it

```bash
./bf build hello.bf # Output: output (executable)
./output # Output: Hello World!
```

If you want to save the generated intermediate code to a C file, use the --save or -s flag

```bash
./bf build hello.bf --save # Output: output.c (C file)
```

And you can now compile it manually if you wish to do so

```bash
clang output.c -o hello
./hello # Output: Hello World!
```

## Example

```brainfuck
++++++++++[>+++++++>++++++++++>+++>+<<<<-]>++.>+.+++++++..+++.>++.<<+++++++++++++++.>.+++.------.--------.>+.>.
```

Output:

```txt
Hello World!
```

## License

GNU General Public License v3.0
