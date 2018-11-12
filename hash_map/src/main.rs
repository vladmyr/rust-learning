use std::collections::HashMap;

struct VecProps {
    mean: f64,
    median: f64,
    mode: i32
}

enum MedianIndex {
    None,
    Single(usize),
    Double(usize, usize)
}

fn calc_props(v: &Vec<i32>) -> VecProps {
    let len = v.len();

    let mut v_sorted = v.clone();
    let mut map: HashMap<i32, i32> = HashMap::new();

    // calculate median
    v_sorted.sort();
    let median_index = match len {
        0 => MedianIndex::None,
        _ => match len % 2 == 0 {
            false => MedianIndex::Single((len - 1) / 2),
            true => {
                let upper_index = len / 2;
                MedianIndex::Double(upper_index - 1, upper_index)
            },
        }
    };

    let median = (match median_index {
        MedianIndex::None => 0,
        MedianIndex::Single(index) => v_sorted[index],
        MedianIndex::Double(lower_index, upper_index) => (v_sorted[lower_index] + v_sorted[upper_index]) / 2
    }) as f64;

    // calculate mean & mode
    let mut mean: f64 = 0.0;
    let mut mode = 0;
    let mut mode_count = 0;

    if len > 0 {
        for n in v {
        let count = map.entry(*n).or_insert(1);
            *count += 1;

            mean += *n as f64;
        }

        mean /= len as f64;

        // get mode
        for (key, value) in map {
            if mode_count < value {
                mode = key;
                mode_count = value;
            }
        }
    }

    VecProps {
        median,
        mean,
        mode
    }
}

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 5);

    println!("scores are {:?}", scores);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    // `HashMap<_, _>` is needed, compiler infers the types that the hash map 
    // contains based on tye types of the data in the vectors
    let scores: HashMap<_, _> = teams
        .iter()
        .zip(initial_scores.iter())
        .collect();

    println!("collected scores are {:?}", scores);

    // ownership

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    // both key and value variables get `move`'d and cannot longer be used
    map.insert(field_name, field_value);

    // accessing values

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    println!("The scopre of \"{}\" team is {:?}", team_name, score);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // updating values
    // 1. overwriting
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);    // defining
    scores.insert(String::from("Blue"), 25);    // overwriting

    println!("{:?}", scores);

    // 2. insert a value if the key has no value
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yello")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);

    // 3. update values based on the old value
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);

    // exercises
    // 1. Given a list of integers, use a vector and return the mean 
    // (the average value), median (when sorted, the value in the middle 
    // position), and mode (the value that occurs most often; a hash map will be
    // helpful here) of the list.
    let v: Vec<i32> = vec![];
    let vec_props = calc_props(&v);

    assert_eq!(vec_props.mean, 0.0);
    assert_eq!(vec_props.median, 0.0);
    assert_eq!(vec_props.mode, 0);

    let v = vec![100];
    let vec_props = calc_props(&v);
    
    assert_eq!(vec_props.mean, 100.0);
    assert_eq!(vec_props.median, 100.0);
    assert_eq!(vec_props.mode, 100);

    let v = vec![100, 55, 100];
    let vec_props = calc_props(&v);
    
    assert_eq!(vec_props.mean, 85.0);
    assert_eq!(vec_props.median, 100.0);
    assert_eq!(vec_props.mode, 100);

    // 5, 12, 21, 47, 56, 56, 56, 87, 100, 100
    let v = vec![100, 56, 47, 21, 100, 5, 12, 87, 56, 56];
    let vec_props = calc_props(&v);
    
    assert_eq!(vec_props.mean, 54.0);
    assert_eq!(vec_props.median, 56.0);
    assert_eq!(vec_props.mode, 56);
}
