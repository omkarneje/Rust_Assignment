use std::io;

fn shortest_word(sentence: &str) -> Option<&str> {
    sentence.split_whitespace().min_by_key(|word| word.len())
}

fn main() {
    // Input string of words
    println!("Enter a string:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input = input.trim();

    // Find the shortest word
    match shortest_word(&input) {
        Some(shortest) => println!("{}", shortest),
        None => println!("No words found in the input"),
    }
}
