use crate::state::*;
use crate::engine::*;

impl CardDB for Card {
    fn archetype(&self) -> Box<dyn CardArchetype> {
        match self.archetype.as_str() {
            "Alakazam (BS 1)"                   => Pokemon::create::<Alakazam>(),
            "Growlithe (BS 28)"                 => Pokemon::create::<Growlithe>(),
            "Squirtle (BS 63)"                  => Pokemon::create::<Squirtle>(),
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
            "Psyduck (FO 53)"                   => Pokemon::create::<Psyduck>(),
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

struct Pokemon {}
impl Pokemon {
    pub fn create<T: Default + CardArchetype + 'static>() -> Box<dyn CardArchetype> {
        Box::new(T::default())
    }
}

#[derive(Default)]
struct Alakazam {}
impl CardArchetype for Alakazam {
    fn stage(&self) -> Option<Stage> {
        Some(Stage::Stage2)
    }
    fn card_actions(&self, _player: Player, _card: &Card, _engine: &GameEngine) -> Vec<Action> {
        vec![]
    }
    fn execute(&self, _player: Player, _card: &Card, engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine.clone()
    }
    fn attacks(&self, _player: Player, _in_play: &InPlayCard, _engine: &GameEngine) -> Vec<Action> {
        vec![]
    }
    fn provides(&self) -> Vec<Type> {
        vec![]
    }
    fn hp(&self) -> Option<usize> {
        Some(80)
    }
}

#[derive(Default)]
struct Growlithe {}
impl CardArchetype for Growlithe {
    fn stage(&self) -> Option<Stage> {
        Some(Stage::Basic)
    }
    fn card_actions(&self, _player: Player, _card: &Card, _engine: &GameEngine) -> Vec<Action> {
        vec![]
    }
    fn execute(&self, _player: Player, _card: &Card, engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine.clone()
    }
    fn attacks(&self, player: Player, in_play: &InPlayCard, engine: &GameEngine) -> Vec<Action> {
        let mut attacks = vec![];

        if engine.is_attack_energy_cost_met(in_play, &[Type::Fire, Type::Colorless]) {
            attacks.push(Action::Attack(player, in_play.clone(), "Flare".into(), Box::new(RFA::new(Self::flare))));
        }

        attacks
    }
    fn provides(&self) -> Vec<Type> {
        vec![]
    }

    fn hp(&self) -> Option<usize> {
        Some(60)
    }
}
impl Growlithe {
    pub fn flare(engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine.damage(20)
    }
}

#[derive(Default)]
struct Psyduck {}
impl CardArchetype for Psyduck {
    fn stage(&self) -> Option<Stage> {
        Some(Stage::Basic)
    }
    fn card_actions(&self, _player: Player, _card: &Card, _engine: &GameEngine) -> Vec<Action> {
        vec![]
    }
    fn execute(&self, _player: Player, _card: &Card, engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine.clone()
    }
    fn attacks(&self, player: Player, in_play: &InPlayCard, engine: &GameEngine) -> Vec<Action> {
        let mut attacks = vec![];

        if engine.is_attack_energy_cost_met(in_play, &[Type::Psychic]) {
            attacks.push(Action::Attack(player, in_play.clone(), "Headache".into(), Box::new(RFA::new(Self::headache))));
        }
        if engine.is_attack_energy_cost_met(in_play, &[Type::Water]) {
            attacks.push(Action::Attack(player, in_play.clone(), "Fury Swipes".into(), Box::new(RFA::new(Self::fury_swipes))));
        }

        attacks
    }
    fn provides(&self) -> Vec<Type> {
        vec![]
    }

    fn hp(&self) -> Option<usize> {
        Some(50)
    }
}
impl Psyduck {
    pub fn headache(engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine.with_effect(Effect {
            name: "PSYDUCK_FO_HEADACHE_NO_TRAINERS".into(),
            source: EffectSource::Attack(engine.player(), engine.attacking().id),
            target: EffectTarget::Player(engine.opponent()),
            consequence: EffectConsequence::BlockTrainerFromHand,
            expires: EffectExpiration::EndOfTurn(engine.opponent(), 0),
        })
    }
    pub fn fury_swipes(engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameEngine {
        let damage = 10 * dm.flip(3).heads();

        engine.damage(damage)
    }
}

#[derive(Default)]
struct Squirtle {}
impl CardArchetype for Squirtle {
    fn stage(&self) -> Option<Stage> {
        Some(Stage::Basic)
    }
    fn card_actions(&self, _player: Player, _card: &Card, _engine: &GameEngine) -> Vec<Action> {
        vec![]
    }
    fn execute(&self, _player: Player, _card: &Card, engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine.clone()
    }
    fn attacks(&self, player: Player, in_play: &InPlayCard, engine: &GameEngine) -> Vec<Action> {
        let mut attacks = vec![];

        if engine.is_attack_energy_cost_met(in_play, &[Type::Water]) {
            attacks.push(Action::Attack(player, in_play.clone(), "Bubble".into(), Box::new(RFA::new(Self::bubble))));
        }
        if engine.is_attack_energy_cost_met(in_play, &[Type::Psychic]) { // fix: it's actually water
            attacks.push(Action::Attack(player, in_play.clone(), "Withdraw".into(), Box::new(RFA::new(Self::withdraw))));
        }

        attacks
    }
    fn provides(&self) -> Vec<Type> {
        vec![]
    }

    fn hp(&self) -> Option<usize> {
        Some(40)
    }
}
impl Squirtle {
    pub fn bubble(engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameEngine {
        let paralyzed = dm.flip(1).heads() == 1;

        engine.damage(10).then_if(paralyzed, GameEngine::paralyze)
    }
    pub fn withdraw(engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine.clone()
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
            vec![Action::AttachFromHand(player, card.clone())]
        } else {
            vec![]
        }
    }

    fn execute(&self, player: Player, card: &Card, engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameEngine {
        let targets = engine.attachment_from_hand_targets(player, card);
        let target = dm.pick_target(player, &targets);

        engine
            .attach_from_hand(player, card, target)
            .with_effect(Effect {
                source: EffectSource::Energy(player, card.clone()),
                target: EffectTarget::Player(player),
                expires: EffectExpiration::EndOfTurn(player, 0),
                consequence: EffectConsequence::BlockAttachmentFromHand,
                name: "ENERGY_ATTACH_FOR_TURN".into(),
            })
    }

    fn attacks(&self, _player: Player, _in_play: &InPlayCard, _engine: &GameEngine) -> Vec<Action> {
        vec![]
    }

    fn provides(&self) -> Vec<Type> {
        vec![self.energy_type.clone()]
    }

    fn hp(&self) -> Option<usize> {
        None
    }
}

trait TrainerCardArchetype {
    fn requirements_ok(&self, player: Player, card: &Card, engine: &GameEngine) -> bool;
    fn execute(&self, player: Player, card: &Card, engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameEngine;
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
        if self.archetype.requirements_ok(player, card, engine) && engine.can_play_trainer_from_hand(card) {
            vec![Action::TrainerFromHand(player, card.clone())]
        } else {
            vec![]
        }
    }

    fn execute(&self, player: Player, card: &Card, engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameEngine {
        engine.with_state(self.archetype.execute(player, card, engine, dm).state.discard_from_hand(player, card))
    }

    fn stage(&self) -> Option<Stage> {
        None
    }

    fn attacks(&self, _player: Player, _in_play: &InPlayCard, _engine: &GameEngine) -> Vec<Action> {
        vec![]
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }

    fn hp(&self) -> Option<usize> {
        None
    }
}

#[derive(Default)]
struct ClefairyDoll {}
impl TrainerCardArchetype for ClefairyDoll {
    fn requirements_ok(&self, player: Player, card: &Card, engine: &GameEngine) -> bool {
        engine.can_bench(player, card)
    }

    fn execute(&self, _player: Player, _card: &Card, _engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
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
    fn execute(&self, player: Player, _card: &Card, engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameEngine {
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

        engine.with_state(state.shuffle_deck(player))
    }
}

#[derive(Default)]
struct ImpostorProfessorOak {}
impl TrainerCardArchetype for ImpostorProfessorOak {
    // effect: shuffle(hand, to: deck); draw(7)
    fn requirements_ok(&self, player: Player, _card: &Card, engine: &GameEngine) -> bool {
        !engine.state.side(player.opponent()).deck.is_empty() || engine.state.side(player.opponent()).hand.len() > 7
    }
    fn execute(&self, player: Player, _card: &Card, engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameEngine {
        let opponent = player.opponent();

        engine.with_state(engine.state
            .shuffle_hand_into_deck(opponent)
            .draw_n_to_hand(opponent, 7, dm.shuffler()))
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
    fn execute(&self, player: Player, _card: &Card, engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameEngine {
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

        engine.with_state(state)
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
    fn execute(&self, _player: Player, _card: &Card, _engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        panic!("not implemented");
    }
}

#[derive(Default)]
struct PokemonBreeder {}
impl TrainerCardArchetype for PokemonBreeder {
    fn requirements_ok(&self, _player: Player, _card: &Card, _engine: &GameEngine) -> bool {
        true // todo: bunch of checks
    }
    fn execute(&self, _player: Player, _card: &Card, _engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        panic!("not implemented");
    }
}

#[derive(Default)]
struct PokemonTrader {}
impl TrainerCardArchetype for PokemonTrader {
    fn requirements_ok(&self, _player: Player, _card: &Card, _engine: &GameEngine) -> bool {
        true // TODO: pokemon communication
    }
    fn execute(&self, _player: Player, _card: &Card, _engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        panic!("not implemented");
    }
}

#[derive(Default)]
struct ScoopUp {}
impl TrainerCardArchetype for ScoopUp {
    fn requirements_ok(&self, player: Player, _card: &Card, engine: &GameEngine) -> bool {
        engine.state.side(player).in_play().iter().any(|p| p.stack.iter().any(|card| engine.stage(card.card()) == Some(Stage::Basic)))
    }
    fn execute(&self, player: Player, _card: &Card, engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameEngine {
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

        engine.with_state(state)
    }
}

#[derive(Default)]
struct Defender {}
impl TrainerCardArchetype for Defender {
    fn requirements_ok(&self, _player: Player, _card: &Card, _engine: &GameEngine) -> bool {
        true
    }
    fn execute(&self, _player: Player, _card: &Card, _engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        panic!("not implemented");
    }
}

#[derive(Default)]
struct EnergyRetrieval {}
impl TrainerCardArchetype for EnergyRetrieval {
    fn requirements_ok(&self, player: Player, card: &Card, engine: &GameEngine) -> bool {
        engine.can_discard_other(player, card, 1) && engine.discard_pile_has_basic_energy(player, card)
    }
    fn execute(&self, player: Player, _card: &Card, engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameEngine {
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

        engine.with_state(state)
    }
}

#[derive(Default)]
struct Maintenance {}
impl TrainerCardArchetype for Maintenance {
    fn requirements_ok(&self, player: Player, card: &Card, engine: &GameEngine) -> bool {
        engine.can_discard_other(player, card, 2) // TODO: not discard but shuffle
    }
    fn execute(&self, player: Player, _card: &Card, engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameEngine {
        let mut state = engine.state.clone();

        let cost = dm.pick_from_hand(player, player, 2, &engine.state.side(player).hand); // TODO: can't pick used card
        for shuffled in cost {
            state = state.shuffle_from_hand_into_deck(player, shuffled);
        }

        engine.with_state(state.draw_n_to_hand(player, 1, dm.shuffler()))
    }
}

#[derive(Default)]
struct PlusPower {}
impl TrainerCardArchetype for PlusPower {
    fn requirements_ok(&self, _player: Player, _card: &Card, _engine: &GameEngine) -> bool {
        true
    }
    fn execute(&self, _player: Player, _card: &Card, _engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        panic!("not implemented");
    }
}

#[derive(Default)]
struct Pokedex {}
impl TrainerCardArchetype for Pokedex {
    fn requirements_ok(&self, player: Player, _card: &Card, engine: &GameEngine) -> bool {
        !engine.state.side(player).deck.is_empty()
    }
    fn execute(&self, _player: Player, _card: &Card, _engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        panic!("not implemented");
    }
}

#[derive(Default)]
struct ProfessorOak {}
impl TrainerCardArchetype for ProfessorOak {
    fn requirements_ok(&self, player: Player, _card: &Card, engine: &GameEngine) -> bool {
        !engine.state.side(player).deck.is_empty()
    }
    fn execute(&self, player: Player, _card: &Card, engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameEngine {
        let mut state = engine.state.clone();

        for discarded in engine.state.side(player).hand.iter() {
            state = state.discard_from_hand(player, discarded);
        }

        engine.with_state(state.draw_n_to_hand(player, 7, dm.shuffler()))
    }
}

#[derive(Default)]
struct Bill {}
impl TrainerCardArchetype for Bill {
    fn requirements_ok(&self, player: Player, _card: &Card, engine: &GameEngine) -> bool {
        !engine.state.side(player).deck.is_empty()
    }
    fn execute(&self, player: Player, _card: &Card, engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameEngine {
        engine.with_state(engine.state.draw_n_to_hand(player, 2, dm.shuffler()))
    }
}

#[derive(Default)]
struct GustOfWind {}
impl TrainerCardArchetype for GustOfWind {
    fn requirements_ok(&self, player: Player, _card: &Card, engine: &GameEngine) -> bool {
        !engine.state.side(player.opponent()).bench.is_empty()
    }
    fn execute(&self, player: Player, _card: &Card, engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameEngine {
        let chosen = dm.pick_in_play(player, 1, &engine.state.side(player.opponent()).bench);

        // TODO: clear effects on defending
        engine.with_state(engine.state.switch_active_with(&chosen[0]))
    }
}

#[derive(Default)]
struct Switch {}
impl TrainerCardArchetype for Switch {
    fn requirements_ok(&self, player: Player, _card: &Card, engine: &GameEngine) -> bool {
        !engine.state.side(player).bench.is_empty()
    }
    fn execute(&self, player: Player, _card: &Card, engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameEngine {
        let chosen = dm.pick_in_play(player, 1, &engine.state.side(player).bench);

        // TODO: clear effects on defending
        engine.with_state(engine.state.switch_active_with(&chosen[0]))
    }
}

#[derive(Default)]
struct NOOP {}
impl CardArchetype for NOOP {
    fn card_actions(&self, _player: Player, _card: &Card, _engine: &GameEngine) -> Vec<Action> {
        vec![]
    }
    fn execute(&self, _player: Player, _card: &Card, _engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        panic!("not implemented");
    }
    fn stage(&self) -> Option<Stage> {
        None
    }
    fn attacks(&self, _player: Player, _in_play: &InPlayCard, _engine: &GameEngine) -> Vec<Action> {
        vec![]
    }
    fn provides(&self) -> Vec<Type> {
        vec![]
    }
    fn hp(&self) -> Option<usize> {
        None
    }
}
