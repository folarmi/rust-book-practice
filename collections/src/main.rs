use std::collections::HashMap;
fn main() {
    println!("Hello, world!");

    // Vectors allows you to store more than one value in a single data structure,
    // it puts all the value next to each other in memory
    // can only store value of the same type

    let mut v: Vec<i32> = Vec::new();
    let v_two = vec![1,2,3,4,5];

    v.push(6);

    let third:&i32 = &v_two[2];
    let fouth: Option<&i32> = v_two.get(3);
    match fouth {
        Some(_t) => println!("The third element is {third}"),
        None => println!("There is no third element"),
    }

    println!("The third element is {third}");
    for i in &v_two {
        println!("{i}");
    }


    //  Hash Maps(Objects in JavaScript)
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("The new Hash i {:?}", scores);

    let text = "Hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    // Solution to exercises
    // 1) // Given a list of integers, use a vector and return the median 
    //     (when sorted, the value in the middle position) and mode 
    //     (the value that occurs most often; a hash map will be helpful here) of the list.

    // Initial solution
    //     let list_of_integers = vec![1,1,1,2,2,2,4,4,4,4,5,5];
    //     let mut mode: i32 = 0;
    //     let mut list_of_integers_map: HashMap<i32, i32>  = HashMap::new();

    //     for number in list_of_integers {
    //         let count = list_of_integers_map.entry(number).or_insert(0);
    //         *count +=1;
    //     }

    // if let Some((&key, &value)) = list_of_integers_map.iter().max_by_key(|&(_,count)| count){
    //     mode = key;
    //     println!("The mode is: {} (appears {} times)", mode, value)
    // } else {
    //     println!("The list is empty")
    // }

    fn _calculate_median_and_mode(mut numbers: Vec<i32>) -> (Option<f64>, Option<i32>) {
        if numbers.is_empty() {
            return (None, None);
        }

        numbers.sort();
        let len = numbers.len();

        let median = if len % 2 == 0 {
            let mid1 = numbers[len / 2 - 1];
            let mid2 = numbers[len / 2];
            (mid1 + mid2) as f64 / 2.0
        } else {
            numbers[len / 2] as f64
        };

        let mut frequency_map = std::collections::HashMap::new();
        for &number in &numbers {
            *frequency_map.entry(number).or_insert(0) += 1;
        }

        let mode = frequency_map.iter().max_by_key(|&(_,&count) | count).map(|(&number,_) | number);

        (Some(median), mode)
    }    


    // Convert strings to pig latin. The first consonant of each word is moved to 
    // the end of the word and ay is added, so first becomes irst-fay. Words that
    //  start with a vowel have hay added to the end instead (apple becomes apple-hay). 
    //  Keep in mind the details about UTF-8 encoding!   

    fn pig_latin_converter(word: String) -> String  {
        // Get the first character of the word
        let first_character = word.chars().nth(0);
        
        // Match on the first character and handle None
        match first_character {
            Some(c) => c,     
            None => ' ',      
        };
        let rest_of_word = &word[1..];

        if !matches!(first_character, Some('a') | Some('e') | Some('i') | Some('o') | Some('u')) {
            format!("{rest_of_word}-{}ay", first_character.unwrap_or(' '))
        } else {
            format!("{word}-hay")
        }
    }

    println!("{}",pig_latin_converter(String::from("apple")))

    // Using a hash map and vectors, create a text interface to allow a user to add employee 
    // names to a department in a company; for example, “Add Sally to Engineering” or “Add Amir to Sales.” 
    // Then let the user retrieve a list of all people in a department or all people in
    //  the company by department, sorted alphabetically.

    
}







