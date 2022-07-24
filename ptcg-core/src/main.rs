#![feature(trait_upcasting)]

extern crate rand;
use crate::rand::Rng;

mod state;
mod cli;
mod engine;

use state::*;
use engine::*;

impl CardDB for Card {
    fn archetype(&self) -> Box<dyn CardArchetype> {
        match self.archetype.as_str() {
            "Clefairy Doll (BS 70)"             => Trainer::create::<ClefairyDoll>(),
            "Computer Search (BS 71)"           => Trainer::create::<ComputerSearch>(),
            //"Devolution Spray (BS 72)"        => Trainer::create::<DevolutionSpray>(),
            "Impostor Professor Oak (BS 73)"    => Trainer::create::<ImpostorProfessorOak>(),
            "Item Finder (BS 74)"               => Trainer::create::<ItemFinder>(),
            "Lass (BS 75)"                      => Trainer::create::<Lass>(),
            "Pokemon Breeder (BS 76)"           => Trainer::create::<PokemonBreeder>(),
            "Pokemon Trader (BS 77)"            => Trainer::create::<PokemonTrader>(),
            "Scoop Up (BS 78)"                  => Trainer::create::<ScoopUp>(),
            //"Super Energy Removal (BS 79)"    => Trainer::create::<SuperEnergyRemoval>(),
            "Defender (BS 80)"                  => Trainer::create::<Defender>(),
            "Energy Retrieval (BS 81)"          => Trainer::create::<EnergyRetrieval>(),
            //"Full Heal (BS 82)"               => Trainer::create::<FullHeal>(),
            "Maintenance (BS 83)"               => Trainer::create::<Maintenance>(),
            "PlusPower (BS 84)"                 => Trainer::create::<PlusPower>(),
            //"Pokemon Center (BS 85)"          => Trainer::create::<PokemonCenter>(),
            //"Pokemon Flute (BS 86)"           => Trainer::create::<PokemonFlute>(),
            "Pokedex (BS 87)"                   => Trainer::create::<Pokedex>(),
            "Professor Oak (BS 88)"             => Trainer::create::<ProfessorOak>(),
            //"Revive (BS 89)"                  => Trainer::create::<Revive>(),
            //"Super Potion (BS 90)"            => Trainer::create::<SuperPotion>(),
            "Bill (BS 91)"                      => Trainer::create::<Bill>(),
            //"Energy Removal (BS 92)"          => Trainer::create::<EnergyRemoval>(),
            "Gust of Wind (BS 93)"              => Trainer::create::<GustOfWind>(),
            //"Potion (BS 94)"                  => Trainer::create::<Potion>(),
            "Switch (BS 95)"                    => Trainer::create::<Switch>(),
            "Fighting Energy (BS 97)"           => BasicEnergy::create(Type::Fighting),
            "Fire Energy (BS 98)"               => BasicEnergy::create(Type::Fire),
            "Grass Energy (BS 99)"              => BasicEnergy::create(Type::Grass),
            "Electric Energy (BS 100)"          => BasicEnergy::create(Type::Lightning),
            "Psychic Energy (BS 101)"           => BasicEnergy::create(Type::Psychic),
            "Water Energy (BS 102)"             => BasicEnergy::create(Type::Water),
            _                                   => Box::new(NOOP::default()),
            //"Devolution Spray (BS 72)" => mine.in_play.any(is_evolution),
            //"Super Energy Removal (BS 79)" => mine.in_play.any(energy_attached(1..)) && opp.in_play.any(energy_attached(1..)),
            //"Full Heal (BS 82)" => self.active.any(asleep|confused|paralyzed|poisoned),
            //"Pokemon Center (BS 85)" => mine.in_play.any((has_damage_counters|energy_attached(1..))&can_damage_counters_be_removed),
            //"Pokemon Flute (BS 86)" => self.discard_pile_has_basic_pokemon(opp) && !opp.can_bench
            //"Revive (BS 89)" => same as pokeflute but for ourselves,
            //"Super Potion (BS 90)" => mine.in_play.any(has_damage_counters&energy_attached(1..)),
            //"Energy Removal (BS 92)" => opp.in_play.any(energy_attached(1..))
            //"Potion (BS 94)" => mine.in_play.any(has_damage_counters),
        }
    }
}

struct BasicEnergy {
    energy_type: Type,
}
impl BasicEnergy {
    pub fn create(energy_type: Type) -> Box<dyn CardArchetype> {
        Box::new(BasicEnergy { energy_type })
    }
}
impl CardArchetype for BasicEnergy {
    fn stage(&self) -> Option<Stage> {
        None
    }

    fn card_actions(&self, player: Player, card: &Card, engine: &GameEngine) -> Vec<Action> {
        if !engine.attachment_from_hand_targets(player, card).is_empty() {
            vec![Action::AttachFromHand(card.clone())]
        } else {
            vec![]
        }
    }

    fn execute(&self, player: Player, card: &Card, engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameState {
        let targets = engine.attachment_from_hand_targets(player, card);
        let target = dm.pick_target(player, &targets);

        let mut state = engine.state.attach_from_hand(player, card, *target);
        if engine.is_energy(card) {
            let effect = Effect {
                source: EffectSource::Energy(player, card.clone()),
                target: EffectTarget::Player(player),
                expires: EffectExpiration::EndOfTurn(player, 0),
                consequence: EffectConsequence::BlockAttachmentFromHand,
                name: "ENERGY_ATTACH_FOR_TURN".into(),
            };
            state.effects.push(effect);
        }

        state
    }
}

trait TrainerCardArchetype {
    fn requirements_ok(&self, player: Player, card: &Card, engine: &GameEngine) -> bool;
    fn execute(&self, player: Player, card: &Card, engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameState;
}

struct Trainer {
    archetype: Box<dyn TrainerCardArchetype>,
}
impl Trainer {
    pub fn create<T: Default + TrainerCardArchetype + 'static>() -> Box<dyn CardArchetype> {
        Box::new(Self { archetype: Box::new(T::default()) })
    }
}
impl CardArchetype for Trainer {
    fn card_actions(&self, player: Player, card: &Card, engine: &GameEngine) -> Vec<Action> {
        if self.archetype.requirements_ok(player, card, engine) {
            vec![Action::TrainerFromHand(card.clone())]
        } else {
            vec![]
        }
    }

    fn execute(&self, player: Player, card: &Card, engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameState {
        self.archetype.execute(player, card, engine, dm).discard_from_hand(player, card)
    }

    fn stage(&self) -> Option<Stage> {
        None
    }
}

#[derive(Default)]
struct ClefairyDoll {}
impl TrainerCardArchetype for ClefairyDoll {
    fn requirements_ok(&self, player: Player, card: &Card, engine: &GameEngine) -> bool {
        engine.can_bench(player, card)
    }

    fn execute(&self, _player: Player, _card: &Card, _engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameState {
        panic!("not implemented");
    }
}

#[derive(Default)]
struct ComputerSearch {}
impl TrainerCardArchetype for ComputerSearch {
    // cost: discard(2, from: hand)
    // effect: search(1, from: deck); move(it, to: hand)
    fn requirements_ok(&self, player: Player, card: &Card, engine: &GameEngine) -> bool {
        engine.can_discard_other(player, card, 2) && !engine.state.side(player).deck.is_empty()
    }
    fn execute(&self, player: Player, _card: &Card, engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameState {
        let mut state = engine.state.clone();

        let cost = dm.pick_from_hand(player, player, 2, &engine.state.side(player).hand); // TODO: can't pick used card
        for discarded in cost {
            state = state.discard_from_hand(player, discarded);
        }

        let deck_cards = engine.state.side(player).deck.cards();
        let chosen = dm.search_deck(player, player, 1, &deck_cards);
        for searched in chosen {
            state = state.tutor_to_hand(player, searched);
        }

        state.shuffle_deck(player)
    }
}

#[derive(Default)]
struct ImpostorProfessorOak {}
impl TrainerCardArchetype for ImpostorProfessorOak {
    // effect: shuffle(hand, to: deck); draw(7)
    fn requirements_ok(&self, player: Player, _card: &Card, engine: &GameEngine) -> bool {
        !engine.state.side(player.opponent()).deck.is_empty() || engine.state.side(player.opponent()).hand.len() > 7
    }
    fn execute(&self, player: Player, _card: &Card, engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameState {
        let opponent = player.opponent();

        engine.state.shuffle_hand_into_deck(opponent).draw_n_to_hand(opponent, 7, dm)
    }
}

#[derive(Default)]
struct ItemFinder {}
impl TrainerCardArchetype for ItemFinder {
    // cost: discard(2, from: hand)
    // effect: me.search(1.item, from: discard); move(it, to: hand)
    fn requirements_ok(&self, player: Player, card: &Card, engine: &GameEngine) -> bool {
        engine.can_discard_other(player, card, 2) && engine.discard_pile_has_trainer(player, card)
    }
    fn execute(&self, player: Player, _card: &Card, engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameState {
        let mut state = engine.state.clone();

        let discard_cards = engine.state.side(player).discard.clone();
        let searchable_cards = discard_cards.iter().filter(|c| engine.is_trainer(c)).cloned().collect();

        let cost = dm.pick_from_hand(player, player, 2, &engine.state.side(player).hand); // TODO: can't pick used card

        for discarded in cost {
            state = state.discard_from_hand(player, discarded);
        }

        let chosen = dm.pick_from_discard(player, player, 1, &discard_cards, &searchable_cards);
        for searched in chosen {
            state = state.discard_to_hand(player, searched);
        }

        state
    }
}

#[derive(Default)]
struct Lass {}
impl TrainerCardArchetype for Lass {
    // effect: me.reveal(all.trainer, from: hand); me.shuffle(it, to: deck)
    // effect: opp.reveal(all.trainer, from: hand); opp.shuffle(it, to: deck)
    fn requirements_ok(&self, _player: Player, _card: &Card, _engine: &GameEngine) -> bool {
        true
    }
    fn execute(&self, player: Player, card: &Card, engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameState {
        panic!("not implemented");
    }
}

#[derive(Default)]
struct PokemonBreeder {}
impl TrainerCardArchetype for PokemonBreeder {
    fn requirements_ok(&self, _player: Player, _card: &Card, _engine: &GameEngine) -> bool {
        true // todo: bunch of checks
    }
    fn execute(&self, player: Player, card: &Card, engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameState {
        panic!("not implemented");
    }
}

#[derive(Default)]
struct PokemonTrader {}
impl TrainerCardArchetype for PokemonTrader {
    fn requirements_ok(&self, _player: Player, _card: &Card, _engine: &GameEngine) -> bool {
        true // TODO: pokemon communication
    }
    fn execute(&self, player: Player, card: &Card, engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameState {
        panic!("not implemented");
    }
}

#[derive(Default)]
struct ScoopUp {}
impl TrainerCardArchetype for ScoopUp {
    fn requirements_ok(&self, player: Player, _card: &Card, engine: &GameEngine) -> bool {
        engine.state.side(player).in_play().iter().any(|p| p.stack.iter().any(|card| engine.stage(card.card()) == Some(Stage::Basic)))
    }
    fn execute(&self, player: Player, _card: &Card, engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameState {
        let choices = engine.state.side(player).in_play().into_iter().filter(|p| p.stack.iter().any(|card| engine.stage(card.card()) == Some(Stage::Basic))).cloned().collect();

        let chosen = dm.pick_in_play(player, 1, &choices);

        let mut state = engine.state.clone();
        for card in chosen[0].cards() {
            if engine.stage(card) == Some(Stage::Basic) {
                state = state.move_card_to_hand(card);
            } else {
                state = state.move_card_to_discard(card);
            }
        }

        state
    }
}

#[derive(Default)]
struct Defender {}
impl TrainerCardArchetype for Defender {
    fn requirements_ok(&self, _player: Player, _card: &Card, _engine: &GameEngine) -> bool {
        true
    }
    fn execute(&self, player: Player, card: &Card, engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameState {
        panic!("not implemented");
    }
}

#[derive(Default)]
struct EnergyRetrieval {}
impl TrainerCardArchetype for EnergyRetrieval {
    fn requirements_ok(&self, player: Player, card: &Card, engine: &GameEngine) -> bool {
        engine.can_discard_other(player, card, 1) && engine.discard_pile_has_basic_energy(player, card)
    }
    fn execute(&self, player: Player, _card: &Card, engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameState {
        let mut state = engine.state.clone();

        let discard_cards = engine.state.side(player).discard.clone();
        let searchable_cards = discard_cards.iter().filter(|c| engine.is_basic_energy(c)).cloned().collect();

        let cost = dm.pick_from_hand(player, player, 2, &engine.state.side(player).hand); // TODO: can't pick used card

        for discarded in cost {
            state = state.discard_from_hand(player, discarded);
        }

        let chosen = dm.pick_from_discard(player, player, 1, &discard_cards, &searchable_cards);
        for searched in chosen {
            state = state.discard_to_hand(player, searched);
        }

        state
    }
}

#[derive(Default)]
struct Maintenance {}
impl TrainerCardArchetype for Maintenance {
    fn requirements_ok(&self, player: Player, card: &Card, engine: &GameEngine) -> bool {
        engine.can_discard_other(player, card, 2) // TODO: not discard but shuffle
    }
    fn execute(&self, player: Player, _card: &Card, engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameState {
        let mut state = engine.state.clone();

        let cost = dm.pick_from_hand(player, player, 2, &engine.state.side(player).hand); // TODO: can't pick used card
        for shuffled in cost {
            state = state.shuffle_from_hand_into_deck(player, shuffled);
        }

        state.draw_n_to_hand(player, 1, dm)
    }
}

#[derive(Default)]
struct PlusPower {}
impl TrainerCardArchetype for PlusPower {
    fn requirements_ok(&self, _player: Player, _card: &Card, _engine: &GameEngine) -> bool {
        true
    }
    fn execute(&self, player: Player, card: &Card, engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameState {
        panic!("not implemented");
    }
}

#[derive(Default)]
struct Pokedex {}
impl TrainerCardArchetype for Pokedex {
    fn requirements_ok(&self, player: Player, _card: &Card, engine: &GameEngine) -> bool {
        !engine.state.side(player).deck.is_empty()
    }
    fn execute(&self, player: Player, card: &Card, engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameState {
        panic!("not implemented");
    }
}

#[derive(Default)]
struct ProfessorOak {}
impl TrainerCardArchetype for ProfessorOak {
    fn requirements_ok(&self, player: Player, _card: &Card, engine: &GameEngine) -> bool {
        !engine.state.side(player).deck.is_empty()
    }
    fn execute(&self, player: Player, _card: &Card, engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameState {
        let mut state = engine.state.clone();

        for discarded in engine.state.side(player).hand.iter() {
            state = state.discard_from_hand(player, discarded);
        }

        state.draw_n_to_hand(player, 7, dm)
    }
}

#[derive(Default)]
struct Bill {}
impl TrainerCardArchetype for Bill {
    fn requirements_ok(&self, player: Player, _card: &Card, engine: &GameEngine) -> bool {
        !engine.state.side(player).deck.is_empty()
    }
    fn execute(&self, player: Player, _card: &Card, engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameState {
        engine.state.draw_n_to_hand(player, 2, dm)
    }
}

#[derive(Default)]
struct GustOfWind {}
impl TrainerCardArchetype for GustOfWind {
    fn requirements_ok(&self, player: Player, _card: &Card, engine: &GameEngine) -> bool {
        !engine.state.side(player.opponent()).bench.is_empty()
    }
    fn execute(&self, player: Player, _card: &Card, engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameState {
        let chosen = dm.pick_in_play(player, 1, &engine.state.side(player.opponent()).bench);

        // TODO: clear effects on defending
        engine.state.switch_active_with(&chosen[0])
    }
}

#[derive(Default)]
struct Switch {}
impl TrainerCardArchetype for Switch {
    fn requirements_ok(&self, player: Player, _card: &Card, engine: &GameEngine) -> bool {
        !engine.state.side(player).bench.is_empty()
    }
    fn execute(&self, player: Player, _card: &Card, engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameState {
        let chosen = dm.pick_in_play(player, 1, &engine.state.side(player).bench);

        // TODO: clear effects on defending
        engine.state.switch_active_with(&chosen[0])
    }
}

#[derive(Default)]
struct NOOP {}
impl CardArchetype for NOOP {
    fn card_actions(&self, _player: Player, _card: &Card, _engine: &GameEngine) -> Vec<Action> {
        vec![]
    }
    fn execute(&self, _player: Player, _card: &Card, _engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameState {
        panic!("not implemented");
    }
    fn stage(&self) -> Option<Stage> {
        None
    }
}

struct CLI {
}

impl Shuffler for CLI {
    fn random_card(&mut self, n: usize) -> usize {
        rand::thread_rng().gen_range(0..n)
    }
}

impl DecisionMaker for CLI {
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

    fn pick_in_play<'a>(&mut self, _p: Player, how_many: usize, searchable: &'a Vec<InPlayCard>) -> Vec<&'a InPlayCard> {
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

    fn search_deck<'a>(&mut self, p: Player, whose: Player, how_many: usize, deck: &'a Vec<Card>) -> Vec<&'a Card> {
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

    let mut game = GameEngine { state };
    let mut cli = CLI {};

    game.play(&mut cli);
}
