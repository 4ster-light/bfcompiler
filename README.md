# Brainfuck compiler

This a simple implementation of a Brainfuck compiler in Rust, has several modes
of operation, and can be used as a REPL.

## Installation

```bash
nix profile add github:4ster-light/bfcompiler
```

If on NixOS, you can also add it to your home-manager configuration.

```bash
cargo install --git https://github.com/4ster-light/bfcompiler.git
```

## Usage

### Build

```bash
bf build <FILE>
```

If you want to save the intermediate Rust code representation, you can add the
`--save` flag:

```bash
bf build <FILE> --save # or -s
```

### Run

You can run the file directly like any other interpreted language would with:

```bash
bf run <FILE>
```

Or you can use the REPL mode with:

```bash
bf repl
```

And then you can enter your Brainfuck code, and it will be executed.

## Example

You can find a file with this same code at the repository root called `hello.bf`, and you can run it with:

```bash
bf run hello.bf
```

```brainfuck
++++++++++[>+++++++>++++++++++>+++>+<<<<-]>++.>+.+++++++..+++.>++.<<+++++++++++++++.>.+++.------.--------.>+.>.
Hello World!
```

## License

MIT
