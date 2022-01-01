fn main() {
    let mut num = 2;
    let result = loop {
        if is_divisible(num) {
            break num; // sets result to num
        }
        num += 2; // we want only even numbers anyways (se we skip every odd one (2,4,6,8,10,12))
    };
    println!("Found it! {}", result);
}


fn is_divisible(n: u32) -> bool {
   for i in 2..20 {
       if n % i != 0 {
           return false
       }
   }
   true
    // could be more efficient to check divisibilty by 11, 12, 13, 14, 16, 17, 18, 19 and 20
    // shown below:
    //  if n % 11 == 0 && n % 12 == 0 && n % 13 == 0 && n % 14 == 0 && n % 17 == 0 && n % 18 == 0 && n % 19 == 0 && n % 20 == 0 && n % 16 == 0{
        //  return true;
    //  }
    //  false
}