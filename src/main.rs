use std::collections::HashMap;
use std::io;

fn main() {
    println!("Enter a string:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    let mut vowel_count = HashMap::new();
    let vowels = "aeiouAEIOU";

    for char in input.chars() {
        if vowels.contains(char) {
            *vowel_count.entry(char).or_insert(0) += 1;
        }
    }

    let total_vowels: usize = vowel_count.values().sum();

    println!("Total vowels: {}", total_vowels);
    println!("Individual vowel counts:");
    for (vowel, count) in &vowel_count {
        println!("{}: {}", vowel, count);
    }
}
