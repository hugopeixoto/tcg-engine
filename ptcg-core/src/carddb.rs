use crate::state::*;
use crate::engine::*;
use crate::*;
use crate::sets::*;

impl CardDB for Card {
    fn archetype(&self) -> Box<dyn CardArchetype> {
        match self.archetype.as_str() {
            "Alakazam (BS 1)"                   => Pokemon::create::<base_set::Alakazam>(),
            "Blastoise (BS 2)"                  => Pokemon::create::<base_set::Blastoise>(),
            "Chansey (BS 3)"                    => Pokemon::create::<base_set::Chansey>(),
            "Charizard (BS 4)"                  => Pokemon::create::<base_set::Charizard>(),
            "Clefairy (BS 5)"                   => Pokemon::create::<base_set::Clefairy>(),
            "Gyarados (BS 6)"                   => Pokemon::create::<base_set::Gyarados>(),
            "Hitmonchan (BS 7)"                 => Pokemon::create::<base_set::Hitmonchan>(),
            "Machamp (BS 8)"                    => Pokemon::create::<base_set::Machamp>(),
            "Magneton (BS 9)"                   => Pokemon::create::<base_set::Magneton>(),
            "Mewtwo (BS 10)"                    => Pokemon::create::<base_set::Mewtwo>(),
            "Nidoking (BS 11)"                  => Pokemon::create::<base_set::Mewtwo>(),
            "Ninetales (BS 12)"                 => Pokemon::create::<base_set::Ninetales>(),
            "Poliwrath (BS 13)"                 => Pokemon::create::<base_set::Poliwrath>(),
            "Raichu (BS 14)"                    => Pokemon::create::<base_set::Raichu>(),
            "Venusaur (BS 15)"                  => Pokemon::create::<base_set::Venusaur>(),
            "Zapdos (BS 16)"                    => Pokemon::create::<base_set::Zapdos>(),
            "Beedrill (BS 17)"                  => Pokemon::create::<base_set::Beedrill>(),
            "Dragonair (BS 18)"                 => Pokemon::create::<base_set::Dragonair>(),
            "Dugtrio (BS 19)"                   => Pokemon::create::<base_set::Dugtrio>(),
            "Electabuzz (BS 20)"                => Pokemon::create::<base_set::Electabuzz>(),
            "Electrode (BS 21)"                 => Pokemon::create::<base_set::Electrode>(),
            "Pidgeotto (BS 22)"                 => Pokemon::create::<base_set::Pidgeotto>(),
            "Arcanine (BS 23)"                  => Pokemon::create::<base_set::Arcanine>(),
            "Charmeleon (BS 24)"                => Pokemon::create::<base_set::Charmeleon>(),
            "Dewgong (BS 25)"                   => Pokemon::create::<base_set::Dewgong>(),
            "Dratini (BS 26)"                   => Pokemon::create::<base_set::Dratini>(),
            "Farfetch'd (BS 27)"                => Pokemon::create::<base_set::FarfetchD>(),
            "Growlithe (BS 28)"                 => Pokemon::create::<base_set::Growlithe>(),
            "Haunter (BS 29)"                   => Pokemon::create::<base_set::Haunter>(),
            "Ivysaur (BS 30)"                   => Pokemon::create::<base_set::Ivysaur>(),
            "Jynx (BS 31)"                      => Pokemon::create::<base_set::Jynx>(),
            "Kadabra (BS 32)"                   => Pokemon::create::<base_set::Kadabra>(),
            "Kakuna (BS 33)"                    => Pokemon::create::<base_set::Kakuna>(),
            "Machoke (BS 34)"                   => Pokemon::create::<base_set::Machoke>(),
            "Magikarp (BS 35)"                  => Pokemon::create::<base_set::Magikarp>(),
            "Magmar (BS 36)"                    => Pokemon::create::<base_set::Magmar>(),
            "Nidorino (BS 37)"                  => Pokemon::create::<base_set::Nidorino>(),
            "Poliwhirl (BS 38)"                 => Pokemon::create::<base_set::Poliwhirl>(),
            "Porygon (BS 39)"                   => Pokemon::create::<base_set::Porygon>(),
            "Raticate (BS 40)"                  => Pokemon::create::<base_set::Raticate>(),
            "Seel (BS 41)"                      => Pokemon::create::<base_set::Seel>(),
            "Wartortle (BS 42)"                 => Pokemon::create::<base_set::Wartortle>(),
            "Abra (BS 43)"                      => Pokemon::create::<base_set::Abra>(),
            "Bulbasaur (BS 44)"                 => Pokemon::create::<base_set::Bulbasaur>(),
            "Caterpie (BS 45)"                  => Pokemon::create::<base_set::Caterpie>(),
            "Charmander (BS 46)"                => Pokemon::create::<base_set::Charmander>(),
            "Diglett (BS 47)"                   => Pokemon::create::<base_set::Diglett>(),
            "Doduo (BS 48)"                     => Pokemon::create::<base_set::Doduo>(),
            "Drowzee (BS 49)"                   => Pokemon::create::<base_set::Drowzee>(),
            "Gastly (BS 50)"                    => Pokemon::create::<base_set::Gastly>(),
            "Koffing (BS 51)"                   => Pokemon::create::<base_set::Koffing>(),
            "Machop (BS 52)"                    => Pokemon::create::<base_set::Machop>(),
            "Magnemite (BS 53)"                 => Pokemon::create::<base_set::Magnemite>(),
            "Metapod (BS 54)"                   => Pokemon::create::<base_set::Metapod>(),
            "Nidoran ♂ (BS 55)"                 => Pokemon::create::<base_set::NidoranM>(),
            "Onix (BS 56)"                      => Pokemon::create::<base_set::Onix>(),
            "Pidgey (BS 57)"                    => Pokemon::create::<base_set::Pidgey>(),
            "Pikachu (BS 58)"                   => Pokemon::create::<base_set::Pikachu>(),
            "Poliwag (BS 59)"                   => Pokemon::create::<base_set::Poliwag>(),
            "Ponyta (BS 60)"                    => Pokemon::create::<base_set::Ponyta>(),
            "Rattata (BS 61)"                   => Pokemon::create::<base_set::Rattata>(),
            "Sandshrew (BS 62)"                 => Pokemon::create::<base_set::Sandshrew>(),
            "Squirtle (BS 63)"                  => Pokemon::create::<base_set::Squirtle>(),
            "Starmie (BS 64)"                   => Pokemon::create::<base_set::Starmie>(),
            "Staryu (BS 65)"                    => Pokemon::create::<base_set::Staryu>(),
            "Tangela (BS 66)"                   => Pokemon::create::<base_set::Tangela>(),
            "Voltorb (BS 67)"                   => Pokemon::create::<base_set::Voltorb>(),
            "Vulpix (BS 68)"                    => Pokemon::create::<base_set::Vulpix>(),
            "Weedle (BS 69)"                    => Pokemon::create::<base_set::Weedle>(),
            "Clefairy Doll (BS 70)"             => Trainer::create::<ClefairyDoll>(),
            "Computer Search (BS 71)"           => Trainer::create::<ComputerSearch>(),
            "Devolution Spray (BS 72)"          => Trainer::create::<DevolutionSpray>(),
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
            "Double Colorless Energy (BS 96)"   => Box::new(DoubleColorlessEnergy::default()),
            "Fighting Energy (BS 97)"           => BasicEnergy::create("Fighting Energy", Type::Fighting),
            "Fire Energy (BS 98)"               => BasicEnergy::create("Fire Energy", Type::Fire),
            "Grass Energy (BS 99)"              => BasicEnergy::create("Grass Energy", Type::Grass),
            "Lightning Energy (BS 100)"         => BasicEnergy::create("Lightning Energy", Type::Lightning),
            "Psychic Energy (BS 101)"           => BasicEnergy::create("Psychic Energy", Type::Psychic),
            "Water Energy (BS 102)"             => BasicEnergy::create("Water Energy", Type::Water),
            "Articuno (FO 2)"                   => Pokemon::create::<fossil::Articuno>(),
            "Articuno (FO 17)"                  => Pokemon::create::<fossil::Articuno>(),
            "Psyduck (FO 53)"                   => Pokemon::create::<fossil::Psyduck>(),
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
struct DoubleColorlessEnergy {}
impl CardArchetype for DoubleColorlessEnergy {
    card_name!("Double Colorless Energy");
    not_a_pokemon!();

    fn card_actions(&self, player: Player, card: &Card, engine: &GameEngine) -> Vec<Action> {
        if !engine.attachment_from_hand_targets(player, card).is_empty() {
            vec![Action::AttachFromHand(player, card.clone())]
        } else {
            vec![]
        }
    }
    fn execute(&self, player: Player, card: &Card, engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameEngine {
        let targets = engine.attachment_from_hand_targets(player, card);
        let target = dm.pick_in_play(player, 1, &targets)[0];

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
        vec![Type::Colorless, Type::Colorless]
    }
}

struct BasicEnergy {
    name: String,
    energy_type: Type,
}
impl BasicEnergy {
    pub fn create(name: &str, energy_type: Type) -> Box<dyn CardArchetype> {
        Box::new(BasicEnergy { name: name.into(), energy_type })
    }
}
impl CardArchetype for BasicEnergy {
    not_a_pokemon!();
    fn name(&self) -> String {
        self.name.clone()
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
        let target = dm.pick_in_play(player, 1, &targets)[0];

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

}

trait TrainerCardArchetype {
    fn requirements_ok(&self, player: Player, card: &Card, engine: &GameEngine) -> bool;
    fn execute(&self, player: Player, card: &Card, engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameEngine;
    fn name(&self) -> String;
    fn cost(&self, engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameEngine {
        engine.clone()
    }
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
    not_a_pokemon!();
    fn name(&self) -> String {
        self.archetype.name()
    }

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

    fn attacks(&self, _player: Player, _in_play: &InPlayCard, _engine: &GameEngine) -> Vec<Action> {
        vec![]
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}

#[derive(Default)]
struct ClefairyDoll {}
impl TrainerCardArchetype for ClefairyDoll {
    fn name(&self) -> String { "Clefairy Doll".into() }
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
    card_name!("Computer Search");

    fn requirements_ok(&self, player: Player, card: &Card, engine: &GameEngine) -> bool {
        engine
            .push_action(Action::TrainerFromHand(player, card.clone()))
            .then(|e| self.cost(e, &mut FakeDM::default()))
            .pop_action()
            .is_good()
    }

    // cost: discard(2, from: hand)
    // effect: search(1, from: deck); move(it, to: hand)
    fn cost(&self, engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameEngine {
        engine
            .ensure_deck_not_empty(engine.player())
            .ensure_discard_other(engine.player(), 2, dm)
    }

    fn execute(&self, player: Player, _card: &Card, engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameEngine {
        self
            .cost(engine, dm)
            .search_deck_to_hand(player, 1, |_c| true, dm)
    }
}

#[derive(Default)]
//#[card_named("Devolution Spray")]
struct DevolutionSpray {}
impl TrainerCardArchetype for DevolutionSpray {
    card_name!("Devolution Spray");

    fn requirements_ok(&self, _player: Player, _card: &Card, _engine: &GameEngine) -> bool {
        true
    }
    fn execute(&self, _player: Player, _card: &Card, _engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        unimplemented!();
    }
}

#[derive(Default)]
struct ImpostorProfessorOak {}
impl TrainerCardArchetype for ImpostorProfessorOak {
    fn name(&self) -> String { "Impostor Professor Oak".into() }

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
    fn name(&self) -> String { "Item Finder".into() }
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
    fn name(&self) -> String { "Lass".into() }
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
    fn name(&self) -> String { "Pokémon Breeder".into() }
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
    fn name(&self) -> String { "Pokémon Trader".into() }
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
    fn name(&self) -> String { "Scoop Up".into() }
    fn requirements_ok(&self, player: Player, _card: &Card, engine: &GameEngine) -> bool {
        engine.state.side(player).all_in_play().iter().any(|p| p.stack.iter().any(|card| engine.stage(card.card()) == Some(Stage::Basic)))
    }
    fn execute(&self, player: Player, _card: &Card, engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameEngine {
        let choices = engine.state.side(player).all_in_play().into_iter().filter(|p| p.stack.iter().any(|card| engine.stage(card.card()) == Some(Stage::Basic))).cloned().collect();

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
    fn name(&self) -> String { "Defender".into() }
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
    fn name(&self) -> String { "Energy Retrieval".into() }
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
    fn name(&self) -> String { "Maintenance".into() }
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
    fn name(&self) -> String { "Plus Power".into() }
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
    fn name(&self) -> String { "Pokédex".into() }
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
    fn name(&self) -> String { "Professor Oak".into() }
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
    fn name(&self) -> String { "Bill".into() }
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
    fn name(&self) -> String { "Gust of Wind".into() }
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
    fn name(&self) -> String { "Switch".into() }
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
    card_name!("<Unknown>");
    not_a_pokemon!();

    fn card_actions(&self, _player: Player, _card: &Card, _engine: &GameEngine) -> Vec<Action> {
        vec![]
    }
    fn execute(&self, _player: Player, _card: &Card, _engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        panic!("not implemented");
    }
    fn attacks(&self, _player: Player, _in_play: &InPlayCard, _engine: &GameEngine) -> Vec<Action> {
        vec![]
    }
    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
