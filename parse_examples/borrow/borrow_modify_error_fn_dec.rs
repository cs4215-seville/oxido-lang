fn main() {
    let s = StringFrom("hello");

    change(&mut s); // &s also the same error. // i.e. method dec check comes before application check
}

fn change(some_string: &String) {
    PushStr(some_string, ", world");
}