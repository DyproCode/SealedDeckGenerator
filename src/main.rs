mod card;
mod sealed_deck;
mod file_generator;
use file_generator::FileGenerator;
use sealed_deck::SealedDeck;

fn main() {
    let mut file_generator = FileGenerator::new();

    for _ in 0..2{
        let sealed = SealedDeck::new(8);
        file_generator.write_cards(&sealed.cards);
    }
}
