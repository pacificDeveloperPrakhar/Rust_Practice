pub fn factorial(x:i32)->i32
{
 if(x<=0)
 {return 1;}
 return x*factorial(x-1);
}
fn main()
{
    let var:i32=factorial(8);
    println!("x:{}",var);
}