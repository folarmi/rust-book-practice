fn main() {
    println!("Hello, world!");
    // A crate is the smallest amount of code the rust complier considers at a time
    // Binary crates are executable such as the ones run with cargo run
    // Library crates don’t have a main function, and they don’t 
    // compile to an executable. Instead, they define functionality 
    // intended to be shared with multiple projects. 

    // A package is a bundle of one or more crates that provides a set of functionality. 
    // A package contains a Cargo.toml file that describes how to build those crates.
    // A package can contain as many binary crates as you like, but at most only one library crate.
    // A package must contain at least one crate, whether that’s a library or binary crate.
}
