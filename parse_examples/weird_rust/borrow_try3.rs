fn main() {
    let mut mys = string_from("hello");
    let myy = &mut mys;
    getlen(mys);        
}

fn getlen(input_str : String) -> i64 {
    len(input_str)
}