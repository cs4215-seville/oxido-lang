fn main() {
    let mut s = StringFrom("hello");

    change(&mut s);
    change(&mut s);
    println("{}", s);
}

fn change(some_string: &mut String) {
    PushStr(some_string, ", world");
}
