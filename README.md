# Brainfuck compiler

This a simple implementation of a Brainfuck compiler in Rust, has several modes
of operation, and can be used as a REPL.

You need to have Rust installed, which is usually done via curl to get _rustup_
(further instructions in the Rust home page), or at least rustc rust compiler
with the std lib.

## Brainfuck Interpreter Example scripts

This project includes several examples in many different programming languages
with the same approach to making a simple brainfuck interpreter, these are
purely meant to see the difference on how the same program can look very
different on some languages and to highlight the importance of writing idiomatic
code.

## Installation

If you want to install it, after cloning the repo, you can do so with:

```bash
cargo install --path .
```

## Usage

Everything will be explained with Cargo, but you can also use the binary if you
installed it.

### Build

```bash
cargo run --release -- build <file>
```

If you want to save the intermediate code representation, you can add the
`--save` flag:

```bash
cargo run --release -- build <file> --save # or -s
```

### Run

You can run the file directly like any other interpreted language would with:

```bash
cargo run --release -- run <file>
```

Or you can use the REPL mode with:

```bash
cargo run --release -- repl
```

And then you can enter your Brainfuck code, and it will be executed.

### Help

If no command is given or if you use the `--help` flag it'll display the due
help message and if the same flag is used in any of the commands it'll show
relevant information too.

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
