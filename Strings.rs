fn main()
{
    let str:String=String::from("abcdefghijklmnopqrstuvwxyz");
    let c:Option<char>=str.chars().nth(1);
    //unwrap just denotes that we are okay with any sort of runtime exception
    println!("x:{}",c.unwrap())

}