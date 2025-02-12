pub fn first_word(sentence:String)->String
{ let mut ans:String=String::from("");
  for c in sentence.chars()
  {
   if c==' '{
    return ans;
}
ans.push(c);
}
return String::from("");
}
pub fn main()
{
    println!("first_word:{}",first_word(String::from("apple is red")));
}