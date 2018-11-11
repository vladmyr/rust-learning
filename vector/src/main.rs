// fn v_iterator<T>(&v: &Vec<T>) {
//     for i in &v {
//         println!("{:?}", i);
//     }
// }

#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let v: Vec<i32> = Vec::new();
    let v1 = vec![0, 1, 2, 3];
    let mut v2 = v1.clone();

    println!("Vector v is {:?}", v);
    println!("Vector v1 is {:?}", v1);
    println!("Vector v2 before changes is {:?}", v2);

    v2.push(4);
    v2.push(5);
    v2.push(6);

    println!("Vector v2 after changes is {:?}", v2);

    let v3 = vec![0, 1, 2, 3, 4, 5];
    let third: &i32 = &v3[2];
    let v_index = 2;

    println!("Third element of vector v3 is {}", third);

    match v3.get(v_index) {
        Some(_) => { println!("Reachable element at index: {}", v_index); }
        None => { println!("Unreachable element at index: {}", v_index); }
    }

    // let does_not_exist = &v3[100];       // panic
    let does_not_exist = v.get(100);        // returns None

    println!("does_not_exist is {:?}", does_not_exist);

    println!("iterating over v3");
    for i in &v3 {
        println!("{}", i);
    }

    let mut v4 = vec![100, 32, 57];
    println!("iterating over mutable v4");
    for i in &mut v4 {
        *i += 50;           // * is a derefernce operator, used to get to the 
                            // value of i
        println!("{}", i);
    }

    // using as Enum to store multiple types
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    println!("row is {:?}", row);
}
