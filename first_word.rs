use std::io;

fn main() {
    let s = String::from("hello how are you");  
    let sliced:&str = first_word(&s);  
    println!("{}", sliced);
}

pub fn first_word(s: &String) -> &str {  // ✅ Use &str instead of &String
    for (i, &val) in s.as_bytes().iter().enumerate() {
        if val == b' ' {
            return &s[..i];  // ✅ Return slice safely
        }
    }
    &s[..]
}
