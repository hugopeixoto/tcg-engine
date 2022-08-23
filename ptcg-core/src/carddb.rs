use crate::*;

pub struct Pokemon {}
impl Pokemon {
    pub fn create<T: Default + CardArchetype + 'static>() -> Box<dyn CardArchetype> {
        Box::new(T::default())
    }
}

pub trait TrainerCardArchetype {
    fn requirements_ok(&self, player: Player, card: &Card, engine: &GameEngine) -> bool {
        engine
            .push_action(Action::TrainerFromHand(player, card.clone()))
            .then(|e| self.cost(e, &mut FakeDM::default()))
            .pop_action()
            .is_good()
    }

    fn execute(&self, player: Player, card: &Card, engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameEngine;
    fn identifier(&self) -> String;
    fn name(&self) -> String;
    fn cost(&self, engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine.clone()
    }

    fn hp(&self, _card: &Card, _engine: &GameEngine) -> Option<usize> {
        None
    }
    fn attacking_damage_effect(&self, _card: &Card, _engine: &GameEngine, _damage: usize) -> Option<usize> {
        None
    }
    fn defending_damage_effect(&self, _card: &Card, _engine: &GameEngine, _damage: usize) -> Option<usize> {
        None
    }
    fn on_turn_end(&self, _card: &Card, _engine: &GameEngine) -> Option<GameEngine> {
        None
    }
}

pub struct Trainer {
    archetype: Box<dyn TrainerCardArchetype>,
}
impl Trainer {
    pub fn create<T: Default + TrainerCardArchetype + 'static>() -> Box<dyn CardArchetype> {
        Box::new(Self { archetype: Box::new(T::default()) })
    }
}
impl CardArchetype for Trainer {
    fn identifier(&self) -> String {
        self.archetype.identifier()
    }

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

    fn hp(&self, card: &Card, engine: &GameEngine) -> Option<usize> {
        self.archetype.hp(card, engine)
    }
    fn stage(&self) -> Option<Stage> {
        None
    }
    fn is_trainer(&self, _card: &Card, _engine: &GameEngine) -> bool {
        true
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
    fn attacking_damage_effect(&self, card: &Card, engine: &GameEngine, damage: usize) -> Option<usize> {
        self.archetype.attacking_damage_effect(card, engine, damage)
    }
    fn defending_damage_effect(&self, card: &Card, engine: &GameEngine, damage: usize) -> Option<usize> {
        self.archetype.defending_damage_effect(card, engine, damage)
    }
    fn on_turn_end(&self, card: &Card, engine: &GameEngine) -> Option<GameEngine> {
        self.archetype.on_turn_end(card, engine)
    }
}
