fn main()
{
  take_knowledge()
}
//at a time one ownershipis allowed
fn take_knowledge()
{
    let obj1=String::from("hello how are you");
    let obj2=obj1;
    println!("{}",obj1)
}
//at a time one mutable reference is allowed
fn mutable_reference()
{
  let mut obj1=String::from("hell");
  let obj2=& mut obj1;
  let obj3=& mut obj1;
  obj3.push_str("o");
  obj2.push_str(" how");
}