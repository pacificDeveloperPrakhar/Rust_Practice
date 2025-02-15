fn main()
{
    let s1=String::from("Hello, ");
    let s2=String::from("World!");
    let a="😒😁😁😁😁";
    let x=&a[0..4];
    println!("{}",x);
    let s3=s1+&s2;
    println!("{}",s3);
    // println!("{}",s1); // this will give error because s1 is moved to s3
}
