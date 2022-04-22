fn main() {
    let s = StringFrom("hello");

    change(&s);
}

fn change(some_string: &mut String) {
    PushStr(some_string, ", world");
}