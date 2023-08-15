fn main() {
   let five = Some(5);
   let n: Option<i32> = None;
    let add: i32 = 10;
    let five_add = plus_option(five, add);
    //println!("{:?}", five_add);
    let plus_r = get_value(five_add);
    let (plus_r1, plus_r1_op) = get_value_1(n);
    println!("{} {}", plus_r, plus_r1);
    let x;
    {
        x = 10;
        println!("{}", x)
    }
    println!("x ot of scope {}", x)
}
fn plus_option(x: Option<i32>, add: i32) -> Option<i32>{
    match x{
        None => None,
        Some(x) => Some(x+add),
    }
}
fn get_value(x: Option<i32>) -> i32{
    if let Some(x) = x{
        x
    } 
    else{
        1
    }
}
fn get_value_1(x: Option<i32>)->(i32, Option<i32>){
    match x{
        Some(x) => (x, Some(x)),
        None => (0, None),
    }
}