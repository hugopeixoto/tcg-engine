use crate::state::*;
use crate::engine::*;
use crate::*;
use crate::carddb::TrainerCardArchetype;

#[derive(Default)]
pub struct ClefairyDoll70 {}
impl TrainerCardArchetype for ClefairyDoll70 {
    identifier!("Clefairy Doll (BS 70)");
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
pub struct ComputerSearch71 {}
impl TrainerCardArchetype for ComputerSearch71 {
    identifier!("Computer Search (BS 71)");
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
pub struct DevolutionSpray72 {}
impl TrainerCardArchetype for DevolutionSpray72 {
    identifier!("Devolution Spray (BS 72)");
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
impl DevolutionSpray72 {
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
pub struct ImpostorProfessorOak73 {}
impl TrainerCardArchetype for ImpostorProfessorOak73 {
    identifier!("Impostor Professor Oak (BS 73)");
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
pub struct ItemFinder74 {}
impl TrainerCardArchetype for ItemFinder74 {
    identifier!("Item Finder (BS 74)");
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
pub struct Lass75 {}
impl TrainerCardArchetype for Lass75 {
    identifier!("Lass (BS 75)");
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
pub struct PokemonBreeder76 {}
impl TrainerCardArchetype for PokemonBreeder76 {
    identifier!("Pokémon Breeder (BS 76)");
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
impl PokemonBreeder76 {
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
pub struct PokemonTrader77 {}
impl TrainerCardArchetype for PokemonTrader77 {
    identifier!("Pokémon Trader (BS 77)");
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
pub struct ScoopUp78 {}
impl TrainerCardArchetype for ScoopUp78 {
    identifier!("Scoop Up (BS 78)");
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
pub struct SuperEnergyRemoval79 {}
impl TrainerCardArchetype for SuperEnergyRemoval79 {
    identifier!("Super Energy Removal (BS 79)");
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
impl SuperEnergyRemoval79 {
    pub fn energy_removal_targets(&self, player: Player, engine: &GameEngine) -> Vec<InPlayCard> {
        engine.state.side(player).all_in_play().into_iter().filter(|p| GameEngine::has_energy_cards_attached(engine, p)).cloned().collect()
    }
}

#[derive(Default)]
pub struct Defender80 {}
impl TrainerCardArchetype for Defender80 {
    identifier!("Defender (BS 80)");
    card_name!("Defender");

    fn requirements_ok(&self, _player: Player, _card: &Card, _engine: &GameEngine) -> bool {
        true
    }
    fn execute(&self, player: Player, card: &Card, engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameEngine {
        let targets = engine.state.side(player).all_in_play().into_iter().cloned().collect();
        let target = dm.pick_in_play(player, 1, &targets)[0];

        engine
            .attach_from_hand(card, target)
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
pub struct EnergyRetrieval81 {}
impl TrainerCardArchetype for EnergyRetrieval81 {
    identifier!("Energy Retrieval (BS 81)");
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
pub struct FullHeal82 {}
impl TrainerCardArchetype for FullHeal82 {
    identifier!("Full Heal (BS 82)");
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
impl FullHeal82 {
    fn affected_in_play(&self, player: Player, engine: &GameEngine) -> Vec<InPlayCard> {
        engine.state.side(player).all_in_play().into_iter().filter(|p| GameEngine::has_special_condition(engine, p)).cloned().collect()
    }
}

#[derive(Default)]
pub struct Maintenance83 {}
impl TrainerCardArchetype for Maintenance83 {
    identifier!("Maintenance (BS 83)");
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
pub struct PlusPower84 {}
impl TrainerCardArchetype for PlusPower84 {
    identifier!("Plus Power (BS 84)");
    card_name!("Plus Power");

    fn requirements_ok(&self, _player: Player, _card: &Card, _engine: &GameEngine) -> bool {
        true
    }
    fn execute(&self, player: Player, card: &Card, engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameEngine {
        let targets = engine.state.side(player).active.iter().cloned().collect();
        let target = dm.pick_in_play(player, 1, &targets)[0];

        engine
            .attach_from_hand(card, target)
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
pub struct PokemonCenter85 {}
impl TrainerCardArchetype for PokemonCenter85 {
    identifier!("Pokémon Center (BS 85)");
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
pub struct PokemonFlute86 {}
impl TrainerCardArchetype for PokemonFlute86 {
    identifier!("Pokémon Flute (BS 86)");
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
impl PokemonFlute86 {
    fn discarded_basics(&self, player: Player, engine: &GameEngine) -> Vec<Card> {
        engine.state.side(player).discard.iter().filter(|&c| engine.stage(c) == Some(Stage::Basic)).cloned().collect()
    }
}

#[derive(Default)]
pub struct Pokedex87 {}
impl TrainerCardArchetype for Pokedex87 {
    identifier!("Pokédex (BS 87)");
    card_name!("Pokédex");

    fn requirements_ok(&self, player: Player, _card: &Card, engine: &GameEngine) -> bool {
        !engine.state.side(player).deck.is_empty()
    }
    fn execute(&self, player: Player, _card: &Card, engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameEngine {
        engine.rearrange_topdeck(player, player, 5, dm)
    }
}

#[derive(Default)]
pub struct ProfessorOak88 {}
impl TrainerCardArchetype for ProfessorOak88 {
    identifier!("Professor Oak (BS 88)");
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
pub struct Revive89 {}
impl TrainerCardArchetype for Revive89 {
    identifier!("Revive (BS 89)");
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
impl Revive89 {
    fn discarded_basics(&self, player: Player, engine: &GameEngine) -> Vec<Card> {
        engine.state.side(player).discard.iter().filter(|&c| engine.stage(c) == Some(Stage::Basic)).cloned().collect()
    }
}

#[derive(Default)]
pub struct SuperPotion90 {}
impl TrainerCardArchetype for SuperPotion90 {
    identifier!("Super Potion (BS 90)");
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
impl SuperPotion90 {
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
pub struct Bill90 {}
impl TrainerCardArchetype for Bill90 {
    identifier!("Bill (BS 90)");
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
pub struct EnergyRemoval91 {}
impl TrainerCardArchetype for EnergyRemoval91 {
    identifier!("Energy Removal (BS 91)");
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
impl EnergyRemoval91 {
    pub fn energy_removal_targets(&self, engine: &GameEngine) -> Vec<InPlayCard> {
        engine.state.side(engine.opponent()).all_in_play().into_iter().filter(|p| GameEngine::has_energy_cards_attached(engine, p)).cloned().collect()
    }
}

#[derive(Default)]
pub struct GustOfWind92 {}
impl TrainerCardArchetype for GustOfWind92 {
    identifier!("Gust of Wind (BS 92)");
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
pub struct Potion93 {}
impl TrainerCardArchetype for Potion93 {
    identifier!("Potion (BS 93)");
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
pub struct Switch94 {}
impl TrainerCardArchetype for Switch94 {
    identifier!("Switch (BS 94)");
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
