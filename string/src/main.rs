fn main() {
    let s = String::new();
    let data = "initial contents";

    let s = data.to_string();
    let s = "initial contents".to_string();
    let s = String::from("initial contents");
    let s = String::from(data);

    // thanks to utf8
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    // updating a String
    let mut s = String::from("foo");
    let s2 = "bar";
    
    s.push_str("bar");
    s.push_str(s2);

    println!("s is {}", s);
    println!("s2 is {}", s2);

    let mut s = String::from("lo");
    s.push('l');

    println!("s is {}", s);

    let s = String::from("Hello, ");
    let s2 = String::from("world!");
    // `+` operator uses add method, whose signature is as following:
    // fn add(self, s: &str) -> String
    // the result of the execution is: 
    // - `s` is moved and no more valid
    // - `s2` is available afterwards
    let s3 = s + &s2;

    println!("s3 is {}", s3);

    let s = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s + "-" + &s2 + "-" + &s3;

    println!("s is {}", s);

    // none of variables were moved
    let s = format!("{}-{}-{}", s, s2, s3);
    println!("s is {}", s);

    // string internal representation
    let hola = "Hola";
    let len = String::from("Hola").len();
    println!("Length of string {} is {}", hola, len);

    let hello_rus = "Здравствуйте";
    let len = String::from(hello_rus).len();
    // length is 24 instead of 12 as every character takes 2 bytes of storage
    println!("Length of string {} is {}", hello_rus, len);

    // indexing is not allowed because of variadic character byte length
    // let answer = &hello_rus[0];

    // bytes, scalar value, grapheme clusters
    // hindi has six char values, but only four letter
    let hindi_literal = "नमस्ते";
    let hindi = String::from("नमस्ते");
    // Length is 18, every character value takes 3 bytes
    println!("Lengh of string {} is {}", hindi, hindi.len());

    // slicing String
    let hello_rus = "Здравствуйте";
    let s = &hello_rus[0..4];       // returns `Зд`
    // let s2 = &hello_rus[0..1];      // panic at runtime

    // iterating over String
    println!("Hindi word \"{}\"", hindi_literal);
    println!("Print chars:");
    for c in hindi_literal.chars() {
        println!("{}", c);
    }

    println!("Print bytes:");
    for b in hindi_literal.bytes() {
        println!("{}", b);
    }
}
