extern crate rand;
use crate::rand::Rng;

mod state;
mod cli;
mod engine;
mod carddb;

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

    fn pick_action<'a>(&mut self, _p: Player, actions: &'a Vec<Action>) -> &'a Action {
        let mut choice = None;

        while choice.is_none() {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).expect("Failed to read input");
            choice = input.trim().parse::<usize>().ok();
        }

        &actions[choice.unwrap() - 1]
    }

    fn pick_target<'a>(&mut self, _p: Player, targets: &'a Vec<InPlayID>) -> &'a InPlayID {
        let mut choice = None;

        println!("available targets: {:?}", targets);

        while choice.is_none() || !targets.contains(&choice.unwrap()) {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).expect("Failed to read input");
            choice = input.trim().parse::<InPlayID>().ok();
        }

        targets.iter().find(|&&x| x == choice.unwrap()).unwrap()
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

    fn pick_from_discard<'a>(&mut self, _p: Player, whose: Player, how_many: usize, _discard: &Vec<Card>, searchable: &'a Vec<Card>) -> Vec<&'a Card> {
        let mut choice = None;

        println!("Pick {} cards from {:?}'s hand:", how_many, whose);
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
}

fn main() {
    let state = GameState::initial(
        &[
            "Psyduck (FO 53)", "Psyduck (FO 53)", "Psyduck (FO 53)", "Psyduck (FO 53)",
            "Voltorb (BS 67)", "Voltorb (BS 67)", "Voltorb (BS 67)", "Voltorb (BS 67)",
            "Electrode (BS 21)", "Electrode (BS 21)", "Electrode (BS 21)", "Electrode (BS 21)",
            "Growlithe (BS 28)", "Growlithe (BS 28)", "Growlithe (BS 28)",
            "Arcanine (BS 23)", "Arcanine (BS 23)", "Arcanine (BS 23)",
            "Gastly (FO 33)",
            "Bill (BS 91)", "Bill (BS 91)", "Bill (BS 91)", "Bill (BS 91)",
            "Computer Search (BS 71)", "Computer Search (BS 71)", "Computer Search (BS 71)", "Computer Search (BS 71)",
            "Item Finder (BS 74)", "Item Finder (BS 74)", "Item Finder (BS 74)", "Item Finder (BS 74)",
            "Professor Oak (BS 88)", "Professor Oak (BS 88)", "Professor Oak (BS 88)", "Professor Oak (BS 88)",
            "Lass (BS 75)", "Lass (BS 75)", "Lass (BS 75)",
            "Switch (BS 95)", "Switch (BS 95)",
            "Pokemon Trader (BS 77)", "Pokemon Trader (BS 77)",
            "Maintenance (BS 83)", "Maintenance (BS 83)",
            "Defender (BS 80)", "Defender (BS 80)",
            "Energy Removal (BS 92)", "Energy Removal (BS 92)",
            "PlusPower (BS 84)",
            "Scoop Up (BS 78)",
            "Psychic Energy (BS 101)", "Psychic Energy (BS 101)", "Psychic Energy (BS 101)", "Psychic Energy (BS 101)",
            "Psychic Energy (BS 101)", "Psychic Energy (BS 101)",
            "Double Colorless Energy (BS 96)", "Double Colorless Energy (BS 96)", "Double Colorless Energy (BS 96)", "Double Colorless Energy (BS 96)",
        ],
        &[
            "Squirtle (BS 63)", "Squirtle (BS 63)", "Squirtle (BS 63)", "Squirtle (BS 63)",
            "Wartortle (BS 42)",
            "Blastoise (BS 2)", "Blastoise (BS 2)", "Blastoise (BS 2)",
            "Articuno (FO 17)", "Articuno (FO 17)", "Articuno (FO 17)", "Articuno (FO 17)",
            "Bill (BS 91)", "Bill (BS 91)", "Bill (BS 91)", "Bill (BS 91)",
            "Computer Search (BS 71)", "Computer Search (BS 71)", "Computer Search (BS 71)", "Computer Search (BS 71)",
            "Energy Retrieval (BS 81)", "Energy Retrieval (BS 81)", "Energy Retrieval (BS 81)", "Energy Retrieval (BS 81)",
            "Pokemon Breeder (BS 76)", "Pokemon Breeder (BS 76)", "Pokemon Breeder (BS 76)", "Pokemon Breeder (BS 76)",
            "Professor Oak (BS 88)", "Professor Oak (BS 88)", "Professor Oak (BS 88)", "Professor Oak (BS 88)",
            "Item Finder (BS 74)", "Item Finder (BS 74)", "Item Finder (BS 74)",
            "Maintenance (BS 83)", "Maintenance (BS 83)",
            "Super Energy Removal (BS 79)", "Super Energy Removal (BS 79)",
            "Super Potion (BS 90)", "Super Potion (BS 90)",
            "Switch (BS 95)", "Switch (BS 95)",
            "PlusPower (BS 84)",
            "Gust of Wind (BS 93)",
            "Lass (BS 75)",
            "Water Energy (BS 102)", "Water Energy (BS 102)", "Water Energy (BS 102)", "Water Energy (BS 102)",
            "Water Energy (BS 102)", "Water Energy (BS 102)", "Water Energy (BS 102)", "Water Energy (BS 102)",
            "Water Energy (BS 102)", "Water Energy (BS 102)", "Water Energy (BS 102)", "Water Energy (BS 102)",
            "Water Energy (BS 102)", "Water Energy (BS 102)",
        ],
    );

    GameEngine { resolving_actions: vec![], state }.play(&mut CLI { });
}
