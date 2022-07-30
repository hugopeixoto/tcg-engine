use crate::state::*;
use crate::cli::CLIDrawTarget;

pub trait CardArchetype {
    // probably want to add the Zone of the card
    fn card_actions(&self, player: Player, card: &Card, engine: &GameEngine) -> Vec<Action>;
    fn execute(&self, player: Player, card: &Card, engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameEngine;
    fn stage(&self) -> Option<Stage>;
    fn attacks(&self, player: Player, in_play: &InPlayCard, engine: &GameEngine) -> Vec<Action>;
    fn provides(&self) -> Vec<Type>;
    fn hp(&self) -> Option<usize>;
}

pub trait CardDB {
    fn archetype(&self) -> Box<dyn CardArchetype>;
}

#[derive(Clone)]
pub struct RFA {
    code: fn(&GameEngine, &mut dyn DecisionMaker) -> GameEngine,
}

impl RFA {
    pub fn new(code: fn(&GameEngine, &mut dyn DecisionMaker) -> GameEngine) -> Self {
        RFA { code }
    }
}

impl AttackBody for RFA {
    fn run(&self, engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameEngine {
        (self.code)(engine, dm)
    }

    fn boxed_clone(&self) -> Box<dyn AttackBody> {
        Box::new(self.clone())
    }
}


#[derive(Clone)]
pub struct GameEngine {
    pub state: GameState,
    pub resolving_actions: Vec<Action>,
}

#[derive(Default)]
pub struct Flips {
    results: Vec<bool>,
}

impl Flips {
    pub fn from_results(results: Vec<bool>) -> Self {
        Flips { results }
    }

    pub fn heads(&self) -> usize {
        self.results.iter().filter(|&x| *x).count()
    }
}

pub trait DecisionMaker {
    fn shuffler(&mut self) -> &mut dyn Shuffler;
    fn flip(&mut self, number_of_coins: usize) -> Flips;

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Fighting,
    Fire,
    Grass,
    Lightning,
    Psychic,
    Water,
    Dark,
    Metal,
    Fairy,
    Dragon,
    Colorless,
}

#[derive(PartialEq, Eq, Debug)]
pub enum SetupActiveSelection {
    Mulligan,
    Place(Card),
}

pub trait AttackBody {
    fn run(&self, engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameEngine;
    fn boxed_clone(&self) -> Box<dyn AttackBody>;
}

impl Clone for Box<dyn AttackBody> {
    fn clone(&self) -> Self {
        self.boxed_clone()
    }
}

#[derive(Clone)]
pub enum Action {
    Pass,
    TrainerFromHand(Player, Card),
    AttachFromHand(Player, Card),
    BenchFromHand(Player, Card),
    Attack(Player, InPlayCard, String, Box<dyn AttackBody>),
}
impl std::fmt::Debug for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Action::TrainerFromHand(_player, card) => { write!(f, "Play {}", card.archetype) },
            Action::AttachFromHand(_player, card) => { write!(f, "Attach {}", card.archetype) },
            Action::BenchFromHand(_player, card) => { write!(f, "Bench {}", card.archetype) },
            Action::Attack(_player, in_play, name, _) => { write!(f, "Attack with {}: {}", in_play.stack[0].card().archetype, name) },
            Action::Pass => { write!(f, "Pass") },
        }
    }
}

impl GameEngine {
    pub fn play(&self, dm: &mut dyn DecisionMaker) -> Self {
        let mut engine = self.clone();
        while !engine.is_finished() {
            CLIDrawTarget::print(&engine);
            engine = engine.step(dm);
        }

        engine
    }

    pub fn is_finished(&self) -> bool {
        match self.state.stage {
            GameStage::Winner(_) | GameStage::Tie => { true },
            _ => { false },
        }
    }

    pub fn step(&self, dm: &mut dyn DecisionMaker) -> Self {
        let mut engine = match self.state.stage {
            GameStage::Uninitialized => { self.setup(dm) },
            GameStage::Winner(_) => { self.clone() },
            GameStage::Tie => { self.clone() },
            GameStage::StartOfTurn(player) => {
                if self.state.side(player).deck.is_empty() {
                    self.with_state(self.state.with_stage(GameStage::Winner(player.opponent())))
                } else {
                    self.with_state(self.state.draw_to_hand(player, dm.shuffler()).with_stage(GameStage::Turn(player)))
                }
            },
            GameStage::Turn(player) => {
                println!("available actions for {:?}:", player);
                let actions = self.available_actions(player);
                for (i, action) in actions.iter().enumerate() {
                    println!(" {}. {:?}", i + 1, action);
                }

                let action = dm.pick_action(player, &actions);

                match &action {
                    Action::Pass => {
                        self.end_turn()
                    },
                    Action::TrainerFromHand(_, card) => {
                        card.archetype().execute(player, card, &self, dm)
                    },
                    Action::AttachFromHand(_, card) => {
                        card.archetype().execute(player, card, &self, dm)
                    },
                    Action::Attack(_, _in_play, _name, executor) => {
                       self
                            .push_action(action.clone())
                            .then(|e| executor.run(&e, dm))
                            .pop_action()
                            .end_turn()
                    },
                    Action::BenchFromHand(_, card) => {
                        self.with_state(self.state.bench_from_hand(player, card))
                    },
                }
            },
            GameStage::PokemonCheckup(player) => {
                self.with_state(self.state.with_stage(GameStage::StartOfTurn(player)))
            }
        };

        // Q. When both active Pokémon are knocked out, who places a new active first?
        // A. The player whose turn would be next.
        // https://compendium.pokegym.net/ruling/818/
        let who_first = match engine.state.stage {
            GameStage::Turn(player) => player.opponent(),
            GameStage::StartOfTurn(player) => player.opponent(),
            _ => Player::One,
        };

        for who in [who_first, who_first.opponent()] {
            // TODO: 2v2 games
            while engine.state.side(who).active.len() < 1 && !engine.state.side(who).bench.is_empty() {
                let chosen = dm.pick_in_play(who, 1, &engine.state.side(who).bench);
                engine.state = engine.state.promote(chosen[0]);
            }
        }

        // Well, if one of you has a Benched Pokémon to replace your Active Pokémon and
        // the other player doesn't, then the person who can replace his or her Active
        // Pokémon wins. Otherwise, you play Sudden Death. This is explained in the
        // Pokémon rules in the Expert Rules section under "What Happens if Both Players
        // Win at the Same Time?"
        // https://compendium.pokegym.net/ruling/882/
        let [a, b] = [Player::One, Player::Two].map(|player| {
            let prize_done = engine.state.side(player).prizes.is_empty();
            let no_active = engine.state.side(player.opponent()).active.is_empty();

            (if prize_done { 1 } else { 0 }) + (if no_active { 1 } else { 0 })
        });

        if a > 0 && a > b {
            engine.state.stage = GameStage::Winner(Player::One);
        } else if b > 0 && b > a {
            engine.state.stage = GameStage::Winner(Player::Two);
        } else if b > 0 && b == a {
            engine.state.stage = GameStage::Tie;
        }

        engine
    }

    pub fn attach_from_hand(&self, player: Player, card: &Card, target: &InPlayID) -> Self {
        self.with_state(self.state.attach_from_hand(player, card, target))
    }

    // attack in flight
    pub fn player(&self) -> Player {
        match self.resolving_actions.last() {
            Some(Action::Attack(player, _, _, _)) => player.clone(),
            _ => { panic!("Error accessing GameEngine::player() while not attacking"); }
        }
    }

    pub fn opponent(&self) -> Player {
        match self.resolving_actions.last() {
            Some(Action::Attack(player, _, _, _)) => player.opponent(),
            _ => { panic!("Error accessing GameEngine::opponent() while not attacking"); }
        }
    }

    pub fn attacking(&self) -> &InPlayCard {
        match self.resolving_actions.last() {
            Some(Action::Attack(_, attacking, _, _)) => attacking,
            _ => { panic!("Error accessing GameEngine::attacking() while not attacking"); }
        }
    }

    pub fn defending(&self) -> &InPlayCard {
        match self.resolving_actions.last() {
            Some(Action::Attack(player, _, _, _)) => &self.state.side(player.opponent()).active[0],
            _ => { panic!("Error accessing GameEngine::defending() while not attacking"); }
        }
    }

    pub fn damage(&self, damage: usize) -> Self {
        match self.resolving_actions.last() {
            Some(Action::Attack(player, _, _, _)) => {
                let defending = &self.state.side(player.opponent()).active[0];
                self.with_state(self.state.add_damage_counters(defending, damage/10))
            },
            _ => { panic!("wat"); },
        }
    }

    pub fn paralyze(&self) -> Self {
        match self.resolving_actions.last() {
            Some(Action::Attack(player, _, _, _)) => {
                let defending = &self.state.side(player.opponent()).active[0];
                self.with_state(self.state.paralyze(defending))
            },
            _ => { panic!("wat"); },
        }
    }

    pub fn end_turn(&self) -> Self {
        let mut engine = self.clone();
        let player = match engine.state.stage {
            GameStage::Turn(player) => player,
            stage => { panic!("Can't end turn while in stage {:?}", stage); }
        };

        engine.state = engine.state.with_stage(GameStage::PokemonCheckup(player.opponent()));

        engine.state.effects.retain(|e| match e.expires {
            EffectExpiration::EndOfTurn(p, 0) => p != player,
            _ => true,
        });
        for effect in engine.state.effects.iter_mut() {
            match effect.expires {
                EffectExpiration::EndOfTurn(p, t) => {
                    if p == player {
                        effect.expires = EffectExpiration::EndOfTurn(p, t - 1)
                    }
                },
                _ => {},
            }
        }

        engine
    }

    pub fn with_effect(&self, effect: Effect) -> Self {
        let mut engine = self.clone();
        engine.state.effects.push(effect);
        engine
    }

    // TODO: this should be removed, it's a temporary thing
    pub fn with_state(&self, state: GameState) -> Self {
        Self { resolving_actions: self.resolving_actions.clone(), state }
    }

    pub fn push_action(&self, action: Action) -> Self {
        let mut engine = self.clone();
        engine.resolving_actions.push(action);
        engine
    }

    pub fn pop_action(&self) -> Self {
        let mut engine = self.clone();
        engine.resolving_actions.pop();
        engine
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

        for active in self.state.side(player).active.iter() {
            actions.extend(self.in_play_actions(player, active, true));
        }

        for benched in self.state.side(player).bench.iter() {
            actions.extend(self.in_play_actions(player, benched, false));
        }

        actions.push(Action::Pass);

        actions
    }

    pub fn in_play_actions(&self, player: Player, in_play: &InPlayCard, active: bool) -> Vec<Action> {
        if active && in_play.rotational_status != RotationalStatus::Paralyzed {
            in_play.stack[0].card().archetype().attacks(player, in_play, self)
        } else {
            vec![]
        }
    }

    pub fn card_actions(&self, player: Player, card: &Card) -> Vec<Action> {
        if self.is_trainer(card) {
            return card.archetype().card_actions(player, card, self);
        } else if self.is_energy(card) {
            return card.archetype().card_actions(player, card, self);
        }

        if self.can_bench_from_hand(card) {
            return vec![Action::BenchFromHand(player, card.clone())]
        }

        vec![]
    }

    pub fn can_play_trainer_from_hand(&self, card: &Card) -> bool {
        self.state.effects.iter()
            .filter(|e| e.target == EffectTarget::Player(card.owner))
            .filter(|e| e.consequence == EffectConsequence::BlockTrainerFromHand)
            .count() == 0
    }

    pub fn is_attack_energy_cost_met(&self, in_play: &InPlayCard, cost: &[Type]) -> bool {
        let mut energy = vec![];
        for attached in in_play.attached.iter() {
            if self.is_energy(attached.card()) {
                energy.extend(attached.card().archetype().provides());
            }
        }

        for required in cost {
            match energy.iter().position(|c| c == required) {
                Some(p) => { energy.remove(p); },
                None => { return false; }
            }
        }

        true
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

    pub fn setup(&self, dm: &mut dyn DecisionMaker) -> Self {
        let mut engine = self.clone();

        let mut p1selection = SetupActiveSelection::Mulligan;
        let mut p2selection = SetupActiveSelection::Mulligan;

        while p1selection == SetupActiveSelection::Mulligan && p2selection == SetupActiveSelection::Mulligan {
            // 1. each player shuffles their deck
            engine.state = engine.state.shuffle_hand_into_deck(Player::One).shuffle_hand_into_deck(Player::Two);

            // 2. each player draws 7 cards
            engine.state = engine.state
                .draw_n_to_hand(Player::One, 7, dm.shuffler())
                .draw_n_to_hand(Player::Two, 7, dm.shuffler());

            // 3. players pick a card to be their active pokemon (face down)
            p1selection = engine.confirm_setup_selection(Player::One, dm);
            p2selection = engine.confirm_setup_selection(Player::Two, dm);
        }

        // place selections
        if let SetupActiveSelection::Place(card) = &p1selection {
            engine.state = engine.state.play_from_hand_to_active_face_down(Player::One, card);
        }

        if let SetupActiveSelection::Place(card) = &p2selection {
            engine.state = engine.state.play_from_hand_to_active_face_down(Player::Two, card);
        }

        while p2selection == SetupActiveSelection::Mulligan {
            // p2 shuffles, draws 7, selects again
            engine.state = engine.state.shuffle_hand_into_deck(Player::Two).draw_n_to_hand(Player::Two, 7, dm.shuffler());
            p2selection = engine.confirm_setup_selection(Player::Two, dm);

            // p1 is asked to draw 0,1,2 cards
            let n = dm.confirm_mulligan_draw(Player::One, 2);
            engine.state = engine.state.draw_n_to_hand(Player::One, n, dm.shuffler());
        }

        while p1selection == SetupActiveSelection::Mulligan {
            // p1 shuffles, draws 7, selects again
            engine.state = engine.state.shuffle_hand_into_deck(Player::One).draw_n_to_hand(Player::One, 7, dm.shuffler());
            p1selection = engine.confirm_setup_selection(Player::One, dm);

            // p2 is asked to draw 0,1,2 cards
            let n = dm.confirm_mulligan_draw(Player::Two, 2);
            engine.state = engine.state.draw_n_to_hand(Player::Two, n, dm.shuffler());
        }

        if let SetupActiveSelection::Place(card) = &p1selection {
            if engine.state.p1.active.is_empty() {
                engine.state = engine.state.play_from_hand_to_active_face_down(Player::One, card);
            }
        }

        if let SetupActiveSelection::Place(card) = &p2selection {
            if engine.state.p2.active.is_empty() {
                engine.state = engine.state.play_from_hand_to_active_face_down(Player::Two, card);
            }
        }

        // TODO: flip coin to decide who goes first, or check for First Ticket DRV 19.
        engine = engine.setup_bench(dm).setup_prizes(dm).setup_reveal_pokemon();

        // TODO: check for abilities that activate on reveal, like Sableye SF 48

        engine.state.stage = GameStage::Turn(Player::One);

        println!("Hand sizes: {}, {}", engine.state.p1.hand.len(), engine.state.p2.hand.len());
        engine
    }

    pub fn setup_bench(&self, dm: &mut dyn DecisionMaker) -> Self {
        let mut engine = self.clone();

        let p1bench = engine.confirm_bench_selection(Player::One, dm);
        let p2bench = engine.confirm_bench_selection(Player::Two, dm);

        for card in p1bench {
            engine.state = engine.state.play_from_hand_to_bench_face_down(Player::One, &card);
        }
        for card in p2bench {
            engine.state = engine.state.play_from_hand_to_bench_face_down(Player::Two, &card);
        }

        engine
    }

    pub fn setup_prizes(&self, dm: &mut dyn DecisionMaker) -> Self {
        let mut engine = self.clone();

        for _ in 0..6 {
            engine.state = engine.state.draw_to_prizes(Player::One, dm.shuffler());
        }

        for _ in 0..6 {
            engine.state = engine.state.draw_to_prizes(Player::Two, dm.shuffler());
        }

        engine
    }

    pub fn setup_reveal_pokemon(&self) -> Self {
        let mut engine = self.clone();

        engine.state = engine.state.reveal_pokemon(Player::One);
        engine.state = engine.state.reveal_pokemon(Player::Two);

        engine
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
        if card.archetype == "Mysterious Fossil (FO 62)" {
            Maybe::Maybe
        } else if self.stage(card) == Some(Stage::Basic) {
            Maybe::Yes
        } else {
            Maybe::No
        }
    }

    pub fn placeable_as_benched_during_setup(&self, card: &Card) -> bool {
        if card.archetype == "Mysterious Fossil (FO 62)" {
            true
        } else if self.stage(card) == Some(Stage::Basic) {
            true
        } else {
            false
        }
    }

    pub fn can_bench_from_hand(&self, card: &Card) -> bool {
        if card.archetype == "Mysterious Fossil (FO 62)" {
            true
        } else if self.stage(card) == Some(Stage::Basic) {
            true
        } else {
            false
        }
    }

    pub fn is_trainer(&self, card: &Card) -> bool {
        match card.archetype.as_str() {
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

    pub fn remaining_hp(&self, in_play: &InPlayCard) -> usize {
        in_play.stack[0].card().archetype().hp().unwrap_or(0).saturating_sub(in_play.damage_counters * 10)
    }
    pub fn is_energy(&self, card: &Card) -> bool {
        self.is_basic_energy(card) || card.archetype == "Double Colorless Energy (BS 96)"
    }

    pub fn is_basic_energy(&self, card: &Card) -> bool {
        match card.archetype.as_str() {
            "Psychic Energy (BS 101)" => true,
            "Water Energy (BS 102)" => true,
            _ => false,
        }
    }

    pub fn stage(&self, card: &Card) -> Option<Stage> {
        match card.archetype.as_str() {
            "Psyduck (FO 53)" | "Voltorb (BS 67)" | "Growlithe (BS 28)" | "Gastly (FO 33)" => Some(Stage::Basic),
            "Squirtle (BS 63)" | "Articuno (FO 17)" => Some(Stage::Basic),
            "Electrode (BS 21)" | "Arcanine (BS 23)" | "Wartortle (BS 42)" => Some(Stage::Stage1),
            "Blastoise (BS 2)" => Some(Stage::Stage2),
            _ => None,
        }
    }

    pub fn then_if<F>(&self, condition: bool, f: F) -> Self where F: FnOnce(&Self) -> Self {
        if condition {
            f(self)
        } else {
            self.clone()
        }
    }

    pub fn then<F>(&self, f: F) -> Self where F: FnOnce(&Self) -> Self {
        f(self)
    }
}
