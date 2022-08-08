#![feature(int_roundings)]

extern crate rand;
use crate::rand::Rng;

mod state;
mod cli;
mod engine;
mod carddb;
mod card_macros;

mod sets;

use state::*;
use engine::*;

struct CLI {
}

impl Shuffler for CLI {
    fn random_card(&mut self, n: usize) -> usize {
        rand::thread_rng().gen_range(0..n)
    }
}

impl DecisionMaker for CLI {
    fn shuffler(&mut self) -> &mut dyn Shuffler {
        self
    }

    fn flip(&mut self, number_of_coins: usize) -> Flips {
        let mut flips = vec![];

        for _ in 0..number_of_coins {
            let result = rand::thread_rng().gen();
            println!("coin flip: {}", if result { "Heads" } else { "Tails" });
            flips.push(result);
        }

        Flips::from_results(flips)
    }

    fn confirm_setup_mulligan(&mut self, _p: Player) {
    }

    fn confirm_setup_active_or_mulligan(&mut self, _p: Player, _maybe: &Vec<Card>) -> SetupActiveSelection {
        SetupActiveSelection::Mulligan
    }

    fn confirm_setup_active(&mut self, _p: Player, yes: &Vec<Card>, _maybe: &Vec<Card>) -> Card {
        yes[0].clone()
    }

    fn confirm_mulligan_draw(&mut self, _p: Player, upto: usize) -> usize {
        upto
    }

    fn confirm_setup_bench_selection(&mut self, _p: Player, cards: &Vec<Card>) -> Vec<Card> {
        cards.clone()
    }

    fn pick_action<'a>(&mut self, player: Player, actions: &'a Vec<Action>) -> &'a Action {
        let mut choice = None;

        println!("available actions for {:?}:", player);
        for (i, action) in actions.iter().enumerate() {
            println!(" {}. {:?}", i + 1, action);
        }

        println!("  d. Print my discard");
        println!("  D. Print opponent's discard");
        println!("  h. Print my hand");

        while choice.is_none() {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).expect("Failed to read input");
            match input.trim() {
                "d" => {},
                "D" => {},
                "h" => {},
                text => { choice = text.parse::<usize>().ok(); },
            }
        }

        &actions[choice.unwrap() - 1]
    }

    fn pick_stage<'a>(&mut self, player: Player, items: &'a Vec<Stage>) -> &'a Stage {
        let mut choice = None;

        println!("available stages for {:?}:", player);
        for (i, item) in items.iter().enumerate() {
            println!(" {}. {:?}", i + 1, item);
        }

        while choice.is_none() {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).expect("Failed to read input");
            choice = input.trim().parse::<usize>().ok();
        }

        &items[choice.unwrap() - 1]
    }

    fn pick_from_hand<'a>(&mut self, _p: Player, whose: Player, how_many: usize, hand: &'a Vec<Card>) -> Vec<&'a Card> {
        let mut choice = None;

        println!("Pick {} cards from {:?}'s hand:", how_many, whose);
        for (i, card) in hand.iter().enumerate() {
            println!("{}. {}", i + 1, card.archetype);
        }

        while choice.is_none() {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).expect("Failed to read input");
            let chosen = input.trim().split(",").filter_map(|c| c.parse::<usize>().ok()).collect::<Vec<_>>();
            if chosen.len() == how_many && chosen.iter().all(|&x| 1 <= x && x <= hand.len()) {
                choice = Some(chosen.iter().map(|i| &hand[i - 1]).collect());
            }
        }

        choice.unwrap()
    }

    fn pick_from_discard<'a>(&mut self, _p: Player, whose: Player, how_many: usize, searchable: &'a Vec<Card>) -> Vec<&'a Card> {
        let mut choice = None;

        println!("Pick {} cards from {:?}'s discard:", how_many, whose);
        for (i, card) in searchable.iter().enumerate() {
            println!("{}. {}", i + 1, card.archetype);
        }

        while choice.is_none() {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).expect("Failed to read input");
            let chosen = input.trim().split(",").filter_map(|c| c.parse::<usize>().ok()).collect::<Vec<_>>();
            if chosen.len() == how_many && chosen.iter().all(|&x| 1 <= x && x <= searchable.len()) {
                choice = Some(chosen.iter().map(|i| &searchable[i - 1]).collect());
            }
        }

        choice.unwrap()
    }

    fn pick_from_prizes<'a>(&mut self, _who: Player, whose: Player, how_many: usize, searchable: &'a Vec<PrizeCard>) -> Vec<&'a PrizeCard> {
        let mut choice = None;

        println!("Pick {} cards from {:?}'s hand:", how_many, whose);
        for (i, card) in searchable.iter().enumerate() {
            println!("{}. {:?}", i + 1, card.card);
        }

        while choice.is_none() {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).expect("Failed to read input");
            let chosen = input.trim().split(",").filter_map(|c| c.parse::<usize>().ok()).collect::<Vec<_>>();
            if chosen.len() == how_many && chosen.iter().all(|&x| 1 <= x && x <= searchable.len()) {
                choice = Some(chosen.iter().map(|i| &searchable[i - 1]).collect());
            }
        }

        choice.unwrap()
    }

    fn pick_in_play<'a>(&mut self, _player: Player, how_many: usize, searchable: &'a Vec<InPlayCard>) -> Vec<&'a InPlayCard> {
        let mut choice = None;

        println!("Pick {} in play pokemon:", how_many);
        for (i, card) in searchable.iter().enumerate() {
            println!("{}. {:?}", i + 1, card);
        }

        while choice.is_none() {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).expect("Failed to read input");
            let chosen = input.trim().split(",").filter_map(|c| c.parse::<usize>().ok()).collect::<Vec<_>>();
            if chosen.len() == how_many && chosen.iter().all(|&x| 1 <= x && x <= searchable.len()) {
                choice = Some(chosen.iter().map(|i| &searchable[i - 1]).collect());
            }
        }

        choice.unwrap()
    }

    fn pick_attached<'a>(&mut self, _player: Player, how_many: std::ops::RangeInclusive<usize>, searchable: &'a Vec<Card>) -> Vec<&'a Card> {
        let mut choice = None;

        println!("Pick {:?} in play pokemon:", how_many);
        for (i, card) in searchable.iter().enumerate() {
            println!("{}. {:?}", i + 1, card);
        }

        while choice.is_none() {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).expect("Failed to read input");
            let chosen = input.trim().split(",").filter_map(|c| c.parse::<usize>().ok()).collect::<Vec<_>>();
            if how_many.contains(&chosen.len()) && chosen.iter().all(|&x| 1 <= x && x <= searchable.len()) { // TODO: all different
                choice = Some(chosen.iter().map(|i| &searchable[i - 1]).collect());
            }
        }

        choice.unwrap()
    }

    fn search_deck<'a>(&mut self, _player: Player, whose: Player, how_many: usize, deck: &'a Vec<Card>) -> Vec<&'a Card> {
        let mut choice = None;

        println!("Pick {} cards from {:?}'s deck:", how_many, whose);
        for (i, card) in deck.iter().enumerate() {
            println!("{}. {}", i + 1, card.archetype);
        }

        while choice.is_none() {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).expect("Failed to read input");
            let chosen = input.trim().split(",").filter_map(|c| c.parse::<usize>().ok()).collect::<Vec<_>>();
            if chosen.len() == how_many && chosen.iter().all(|&x| 1 <= x && x <= deck.len()) {
                choice = Some(chosen.iter().map(|i| &deck[i - 1]).collect());
            }
        }

        choice.unwrap()
    }

    fn rearrange<'a>(&mut self, _p: Player, cards: &'a Vec<Card>) -> Vec<&'a Card> {
        let mut choice = vec![];

        println!("Rearrange the following {} cards:", cards.len());
        for (i, card) in cards.iter().enumerate() {
            println!("{}. {}", i + 1, card.archetype);
        }

        while choice.len() < cards.len() {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).expect("Failed to read input");
            let chosen = input.trim().parse::<usize>();
            if let Ok(chosen) = chosen {
                if 1 <= chosen && chosen <= cards.len() && !choice.contains(&(chosen - 1)) {
                    choice.push(chosen - 1);
                } else {
                    println!("Invalid card.");
                }
            }
        }

        choice.iter().map(|x| &cards[*x]).collect()
    }
}

use std::io::BufRead;
fn load_deck(filename: &str) -> Result<Vec<String>, std::io::Error> {
    let file = std::fs::File::open(filename)?;

    let mut lines = vec![];
    for line in std::io::BufReader::new(file).lines() {
        match line {
            Ok(line) => {
                match line.split_once(" ") {
                    None => { return Err(std::io::Error::new(std::io::ErrorKind::Other, "")); },
                    Some((how_many, who)) => {
                        match how_many.parse::<usize>() {
                            Err(_) => { return Err(std::io::Error::new(std::io::ErrorKind::Other, "")); },
                            Ok(how_many) => {
                                for _ in 0..how_many {
                                    lines.push(who.into());
                                }
                            }
                        }
                    }
                }
            },
            Err(err) => {
                return Err(err);
            },
        }
    }

    Ok(lines)
}

fn main() {
    let _raindance = load_deck("decks/base-fossil-rain-dance.deck").unwrap();
    let _arcanine_electrode = load_deck("decks/base-fossil-arcanine-electrode.deck").unwrap();
    let random_cards = load_deck("decks/base-fossil-random-cards.deck").unwrap();

    let state = GameState::initial(&random_cards, &random_cards);

    GameEngine::from_state(state).play(&mut CLI { });
}
