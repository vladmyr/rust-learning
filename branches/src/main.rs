fn main() {
    // *** "if" statement
    let number = 6;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3 or 2");
    }

    let condition = true;
    let number = if condition {
        5
    } else {
        6
        // "six"    // compile time error: the return type of all branches must be the same
    };

    println!("The value of number is: {}", number);

    // *** Repetition with loops
    // a never ending loop
    // loop {
    //     println!("again!")
    // }

    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    assert_eq!(result, 20);

    let mut number = 3;
    while number != 0 {
        println!("{}", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");

    // an alternative to above
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");

    // accessing array elements
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    // error prone
    while index < 5 {
        println!("The value is: {}", a[index]);
        index += 1;
    }

    // a concise alternative
    for element in a.iter() {
        println!("The value is: {}", element);
    }
}
