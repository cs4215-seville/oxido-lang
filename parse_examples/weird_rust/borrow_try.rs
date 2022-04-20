fn main() {
    let mut x = 0;
    let y = &mut x;
    
    change_pts(y);
    change_num(x);
    println("x: {}", x);

    let mut mys = 2;
    let myy = &mut mys;
    let myz = *myy;

}

fn change_num(input_num : i64) -> i64{
    10
}
fn change_pts(input_ptr : &mut i64) {
   *input_ptr = 5;
}

fn mine(input_str : String) -> i64 {
    len(input_str)
}