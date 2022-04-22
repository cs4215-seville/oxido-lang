fn main() {
    let mut mys = StringFrom("hello");
    let myy = &mut mys;
    getlen(mys);        
}

fn getlen(input_str : String) -> i64 {
    Len(input_str)
}