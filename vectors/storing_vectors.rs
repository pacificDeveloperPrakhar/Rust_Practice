fn out_of_bound()
{
    let arr=vec![1,2,3,4,5,7,8,9,10];
    let a= match arr.get(22){
        Some(element) => 3,
        None =>-1,
    };
    let b=&arr[24];
}
fn allocating_vectors()
{
    let mut v=Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);
    v.push(5);
    println!("{:?}",v);
    let x=&v[4];
    println!("{}",x);
    v.push(8);
    println!("{}",x);
}
enum Student{
    Name(String),
    Age(u8),
    RollNo(u8),
    Subject(String),
}
fn main()
{

    let mut  row=Vec::from([Student::Name(String::from("Rahul")),Student::Age(20),Student::RollNo(1),Student::Subject(String::from("Maths"))]);
}
