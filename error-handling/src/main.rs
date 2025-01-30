use std::fs::{self, File};
use std::io::{self, Read};

fn main() {
    println!("Hello, world!");
    let _greeting_file_result = File::open("hello.txt");
    let _greeting_file = match _greeting_file_result {
        Ok (file) => file,
        Err(error) => panic!("There was an error fetching the file {error:?}"),
    };
    fn _read_username_from_file () -> Result<String, io::Error> {
            let username_text = File::open("hello.txt");

            let mut username_file = match username_text {
                Ok(file) => file,
                Err(e) => return Err(e),
            };

            let mut username = String::new();

            match username_file.read_to_string(&mut username) {
                Ok(_) => Ok(username),
                Err(e) => Err(e)
            }
    }
    // The above is simplified by using a ? after the Result value
    // as shown below
    fn _shorter_read_username_from_text () -> Result<String, io::Error> {
        let mut username_text = File::open("hello.txt")?;
        let mut username = String::new();
        username_text.read_to_string(&mut username)?;
        Ok(username)
    }
    // The above could be further shorthened to 
    fn _concise_read_username_from_text ()  -> Result<String, io::Error>  {
        fs::read_to_string("hello.txt")
    }

}
