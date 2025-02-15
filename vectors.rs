fn main()
{
  let arr:Vec<u8>=Vec::from([1,2,3,4,5]);
  let index=searchBineary(&arr,0,arr.len()-1,5);
  println!("{}",index);
}
fn searchBineary(arr:&Vec<u8>,start:usize,end:usize,element:u8)->i8
{
    if (end<start)
    {
    return -1;
    }
    let mid:usize=(end-start)/2;
    if arr[start+mid]==element
    {
        return (start+mid) as i8;
    }
    if arr[mid+start]<element
    {
        return searchBineary(&arr,start+mid+1,end,element);
    }
    else
    {
        return searchBineary(&arr,start,mid+start-1,element);
    }
}