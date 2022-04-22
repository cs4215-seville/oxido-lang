fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = string_from("hello");
    &s
}