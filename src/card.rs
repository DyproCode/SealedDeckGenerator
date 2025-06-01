use std::fs::File;
use std::io::{self, BufRead};
use once_cell::sync::Lazy;

#[derive(Debug, Clone)]
pub struct Card {
    pub name: String,
    pub color: String,
    pub super_type: String,
    pub card_type: String,
    pub sub_type: String,
    pub cost: String,
    pub card_text: String,
    pub color_requirements: String,
    pub attack: String,
    pub health: String,
    pub rarity: String,
}

impl Card {
    pub fn to_string(&self)
    {
        println!("Name: {}", self.name);
    }
}

pub fn read_cards() -> Vec<Card> {
    let mut cards: Vec<Card> = Vec::new();

    let path = "cards.csv";
    let file = File::open(path)
        .expect("file not found");
    let reader = io::BufReader::new(file);
    let lines = reader.lines();

    for line in lines {
        let line = line.expect("Could not read line");
        let fields: Vec<&str> = line.split(',').collect();

        let card = Card {
            name: fields[0].trim().to_string(),
            color: fields[1].trim().to_string(),
            super_type: fields[2].trim().to_string(),
            card_type: fields[3].trim().to_string(),
            sub_type: fields[4].trim().to_string(),
            cost: fields[5].trim().to_string(),
            card_text: fields[6].trim().to_string(),
            color_requirements: fields[7].trim().to_string(),
            attack: fields[8].trim().to_string(),
            health: fields[9].trim().to_string(),
            rarity: fields[10].trim().to_string(),
        };

        cards.push(card);
    }

    cards
}

pub static ALL_CARDS: Lazy<Vec<Card>> = Lazy::new(|| {
    read_cards()
});

