fn reverse_string(input: &str) -> String {
    let mut chars: Vec<char> = input.chars().collect();
    chars.reverse();
    let reversed_string: String = chars.into_iter().collect();
    reversed_string
}

fn main() {
    let original_string = "Hello, World!";
    let reversed_string = reverse_string(original_string);
    println!("Original string: {}", original_string);
    println!("Reversed string: {}", reversed_string);
}