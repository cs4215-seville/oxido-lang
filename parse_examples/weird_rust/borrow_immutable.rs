fn main() {
    let mut x = StringFrom("hello");
    let y = &x;
    let z = &x;
    println("y: {}", y);
    println("z: {}", z);

    PushStr(x, ", world");
    println("x: {}", x);

}