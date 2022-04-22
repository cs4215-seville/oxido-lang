fn main() {
    let mut mys = StringFrom("hello");
    let myy = &mut mys;
    let myz = *myy;
}