use std::fs;
use std::env;
use std::collections::HashMap;
use std::collections::hash_map::Entry;

use rand::Rng;


fn build_dictionary(file_path: &str) -> HashMap<String, HashMap<String, i32>> {
    let mut dictionary: HashMap<String, HashMap<String, i32>> = HashMap::new();
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut previous = "";
    for word in contents.split_whitespace() {
        let values = match dictionary.entry(previous.to_string()) {
            Entry::Occupied(o) => o.into_mut(),
            Entry::Vacant(v) => v.insert(HashMap::new()),
        };
        *values.entry(word.to_owned()).or_insert(1) += 1;
        previous = word;
    }

    return dictionary;
}


fn main() {
    let mut dictionary = build_dictionary("alice.txt");
    let args: Vec<String> = env::args().collect();
    let mut seed = args[1].to_owned();
    let iterations = args[2].parse::<i32>().unwrap_or(10);

    let mut rng = rand::thread_rng();

    print!("{} ", seed);

    for _ in 1..=iterations {
        let values = match dictionary.entry(seed.clone()) {
            Entry::Occupied(o) => o.into_mut(),
            Entry::Vacant(_) => break,
        };
        
        let max_value = values.into_iter().map(|(_, v)| *v).sum();

        let mut choice = rng.gen_range(0..=max_value);
        for (key, value) in values.iter() {
            choice -= *value;
            if choice <= 0 {
                print!("{} ", key);
                seed = String::from(key);
                break;
            }
        }
    }
    println!();
}
