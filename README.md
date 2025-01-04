# Brainfuck compiler

This a simple implementation of a Brainfuck compiler in Rust, has several modes of operation, and can be used as a REPL.

You need to have Rust installed, which is usually done via curl to get Rust up (further instructions in the Rust home page), or at least rustc rust compiler with the std lib.

If you want to install it after cloning the repo, you can do so with

```bash
cargo install --path .
```

## Usage

Everything will be explained with Cargo, but you can also use the binary if you installed it.

### Build

```bash
cargo build <file>
```

If you want to save the intermediate code representation, you can add the `--save` flag.

```bash
cargo build <file> --save # or -s
```

### Run

You can run the file directly like any other interpreted language would with:

```bash
cargo run <file>
```

Or you can use the REPL mode with:

```bash
cargo run repl
```

And then you can enter your Brainfuck code and it will be executed.

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
