use serde::Deserialize;
use std::collections::HashMap;
use include_dir::{include_dir, Dir};

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

// temp for now
static DB: Dir = include_dir!("$CARGO_MANIFEST_DIR/src/db");

pub fn load_deck_data() -> HashMap<String, Deck> {

    let mut decks: HashMap<String, Deck> = HashMap::new();

    for file in DB.files() {
        let entry = file.path();
        let filename = entry
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        let json_data = file.contents_utf8().unwrap();

        let cards: Vec<FrontEndCard> = serde_json::from_str(&json_data).expect("Failed to parse desk JSON");
        let deck = Deck { cards };

        decks.insert(filename, deck);
    }
    

    log::info!("Loaded {} decks!", decks.len());

    decks
}
