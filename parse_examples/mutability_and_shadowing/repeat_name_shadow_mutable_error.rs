fn main() {
    let mut spaces = "   ";
    println("There are spaces {} here.", spaces);

    spaces = Len(spaces);
    println("The space is of length {}.", spaces);
}
/*
error[E0308]: mismatched types
  |
3 |     spaces = spaces.len();
  |              ^^^^^^^^^^^^ expected `&str`, found `usize`

  */