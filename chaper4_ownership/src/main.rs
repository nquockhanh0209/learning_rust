use std::io;
fn main() {
    let mut hello_mydear = String::from("Hello: ");
  
    let mut name = String::new();
    println!("Please enter your name: ");
    io::stdin().read_line(&mut name).expect("Failed to get name");
    hello_mydear.push_str(&name);
    println!("{}", hello_mydear);
    move_data();
    let (s,l) = test_get_string(String::from("Hello, "));
    println!("{} {}", s,l)
}
fn move_data(){
    let s1 = String::from("Hello: ");
    //correct
    let s2= s1.clone();
    //s1 data was moved to s2
    //let s2= s1;
    println!("{}, world!", s1);
}
fn test_get_string(randomString: String) -> (String, usize){
    let mut return_string = String::from(randomString);
    
     return_string.push_str("my dear");
    let new_string = String::from(&return_string);
    let l: usize = new_string.len();
    (return_string, l)
}