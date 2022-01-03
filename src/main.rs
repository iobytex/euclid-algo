use std::io;

fn main()  {
   ///Accept two numbers from the cli  i.e. cargo run -- 7496 2464
   let mut argument = std::env::args().skip(1);
   let a = argument.next().unwrap().parse::<isize>().unwrap();
   let b = argument.next().unwrap().parse::<isize>().unwrap();

   //check if a is greater than b based on the euclid algorithm
   if a > b {
      //pass the argument to euclid function
     let (r,rn) =  euclid(a,b);
      println!("{} {}",r,rn);
   }else{
      panic!("the left value has to be greater then the right value")
   }
}

//we accept the value by copy since it is a known size
//we added a mut to the parameters . Note we didn't add the lifetime elision rule since we are returning both parameters but you can though
fn euclid(mut a: isize, mut b: isize) -> (isize, isize){
   //loop through the logic
   loop {
      //get the remainder . Note: i didn't add the quotient because we finding for x and y in a equation i.e. 7496x +  2464y =8
      //multiplication inverse with the quotient can be used in the math question you gave
      let r = a % b;

      //we check if the remainder is 0 and break i.e. no new b value
      if r == 0 {
         break
      } else {
         //we check if r is greater then zero and r is less the b because we need to make sure the the new a can be divide by the new b
         if 0 < r && r < b {
            a = b;
            b = r;
            continue
         } else {
            panic!("zero is greater than r and r is greater b {} {}",r,b)
         }
      }
   }
   //we return a tuple
   (a,b)
}

#[cfg(test)]
mod test{
   use crate::euclid;

   #[test]
   fn checkgcd(){
      assert_eq!(euclid(7496,2464),(32,8))
   }
}