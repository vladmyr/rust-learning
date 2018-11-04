fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}

fn calculate_length_with_reference(s: &String) -> usize {   // s is a reference to a String
    s.len()
}   // Here, s goes out of scope. But because it does not have ownership of what 
    // it refers to, nothing happens

// borrowed argument must explicity set to mutable in order to mutate then
// in the body of a function
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// Example of dangling reference
// fn dange() -> &String {
//     let s = String::from("hello");
//     &s       // funtion returns a reference to a String
// }    // Here, s goes out of scope, and is dropped. Its memory goes away. Danger!

fn main() {
    // without references
    let s1 = String::from("hello");
    let (s1, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s1, len);

    // with references
    let mut s2 = s1.clone();
    change(&mut s2);
    let len = calculate_length_with_reference(&s2); // s2 is "borrowed" by the function

    println!("The length of '{}' is {}.", s2, len);

    // You can have only a single mutable reference to a particular piece of 
    // data in a particular scope. Restriction is defined in order to prevent
    // data reces at compile time
    let r1 = &mut s2;
    // let r2 = &mut s2;    // a compile time error will occur
    // similar rule exists for combining mutable and immutable references
    
    r1.push_str(" (this string was updated via r1 mutable reference)");
}
