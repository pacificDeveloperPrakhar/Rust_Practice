enum IpKind{
    IpV4,Ipv6
}
struct Routes{
    ip:IpKind,
    name:String
}
fn main()
{
  let Dice=9;
  match Dice{
    9=>println!("9 is the number"),
    3=>println!("3 is your number"),
    other=>println!("your number is something else")
    
  }
  let var:Option<i32>=Option::<i32>::Some(13i32);
}

enum IpAddr{
    Ipv4{
        value:String,
        encoding:String
    },
    Ipv6
    {
        value:String,
        encoding:String
    }
}

enum Coin{
    Nickel,Zinc,Copper,Gold,Silver
}

