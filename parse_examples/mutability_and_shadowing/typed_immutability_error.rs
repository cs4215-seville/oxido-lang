fn main() {
    let x : i64 = 5;
    println("The value of x is: {}", x);
    x = 6;
    println("The value of x is: {}", x);
}

/*
error[E0384]: cannot assign twice to immutable variable `x`
 --> typed_immutability_error.rs:4:5
  |
2 |     let x : u32 = 5;
  |         -
  |         |
  |         first assignment to `x`
  |         help: consider making this binding mutable: `mut x`
3 |     println("The value of x is: {}", x);
4 |     x = 6;
  |     ^^^^^ cannot assign twice to immutable variable
  */
