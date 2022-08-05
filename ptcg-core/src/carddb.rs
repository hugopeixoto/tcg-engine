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
            "Super Energy Removal (BS 79)"      => Trainer::create::<SuperEnergyRemoval>(),
            "Defender (BS 80)"                  => Trainer::create::<Defender>(),
            "Energy Retrieval (BS 81)"          => Trainer::create::<EnergyRetrieval>(),
            "Full Heal (BS 82)"                 => Trainer::create::<FullHeal>(),
            "Maintenance (BS 83)"               => Trainer::create::<Maintenance>(),
            "PlusPower (BS 84)"                 => Trainer::create::<PlusPower>(),
            "Pokemon Center (BS 85)"            => Trainer::create::<PokemonCenter>(),
            "Pokemon Flute (BS 86)"             => Trainer::create::<PokemonFlute>(),
            "Pokedex (BS 87)"                   => Trainer::create::<Pokedex>(),
            "Professor Oak (BS 88)"             => Trainer::create::<ProfessorOak>(),
            "Revive (BS 89)"                    => Trainer::create::<Revive>(),
            "Super Potion (BS 90)"              => Trainer::create::<SuperPotion>(),
            "Bill (BS 91)"                      => Trainer::create::<Bill>(),
            "Energy Removal (BS 92)"            => Trainer::create::<EnergyRemoval>(),
            "Gust of Wind (BS 93)"              => Trainer::create::<GustOfWind>(),
            "Potion (BS 94)"                    => Trainer::create::<Potion>(),
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
    fn requirements_ok(&self, player: Player, card: &Card, engine: &GameEngine) -> bool {
        engine
            .push_action(Action::TrainerFromHand(player, card.clone()))
            .then(|e| self.cost(e, &mut FakeDM::default()))
            .pop_action()
            .is_good()
    }

    fn execute(&self, player: Player, card: &Card, engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameEngine;
    fn name(&self) -> String;
    fn cost(&self, engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine.clone()
    }

    fn hp(&self, _card: &Card, _engine: &GameEngine) -> Option<usize> {
        None
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
        self.archetype
            .execute(player, card, engine, dm)
            .discard_from_hand(player, card, dm)
    }

    fn attacks(&self, _player: Player, _in_play: &InPlayCard, _engine: &GameEngine) -> Vec<Action> {
        vec![]
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }

    fn hp(&self, card: &Card, engine: &GameEngine) -> Option<usize> {
        self.archetype.hp(card, engine)
    }
    fn stage(&self) -> Option<Stage> {
        None
    }
    fn evolves_from(&self) -> Option<String> {
        None
    }
    fn weakness(&self) -> Weakness {
        (0, vec![])
    }
    fn resistance(&self) -> Resistance {
        (0, vec![])
    }
    fn pokemon_type(&self) -> Vec<Type> {
        vec![]
    }
    fn retreat(&self) -> usize {
        0
    }
}

#[derive(Default)]
struct ClefairyDoll {}
impl TrainerCardArchetype for ClefairyDoll {
    card_name!("Clefairy Doll");

    fn requirements_ok(&self, player: Player, card: &Card, engine: &GameEngine) -> bool {
        engine.can_bench(player, card)
    }

    fn execute(&self, player: Player, card: &Card, engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine.bench_from_hand(player, card)
    }

    fn hp(&self, card: &Card, engine: &GameEngine) -> Option<usize> {
        match engine.zone(card) {
            Zone::InPlay(_) => Some(10),
            _ => None,
        }
    }
}

#[derive(Default)]
struct ComputerSearch {}
impl TrainerCardArchetype for ComputerSearch {
    card_name!("Computer Search");

    fn cost(&self, engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameEngine {
        engine
            .ensure_discard_other(engine.player(), 2, dm)
            .ensure_deck_not_empty(engine.player())
    }

    fn execute(&self, player: Player, _card: &Card, engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameEngine {
        self
            .cost(engine, dm)
            .search_any_deck_to_hand(player, 1, dm)
    }
}

#[derive(Default)]
struct DevolutionSpray {}
impl TrainerCardArchetype for DevolutionSpray {
    card_name!("Devolution Spray");

    fn cost(&self, engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine.ensure(|e| !self.evolutions_in_play(e.player(), e).is_empty())
    }
    fn execute(&self, player: Player, _card: &Card, engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameEngine {
        let choices = self.evolutions_in_play(player, engine);
        let chosen = dm.pick_in_play(player, 1, &choices);

        let mut stages = self.stages(&chosen[0], engine);
        stages.remove(0);

        let stage = dm.pick_stage(player, &stages);

        self
            .cost(engine, dm)
            .devolve(&chosen[0], stage, &Zone::Discard(player))
    }
}
impl DevolutionSpray {
    fn evolutions_in_play(&self, player: Player, engine: &GameEngine) -> Vec<InPlayCard> {
        engine
            .state.side(player)
            .all_in_play().into_iter()
            .filter(|in_play| match engine.stage(in_play.stack[0].card()) {
                Some(Stage::Stage1 | Stage::Stage2) => true,
                _ => false,
            })
            .cloned().collect()
    }

    fn stages(&self, in_play: &InPlayCard, engine: &GameEngine) -> Vec<Stage> {
        engine
            .state.side(in_play.owner)
            .in_play(&in_play.id).unwrap()
            .stack.iter()
            .map(|c| engine.stage(c.card()))
            .filter(Option::is_some).map(|x| x.unwrap()).collect()
    }
}

#[derive(Default)]
struct ImpostorProfessorOak {}
impl TrainerCardArchetype for ImpostorProfessorOak {
    card_name!("Impostor Professor Oak");

    fn cost(&self, engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine.ensure(|e| !e.state.side(e.opponent()).deck.is_empty() || !e.state.side(e.opponent()).hand.is_empty())
    }

    fn execute(&self, player: Player, _card: &Card, engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameEngine {
        self
            .cost(engine, dm)
            .shuffle_hand_into_deck(player.opponent(), dm)
            .draw(player.opponent(), 7, dm)
    }
}

#[derive(Default)]
struct ItemFinder {}
impl TrainerCardArchetype for ItemFinder {
    card_name!("Item Finder");

    fn cost(&self, engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameEngine {
        engine
            .ensure_discard_contains(engine.player(), 1, |e, c| e.is_trainer(c))
            .ensure_discard_other(engine.player(), 2, dm)
    }

    fn execute(&self, player: Player, _card: &Card, engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameEngine {
        // TODO: searchable_cards happening before calling cost avoids recycling discarded cards
        let searchable_cards = engine.state.side(player).discard.iter().filter(|c| engine.is_trainer(c)).cloned().collect::<Vec<_>>();

        self.cost(engine, dm)
            .search_discard_to_hand(player, 1, |c| searchable_cards.contains(c), dm)
    }
}

#[derive(Default)]
struct Lass {}
impl TrainerCardArchetype for Lass {
    card_name!("Lass");

    fn cost(&self, engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine
            .ensure(|e| !e.state.side(e.player()).hand.is_empty() || e.state.side(e.opponent()).hand.is_empty())
    }
    fn execute(&self, player: Player, _card: &Card, engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameEngine {
        self
            .cost(engine, dm)
            .shuffle_all_from_hand_into_deck(player, |e, c| e.is_trainer(c), dm)
            .shuffle_all_from_hand_into_deck(player.opponent(), |e, c| e.is_trainer(c), dm)
    }
}

#[derive(Default)]
struct PokemonBreeder {}
impl TrainerCardArchetype for PokemonBreeder {
    card_name!("Pokémon Breeder");

    fn requirements_ok(&self, _player: Player, _card: &Card, _engine: &GameEngine) -> bool {
        true
    }
    fn execute(&self, _player: Player, _card: &Card, _engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        unimplemented!();
    }
}

#[derive(Default)]
struct PokemonTrader {}
impl TrainerCardArchetype for PokemonTrader {
    card_name!("Pokémon Trader");

    fn cost(&self, engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameEngine {
         engine
            .ensure_shuffle_other(engine.player(), 1, GameEngine::is_pokemon, dm)
    }
    fn execute(&self, player: Player, _card: &Card, engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameEngine {
        self.cost(engine, dm)
            .search_deck_to_hand(player, 1, GameEngine::is_pokemon, dm)
    }
}

#[derive(Default)]
struct ScoopUp {}
impl TrainerCardArchetype for ScoopUp {
    card_name!("Scoop Up");

    fn requirements_ok(&self, _player: Player, _card: &Card, _engine: &GameEngine) -> bool {
        true
    }

    fn execute(&self, player: Player, _card: &Card, engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameEngine {
        let choices = engine.state.side(player).all_in_play().into_iter().cloned().collect();

        let chosen = dm.pick_in_play(player, 1, &choices);

        engine.scoop_up(chosen[0], |e, c| e.stage(c) == Some(Stage::Basic))
    }
}

#[derive(Default)]
struct SuperEnergyRemoval {}
impl TrainerCardArchetype for SuperEnergyRemoval {
    card_name!("Super Energy Removal");

    fn cost(&self, engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine.clone()
    }

    fn execute(&self, _player: Player, _card: &Card, _engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        unimplemented!();
    }
}

#[derive(Default)]
struct Defender {}
impl TrainerCardArchetype for Defender {
    card_name!("Defender");

    fn requirements_ok(&self, _player: Player, _card: &Card, _engine: &GameEngine) -> bool {
        true
    }
    fn execute(&self, _player: Player, _card: &Card, _engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        unimplemented!();
    }
}

#[derive(Default)]
struct EnergyRetrieval {}
impl TrainerCardArchetype for EnergyRetrieval {
    card_name!("Energy Retrieval");

    fn cost(&self, engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameEngine {
        engine
            .ensure_discard_contains(engine.player(), 1, |e, c| e.is_basic_energy(c))
            .ensure_discard_other(engine.player(), 1, dm)
    }
    fn execute(&self, player: Player, _card: &Card, engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameEngine {
        // TODO: searchable_cards happening before calling cost avoids recycling discarded cards
        let searchable_cards = engine.state.side(player).discard.iter().filter(|c| engine.is_basic_energy(c)).cloned().collect::<Vec<_>>();

        self.cost(engine, dm)
            .search_discard_to_hand(player, 1, |c| searchable_cards.contains(c), dm)
    }
}

#[derive(Default)]
struct FullHeal {}
impl TrainerCardArchetype for FullHeal {
    card_name!("Full Heal");

    fn cost(&self, engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine.ensure(|e| !self.affected_in_play(e.player(), e).is_empty())
    }
    fn execute(&self, player: Player, _card: &Card, engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameEngine {
        let targets = self.affected_in_play(player, engine);
        let target = dm.pick_in_play(player, 1, &targets)[0];

        engine.remove_special_conditions(&target)
    }
}
impl FullHeal {
    fn affected_in_play(&self, player: Player, engine: &GameEngine) -> Vec<InPlayCard> {
        engine.state.side(player).all_in_play().into_iter().filter(|p| GameEngine::has_special_condition(engine, p)).cloned().collect()
    }
}

#[derive(Default)]
struct Maintenance {}
impl TrainerCardArchetype for Maintenance {
    card_name!("Maintenance");

    fn cost(&self, engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameEngine {
        engine
            .ensure_shuffle_any_other(engine.player(), 2, dm)
    }

    fn execute(&self, player: Player, _card: &Card, engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameEngine {
        self
            .cost(engine, dm)
            .draw(player, 1, dm)
    }
}

#[derive(Default)]
struct PlusPower {}
impl TrainerCardArchetype for PlusPower {
    card_name!("Plus Power");

    fn requirements_ok(&self, _player: Player, _card: &Card, _engine: &GameEngine) -> bool {
        true
    }
    fn execute(&self, _player: Player, _card: &Card, _engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        unimplemented!();
    }
}

#[derive(Default)]
struct PokemonCenter {}
impl TrainerCardArchetype for PokemonCenter {
    card_name!("Pokémon Center");

    fn requirements_ok(&self, _player: Player, _card: &Card, _engine: &GameEngine) -> bool {
        true
    }
    fn execute(&self, _player: Player, _card: &Card, _engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        unimplemented!();
    }
}

#[derive(Default)]
struct PokemonFlute {}
impl TrainerCardArchetype for PokemonFlute {
    card_name!("Pokémon Flute");

    fn requirements_ok(&self, _player: Player, _card: &Card, _engine: &GameEngine) -> bool {
        true
    }
    fn execute(&self, _player: Player, _card: &Card, _engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        unimplemented!();
    }
}

#[derive(Default)]
struct Pokedex {}
impl TrainerCardArchetype for Pokedex {
    card_name!("Pokédex");

    fn requirements_ok(&self, player: Player, _card: &Card, engine: &GameEngine) -> bool {
        !engine.state.side(player).deck.is_empty()
    }
    fn execute(&self, _player: Player, _card: &Card, _engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        unimplemented!();
    }
}

#[derive(Default)]
struct ProfessorOak {}
impl TrainerCardArchetype for ProfessorOak {
    card_name!("Professor Oak");

    fn cost(&self, engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameEngine {
        engine
            .ensure_discard_all(engine.player(), dm)
            .ensure_deck_not_empty(engine.player())
    }
    fn execute(&self, player: Player, _card: &Card, engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameEngine {
        self
            .cost(engine, dm)
            .draw(player, 7, dm)
    }
}

#[derive(Default)]
struct Revive {}
impl TrainerCardArchetype for Revive {
    card_name!("Revive");

    fn cost(&self, engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine.clone()
    }
    fn execute(&self, _player: Player, _card: &Card, _engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        unimplemented!();
    }
}

#[derive(Default)]
struct SuperPotion {}
impl TrainerCardArchetype for SuperPotion {
    card_name!("Super Potion");

    fn cost(&self, engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine.clone()
    }
    fn execute(&self, _player: Player, _card: &Card, _engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        unimplemented!();
    }
}

#[derive(Default)]
struct Bill {}
impl TrainerCardArchetype for Bill {
    card_name!("Bill");

    fn cost(&self, engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine
            .ensure_deck_not_empty(engine.player())
    }
    fn execute(&self, player: Player, _card: &Card, engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameEngine {
        self
            .cost(engine, dm)
            .draw(player, 2, dm)
    }
}

#[derive(Default)]
struct EnergyRemoval {}
impl TrainerCardArchetype for EnergyRemoval {
    card_name!("Energy Removal");

    fn cost(&self, engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine
            .ensure_deck_not_empty(engine.player())
    }
    fn execute(&self, player: Player, _card: &Card, engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameEngine {
        self
            .cost(engine, dm)
            .draw(player, 2, dm)
    }
}

#[derive(Default)]
struct GustOfWind {}
impl TrainerCardArchetype for GustOfWind {
    card_name!("Gust of Wind");

    fn cost(&self, engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine
            .ensure(|e| !e.state.side(engine.opponent()).bench.is_empty())
    }
    fn execute(&self, player: Player, _card: &Card, engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameEngine {
        self
            .cost(engine, dm)
            .gust(player, dm)
    }
}

#[derive(Default)]
struct Potion {}
impl TrainerCardArchetype for Potion {
    card_name!("Potion");

    fn cost(&self, engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine.clone()

        // at least one in play must have damage counters
    }
    fn execute(&self, player: Player, card: &Card, engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameEngine {
        let targets = engine.attachment_from_hand_targets(player, card);
        let target = dm.pick_in_play(player, 1, &targets)[0];

        engine.heal(target, 20)
    }
}

#[derive(Default)]
struct Switch {}
impl TrainerCardArchetype for Switch {
    card_name!("Switch");

    fn cost(&self, engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine
            .ensure(|e| !e.state.side(engine.player()).bench.is_empty())
    }
    fn execute(&self, player: Player, _card: &Card, engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameEngine {
        self
            .cost(engine, dm)
            .switch(player, dm)
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
        unimplemented!();
    }
    fn attacks(&self, _player: Player, _in_play: &InPlayCard, _engine: &GameEngine) -> Vec<Action> {
        vec![]
    }
    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
