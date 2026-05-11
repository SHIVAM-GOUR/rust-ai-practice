use std::collections::HashMap;

fn main() {
    let text: String = String::from("hello world hello rust world hello");
    println!("{}", text);

    let mut counts: HashMap<&str, i32> = HashMap::new();

    for word in text.split_whitespace() {
        let count = counts.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:#?}", counts);
}
