# oxido-lang

## Preliminaries
Please ensure that you have the latest stable release of Rust installed. Otherwise, do follow the installation instructions [here](https://www.rust-lang.org/tools/install).

## Project Setup
Firstly, clone the repository to your local machine.
```
git clone https://github.com/cs4215-seville/oxido-lang.git
```
Once you've cloned the repository, you may execute the project by running the following command:
```
cargo run <DIR_TO_RUST_FILE>
```
For more example programs to try, do explore the [parse examples](/parse_examples) directory.

## What release v0 contains

Parser: parse the Rust-subset langauge input to an AST  
(completed, but AST datatype incorrect defines `PushStr` as a unary operation instead of a binary one. One word fix required in [ast.rs](/src/parser/ast.rs).)  
  
  
Static Checker: process the AST to output an error message, or a Mapping of line_no : variable_names, that specifies after which line, which variables should be dropped.  
(incomplete. Updating of the borrow stack at the instatiation of every variable occurs, but deletion is partial.)  
  
  
Compiler/VM : runs the code using the AST and mappings of variable drops fed by the static checker. Shows that all allocated memory is cleared at the end of program.
