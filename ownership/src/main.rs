// scope
fn another_function() {
                        // s is not valid here, it's not yed declared
    let s = "hello";    // s is valid from this point forward
                        // s value located in stack memory
    println!("{}", s);
}                       // this scope is now over and s is no longer valid

fn yet_another_function() {
    let mut s = String::from("hello");  // String type is mutable
                                        // s value located in heap memory
    s.push_str(", world!");             // appends a literal to a string
    println!("{}", s);
}   // this scope is now over and s is no longer valid

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}   // some_integer goes out of scope and nothing happens

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}   // some_string goes out of scope and `drop` is called. Memory is free now

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string     // returns ownedship to calling function
}

// takes_and_gives_back will take a string and return one
fn takes_and_gives_back(a_string: String) -> String {
    a_string    // string is returned and moved out to calling function
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn main() {
    println!("Hello, world!");

    // scope example
    another_function();

    // working with Heap memory
    yet_another_function();

    // String is made up of three parts:
    // - ptr
    // - len - actual length of content in bytes
    // - capacity - total allocated memory size in bytes
    // this group of data is stored on the stack
    // ptr points to a memory location on the heap where the content (in this case 
    // "hello") is stored
    let s1 = String::from("hello");
    let s2 = s1;    // s1 was "moved" to s2
    // the result of assignment is as following:
    // - ptr, len, capacity of s2 are pushed into stack
    // - ptr of s2 points to the same location the ptr of s1 does
    // At this point s1 is no longer valid, 
    // try compiling line below and you'll get an error:
    // println!("{}, world!", s1);

    // make "deep" copy of s2
    let mut s3 = s2.clone();
    s3.push_str(" (held by s3)");

    // *** ownership and functions
    let mut x = 5;
    makes_copy(x);          // i32 is Copy, it is OK to still use x afterwards
    x += 1;

    println!("{}", x);

    let s = String::from("hello");
    takes_ownership(s);     // s's value moves into the function...
                            // ...and so is no longer valid here
    // println!("{}", s);   // produces compile time error
    
    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);

    println!("s1: {}", s1);
    // println!("s2: {}", s2);
    println!("s3: {}", s3);
    // once out of scope:
    // - s1 and s3 get dropped
    // - s2 was moved so nothing happens

    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}.", s2, len);
}
