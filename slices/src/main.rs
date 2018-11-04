// problem: write a function that takes a string and returns the first word it 
// finds in that string. If the function doesn’t find a space in the string, 
// the whole string must be one word, so the entire string should be returned.
// An example of bad realisation that returns index of an a space:
// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' '{
//             return i;
//         }
//     }

//     s.len()
// }

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn main() {
    let s = String::from("hello world");
    let x = &s[0..5];           // s gets borrowed as immutable
    let y = &s[6..11];          // s gets borrowed as immutable

    println!("The x is {}", x);
    println!("The y is {}", y);

    let z = first_word(&s);     // s gets borrowed as immutable

    println!("The z is {}", z);

    // s.clear();       // error, cannot borrow s as mutable as...
                        // ...it also borrowed as immutable
}
