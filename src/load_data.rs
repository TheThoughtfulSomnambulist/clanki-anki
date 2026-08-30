use serde::Deserialize;
use std::fs;

#[derive(Deserialize, Debug)]
struct FrontEndCard {
    id: String,
    question: String,
    answer: String,
    difficulty: i32,
}
#[derive(Debug)]
struct Deck {
    cards: Vec<FrontEndCard>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_directories = fs::read_dir("src/data/").unwrap();

    // initialising decks
    let mut decks: Vec<Deck> = Vec::new();
    let mut cards: Vec<FrontEndCard> = Vec::new();
    let mut filepath_str: String = String::new();

    for file in file_directories {
        let entry = file.unwrap();
        let json_data = fs::read_to_string(entry.path()).unwrap();
        cards = serde_json::from_str(&json_data).unwrap();
        decks.push(Deck { cards });
    }

    // checking length
    println!("Loaded {} decks!", decks.len());
}
