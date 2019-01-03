use std::fmt::Display;

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// function signature compiles 
fn longest_2<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    fn announce_and_return_part(&self, annoucement: &str) -> &str {
        println!("Attention please: {}", annoucement);
        self.part
    }
}

// Geberic Type Parameters, Trait Bounds, and Lifetimes Together
fn longest_with_an_annoucement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str 
    where T: Display
{
    println!("Announcement!: {}", ann);

    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let x = 5;
    let r = &x;

    println!("r: {}", r);

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    let result = longest_2(string1.as_str(), string2);
    println!("The longest string is {}", result);

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.')
        .next()
        .expect("Could not find a '.'");

    let i = ImportantExcerpt { part: first_sentence };

    let s: &'static str = "I have a static lifetime.";
}
