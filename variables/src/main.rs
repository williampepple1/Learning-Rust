fn main() {
    let mut a = "The world is here";
    println!("Hello, world!");
    println!("{}",a);
    a = "The world";
    println!("{}",a);
}

// To run, cd into the directory containing this file and run.

//In Rust, variable bindings are immutable by default. When a variable is immutable, after a value is bound to a name, you can't change that value.