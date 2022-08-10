use crate::state::*;
use crate::cli::CLIDrawTarget;
use std::ops::Deref;

pub type Weakness = (usize, Vec<Type>);
pub type Resistance = (usize, Vec<Type>);

pub trait CardArchetype {
    // probably want to add the Zone of the card
    fn card_actions(&self, player: Player, card: &Card, engine: &GameEngine) -> Vec<Action>;
    fn execute(&self, player: Player, card: &Card, engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameEngine;
    fn stage(&self) -> Option<Stage>;
    fn evolves_from(&self) -> Option<String>;
    fn attacks(&self, player: Player, in_play: &InPlayCard, engine: &GameEngine) -> Vec<Action>;
    fn provides(&self) -> Vec<Type>;
    fn hp(&self, card: &Card, engine: &GameEngine) -> Option<usize>;
    fn weakness(&self) -> Weakness;
    fn resistance(&self) -> Resistance;
    fn pokemon_type(&self) -> Vec<Type>;
    fn name(&self) -> String;
    fn retreat(&self) -> usize;
    fn is_pokemon(&self, _card: &Card, _engine: &GameEngine) -> bool {
        self.stage().is_some()
    }
    fn is_trainer(&self, _card: &Card, _engine: &GameEngine) -> bool {
        false
    }
    fn defending_damage_effect(&self, _card: &Card, _engine: &GameEngine, _damage: usize) -> Option<usize> {
        None
    }
    fn attacking_damage_effect(&self, _card: &Card, _engine: &GameEngine, _damage: usize) -> Option<usize> {
        None
    }
    fn on_turn_end(&self, _card: &Card, _engine: &GameEngine) -> Option<GameEngine> {
        None
    }
}

pub trait CardDB {
    fn archetype<'a>(&self, format: &'a dyn Format) -> &'a dyn CardArchetype;
}

impl CardDB for Card {
    fn archetype<'a>(&self, format: &'a dyn Format) -> &'a dyn CardArchetype {
        format.behavior(self)
    }
}

#[derive(PartialEq, Eq)]
pub enum AttackingEffectsWhen {
    BeforeWR,
    AfterWR,
}

pub trait Format {
    fn behavior(&self, card: &Card) -> &dyn CardArchetype;
    fn attacking_effects(&self) -> AttackingEffectsWhen;
    fn basic_for_stage2(&self, card: &Card) -> String;
    fn boxed_clone(&self) -> Box<dyn Format>;
}

impl Clone for Box<dyn Format> {
    fn clone(&self) -> Box<dyn Format> {
        self.boxed_clone()
    }
}

#[derive(Clone)]
pub struct RFA {
    code: fn(AttackBuilder) -> AttackBuilder,
}

impl RFA {
    pub fn new(code: fn(AttackBuilder) -> AttackBuilder) -> Self {
        RFA { code }
    }
}

impl AttackBody for RFA {
    fn build<'a>(&self, engine: &GameEngine, dm: &'a mut dyn DecisionMaker) -> AttackBuilder<'a> {
        let builder = engine.attack_builder(dm);
        (self.code)(builder)
    }

    fn run(&self, engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameEngine {
        (&self.build(engine, dm)).into()
    }

    fn boxed_clone(&self) -> Box<dyn AttackBody> {
        Box::new(self.clone())
    }
}


#[derive(Clone)]
pub struct GameEngine {
    pub state: GameState,
    pub resolving_actions: Vec<Action>,
    pub attack_target_stack: Vec<(InPlayID, InPlayID)>,
    pub good: bool,
    pub format: Box<dyn Format>,
}

#[derive(Default)]
pub struct Flips {
    results: Vec<bool>,
}

impl Flips {
    pub fn from_results(results: Vec<bool>) -> Self {
        Flips { results }
    }

    pub fn is_heads(&self) -> bool {
        *self.results.first().unwrap()
    }

    pub fn is_tails(&self) -> bool {
        !*self.results.first().unwrap()
    }

    pub fn heads(&self) -> usize {
        self.results.iter().filter(|&x| *x).count()
    }

    pub fn tails(&self) -> usize {
        self.results.iter().filter(|&x| !*x).count()
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
    fn pick_stage<'a>(&mut self, p: Player, items: &'a Vec<Stage>) -> &'a Stage;
    fn pick_from_hand<'a>(&mut self, p: Player, whose: Player, how_many: usize, hand: &'a Vec<Card>) -> Vec<&'a Card>;
    fn pick_from_discard<'a>(&mut self, p: Player, whose: Player, how_many: usize, searchable: &'a Vec<Card>) -> Vec<&'a Card>;
    fn pick_in_play<'a>(&mut self, p: Player, how_many: usize, searchable: &'a Vec<InPlayCard>) -> Vec<&'a InPlayCard>;
    fn pick_attached<'a>(&mut self, p: Player, how_many: std::ops::RangeInclusive<usize>, searchable: &'a Vec<Card>) -> Vec<&'a Card>;
    fn pick_from_prizes<'a>(&mut self, who: Player, whose: Player, how_many: usize, searchable: &'a Vec<PrizeCard>) -> Vec<&'a PrizeCard>;
    fn search_deck<'a>(&mut self, p: Player, whose: Player, how_many: usize, deck: &'a Vec<Card>) -> Vec<&'a Card>;
    fn rearrange<'a>(&mut self, p: Player, cards: &'a Vec<Card>) -> Vec<&'a Card>;
}

#[derive(Default)]
pub struct FakeDM {}
impl Shuffler for FakeDM {
    fn random_card(&mut self, _n: usize) -> usize { 0 }
}
impl DecisionMaker for FakeDM {
    fn shuffler(&mut self) -> &mut dyn Shuffler { self }
    fn flip(&mut self, _number_of_coins: usize) -> Flips {
        Flips::from_results(vec![])
    }

    fn confirm_setup_mulligan(&mut self, _p: Player) {}
    fn confirm_setup_active_or_mulligan(&mut self, _p: Player, _maybe: &Vec<Card>) -> SetupActiveSelection { SetupActiveSelection::Mulligan }
    fn confirm_setup_active(&mut self, _p: Player, yes: &Vec<Card>, maybe: &Vec<Card>) -> Card { if !yes.is_empty() { yes[0].clone() } else { maybe[0].clone() } }
    fn confirm_mulligan_draw(&mut self, _p: Player, upto: usize) -> usize { upto }
    fn confirm_setup_bench_selection(&mut self, _p: Player, _cards: &Vec<Card>) -> Vec<Card> { vec![] }
    fn pick_action<'a>(&mut self, _p: Player, actions: &'a Vec<Action>) -> &'a Action { &actions[0] }
    fn pick_stage<'a>(&mut self, _p: Player, items: &'a Vec<Stage>) -> &'a Stage { &items[0] }
    fn pick_from_hand<'a>(&mut self, _p: Player, _whose: Player, how_many: usize, hand: &'a Vec<Card>) -> Vec<&'a Card> { hand[0..how_many].iter().collect() }
    fn pick_from_discard<'a>(&mut self, _p: Player, _whose: Player, how_many: usize, searchable: &'a Vec<Card>) -> Vec<&'a Card> { searchable[0..how_many].iter().collect() }
    fn pick_in_play<'a>(&mut self, _p: Player, how_many: usize, searchable: &'a Vec<InPlayCard>) -> Vec<&'a InPlayCard> { searchable[0..how_many].iter().collect() }
    fn pick_attached<'a>(&mut self, _p: Player, how_many: std::ops::RangeInclusive<usize>, searchable: &'a Vec<Card>) -> Vec<&'a Card> { searchable[0..*how_many.end()].iter().collect() }
    fn pick_from_prizes<'a>(&mut self, _who: Player, _whose: Player, how_many: usize, searchable: &'a Vec<PrizeCard>) -> Vec<&'a PrizeCard> { searchable[0..how_many].iter().collect() }
    fn search_deck<'a>(&mut self, _p: Player, _whose: Player, how_many: usize, deck: &'a Vec<Card>) -> Vec<&'a Card> { deck[0..how_many].iter().collect() }
    fn rearrange<'a>(&mut self, _p: Player, cards: &'a Vec<Card>) -> Vec<&'a Card> { cards.iter().collect() }
}

#[derive(PartialEq, Eq)]
pub enum Maybe {
    Yes,
    No,
    Maybe,
}

#[derive(Clone, Debug, PartialEq, Eq)]
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
    Any,
}

//#[derive(Debug, Clone, PartialEq, Eq)]
//pub enum EnergyRequirement {
//    Any,
//    OneOf(Vec<Type>),
//    Is(Type),
//}

#[derive(PartialEq, Eq, Debug)]
pub enum SetupActiveSelection {
    Mulligan,
    Place(Card),
}

pub trait AttackBody {
    fn run(&self, engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameEngine;
    fn build<'a>(&self, engine: &GameEngine, dm: &'a mut dyn DecisionMaker) -> AttackBuilder<'a>;
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
    EvolveFromHand(Player, Card),
    Attack(Player, InPlayCard, String, Box<dyn AttackBody>),
    Retreat(Player, InPlayCard),
}
impl std::fmt::Debug for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Action::TrainerFromHand(_player, card) => { write!(f, "Play {}", card.archetype) },
            Action::AttachFromHand(_player, card) => { write!(f, "Attach {}", card.archetype) },
            Action::BenchFromHand(_player, card) => { write!(f, "Bench {}", card.archetype) },
            Action::EvolveFromHand(_player, card) => { write!(f, "Evolve into {}", card.archetype) },
            Action::Attack(_player, in_play, name, _) => { write!(f, "Attack with {}: {}", in_play.stack[0].card().archetype, name) },
            Action::Retreat(_player, in_play) => { write!(f, "Retreat {}", in_play.stack[0].card().archetype) },
            Action::Pass => { write!(f, "Pass") },
        }
    }
}

#[derive(Clone, Debug)]
pub struct PrizeAward {
    player: Player,
    how_many: usize,
}


impl GameEngine {
    pub fn from_state(state: GameState, format: Box<dyn Format>) -> Self {
        Self {
            attack_target_stack: vec![],
            resolving_actions: vec![],
            state,
            good: true,
            format,
        }
    }

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
                let actions = self.available_actions(player);
                let action = dm.pick_action(player, &actions);

                match &action {
                    Action::Pass => {
                        self.end_turn()
                    },
                    Action::TrainerFromHand(_, card) => {
                        self
                            .push_action(action.clone())
                            .then(|e| self.archetype(card).execute(player, card, &e, dm))
                            .pop_action()
                    },
                    Action::AttachFromHand(_, card) => {
                        self.archetype(card).execute(player, card, &self, dm)
                    },
                    Action::EvolveFromHand(player, card) => {
                        self.evolve(*player, card, dm)
                    },
                    Action::Retreat(player, in_play) => {
                        self.retreat(*player, in_play, dm)
                    },
                    Action::Attack(player, attacking, _name, executor) => {
                       self
                            .push_action(action.clone())
                            .push_target(attacking, &self.state.side(player.opponent()).active[0])
                            .then(|e| executor.run(&e, dm))
                            .pop_target()
                            .pop_action()
                            .end_turn()
                    },
                    Action::BenchFromHand(_, card) => {
                        self.bench_from_hand(player, card)
                    },
                }.check_for_knockouts(dm)
            },
            GameStage::EndOfTurn(_) => {
                self.goto_pokemon_checkup()
            },
            GameStage::PokemonCheckup(player) => {
                self.with_state(self.state.with_stage(GameStage::StartOfTurn(player)).next_turn(player))
            }
        };

        engine = engine.check_promotion_needed(dm);
        engine = engine.check_win_conditions();
        engine
    }

    pub fn check_win_conditions(&self) -> Self {
        // Well, if one of you has a Benched Pokémon to replace your Active Pokémon and
        // the other player doesn't, then the person who can replace his or her Active
        // Pokémon wins. Otherwise, you play Sudden Death. This is explained in the
        // Pokémon rules in the Expert Rules section under "What Happens if Both Players
        // Win at the Same Time?"
        // https://compendium.pokegym.net/ruling/882/
        let [a, b] = [Player::One, Player::Two].map(|player| {
            let prize_done = self.state.side(player).prizes.is_empty();
            let no_active = self.state.side(player.opponent()).active.is_empty();

            (if prize_done { 1 } else { 0 }) + (if no_active { 1 } else { 0 })
        });

        if a > 0 && a > b {
            self.with_state(self.state.with_stage(GameStage::Winner(Player::One)))
        } else if b > 0 && b > a {
            self.with_state(self.state.with_stage(GameStage::Winner(Player::Two)))
        } else if b > 0 && b == a {
            self.with_state(self.state.with_stage(GameStage::Tie))
        } else {
            self.clone()
        }
    }

    pub fn check_promotion_needed(&self, dm: &mut dyn DecisionMaker) -> Self {
        // Q. When both active Pokémon are knocked out, who places a new active first?
        // A. The player whose turn would be next.
        // https://compendium.pokegym.net/ruling/818/
        let who_first = match self.state.stage {
            GameStage::Turn(player) => player.opponent(),
            GameStage::StartOfTurn(player) => player.opponent(),
            _ => Player::One,
        };

        let mut engine = self.clone();
        for who in [who_first, who_first.opponent()] {
            // TODO: 2v2 games
            while engine.state.side(who).active.len() < 1 && !engine.state.side(who).bench.is_empty() {
                let chosen = dm.pick_in_play(who, 1, &engine.state.side(who).bench);
                engine.state = engine.state.promote(chosen[0]);
            }
        }

        engine
    }

    pub fn check_for_knockouts(&self, dm: &mut dyn DecisionMaker) -> Self {
        let mut engine = self.clone();
        // attack:
        //   damage calculation
        //   damage counters
        //   attack effects
        //   hp check: check for pokemon with more damage counters than HP
        //   knock out:
        //     discard cards
        //     take prizes
        //   promote active

        // TODO: (billowing smoke) how do I know that the damage was from an attack?
        let mut prizes = vec![];

        loop {
            let mut at_zero = engine.state.all_in_play().into_iter().filter(|in_play| engine.remaining_hp(in_play) == 0);
            match at_zero.next() {
                None => { break; },
                Some(in_play) => {
                    let (p, e) = engine.knock_out(in_play);
                    engine = e;
                    prizes.push(p);
                },
            }

            break;
        }

        // take n prizes
        for prize in prizes {
            engine = engine.take_prize_card(prize.player, prize.how_many, dm);
        }

        engine
    }

    pub fn take_prize_card(&self, player: Player, how_many: usize, dm: &mut dyn DecisionMaker) -> Self {
        // TODO: intercept for greedy dice, treasure energy, dream ball

        let prizes = self.state.side(player).prizes.clone();
        let choices = dm.pick_from_prizes(player, player, how_many, &prizes);

        let mut engine = self.clone();
        for chosen in choices {
            engine = engine.with_state(engine.state.prize_to_hand(player, &chosen));
        }

        engine
    }

    pub fn knock_out(&self, in_play: &InPlayCard) -> (PrizeAward, Self) {
        // calculate prizes
        let mut engine = self.clone();
        for card in in_play.cards() {
            engine = engine.with_state(engine.state.move_card_to_discard(card));
        }

        (PrizeAward { player: in_play.owner.opponent(), how_many: 1 }, engine)
    }

    pub fn attach_from_hand(&self, player: Player, card: &Card, target: &InPlayCard) -> Self {
        self.with_state(self.state.attach_from_hand(player, card, &target.id))
    }

    // attack in flight
    pub fn player(&self) -> Player {
        match self.resolving_actions.last() {
            Some(Action::Attack(player, _, _, _)) => *player,
            Some(Action::TrainerFromHand(player, _)) => *player,
            _ => { panic!("Error accessing GameEngine::player() while not attacking or using a trainer card"); }
        }
    }

    pub fn opponent(&self) -> Player {
        match self.resolving_actions.last() {
            Some(Action::Attack(player, _, _, _)) => player.opponent(),
            Some(Action::TrainerFromHand(player, _)) => player.opponent(),
            _ => { panic!("Error accessing GameEngine::opponent() while not attacking or using a trainer card"); }
        }
    }

    pub fn attacking(&self) -> &InPlayCard {
        match self.attack_target_stack.last() {
            Some((attacking, _defending)) => self.state.in_play(attacking).unwrap(),
            _ => { panic!("Error accessing GameEngine::attacking() while not attacking"); }
        }
    }

    pub fn opponents_active_pokemon(&self) -> Vec<&InPlayCard> {
        self.state.side(self.opponent()).active.iter().collect()
    }

    pub fn defending(&self) -> &InPlayCard {
        match self.attack_target_stack.last() {
            Some((_attacking, defending)) => self.state.in_play(defending).unwrap(),
            _ => { panic!("Error accessing GameEngine::defending() while not attacking"); }
        }
    }

    pub fn target_all<T, F>(&self, targets: T, f: F) -> Self where T: IntoIterator<Item=InPlayCard>, F: Fn(&Self) -> Self {
        let mut engine = self.clone();
        for target in targets {
            engine = engine
                .push_target(self.attacking(), &target)
                .then(&f)
                .pop_target();
        }

        engine
    }

    pub fn damage(&self, mut damage: usize) -> Self {
        // TODO: when targetting benched pokémon, we shouldn't apply weakness and resistance by
        // default, but leave the door open for some attacks that do apply them.

        if self.format.attacking_effects() == AttackingEffectsWhen::BeforeWR {
            damage = self.effects_on_attacking(damage);
        }
        damage = self.apply_weakness(damage);
        damage = self.apply_resistance(damage);
        if self.format.attacking_effects() == AttackingEffectsWhen::AfterWR {
            damage = self.effects_on_attacking(damage);
        }
        damage = self.effects_on_defending(damage);

        self.with_state(self.state.add_damage_counters(self.defending(), damage/10))
    }

    pub fn damage_self(&self, damage: usize) -> Self {
        let mut engine = self.clone();

        engine = engine.push_target(self.attacking(), self.attacking());
        engine = engine.damage(damage);
        engine = engine.pop_target();

        engine
    }

    pub fn archetype(&self, card: &Card) -> &dyn CardArchetype {
        card.archetype(self.format.deref())
    }

    pub fn apply_weakness(&self, mut damage: usize) -> usize {
        // TODO: +X weaknesses instead of *X
        // TODO: Super effective glasses changing weakness to *3
        let (multiplier, types) = self.archetype(self.defending().stack[0].card()).weakness();
        for weakness in types {
            if self.archetype(self.attacking().stack[0].card()).pokemon_type().contains(&weakness) {
                damage = damage * multiplier;
            }
        }

        damage
    }

    pub fn apply_resistance(&self, mut damage: usize) -> usize {
        let (offset, types) = self.archetype(self.defending().stack[0].card()).resistance();
        for weakness in types {
            if self.archetype(self.attacking().stack[0].card()).pokemon_type().contains(&weakness) {
                damage = damage - offset;
            }
        }

        damage
    }

    pub fn effects_on_attacking(&self, mut damage: usize) -> usize {
        for card in self.state.all_cards() {
            if let Some(new_damage) = self.archetype(&card).attacking_damage_effect(&card, self, damage) {
                damage = new_damage;
            }
        }

        damage
    }

    pub fn effects_on_defending(&self, mut damage: usize) -> usize {
        if self.state.effects.iter()
            .filter(|e| e.target == EffectTarget::InPlay(self.defending().owner, self.defending().id))
            .filter(|e| e.consequence == EffectConsequence::BlockDamage)
            .count() == 0 {
                for card in self.state.all_cards() {
                    if let Some(new_damage) = self.archetype(&card).defending_damage_effect(&card, self, damage) {
                        damage = new_damage;
                    }
                }

                damage
            } else {
                0
            }
    }

    pub fn paralyze(&self) -> Self {
        self.with_state(self.state.paralyze(self.defending()))
    }

    pub fn asleep(&self) -> Self {
        self.with_state(self.state.asleep(self.defending()))
    }

    pub fn poison(&self) -> Self {
        self.with_state(self.state.poison(self.defending(), 1))
    }

    pub fn severe_poison(&self, counters: usize) -> Self {
        self.with_state(self.state.poison(self.defending(), counters))
    }

    pub fn confuse(&self) -> Self {
        self.with_state(self.state.confuse(self.defending()))
    }

    // end attack in flight

    // trainer in flight?
    pub fn trainer_card(&self) -> &Card {
        match self.resolving_actions.last() {
            Some(Action::TrainerFromHand(_player, card)) => card,
            _ => { panic!("Error accessing GameEngine::trainer_card() while not playing a trainer"); }
        }
    }

    pub fn bad(&self) -> Self {
        Self {
            good: false,
            ..self.clone()
        }
    }

    pub fn is_good(&self) -> bool {
        self.good
    }

    pub fn ensure<F>(&self, pred: F) -> Self where F: Fn(&GameEngine) -> bool {
        if pred(self) {
            self.clone()
        } else {
            self.bad()
        }
    }

    pub fn ensure_deck_not_empty(&self, player: Player) -> Self {
        if !self.good { return self.clone(); }

        if self.state.side(player).deck.is_empty() {
            self.bad()
        } else {
            self.clone()
        }
    }

    pub fn ensure_discard_contains<F>(&self, player: Player, how_many: usize, filter: F) -> Self where F: Fn(&GameEngine, &Card) -> bool {
        if self.state.side(player).discard.iter().filter(|c| filter(self, c)).count() >= how_many {
            self.clone()
        } else {
            self.bad()
        }
    }

    pub fn ensure_discard_other(&self, player: Player, how_many: usize, dm: &mut dyn DecisionMaker) -> Self {
        let discardable_cards = self.state.side(player).hand.iter().filter(|&c| c != self.trainer_card()).cloned().collect::<Vec<_>>();

        if discardable_cards.is_empty() {
            self.bad()
        } else {
            let discarded = dm.pick_from_hand(player, player, how_many, &discardable_cards);

            let mut state = self.state.clone();
            for card in discarded {
                state = state.discard_from_hand(player, card);
            }

            self.with_state(state)
        }
    }

    pub fn ensure_discard_all(&self, player: Player, _dm: &mut dyn DecisionMaker) -> Self {
        let mut engine = self.clone();
        for card in self.state.side(player).hand.iter() {
            engine.state = engine.state.discard_from_hand(player, card);
        }

        engine
    }


    pub fn ensure_shuffle_any_other(&self, player: Player, how_many: usize, dm: &mut dyn DecisionMaker) -> Self {
        let shuffleable_cards = self.state.side(player).hand.iter().filter(|&c| c != self.trainer_card()).cloned().collect::<Vec<_>>();

        if shuffleable_cards.is_empty() {
            self.bad()
        } else {
            let mut engine = self.clone();

            let cost = dm.pick_from_hand(player, player, how_many, &shuffleable_cards);

            // TODO: reveal
            for shuffled in cost {
                engine.state = engine.state.shuffle_from_hand_into_deck(player, shuffled);
            }

            engine
        }
    }

    pub fn ensure_shuffle_other<F>(&self, player: Player, how_many: usize, filter: F, dm: &mut dyn DecisionMaker) -> Self where F: Fn(&GameEngine, &Card) -> bool {
        let shuffleable_cards = self.state.side(player).hand.iter()
            .filter(|&card| card != self.trainer_card())
            .filter(|card| filter(self, card))
            .cloned().collect::<Vec<_>>();

        if shuffleable_cards.is_empty() {
            self.bad()
        } else {
            let mut engine = self.clone();

            let cost = dm.pick_from_hand(player, player, how_many, &shuffleable_cards);
            for shuffled in cost {
                engine.state = engine.state.shuffle_from_hand_into_deck(player, shuffled);
            }

            engine
        }
    }

    pub fn search_any_deck_to_hand(&self, who: Player, how_many: usize, dm: &mut dyn DecisionMaker) -> Self {
        let mut engine = self.ensure_deck_not_empty(who);

        let deck_cards = engine.state.side(who).deck.cards().into_iter()
            .collect();

        let chosen = dm.search_deck(who, who, how_many, &deck_cards);
        for searched in chosen {
            engine.state = engine.state.tutor_to_hand(who, searched);
        }

        engine.state = engine.state.shuffle_deck(who);
        engine
    }

    pub fn search_deck_to_hand<F>(&self, who: Player, how_many: usize, filter: F, dm: &mut dyn DecisionMaker) -> Self where F: Fn(&GameEngine, &Card) -> bool {
        let mut engine = self.ensure_deck_not_empty(who);

        let deck_cards = engine.state.side(who).deck.cards().into_iter()
            .filter(|card| filter(&engine, card))
            .collect();

        let chosen = dm.search_deck(who, who, how_many, &deck_cards);
        for searched in chosen {
            engine.state = engine.state.tutor_to_hand(who, searched);
        }

        engine.state = engine.state.shuffle_deck(who);
        engine
    }

    pub fn search_discard_to_hand<F>(&self, who: Player, how_many: usize, filter: F, dm: &mut dyn DecisionMaker) -> Self where F: Fn(&Card) -> bool {
        let mut engine = self.clone();

        let searchable_cards = self.state.side(who).discard.iter().filter(|&c| filter(c)).cloned().collect();
        let chosen = dm.pick_from_discard(who, who, how_many, &searchable_cards);
        for searched in chosen {
            engine.state = engine.state.discard_to_hand(who, searched);
        }

        engine
    }

    // end trainer in flight?


    pub fn goto_pokemon_checkup(&self) -> Self {
        match &self.state.stage {
            GameStage::EndOfTurn(player) => {
                self.with_state(self.state.with_stage(GameStage::PokemonCheckup(player.opponent())))
            },
            stage => { panic!("Can't end turn while in stage {:?}", stage); }
        }
    }

    pub fn end_turn(&self) -> Self {
        let mut engine = self.clone();
        let player = match engine.state.stage {
            GameStage::Turn(player) => player,
            stage => { panic!("Can't end turn while in stage {:?}", stage); }
        };

        engine.state = engine.state.with_stage(GameStage::EndOfTurn(player));

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

        for card in engine.state.all_cards() {
            if let Some(new_engine) = self.archetype(&card).on_turn_end(&card, &engine) {
                engine = new_engine;
            }
        }

        engine
    }

    pub fn with_effect(&self, effect: Effect) -> Self {
        let mut engine = self.clone();
        engine.state.effects.push(effect);
        engine
    }

    fn with_state(&self, state: GameState) -> Self {
        Self {
            state,
            ..self.clone()
        }
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

    pub fn push_target(&self, source: &InPlayCard, target: &InPlayCard) -> Self {
        let mut engine = self.clone();
        engine.attack_target_stack.push((source.id, target.id));
        engine
    }

    pub fn pop_target(&self) -> Self {
        let mut engine = self.clone();
        engine.attack_target_stack.pop();
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

        for active in self.state.side(player).active.iter() {
            if self.can_retreat(player, active) {
                actions.push(Action::Retreat(player, active.clone()));
            }
        }

        actions.push(Action::Pass);

        actions
    }

    pub fn retreat_cost(&self, in_play: &InPlayCard) -> Vec<Type> {
        let how_many = self.archetype(in_play.stack[0].card()).retreat();

        let mut cost = vec![];
        for _ in 0..how_many {
            cost.push(Type::Colorless);
        }

        cost
    }

    pub fn can_retreat(&self, player: Player, in_play: &InPlayCard) -> bool {
        if self.state.side(player).bench.is_empty() {
            return false;
        }

        let mut energy = vec![];
        for attached in in_play.attached.iter() {
            if self.is_energy(attached.card()) {
                energy.extend(self.archetype(attached.card()).provides());
            }
        }

        let cost = self.retreat_cost(in_play);

        self.is_energy_cost_met(&cost, energy)
    }

    pub fn discard_attached_energies(&self, _player: Player, in_play: &InPlayCard, _cost: &[Type], _dm: &mut dyn DecisionMaker) -> Self {
        // TODO: pick energies to discard instead of discarding everything
        let mut state = self.state.clone();
        for attached in in_play.attached.iter() {
            if self.is_energy(attached.card()) {
                state = state.move_card_to_discard(attached.card());
            }
        }

        self.with_state(state)
    }

    pub fn discard_attached_energy_cards(&self, _player: Player, in_play: &InPlayCard, _cost: &[Type], _dm: &mut dyn DecisionMaker) -> (Self, ActionResult) {
        // TODO: pick energies to discard instead of discarding everything
        let mut state = self.state.clone();
        for attached in in_play.attached.iter() {
            if self.is_energy(attached.card()) {
                state = state.move_card_to_discard(attached.card());
            }
        }

        (self.with_state(state), ActionResult::Full)
    }

    pub fn discard_all_attached_energy_cards(&self, _player: Player, in_play: &InPlayCard, _dm: &mut dyn DecisionMaker) -> Self {
        let mut state = self.state.clone();
        for attached in in_play.attached.iter() {
            if self.is_energy(attached.card()) {
                state = state.move_card_to_discard(attached.card());
            }
        }

        self.with_state(state)
    }

    pub fn discard_from_hand(&self, player: Player, card: &Card, _dm: &mut dyn DecisionMaker) -> Self {
        self.with_state(self.state.discard_from_hand(player, card))
    }

    pub fn shuffle_hand_into_deck(&self, player: Player, _dm: &mut dyn DecisionMaker) -> Self {
        self.with_state(self.state.shuffle_hand_into_deck(player))
    }

    pub fn shuffle_all_from_hand_into_deck<F>(&self, player: Player, filter: F, _dm: &mut dyn DecisionMaker) -> Self where F: Fn(&GameEngine, &Card) -> bool {
        // TODO: reveal all!
        let mut engine = self.clone();

        let selected = engine.state.side(player).hand.iter().filter(|c| filter(&engine, c)).cloned().collect::<Vec<_>>();

        for card in selected {
            engine.state = engine.state.shuffle_from_hand_into_deck(player, &card);
        }

        engine
    }

    pub fn retreat(&self, player: Player, in_play: &InPlayCard, dm: &mut dyn DecisionMaker) -> Self {
        let possible_targets = self.state.side(player).bench.clone();
        let chosen = dm.pick_in_play(player, 1, &possible_targets);

        let cost = self.retreat_cost(in_play);

        self
            .discard_attached_energies(player, in_play, &cost, dm)
            .just_switch(player, in_play, &chosen[0])
    }

    pub fn just_switch(&self, _player: Player, _this: &InPlayCard, with: &InPlayCard) -> Self {
        // TODO: clear effects
        self.with_state(self.state.switch_active_with(with))
    }

    pub fn in_play_actions(&self, player: Player, in_play: &InPlayCard, active: bool) -> Vec<Action> {
        if active && in_play.rotational_status != RotationalStatus::Paralyzed {
            self.archetype(in_play.stack[0].card()).attacks(player, in_play, self)
        } else {
            vec![]
        }
    }

    pub fn card_actions(&self, player: Player, card: &Card) -> Vec<Action> {
        if self.is_trainer(card) {
            return self.archetype(card).card_actions(player, card, self);
        } else if self.is_energy(card) {
            return self.archetype(card).card_actions(player, card, self);
        }

        if  self.can_bench_from_hand(card) && self.can_bench(player, card) {
            return vec![Action::BenchFromHand(player, card.clone())];
        }

        if self.can_evolve(card) {
            return vec![Action::EvolveFromHand(player, card.clone())];
        }

        vec![]
    }

    pub fn evolve(&self, player: Player, card: &Card, dm: &mut dyn DecisionMaker) -> Self {
        let possible_targets = self.evolution_targets(card);
        let target = dm.pick_in_play(player, 1, &possible_targets);

        // TODO: clear effects and special conditions
        self.with_state(self.state.evolve_from_hand(player, card, &target[0].id))
    }

    pub fn evolve_into(&self, in_play: &InPlayCard, card: &Card) -> Self {
        self.with_state(self.state.evolve_from_hand(card.owner, card, &in_play.id))
    }

    pub fn devolve(&self, in_play: &InPlayCard, stage: &Stage, destination_zone: &Zone) -> Self {
        let mut engine = self.clone();
        loop {
            let top_card = engine.state.in_play(&in_play.id).unwrap().stack[0].card();
            if engine.stage(top_card) == Some(Stage::Basic) {
                break;
            }
            if engine.stage(top_card) == Some(stage.clone()) {
                break;
            }

            match destination_zone {
                Zone::Discard(_) => { engine = engine.with_state(engine.state.move_card_to_discard(top_card)); },
                _ => { unimplemented!(); }
            }
        }

        // TODO: clear effects and special conditions
        engine
    }

    pub fn can_evolve(&self, card: &Card) -> bool {
        !self.evolution_targets(card).is_empty()
    }

    pub fn ready_to_evolve(&self, in_play: &InPlayCard) -> bool {
        self.state.turns[in_play.put_in_play_turn.saturating_sub(1) ..= self.state.turn.saturating_sub(1)].iter().filter(|&&t| t == in_play.owner).count() > 1
    }

    pub fn evolution_targets(&self, card: &Card) -> Vec<InPlayCard> {
        let mut targets = vec![];

        let name_to_find = match self.evolves_from(card) {
            Some(name) => name,
            _ => { return targets; },
        };

        for in_play in self.state.side(card.owner).all_in_play() {
            if self.archetype(in_play.stack[0].card()).name() == name_to_find {
                if self.ready_to_evolve(in_play) {
                    targets.push(in_play.clone());
                }
            }
        }

        targets
    }

    pub fn healable_targets(&self, player: Player) -> Vec<InPlayCard> {
        let mut targets = vec![];

        for in_play in self.state.side(player).all_in_play() {
            if self.is_healable(in_play) {
                targets.push(in_play.clone());
            }
        }

        targets
    }

    pub fn is_healable(&self, in_play: &InPlayCard) -> bool {
        in_play.damage_counters > 0
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
                energy.extend(self.archetype(attached.card()).provides());
            }
        }

        self.is_energy_cost_met(cost, energy)

    }

    pub fn is_energy_cost_met(&self, cost: &[Type], mut provided: Vec<Type>) -> bool {
        // TODO: This assumes that colorless requirements are at the end to work. Also, it's not
        // ready to work with rainbow energies or any energy that provides more than one type.
        for required in cost {
            match required {
                Type::Colorless => {
                    if provided.is_empty() { return false; }
                    provided.remove(0);
                },
                _ => {
                    match provided.iter().position(|c| c == required) {
                        Some(p) => { provided.remove(p); },
                        None => { return false; }
                    };
                },
            }
        }

        true
    }

    pub fn attachment_from_hand_targets(&self, player: Player, _card: &Card) -> Vec<InPlayCard> {
        let blocks = self.state.effects.iter()
            .filter(|e| e.consequence == EffectConsequence::BlockAttachmentFromHand)
            .filter(|e| e.target.is_player(player))
            .collect::<Vec<_>>();

        let mut targets = vec![];

        for active in self.state.side(player).active.iter() {
            targets.push(active.clone());
        }

        for benched in self.state.side(player).bench.iter() {
            targets.push(benched.clone());
        }

        for block in blocks {
            match &block.target {
                EffectTarget::Player(_) => {
                    return vec![];
                },
                EffectTarget::InPlay(_, id) => {
                    targets.retain(|x| x.id != *id);
                },
            }
        }

        targets
    }

    pub fn scoop_up<F>(&self, in_play: &InPlayCard, filter: F) -> Self where F: Fn(&GameEngine, &Card) -> bool {
        let mut engine = self.clone();
        for card in in_play.cards() {
            if filter(&engine, card) {
                engine.state = engine.state.move_card_to_hand(card);
            } else {
                engine.state = engine.state.move_card_to_discard(card);
            }
        }

        engine
    }

    pub fn gust(&self, player: Player, dm: &mut dyn DecisionMaker) -> Self {
        let target = player.opponent();
        let chosen = dm.pick_in_play(player, 1, &self.state.side(target).bench);

        self.just_switch(target, &self.state.side(target).active[0], &chosen[0])
    }

    pub fn switch(&self, player: Player, dm: &mut dyn DecisionMaker) -> Self {
        let chosen = dm.pick_in_play(player, 1, &self.state.side(player).bench);

        self.just_switch(player, &self.state.side(player).active[0], &chosen[0])
    }

    pub fn draw(&self, player: Player, how_many: usize, dm: &mut dyn DecisionMaker) -> Self {
        self.with_state(self.state.draw_n_to_hand(player, how_many, dm.shuffler()))
    }

    pub fn bench_from_hand(&self, player: Player, card: &Card) -> Self {
        self.with_state(self.state.bench_from_hand(player, card))
    }

    pub fn bench_from_discard(&self, player: Player, card: &Card) -> Self {
        self.with_state(self.state.bench_from_discard(player, card))
    }

    pub fn heal(&self, in_play: &InPlayCard, damage: usize) -> Self {
        self.with_state(self.state.remove_damage_counters(in_play, damage/10))
    }

    pub fn heal_all(&self, in_play: &InPlayCard) -> Self {
        self.with_state(self.state.remove_all_damage_counters(in_play))
    }

    pub fn put_damage_counters(&self, in_play: &InPlayCard, counters: usize) -> Self {
        self.with_state(self.state.add_damage_counters(in_play, counters))
    }

    pub fn remove_special_conditions(&self, in_play: &InPlayCard) -> Self {
        self.with_state(self.state.remove_special_conditions(in_play))
    }

    pub fn remove_attached_cards(&self, cards: &Vec<&Card>) -> Self {
        let mut engine = self.clone();

        for card in cards.iter() {
            engine = engine.with_state(engine.state.move_card_to_discard(card));
        }

        engine
    }

    pub fn bench(&self, player: Player) -> Vec<InPlayCard> {
        self.state.side(player).bench.clone()
    }

    pub fn can_bench(&self, player: Player, _card: &Card) -> bool {
        self.has_bench_space(player)
    }

    pub fn has_bench_space(&self, player: Player) -> bool {
        self.state.side(player).bench.len() < self.bench_size(player)
    }

    pub fn bench_size(&self, _player: Player) -> usize {
        5
    }

    pub fn can_discard_other(&self, player: Player, _card: &Card, n: usize) -> bool {
        self.state.side(player).hand.len() - 1 >= n
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

        engine.state = engine.state
            .with_stage(GameStage::StartOfTurn(Player::One))
            .next_turn(Player::One);

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

    pub fn is_pokemon(&self, card: &Card) -> bool {
        self.archetype(card).is_pokemon(card, self)
    }

    pub fn has_special_condition(&self, in_play: &InPlayCard) -> bool {
        let in_play = self.state.in_play(&in_play.id).unwrap();

        in_play.rotational_status == RotationalStatus::None || in_play.poisoned.is_some() || in_play.burned
    }

    pub fn has_energy_cards_attached(&self, in_play: &InPlayCard) -> bool {
        let in_play = self.state.in_play(&in_play.id).unwrap();

        in_play.attached.iter().any(|c| self.is_energy(c.card()))
    }

    pub fn is_trainer(&self, card: &Card) -> bool {
        self.archetype(card).is_trainer(card, self)
    }

    pub fn full_hp(&self, in_play: &InPlayCard) -> usize {
        self.archetype(in_play.stack[0].card()).hp(in_play.stack[0].card(), self).unwrap_or(0)
    }

    pub fn remaining_hp(&self, in_play: &InPlayCard) -> usize {
        self.full_hp(in_play).saturating_sub(in_play.damage_counters * 10)
    }

    pub fn damage_counters_on(&self, in_play: &InPlayCard) -> usize {
        in_play.damage_counters
    }

    pub fn is_energy(&self, card: &Card) -> bool {
        // TODO: Electrode?
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
        self.archetype(card).stage()
    }

    pub fn evolves_from(&self, card: &Card) -> Option<String> {
        self.archetype(card).evolves_from()
    }

    pub fn zone(&self, card: &Card) -> Zone {
        self.state.zone(card)
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

    pub fn in_play_card(&self, card: &Card) -> Option<InPlayCard> {
        for in_play in self.state.side(card.owner).active.iter() {
            if in_play.stack.iter().any(|c| c.card() == card) || in_play.attached.iter().any(|c| c.card() == card) {
                return Some(in_play.clone());
            }
        }

        for in_play in self.state.side(card.owner).bench.iter() {
            if in_play.stack.iter().any(|c| c.card() == card) || in_play.attached.iter().any(|c| c.card() == card) {
                return Some(in_play.clone());
            }
        }

        None
    }

    pub fn rearrange_topdeck(&self, who: Player, whose: Player, how_many: usize, dm: &mut dyn DecisionMaker) -> Self {
        let mut engine = self.clone();

        for _ in 0..how_many {
            engine.state = engine.state.draw_to_working_area(whose, dm.shuffler());
        }

        println!("working area before: {:?}", engine.state.side(whose).working_area);

        let rearranged = dm.rearrange(who, &engine.state.side(whose).working_area);
        engine.state = engine.state.rearrange_working_area(whose, &rearranged);

        println!("working area after: {:?}", engine.state.side(whose).working_area);

        engine.state = engine.state.put_working_area_on_top_of_deck(whose);
        engine
    }

    pub fn is_end_of_opponents_next_turn(&self, started_on: usize) -> bool {
        match self.state.stage {
            GameStage::EndOfTurn(player) => {
                self.state.turns[started_on - 1] == player.opponent()
            },
            _ => false,
        }
    }

    pub fn turn_attached(&self, card: &Card) -> Option<usize> {
        self.state.attached_card(card).map(|c| c.attached_turn)
    }

    pub fn in_play(&self, player: Player) -> Vec<&InPlayCard> {
        self.state.side(player).all_in_play()
    }

    pub fn basic_for_stage2(&self, card: &Card) -> String {
        self.format.basic_for_stage2(card)
    }

    pub fn attack_builder<'a>(&self, dm: &'a mut dyn DecisionMaker) -> AttackBuilder<'a> {
        AttackBuilder::new(self.clone(), dm)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ActionResult {
    Nothing,
    Partial,
    Full,
}

pub struct AttackBuilder<'a> {
    engine: GameEngine,
    dm: &'a mut dyn DecisionMaker,
    flips: Vec<Flips>,

    failed: bool,
    results: Vec<ActionResult>,
    original: GameEngine,
}

impl<'a> AttackBuilder<'a> {
    pub fn new(engine: GameEngine, dm: &'a mut dyn DecisionMaker) -> Self {
        let original = engine.clone();
        AttackBuilder {
            engine,
            dm,
            flips: vec![],

            results: vec![],
            failed: false,
            original,
        }
    }

    pub fn failed(&self) -> bool {
        self.failed
    }

    pub fn player(&self) -> Player {
        self.engine.player()
    }

    pub fn opponent(&self) -> Player {
        self.engine.opponent()
    }

    pub fn attacking(&self) -> &InPlayCard {
        self.engine.attacking()
    }

    pub fn defending(&self) -> &InPlayCard {
        self.engine.defending()
    }

    pub fn attack_cost(mut self, energy_requirements: &[Type]) -> Self {
        if !self.engine.is_attack_energy_cost_met(self.engine.attacking(), energy_requirements) {
            self.failed = true;
        }
        self
    }

    pub fn flip_a_coin(mut self) -> Self {
        self.flips.push(self.dm.flip(1));
        self
    }

    pub fn flip_coins(mut self, how_many: usize) -> Self {
        self.flips.push(self.dm.flip(how_many));
        self
    }

    pub fn heads(&self) -> usize {
        self.flips.last().unwrap().heads()
    }

    pub fn damage(mut self, damage: usize) -> Self {
        self.engine = self.engine.damage(damage);
        self
    }

    pub fn damage_self(mut self, damage: usize) -> Self {
        self.engine = self.engine.damage_self(damage);
        self
    }

    pub fn if_heads<F>(self, f: F) -> Self where F: Fn(Self) -> Self {
        if self.heads() == 1 {
            f(self)
        } else {
            self
        }
    }

    pub fn if_tails<F>(self, f: F) -> Self where F: Fn(Self) -> Self {
        if self.heads() == 0 {
            f(self)
        } else {
            self
        }
    }

    pub fn then<F>(self, f: F) -> Self where F: Fn(Self) -> Self {
        f(self)
    }

    pub fn must<F>(mut self, f: F) -> Self where F: Fn(Self) -> Self {
        let idx = self.results.len();
        self = f(self);
        if !self.results[idx..].iter().all(|x| match x { ActionResult::Full => true, _ => false }) {
            self.failed = true;
        }
        self
    }

    pub fn each_own_bench<F>(mut self, f: F) -> Self where F: Fn(Self) -> Self {
        for target in self.engine.bench(self.player()) {
            self.engine = self.engine.push_target(self.attacking(), &target);
            self = f(self);
            self.engine = self.engine.pop_target();
        }
        self
    }

    pub fn each_opponents_bench<F>(mut self, f: F) -> Self where F: Fn(Self) -> Self {
        for target in self.engine.bench(self.opponent()) {
            self.engine = self.engine.push_target(self.attacking(), &target);
            self = f(self);
            self.engine = self.engine.pop_target();
        }
        self
    }

    pub fn with_effect(mut self, effect: Effect) -> Self {
        self.engine = self.engine.with_effect(effect);
        self
    }

    pub fn asleep(mut self) -> Self {
        self.engine = self.engine.asleep();
        self
    }

    pub fn confuse(mut self) -> Self {
        self.engine = self.engine.confuse();
        self
    }

    pub fn paralyze(mut self) -> Self {
        self.engine = self.engine.paralyze();
        self
    }

    pub fn poison(mut self) -> Self {
        self.engine = self.engine.poison();
        self
    }

    pub fn severe_poison(mut self, counters: usize) -> Self {
        self.engine = self.engine.severe_poison(counters);
        self
    }

    pub fn discard_defending_energy_cards(mut self, energy_requirements: &[Type]) -> Self {
        let (engine, result) = self.engine.discard_attached_energy_cards(self.player(), &self.engine.defending(), energy_requirements, self.dm);

        self.engine = engine;
        self.results.push(result);
        self
    }

    pub fn discard_all_attacking_energy_cards(mut self) -> Self {
        self.engine = self.engine.discard_all_attached_energy_cards(self.player(), &self.engine.attacking(), self.dm);
        self
    }

    pub fn discard_attacking_energy_cards(mut self, energy_requirements: &[Type]) -> Self {
        let (engine, result) = self.engine.discard_attached_energy_cards(self.player(), &self.engine.attacking(), energy_requirements, self.dm);

        self.engine = engine;
        self.results.push(result);
        self
    }

    pub fn heal_all_attacking(mut self) -> Self {
        self.engine = self.engine.heal_all(self.attacking());
        self
    }

    pub fn damage_per_heads(mut self, damage: usize) -> Self {
        self.engine = self.engine.damage(damage * self.heads());
        self
    }

    pub fn damage_plus_per_energy_card_on_defending(mut self, base_damage: usize, plus: usize) -> Self {
        self.engine = self.engine.damage(base_damage.saturating_add(self.defending().attached.iter().filter(|c| self.engine.is_energy(c.card())).count() * plus));
        self
    }

    pub fn damage_plus_per_damage_counter_on_defending(mut self, base_damage: usize, plus: usize) -> Self {
        self.engine = self.engine.damage(base_damage.saturating_add(self.engine.damage_counters_on(self.defending()) * plus));
        self
    }

    pub fn damage_per_damage_counter_on_itself(mut self, damage_per_counter: usize) -> Self {
        self.engine = self.engine.damage(self.engine.damage_counters_on(self.attacking()) * damage_per_counter);
        self
    }
    pub fn damage_minus_per_damage_counter_on_itself(mut self, base_damage: usize, minus: usize) -> Self {
        self.engine = self.engine.damage(base_damage.saturating_sub(self.engine.damage_counters_on(self.attacking()) * minus));
        self
    }

    pub fn damage_half_defending_remaining_hp(mut self) -> Self {
        self.engine = self.engine.damage(self.engine.remaining_hp(self.defending()).div_ceil(2));
        self
    }

    pub fn prevent_damage_during_opponents_next_turn(mut self) -> Self {
        self.engine = self.engine.with_effect(Effect {
            name: "NO_DAMAGE_DURING_OPPONENTS_NEXT_TURN".into(),
            source: EffectSource::Attack(self.player(), self.attacking().id),
            target: EffectTarget::InPlay(self.player(), self.attacking().id),
            consequence: EffectConsequence::BlockDamage,
            expires: EffectExpiration::EndOfTurn(self.opponent(), 0),
            system: false,
        });
        self
    }

    pub fn prevent_trainers_during_opponents_next_turn(mut self) -> Self {
        self.engine = self.engine.with_effect(Effect {
            name: "PSYDUCK_FO_HEADACHE_NO_TRAINERS".into(),
            source: EffectSource::Attack(self.player(), self.attacking().id),
            target: EffectTarget::Player(self.opponent()),
            consequence: EffectConsequence::BlockTrainerFromHand,
            expires: EffectExpiration::EndOfTurn(self.opponent(), 0),
            system: false,
        });
        self
    }
}

impl<'a> From<&AttackBuilder<'a>> for GameEngine {
    fn from(ab: &AttackBuilder<'a>) -> Self {
        if ab.failed {
            ab.original.clone()
        } else {
            ab.engine.clone()
        }
    }
}
