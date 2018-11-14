use std::collections::HashMap;

// Exercise #1
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

// Exercise #2
// since characters are stored in UTF-8 byte length of each is variadic, hence
// the task can be narrowed down to finding an index in byte sequece that 
// contais byte subsequence
const CONSONANT_LIST: [char; 21] = [
    'b', 'c', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'm', 'n', 'p', 'q', 'r', 's', 't', 'v', 'x', 'z', 'w', 'y'
];

fn calc_is_present(seq: &Vec<u8>, subseq: &Vec<u8>) -> bool {
    let mut is_present = false;

    if seq.len() == 0 {
        return is_present;
    }

    let (seq_head, seq_tail) = seq.split_at(1);
    let (subseq_head, subseq_tail) = subseq.split_at(1);

    match seq_head == subseq_head {
        false => is_present = calc_is_present(&Vec::from(seq_tail), &subseq),
        true => is_present = match subseq_tail.len() {
            0 => true,
            _ => calc_is_present(&Vec::from(seq_tail), &Vec::from(subseq_tail))
        },
    };

    is_present
}

// fn find_sequence(sequence: &Vec<u8>, subsequence: &Vec<Vec<u8>>) {

// }

fn pig_latin(input: &str) -> String {
    let input = String::from(input);
    let mut output = String::new();

    for word in input.split_whitespace() {
        for (i, chr) in word.chars().enumerate() {
            println!("{}: {}", i, chr);
            
        }
    }

    output
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

    // 2. Convert strings to pig latin. The first consonant of each word is 
    // moved to the end of the word and “ay” is added, so “first” becomes 
    // “irst-fay.” Words that start with a vowel have “hay” added to the end 
    // instead (“apple” becomes “apple-hay”). Keep in mind the details about 
    // UTF-8 encoding!

    let seq = vec![0, 5, 6, 7, 2, 3, 7, 7, 9, 0, 1];
    let subseq = vec![3, 7, 7];

    println!("{}", calc_is_present(&seq, &subseq));

    // let first_apple = "first apple";

    // pig_latin(&first_apple);
}
