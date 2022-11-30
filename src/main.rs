use std::fs;
use std::collections::HashMap;
use std::collections::hash_map::Entry;


fn build_dictionary(file_path: &str) -> HashMap<String, HashMap<String, u32>> {
    let mut dictionary: HashMap<String, HashMap<String, u32>> = HashMap::new();
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
    let dictionary = build_dictionary("alice.txt");

    println!("{:?}", dictionary);
}
