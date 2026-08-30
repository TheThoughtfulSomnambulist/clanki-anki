use serde::Deserialize;
use std::fs;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct FrontEndCard {
    id: String,
    question: String,
    answer: String,
    difficulty: i32,
}
#[derive(Debug)]
pub struct Deck {
    cards: Vec<FrontEndCard>,
}


pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_directories = fs::read_dir("src/data/").unwrap();

    let mut decks: HashMap<String, Deck> = HashMap::new();

    for file in file_directories {
        let entry = file.unwrap();
        let filename = entry.path().file_stem().unwrap().to_str().unwrap().to_string();
        let json_data = fs::read_to_string(entry.path()).unwrap();
        
        let cards: Vec<FrontEndCard> = serde_json::from_str(&json_data).unwrap();
        let deck = Deck { cards };

        decks.insert(filename, deck);
    }

    println!("Loaded {} decks!", decks.len());

    Ok(decks)
}
