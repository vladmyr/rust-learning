// statements are instructions that perform some action and do not return a value
// expressions evalue to a resulting value

fn another_function() {
    println!("Another function.");
}

fn yet_another_function(x: i32) {
    println!("The value of x is: {}", x);
}

fn third_function(x: i32, y: i32) {
    println!("The value of x is: {} and the value of y is: {}", x, y);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn main() {
    let x = 5;      // is a statement
    let x2 = five();
    let y = 6;
    let z = {       // is an expression
        let x = 3;
        plus_one(x)       // <- no ending semicolon, expressions do not include one
                            // adding semicolon will turn it into a statement
    };

    println!("Hello, world!");
    another_function();
    yet_another_function(x);
    third_function(x, y);

    println!("The value of x2 is: {}", x2);
    println!("The value of z is: {}", z);
}
