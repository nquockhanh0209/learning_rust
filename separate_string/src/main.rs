
fn main() {
    let s = String::from("Nguyen Khanh");
    let first_s = separate_word(&s);
    println!("{}", first_s);
}
fn separate_word(s: &String) -> &str{
    
    let s_bytes = s.as_bytes();
    for(i, &item) in s_bytes.iter().enumerate(){
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    &s[..]
}