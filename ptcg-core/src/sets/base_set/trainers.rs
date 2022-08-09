use crate::state::*;
use crate::engine::*;
use crate::*;
use crate::carddb::TrainerCardArchetype;

#[derive(Default)]
pub struct ClefairyDoll {}
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
pub struct ComputerSearch {}
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
pub struct DevolutionSpray {}
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
pub struct ImpostorProfessorOak {}
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
pub struct ItemFinder {}
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
pub struct Lass {}
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
pub struct PokemonBreeder {}
impl TrainerCardArchetype for PokemonBreeder {
    card_name!("Pokémon Breeder");

    fn cost(&self, engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine
            .ensure(|e| !self.stage2_cards(e).is_empty())
    }
    fn execute(&self, player: Player, _card: &Card, engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameEngine {
        let stage2s = self.stage2_cards(engine);
        let card = dm.pick_from_hand(player, player, 1, &stage2s)[0];

        let targets = self.targets(engine, card);
        let chosen = dm.pick_in_play(player, 1, &targets)[0];

        engine.evolve_into(chosen, card)
    }
}
impl PokemonBreeder {
    pub fn stage2_cards(&self, engine: &GameEngine) -> Vec<Card> {
        engine
            .state.side(engine.player()).hand
            .iter()
            .filter(|card| engine.stage(card) == Some(Stage::Stage2) && !self.targets(engine, card).is_empty())
            .cloned()
            .collect()
    }

    pub fn targets(&self, engine: &GameEngine, card: &Card) -> Vec<InPlayCard> {
        let basic = engine.basic_for_stage2(card);

        engine
            .in_play(engine.player())
            .into_iter()
            .filter(|ip| engine.ready_to_evolve(ip))
            .filter(|ip| engine.archetype(ip.stack[0].card()).name() == basic)
            .cloned()
            .collect()
    }
}

#[derive(Default)]
pub struct PokemonTrader {}
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
pub struct ScoopUp {}
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
pub struct SuperEnergyRemoval {}
impl TrainerCardArchetype for SuperEnergyRemoval {
    card_name!("Super Energy Removal");

    fn cost(&self, engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine
            .ensure(|e| !self.energy_removal_targets(e.player(), e).is_empty())
            .ensure(|e| !self.energy_removal_targets(e.opponent(), e).is_empty())
    }

    fn execute(&self, _player: Player, _card: &Card, engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameEngine {
        let own_targets = self.energy_removal_targets(engine.player(), engine);
        let own_target = dm.pick_in_play(engine.player(), 1, &own_targets)[0];

        let own_cards = own_target.attached.iter().map(|c| c.card()).cloned().collect();
        let own_discarded = dm.pick_attached(engine.player(), 1..=1, &own_cards);

        let their_targets = self.energy_removal_targets(engine.opponent(), engine);
        let their_target = dm.pick_in_play(engine.player(), 1, &their_targets)[0];

        let their_cards = their_target.attached.iter().map(|c| c.card()).cloned().collect();
        let their_discarded = dm.pick_attached(engine.player(), 1..=2, &their_cards);

        engine
            .remove_attached_cards(&own_discarded)
            .remove_attached_cards(&their_discarded)
    }
}
impl SuperEnergyRemoval {
    pub fn energy_removal_targets(&self, player: Player, engine: &GameEngine) -> Vec<InPlayCard> {
        engine.state.side(player).all_in_play().into_iter().filter(|p| GameEngine::has_energy_cards_attached(engine, p)).cloned().collect()
    }
}

#[derive(Default)]
pub struct Defender {}
impl TrainerCardArchetype for Defender {
    card_name!("Defender");

    fn requirements_ok(&self, _player: Player, _card: &Card, _engine: &GameEngine) -> bool {
        true
    }
    fn execute(&self, player: Player, card: &Card, engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameEngine {
        let targets = engine.state.side(player).all_in_play().into_iter().cloned().collect();
        let target = dm.pick_in_play(player, 1, &targets)[0];

        engine
            .attach_from_hand(player, card, target)
    }

    fn defending_damage_effect(&self, card: &Card, engine: &GameEngine, damage: usize) -> Option<usize> {
        if engine.defending().attached.iter().any(|c| c.card() == card) {
            Some(damage.saturating_sub(20))
        } else {
            None
        }
    }

    fn on_turn_end(&self, card: &Card, engine: &GameEngine) -> Option<GameEngine> {
        if let Some(attached_turn) = engine.turn_attached(card) {
            if engine.is_end_of_opponents_next_turn(attached_turn) {
                return Some(engine.remove_attached_cards(&vec![card]));
            }
        }

        None
    }
}

#[derive(Default)]
pub struct EnergyRetrieval {}
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
pub struct FullHeal {}
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
pub struct Maintenance {}
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
pub struct PlusPower {}
impl TrainerCardArchetype for PlusPower {
    card_name!("Plus Power");

    fn requirements_ok(&self, _player: Player, _card: &Card, _engine: &GameEngine) -> bool {
        true
    }
    fn execute(&self, player: Player, card: &Card, engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameEngine {
        let targets = engine.state.side(player).active.iter().cloned().collect();
        let target = dm.pick_in_play(player, 1, &targets)[0];

        engine
            .attach_from_hand(player, card, target)
    }

    fn attacking_damage_effect(&self, card: &Card, engine: &GameEngine, damage: usize) -> Option<usize> {
        if engine.attacking().attached.iter().any(|c| c.card() == card) {
            if engine.opponents_active_pokemon().contains(&engine.defending()) {
                if damage > 0 {
                    return Some(damage.saturating_add(10));
                }
            }
        }

        None
    }

    fn on_turn_end(&self, card: &Card, engine: &GameEngine) -> Option<GameEngine> {
        engine
            .turn_attached(card)
            .map(|_| engine.remove_attached_cards(&vec![card]))
    }
}

#[derive(Default)]
pub struct PokemonCenter {}
impl TrainerCardArchetype for PokemonCenter {
    card_name!("Pokémon Center");

    fn cost(&self, engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine
            .ensure(|e| !e.healable_targets(e.player()).is_empty())
    }
    fn execute(&self, _player: Player, _card: &Card, engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        let mut engine = engine.clone();

        for target in engine.healable_targets(engine.player()) {
            let cards = target.attached.iter().map(|c| c.card()).collect();
            engine = engine.heal_all(&target).remove_attached_cards(&cards);
        }

        engine
    }
}

#[derive(Default)]
pub struct PokemonFlute {}
impl TrainerCardArchetype for PokemonFlute {
    card_name!("Pokémon Flute");

    fn cost(&self, engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine
            .ensure(|e| e.has_bench_space(e.opponent()))
            .ensure(|e| !self.discarded_basics(e.opponent(), e).is_empty())
    }
    fn execute(&self, _player: Player, _card: &Card, engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameEngine {
        let searchable_cards = self.discarded_basics(engine.opponent(), engine);
        let chosen = dm.pick_from_discard(engine.player(), engine.opponent(), 1, &searchable_cards);

        engine.bench_from_discard(engine.opponent(), chosen[0])
    }
}
impl PokemonFlute {
    fn discarded_basics(&self, player: Player, engine: &GameEngine) -> Vec<Card> {
        engine.state.side(player).discard.iter().filter(|&c| engine.stage(c) == Some(Stage::Basic)).cloned().collect()
    }
}

#[derive(Default)]
pub struct Pokedex {}
impl TrainerCardArchetype for Pokedex {
    card_name!("Pokédex");

    fn requirements_ok(&self, player: Player, _card: &Card, engine: &GameEngine) -> bool {
        !engine.state.side(player).deck.is_empty()
    }
    fn execute(&self, player: Player, _card: &Card, engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameEngine {
        engine.rearrange_topdeck(player, player, 5, dm)
    }
}

#[derive(Default)]
pub struct ProfessorOak {}
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
pub struct Revive {}
impl TrainerCardArchetype for Revive {
    card_name!("Revive");

    fn cost(&self, engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine
            .ensure(|e| e.has_bench_space(e.player()))
            .ensure(|e| !self.discarded_basics(e.player(), e).is_empty())
    }
    fn execute(&self, _player: Player, _card: &Card, engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameEngine {
        let searchable_cards = self.discarded_basics(engine.player(), engine);
        let chosen = dm.pick_from_discard(engine.player(), engine.player(), 1, &searchable_cards);

        engine
            .bench_from_discard(engine.player(), chosen[0])
            .then(|e| {
                let benched = &e.in_play_card(chosen[0]).unwrap();
                e.put_damage_counters(benched, (e.full_hp(benched) / 10).div_ceil(2))
            })
    }
}
impl Revive {
    fn discarded_basics(&self, player: Player, engine: &GameEngine) -> Vec<Card> {
        engine.state.side(player).discard.iter().filter(|&c| engine.stage(c) == Some(Stage::Basic)).cloned().collect()
    }
}

#[derive(Default)]
pub struct SuperPotion {}
impl TrainerCardArchetype for SuperPotion {
    card_name!("Super Potion");

    fn cost(&self, engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine
            .ensure(|e| !self.targets(e).is_empty())
    }
    fn execute(&self, _player: Player, _card: &Card, engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameEngine {
        let own_targets = self.targets(engine);
        let own_target = dm.pick_in_play(engine.player(), 1, &own_targets)[0];

        let own_cards = own_target.attached.iter().map(|c| c.card()).cloned().collect();
        let own_discarded = dm.pick_attached(engine.player(), 1..=1, &own_cards);

        engine
            .remove_attached_cards(&own_discarded)
            .heal(own_target, 40)
    }
}
impl SuperPotion {
    fn targets(&self, engine: &GameEngine) -> Vec<InPlayCard> {
        engine
            .in_play(engine.player())
            .into_iter()
            .filter(|p| engine.has_energy_cards_attached(p))
            .filter(|p| engine.is_healable(p))
            .cloned()
            .collect()
    }
}

#[derive(Default)]
pub struct Bill {}
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
pub struct EnergyRemoval {}
impl TrainerCardArchetype for EnergyRemoval {
    card_name!("Energy Removal");

    fn cost(&self, engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine
            .ensure(|e| !self.energy_removal_targets(e).is_empty())
    }
    fn execute(&self, player: Player, _card: &Card, engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameEngine {
        let targets = self.energy_removal_targets(engine);
        let target = dm.pick_in_play(player, 1, &targets)[0];

        let cards = target.attached.iter().map(|c| c.card()).cloned().collect();
        let discarded = dm.pick_attached(player, 1..=1, &cards);

        engine.remove_attached_cards(&discarded)
    }
}
impl EnergyRemoval {
    pub fn energy_removal_targets(&self, engine: &GameEngine) -> Vec<InPlayCard> {
        engine.state.side(engine.opponent()).all_in_play().into_iter().filter(|p| GameEngine::has_energy_cards_attached(engine, p)).cloned().collect()
    }
}

#[derive(Default)]
pub struct GustOfWind {}
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
pub struct Potion {}
impl TrainerCardArchetype for Potion {
    card_name!("Potion");

    fn cost(&self, engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine
            .ensure(|e| !e.healable_targets(e.player()).is_empty())
    }
    fn execute(&self, player: Player, _card: &Card, engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameEngine {
        let targets = engine.healable_targets(player);
        let target = dm.pick_in_play(player, 1, &targets)[0];

        engine.heal(target, 20)
    }
}

#[derive(Default)]
pub struct Switch {}
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


