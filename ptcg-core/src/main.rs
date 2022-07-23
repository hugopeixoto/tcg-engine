#![feature(derive_default_enum)]
#![feature(trait_upcasting)]

extern crate rand;
use crate::rand::Rng;

mod state;
mod cli;

use state::*;

pub struct GameEngine {
    pub state: GameState,
}

pub trait DecisionMaker: Shuffler {
    fn confirm_setup_mulligan(&mut self, p: Player);
    fn confirm_setup_active_or_mulligan(&mut self, p: Player, maybe: &Vec<Card>) -> SetupActiveSelection;
    fn confirm_setup_active(&mut self, p: Player, yes: &Vec<Card>, maybe: &Vec<Card>) -> Card;
    fn confirm_mulligan_draw(&mut self, p: Player, upto: usize) -> usize;
    fn confirm_setup_bench_selection(&mut self, p: Player, cards: &Vec<Card>) -> Vec<Card>;
    fn pick_action<'a>(&mut self, p: Player, actions: &'a Vec<Action>) -> &'a Action;
    fn pick_target<'a>(&mut self, p: Player, actions: &'a Vec<InPlayID>) -> &'a InPlayID;
    fn pick_from_hand<'a>(&mut self, p: Player, whose: Player, how_many: usize, hand: &'a Vec<Card>) -> Vec<&'a Card>;
    fn pick_from_discard<'a>(&mut self, p: Player, whose: Player, how_many: usize, discard: &Vec<Card>, searchable: &'a Vec<Card>) -> Vec<&'a Card>;
    fn pick_in_play<'a>(&mut self, p: Player, how_many: usize, searchable: &'a Vec<InPlayCard>) -> Vec<&'a InPlayCard>;
    fn search_deck<'a>(&mut self, p: Player, whose: Player, how_many: usize, deck: &'a Vec<Card>) -> Vec<&'a Card>;
}

#[derive(PartialEq, Eq)]
pub enum Maybe {
    Yes,
    No,
    Maybe,
}

#[derive(PartialEq, Eq)]
pub enum Stage {
    // Baby,
    Basic,
    // Break,
    // Legend,
    // LevelUp,
    // Mega,
    // Restored,
    Stage1,
    Stage2,
    // VStar,
    // VUnion,
}

#[derive(PartialEq, Eq, Debug)]
pub enum SetupActiveSelection {
    Mulligan,
    Place(Card),
}

#[derive(Debug, Clone)]
pub enum Action {
    Pass,
    TrainerFromHand(Card),
    AttachFromHand(Card),
    BenchFromHand(Card),
}

trait CardArchetype {
    // probably want to add the Zone of the card
    fn card_actions(&self, player: Player, card: &Card, engine: &GameEngine) -> Vec<Action>;
    fn execute(&self, player: Player, card: &Card, engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameState;
}

trait TrainerCardArchetype {
    fn requirements_ok(&self, player: Player, card: &Card, engine: &GameEngine) -> bool;
    fn execute(&self, player: Player, card: &Card, engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameState;
}

struct GenericCard {
    archetype: Box<dyn CardArchetype>,
}
impl GenericCard {
    pub fn new(archetype: Box<dyn CardArchetype>) -> Self {
        Self { archetype }
    }

    pub fn from(identifier: &str) -> Self {
        match identifier {
            "Clefairy Doll (BS 70)"             => GenericCard::new(Box::new(Trainer::new(Box::new(ClefairyDoll::default())))),
            "Computer Search (BS 71)"           => GenericCard::new(Box::new(Trainer::new(Box::new(ComputerSearch::default())))),
            //"Devolution Spray (BS 72)"          => GenericCard::new(Box::new(Trainer::new(Box::new(DevolutionSpray::default())))),
            "Impostor Professor Oak (BS 73)"    => GenericCard::new(Box::new(Trainer::new(Box::new(ImpostorProfessorOak::default())))),
            "Item Finder (BS 74)"               => GenericCard::new(Box::new(Trainer::new(Box::new(ItemFinder::default())))),
            "Lass (BS 75)"                      => GenericCard::new(Box::new(Trainer::new(Box::new(Lass::default())))),
            "Pokemon Breeder (BS 76)"           => GenericCard::new(Box::new(Trainer::new(Box::new(PokemonBreeder::default())))),
            "Pokemon Trader (BS 77)"            => GenericCard::new(Box::new(Trainer::new(Box::new(PokemonTrader::default())))),
            "Scoop Up (BS 78)"                  => GenericCard::new(Box::new(Trainer::new(Box::new(ScoopUp::default())))),
            //"Super Energy Removal (BS 79)"      => GenericCard::new(Box::new(Trainer::new(Box::new(SuperEnergyRemoval::default())))),
            "Defender (BS 80)"                  => GenericCard::new(Box::new(Trainer::new(Box::new(Defender::default())))),
            "Energy Retrieval (BS 81)"          => GenericCard::new(Box::new(Trainer::new(Box::new(EnergyRetrieval::default())))),
            //"Full Heal (BS 82)"                 => GenericCard::new(Box::new(Trainer::new(Box::new(FullHeal::default())))),
            "Maintenance (BS 83)"               => GenericCard::new(Box::new(Trainer::new(Box::new(Maintenance::default())))),
            "PlusPower (BS 84)"                 => GenericCard::new(Box::new(Trainer::new(Box::new(PlusPower::default())))),
            //"Pokemon Center (BS 85)"            => GenericCard::new(Box::new(Trainer::new(Box::new(PokemonCenter::default())))),
            //"Pokemon Flute (BS 86)"             => GenericCard::new(Box::new(Trainer::new(Box::new(PokemonFlute::default())))),
            "Pokedex (BS 87)"                   => GenericCard::new(Box::new(Trainer::new(Box::new(Pokedex::default())))),
            "Professor Oak (BS 88)"             => GenericCard::new(Box::new(Trainer::new(Box::new(ProfessorOak::default())))),
            //"Revive (BS 89)"                    => GenericCard::new(Box::new(Trainer::new(Box::new(Revive::default())))),
            //"Super Potion (BS 90)"              => GenericCard::new(Box::new(Trainer::new(Box::new(SuperPotion::default())))),
            "Bill (BS 91)"                      => GenericCard::new(Box::new(Trainer::new(Box::new(Bill::default())))),
            //"Energy Removal (BS 92)"            => GenericCard::new(Box::new(Trainer::new(Box::new(EnergyRemoval::default())))),
            "Gust of Wind (BS 93)"              => GenericCard::new(Box::new(Trainer::new(Box::new(GustOfWind::default())))),
            //"Potion (BS 94)"                    => GenericCard::new(Box::new(Trainer::new(Box::new(Potion::default())))),
            "Switch (BS 95)"                    => GenericCard::new(Box::new(Trainer::new(Box::new(Switch::default())))),
            _ => GenericCard::new(Box::new(NOOP::default())),
            // "Devolution Spray (BS 72)" => mine.in_play.any(is_evolution),
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

struct Trainer {
    archetype: Box<dyn TrainerCardArchetype>,
}
impl Trainer {
    pub fn new(archetype: Box<dyn TrainerCardArchetype>) -> Self {
        Self { archetype }
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
}

#[derive(Default)]
struct ClefairyDoll {}
impl TrainerCardArchetype for ClefairyDoll {
    fn requirements_ok(&self, player: Player, card: &Card, engine: &GameEngine) -> bool {
        engine.can_bench(player, card)
    }

    fn execute(&self, player: Player, card: &Card, engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameState {
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
    fn requirements_ok(&self, _player: Player, _card: &Card, _engine: &GameEngine) -> bool {
        true
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

        state.shuffle_deck(player)
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
    fn requirements_ok(&self, _player: Player, _card: &Card, _engine: &GameEngine) -> bool {
        true
    }
    fn execute(&self, player: Player, card: &Card, engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameState {
        panic!("not implemented");
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
    fn execute(&self, player: Player, card: &Card, engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameState {
        panic!("not implemented");
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
    fn execute(&self, player: Player, card: &Card, engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameState {
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
    fn execute(&self, player: Player, card: &Card, engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameState {
        panic!("not implemented");
    }
}

#[derive(Default)]
struct NOOP {}
impl CardArchetype for NOOP {
    fn card_actions(&self, _player: Player, _card: &Card, _engine: &GameEngine) -> Vec<Action> {
        vec![]
    }
    fn execute(&self, player: Player, card: &Card, engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameState {
        panic!("not implemented");
    }
}

impl GameEngine {
    pub fn play(&mut self, dm: &mut dyn DecisionMaker) {
        loop {
            cli::CLIDrawTarget::print(&self.state);
            match self.state.stage {
                GameStage::Uninitialized => { self.setup(dm); },
                GameStage::Winner(_) => { break; },
                GameStage::Tie => { break; },
                GameStage::StartOfTurn(player) => {
                    if self.state.side(player).deck.is_empty() {
                        self.state = self.state.with_stage(GameStage::Winner(player.opponent()));
                    } else {
                        self.state = self.state.draw_to_hand(player, dm);
                        self.state = self.state.with_stage(GameStage::Turn(player));
                    }
                },
                GameStage::Turn(player) => {
                    println!("available actions for {:?}:", player);
                    let actions = self.available_actions(player);
                    for (i, action) in actions.iter().enumerate() {
                        println!(" {}. {:?}", i + 1, action);
                    }

                    let action = dm.pick_action(player, &actions);

                    match action {
                        Action::Pass => {
                            // count down end of turn effects
                            self.state = self.state.with_stage(GameStage::StartOfTurn(player.opponent()));

                            self.state.effects.retain(|e| match e.expires {
                                EffectExpiration::EndOfTurn(p, 0) => p != player,
                                _ => true,
                            });
                            for effect in self.state.effects.iter_mut() {
                                match effect.expires {
                                    EffectExpiration::EndOfTurn(p, t) => {
                                        if p == player {
                                            effect.expires = EffectExpiration::EndOfTurn(p, t - 1)
                                        }
                                    },
                                    _ => {},
                                }
                            }
                        },
                        Action::TrainerFromHand(card) => {
                            self.state = GenericCard::from(card).archetype.execute(player, card, self, dm);
                        },
                        Action::AttachFromHand(card) => {
                            let targets = self.attachment_from_hand_targets(player, card);
                            println!("available targets: {:?}", targets);

                            let target = dm.pick_target(player, &targets);

                            self.state = self.state.attach_from_hand(player, card, *target);
                            if self.is_energy(card) {
                                let effect = Effect {
                                    source: EffectSource::Energy(player, card.clone()),
                                    target: EffectTarget::Player(player),
                                    expires: EffectExpiration::EndOfTurn(player, 0),
                                    consequence: EffectConsequence::BlockAttachmentFromHand,
                                    name: "ENERGY_ATTACH_FOR_TURN".into(),
                                };
                                self.state.effects.push(effect);
                            }
                        },
                        Action::BenchFromHand(card) => {
                            self.state = self.state.bench_from_hand(player, card);
                        },
                    }
                }
            }
        }
    }

    pub fn available_actions(&self, player: Player) -> Vec<Action> {
        let mut actions = vec![];
        // retreat
        // use trainer from hand
        // attach energy from hand
        // use ability actions
        // attack

        for card in self.state.side(player).hand.iter() {
            actions.extend(self.card_actions(player, card));
        }

        actions.push(Action::Pass);

        actions
    }

    pub fn card_actions(&self, player: Player, card: &Card) -> Vec<Action> {
        if self.is_trainer(card) {
            return GenericCard::from(card).archetype.card_actions(player, card, self);
        } else if self.is_energy(card) {
            if !self.attachment_from_hand_targets(player, card).is_empty() {
                return vec![Action::AttachFromHand(card.clone())];
            }
        }

        if self.can_bench_from_hand(card) {
            return vec![Action::BenchFromHand(card.clone())]
        }

        vec![]
    }

    pub fn attachment_from_hand_targets(&self, player: Player, _card: &Card) -> Vec<InPlayID> {
        let blocks = self.state.effects.iter()
            .filter(|e| e.consequence == EffectConsequence::BlockAttachmentFromHand)
            .filter(|e| e.target.is_player(player))
            .collect::<Vec<_>>();

        let mut targets = vec![];

        for active in self.state.side(player).active.iter() {
            targets.push(active.id.clone());
        }

        for benched in self.state.side(player).bench.iter() {
            targets.push(benched.id.clone());
        }

        for block in blocks {
            match &block.target {
                EffectTarget::Player(_) => {
                    return vec![];
                },
                EffectTarget::InPlay(_, id) => {
                    targets.retain(|x| x != id);
                },
            }
        }

        targets
    }

    pub fn can_bench(&self, player: Player, _card: &Card) -> bool {
        self.state.side(player).bench.len() < self.bench_size(player)
    }

    pub fn bench_size(&self, _player: Player) -> usize {
        5
    }

    pub fn can_discard_other(&self, player: Player, card: &Card, n: usize) -> bool {
        let this_card = self.state.side(player).hand.iter().filter(|c| *c == card).count();
        let other_cards = self.state.side(player).hand.iter().filter(|c| *c != card).count();

        this_card - 1 + other_cards >= n
    }

    pub fn discard_pile_has_trainer(&self, player: Player, _card: &Card) -> bool {
        self.state.side(player).discard.iter().find(|c| self.is_trainer(c)) != None
    }

    pub fn discard_pile_has_basic_energy(&self, player: Player, _card: &Card) -> bool {
        self.state.side(player).discard.iter().find(|c| self.is_basic_energy(c)) != None
    }

    pub fn setup(&mut self, dm: &mut dyn DecisionMaker) {
        let mut p1selection = SetupActiveSelection::Mulligan;
        let mut p2selection = SetupActiveSelection::Mulligan;

        while p1selection == SetupActiveSelection::Mulligan && p2selection == SetupActiveSelection::Mulligan {
            // 1. each player shuffles their deck
            self.state = self.state.shuffle_hand_into_deck(Player::One).shuffle_hand_into_deck(Player::Two);

            // 2. each player draws 7 cards
            self.state = self.state.draw_n_to_hand(Player::One, 7, dm).draw_n_to_hand(Player::Two, 7, dm);

            // 3. players pick a card to be their active pokemon (face down)
            p1selection = self.confirm_setup_selection(Player::One, dm);
            p2selection = self.confirm_setup_selection(Player::Two, dm);
        }

        // place selections
        if let SetupActiveSelection::Place(card) = &p1selection {
            self.state = self.state.play_from_hand_to_active_face_down(Player::One, card);
        }

        if let SetupActiveSelection::Place(card) = &p2selection {
            self.state = self.state.play_from_hand_to_active_face_down(Player::Two, card);
        }

        while p2selection == SetupActiveSelection::Mulligan {
            // p2 shuffles, draws 7, selects again
            self.state = self.state.shuffle_hand_into_deck(Player::Two).draw_n_to_hand(Player::Two, 7, dm);
            p2selection = self.confirm_setup_selection(Player::Two, dm);

            // p1 is asked to draw 0,1,2 cards
            let n = dm.confirm_mulligan_draw(Player::One, 2);
            self.state = self.state.draw_n_to_hand(Player::One, n, dm);
        }

        while p1selection == SetupActiveSelection::Mulligan {
            // p1 shuffles, draws 7, selects again
            self.state = self.state.shuffle_hand_into_deck(Player::One).draw_n_to_hand(Player::One, 7, dm);
            p1selection = self.confirm_setup_selection(Player::One, dm);

            // p2 is asked to draw 0,1,2 cards
            let n = dm.confirm_mulligan_draw(Player::Two, 2);
            self.state = self.state.draw_n_to_hand(Player::Two, n, dm);
        }

        if let SetupActiveSelection::Place(card) = &p1selection {
            if self.state.p1.active.is_empty() {
                self.state = self.state.play_from_hand_to_active_face_down(Player::One, card);
            }
        }

        if let SetupActiveSelection::Place(card) = &p2selection {
            if self.state.p2.active.is_empty() {
                self.state = self.state.play_from_hand_to_active_face_down(Player::Two, card);
            }
        }

        self.setup_bench(dm);
        self.setup_prizes(dm);
        // TODO: flip coin to decide who goes first, or check for First Ticket DRV 19.
        self.setup_reveal_pokemon();
        // TODO: check for abilities that activate on reveal, like Sableye SF 48

        self.state.stage = GameStage::Turn(Player::One);

        println!("Hand sizes: {}, {}", self.state.p1.hand.len(), self.state.p2.hand.len());
    }

    pub fn setup_bench(&mut self, dm: &mut dyn DecisionMaker) {
        let p1bench = self.confirm_bench_selection(Player::One, dm);
        let p2bench = self.confirm_bench_selection(Player::Two, dm);

        for card in p1bench {
            self.state = self.state.play_from_hand_to_bench_face_down(Player::One, &card);
        }
        for card in p2bench {
            self.state = self.state.play_from_hand_to_bench_face_down(Player::Two, &card);
        }
    }

    pub fn setup_prizes(&mut self, dm: &mut dyn DecisionMaker) {
        for _ in 0..6 {
            self.state = self.state.draw_to_prizes(Player::One, dm);
        }

        for _ in 0..6 {
            self.state = self.state.draw_to_prizes(Player::Two, dm);
        }
    }

    pub fn setup_reveal_pokemon(&mut self) {
        self.state = self.state.reveal_pokemon(Player::One);
        self.state = self.state.reveal_pokemon(Player::Two);
    }

    pub fn confirm_bench_selection(&self, player: Player, dm: &mut dyn DecisionMaker) -> Vec<Card> {
        let side = self.state.side(player);

        let benchable = side.hand.iter().filter(|c| self.placeable_as_benched_during_setup(c)).cloned().collect::<Vec<_>>();

        dm.confirm_setup_bench_selection(player, &benchable)
    }

    pub fn confirm_setup_selection(&self, player: Player, dm: &mut dyn DecisionMaker) -> SetupActiveSelection {
        let yes = self.state.side(player).hand.iter().filter(|c| self.placeable_as_active_during_setup(c) == Maybe::Yes).cloned().collect::<Vec<_>>();
        let maybe = self.state.side(player).hand.iter().filter(|c| self.placeable_as_active_during_setup(c) == Maybe::Maybe).cloned().collect::<Vec<_>>();

        println!("Player {:?}: {:?}", player, self.state.side(player).hand);
        println!("Player {:?}: Pick from {:?}, {:?}", player, yes, maybe);

        let selection = if yes.is_empty() && maybe.is_empty() {
            dm.confirm_setup_mulligan(player);
            SetupActiveSelection::Mulligan
        } else if yes.is_empty() {
            dm.confirm_setup_active_or_mulligan(player, &maybe)
        } else {
            SetupActiveSelection::Place(dm.confirm_setup_active(player, &yes, &maybe))
        };

        println!("Player {:?}: selected {:?}", player, selection);
        selection
    }

    pub fn placeable_as_active_during_setup(&self, card: &Card) -> Maybe {
        if card == "Mysterious Fossil (FO 62)" {
            Maybe::Maybe
        } else if self.stage(card) == Some(Stage::Basic) {
            Maybe::Yes
        } else {
            Maybe::No
        }
    }

    pub fn placeable_as_benched_during_setup(&self, card: &Card) -> bool {
        if card == "Mysterious Fossil (FO 62)" {
            true
        } else if self.stage(card) == Some(Stage::Basic) {
            true
        } else {
            false
        }
    }

    pub fn can_bench_from_hand(&self, card: &Card) -> bool {
        if card == "Mysterious Fossil (FO 62)" {
            true
        } else if self.stage(card) == Some(Stage::Basic) {
            true
        } else {
            false
        }
    }

    pub fn is_trainer(&self, card: &Card) -> bool {
        match card.as_str() {
            "Clefairy Doll (BS 70)" => true,
            "Computer Search (BS 71)" => true,
            "Devolution Spray (BS 72)" => true,
            "Impostor Professor Oak (BS 73)" => true,
            "Item Finder (BS 74)" => true,
            "Lass (BS 75)" => true,
            "Pokemon Breeder (BS 76)" => true,
            "Pokemon Trader (BS 77)" => true,
            "Scoop Up (BS 78)" => true,
            "Super Energy Removal (BS 79)" => true,
            "Defender (BS 80)" => true,
            "Energy Retrieval (BS 81)" => true,
            "Full Heal (BS 82)" => true,
            "Maintenance (BS 83)" => true,
            "PlusPower (BS 84)" => true,
            "Pokemon Center (BS 85)" => true,
            "Pokemon Flute (BS 86)" => true,
            "Pokedex (BS 87)" => true,
            "Professor Oak (BS 88)" => true,
            "Revive (BS 89)" => true,
            "Super Potion (BS 90)" => true,
            "Bill (BS 91)" => true,
            "Energy Removal (BS 92)" => true,
            "Gust of Wind (BS 93)" => true,
            "Potion (BS 94)" => true,
            "Switch (BS 95)" => true,
            _ => false
        }
    }

    pub fn is_energy(&self, card: &Card) -> bool {
        self.is_basic_energy(card) || card == "Double Colorless Energy (BS 96)"
    }

    pub fn is_basic_energy(&self, card: &Card) -> bool {
        match card.as_str() {
            "Psychic Energy (BS 101)" => true,
            "Water Energy (BS 102)" => true,
            _ => false,
        }
    }

    pub fn stage(&self, card: &Card) -> Option<Stage> {
        match card.as_str() {
            "Psyduck (FO 53)" | "Voltorb (BS 67)" | "Growlithe (BS 28)" | "Gastly (FO 33)" => Some(Stage::Basic),
            "Squirtle (BS 63)" | "Articuno (FO 17)" => Some(Stage::Basic),
            "Electrode (BS 21)" | "Arcanine (BS 23)" | "Wartortle (BS 42)" => Some(Stage::Stage1),
            "Blastoise (BS 2)" => Some(Stage::Stage2),
            _ => None,
        }
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
            println!("{}. {}", i + 1, card);
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
            println!("{}. {}", i + 1, card);
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
            println!("{}. {}", i + 1, card);
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
            "Impostor Professor Oak (BS 73)",
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
