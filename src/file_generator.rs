use std::fs::{File, OpenOptions};
use std::path::Path;
use std::fs;
use std::io::Write;
use crate::card::Card;

pub struct FileGenerator {
    deck_number: i32,
}

impl FileGenerator {
    pub fn new() -> Self {
        let path = Path::new("file_stats.txt");

        if !path.exists() {
            File::create("file_stats.txt").expect("Unable to create file");
        }

        let contents = fs::read_to_string(path).expect("Unable to read file");
        let deck_number = contents.trim().parse().unwrap_or(0);

        FileGenerator {
            deck_number
        }
    }

    fn update(&mut self) {
        self.deck_number += 1;
        let path = Path::new("file_stats.txt");

        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true) // clear previous contents
            .open(path)
            .expect("Unable to open file");

        writeln!(file, "{}", self.deck_number).expect("Unable to write to file");
    }

    pub fn write_cards(&mut self, cards: &[Card]) {
        let file_name = format!("SealedDecks/sealed_deck_{}.csv", self.deck_number);
        let mut file = File::create(file_name).expect("Unable to create file");

        writeln!(file, "Name,Color,Super Type,Card Type,Sub Type,Cost,Color Requirements,Card Text,Attack,Health,Rarity");

        for card in cards {
            writeln!(file, "{},{},{},{},{},{},{},{},{},{},{}",
                     card.name, card.color, card.super_type, card.card_type, card.sub_type, card.cost, card.color_requirements, card.card_text, card.attack, card.health, card.rarity
            ).expect("Unable to write to file");
        }

        self.update();
    }
}

