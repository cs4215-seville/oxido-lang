# oxido-lang

## Introduction
Oxido is an attempt to implement a small subset of Rust. The goal of the project is to explore the theory and implementation of pointer aliasing in Rust. The expected deliverable would comprise of a working parser, type and borrow checker, a compiler, and a virtual machine. Ideally, our final implementation should free/drop any variable who's lifetime is no longer valid. This should be accomplished without a need for any garbage collection. For more information on the project status, please refer to this [section](#project-status).

## Project Status
1. Parser is complete, refer to the [grammar](src/grammar.pest) for more details about its syntax. Additional details are also available under the [testing](#testing) section as well.
2. Type checker and borrow checker is currently both partially implemented and integrated. Refer to the [v0 release section](#what-release-v0-contains) for more details.
3. Compiler presently works for the following:
    - [x] Literal Expressions (e.g. `i64`, `bool`, `()`)
    - [x] Assignments
    - [x] Primitive unary operations (e.g. `!`, `-`)
    - [x] Primitive binary operations (e.g. `+`, `-`, `*`, `/`, `==`, `!=`, `>`, `>=`, `<`, `<=`)
    - [x] Block expressions
    - [x] Function declarations
    - [x] Function applications
    - [ ] String literals
    - [ ] Strings
    - [ ] String related operations
    - [ ] Reference related operations
4. Virtual Machine is not complete.

## Preliminaries
Please ensure that you have the latest stable release of Rust installed. Otherwise, do follow the installation instructions [here](https://www.rust-lang.org/tools/install).

## Project Setup
Firstly, clone the repository to your local machine.
```
git clone https://github.com/cs4215-seville/oxido-lang.git
```
Once you've cloned the repository, proceed to build the project. This will produce a compiled binary in `/target/debug/`.
```
cargo build
```
To execute the compiled binary, run the command below and supply a path to your program. Your program would then be parsed, type-checked, and compiled. The compiled bytecode would then be printed out.
> ⚠️ As the compiler doesn't yet compile for strings, references, and their operations, we recommend that you add the `--skip-compile` flag to avoid crashing the program if you're using any of those features.
```
./target/debug/oxido-lang --file-dir <DIR_TO_RUST_FILE>
```
For more example programs to try, do explore the [parse examples](/parse_examples) directory.

## Testing
By convention, all tests can be found at the bottom of the file they're written for.

At present, tests are available for the parser and the test files can be found in [parse examples](/parse_examples). Of the 60 test files, the parser is able to correctly parse 59 syntatically valid programs, and [correctly reject one](/parse_examples/statement_expr/statement_parse_error.rs). Basic tests are present for the compiler as well. To run the tests, execute the following in the project root.
```
cargo test
```

## What release v0 contains

Parser: parse the Rust-subset langauge input to an AST  
(completed, but one word fix required in [ast.rs](/src/parser/ast.rs) to change `PushStr` from unary operation to binary operation).  
  
  
Static Checker: process the AST to output an error message, or a Mapping of line_no : variable_names, that specifies after which line, which variables should be dropped.  
(incomplete. Updating of the borrow stack at the instatiation of every variable occurs, but deletion is partial.)  
  
  
Compiler/VM : runs the code using the AST and mappings of variable drops fed by the static checker. Shows that all allocated memory is cleared at the end of program.
