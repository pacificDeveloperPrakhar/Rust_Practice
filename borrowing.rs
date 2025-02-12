fn main()
{
    let mut s1=String::from("hello");
    addWord(& mut s1);
    println!("{}",s1);
}
pub fn addWord(x:& mut String)
{
    x.push_str(" world")
}