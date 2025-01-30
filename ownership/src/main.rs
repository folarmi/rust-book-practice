// fn main() {
//     let s = String::from("hello"); 

//     takes_ownership(s);            
//     // println!("S in no longer valid here, {}", s);
//     let x = 5;                    

//     makes_copy(x);      
//     println!("X in still valid here, {}", x);

// }

// fn takes_ownership(some_string: String) {
//     println!("{some_string}");
// } 

// fn makes_copy(some_integer: i32) { 
//     println!("{some_integer}");
// } 

// Without References
// fn main() {
//     let s1 = String::from("hello");

//     let (s2, len) = calculate_length(s1);

//     println!("The length of '{s2}' is {len}.");
// }

// fn calculate_length(s: String) -> (String, usize) {
//     let length = s.len(); // len() returns the length of a String

//     (s, length)
// }

// With References
// fn main () {
//     let s1 = String::from("Hello");
//     let len = calculate_length(&s1);

//     println!("The length of '{s1}' is {len}." )
// }

// fn calculate_length(s: &String) -> usize  {
//     s.len()
// }

// fn main() {
//     let s1 = String::from("hello");

//     let len = calculate_length(&s1);

//     println!("The length of '{s1}' is {len}.");
// }

// fn calculate_length(s: &String) -> usize {
//     s.len()
// }


fn main() {
    let s = String::from("Helloworld");
   let word= first_word(&s);

    println!("The first word is {:?}", word);
}

fn first_word(s: &String) -> &str  {
    let bytes = s.as_bytes();

    // println!("The no. of bytes {:?}", bytes);

    for (i, &item) in bytes.iter().enumerate(){
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
// multiple immutable references are allowed 
// Mutable and immutable references aren't allowed
//  - At any given time, you can have either one mutable reference or any number of immutable references.
//  - References must always be valid.