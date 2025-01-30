fn main() {
   let mut x = 5;
   println!("The value of x is: {}", x);

   x = 6;

   println!("The value of x is: {}", x);

   const TOTAL_YEARS :u32 = 789;
   println!("The value of Total years is: {}", TOTAL_YEARS);

   let _tuple = ("Learning Rust again", 100_000);


}


// if !matches!(first_character, 'a' | 'e' | 'i' | 'o' | 'u') {
//    format!("{word}{first_character}ay");
// }