use crate::card::{Card, ALL_CARDS};
use rand::Rng;

const RARES_PER_PACK: usize = 1;
const UNCOMMON_PER_PACK: usize = 3;
const COMMON_PER_PACK: usize = 6;

pub struct SealedDeck {
    pub cards: Vec<Card>
}

impl SealedDeck {
    pub fn new (num_packs: i32) -> SealedDeck {
        let mut cards = Vec::new();

        for _ in 0..num_packs {
            cards.extend(generate_pack());
        }

        let sealed_deck = SealedDeck {
            cards,
        };

        sealed_deck
    }
}

pub fn get_cards_by_rarity(rarity: &str) -> Vec<Card> {
    let cards_of_rarity = ALL_CARDS.iter()
        .filter(|c| c.rarity == rarity)
        .cloned()
        .collect::<Vec<Card>>();

    cards_of_rarity
}

pub fn get_random_card_from_collection(cards: &[Card]) -> &Card {
    let mut rng = rand::thread_rng();
    let num: usize = rng.gen_range(0..cards.len());
    &cards[num]
}

pub fn generate_pack() -> Vec<Card> {
    let mut pack: Vec<Card> = Vec::new();
    let rare_cards = get_cards_by_rarity("Rare");
    let uncommon_cards = get_cards_by_rarity("Uncommon");
    let common_cards = get_cards_by_rarity("Common");

    for _ in 0..RARES_PER_PACK {
        pack.push(get_random_card_from_collection(&rare_cards).clone())
    }

    for _ in 0..UNCOMMON_PER_PACK {
        pack.push(get_random_card_from_collection(&uncommon_cards).clone())
    }

    for _ in 0 ..COMMON_PER_PACK {
        pack.push(get_random_card_from_collection(&common_cards).clone())
    }

    pack
}





