const MAX_POINTS: u32 = 100_000;

// fn println(key: String, value: String) {
//     println!("the value of {} is {}", key, value)
// }

fn main() {
    println!("The value of MAX_POINTS is: {}", MAX_POINTS);
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // *** Variable shadowing ***
    let x = x + 1;
    println!("The value of x is: {}", x);

    let x = x * 2;
    println!("The value of x is: {}", x);

    let spaces = "   ";
    // let mut spaces = "   ";      // will produce compile-time error when shadowing
    let spaces = spaces.len();

    println!("The length of spaces is: {}", spaces);

    // *** Data Types: Scalar types
    // integer      signed      unsigned
    // 8-bit        i8          u8
    // 16-bit       i16         u16
    // 32-bit       i32         u32
    // 64-bit       i64         u64
    // 128-bit      i128        u128
    // arch         isize       usize
    
    // number literals examples
    let decimal = 98_222;
    let hex = 0xff;
    let oct = 0o77;
    let binary = 0b1111_0000;
    let byte = b'A';

    println!("The value of decimal is: {}", decimal);
    println!("The value of hex is: {}", hex);
    println!("The value of oct is: {}", oct);
    println!("The value of binary is: {}", binary);
    println!("The value of byte is: {}", byte);

    // integer overflow
    // let byte: u8 = 256;      // debug mode: panic
                                // release mode: "two's complement wrapping" means 256 will become 0

    // floating point numbers
    // 32-bit       f32      float
    // 64-bit       f64      double      by default

    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let remainder = 43 % 5;

    println!("The value of difference is: {}", difference);
    println!("The value of product is: {}", product);
    println!("The value of quotient is: {}", quotient);
    println!("The value of remainder is: {}", remainder);

    // booleans, 8-bit size
    let boolean = true;
    let boolean2: bool = false;

    println!("The value of boolean is: {}", boolean);
    println!("The value of boolean2 is: {}", boolean2);

    // the character type
    // represents Unicode Scalar Value
    // ranges from U+0000 to U+D7FF and U+E000 to U+10FFFF inclusive
    let c = 'c';
    let z = 'Z';
    let heart_eyed_cat = '😻';

    println!("The value of c is: {}", c);
    println!("The value of z is: {}", z);
    println!("The value of heart_eyed_cat is: {}", heart_eyed_cat);

    // *** Data Types: Compound types
    // can group multiple values into once type

    // tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;        // destructuring of a tuple
    let int = tup.0;
    let double = tup.1;
    let uint = tup.2;

    println!("The value of x is: {} and is the same as int: {}", x, int);
    println!("The value of y is: {} and is the same as double: {}", y, double);
    println!("The value of z is: {} and is the same as uint: {}", z, uint);

    // array
    // is fixed length
    let nums: [u8; 6] = [0, 1, 2, 3, 4, 5];
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December"
    ];

    println!("The value of 2nd index of array nums is: {}", nums[1]);
    // println!("The value of 10th index of array nums is: {}", nums[9]);   // runtime error
    println!("The value of 6th index of array month is: {}", months[5]);
}
