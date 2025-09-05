struct Counter
{
  
    count:u32,
    i:u32
}

impl Iterator  for Counter
{   type Item=u32;
    fn next(& mut self)->Option<u32>
    {
        if self.i<=self.count
        {   
            let result= Option::Some(self.i);
            self.i=self.i+1;
            return result;
        }
        return Option::None;
    }
}

fn main()
{

}