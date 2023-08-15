use std::io;
fn main() {
    test_functions(5, 'c');
    let x = test_return(5);
    println!("{x}");
    let counter = 0;
    // let result = stucking_in_the_loop(counter);
    // println!("{result}");
    reserve(1, 101);
}
fn test_functions(x: i32, c: char){
    println!("{x}{c}");
}
fn test_return(x: i32) -> i32{
    x+1
}
fn stucking_in_the_loop(x: i32) -> i32{
    let mut x1 = x; 
    loop {
        x1+=1;
        println!("{x1}");
        if x1 == 100
        {
            break;
        }
    }
    x1
}
// fn falling_in_love(x: i32) -> i32{
//     let mut x1 = x;
   
//     'precent_loving: loop {
//         'tell_me: loop{
//             let mut ilu = String::new();
//             io::stdin()
//             .read_line(&mut ilu)
//             .expect("Failed to read line");
            
//             if(ilu != "I love u"){
//                 break 'tell_me;
//             }
//             else{
//                 x1 -= 10;
                
//             }
//             print!("{ilu}{x1}");
//         }
//         if(x1 == 0){
//             break 'precent_loving;
//         }
//     }
//     x1 
// }
fn reserve(start: i32, end: i32){
    for number in (start..end) {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}