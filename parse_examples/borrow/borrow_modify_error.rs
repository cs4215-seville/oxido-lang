fn main() {
    let s = StringFrom("hello");

    change(&s);
}

fn change(some_string: &String) {
    PushStr(some_string, ", world");
}