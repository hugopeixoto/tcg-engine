use crate::*;

pub struct Attacks<'a> {
    player: Player,
    in_play: &'a InPlayCard,
    engine: &'a GameEngine,
    attacks: Vec<Action>,
}

impl<'a> Attacks<'a> {
    pub fn new(player: Player, in_play: &'a InPlayCard, engine: &'a GameEngine) -> Self {
        Attacks {
            player,
            in_play,
            engine: engine,
            attacks: vec![],
        }
    }

    pub fn register(mut self, name: &str, f: fn(AttackBuilder) -> AttackBuilder) -> Attacks<'a> {
        let action = Action::Attack(self.player, self.in_play.clone(), name.into(), Box::new(RFA::new(f)));

        if self.are_attack_requirements_met(&action) {
            self.attacks.push(action);
        }

        self
    }

    pub fn are_attack_requirements_met(&self, action: &Action) -> bool {
        if let Action::Attack(player, attacking, _name, executor) = action {
            let mut dm = FakeDM{};
            let engine = self.engine
                .push_action(action.clone())
                .push_target(&attacking, &self.engine.state.side(player.opponent()).active[0]);

            !executor.build(&engine, &mut dm).failed()
        } else {
            panic!("Can't check the attack requirements for {:?}", action);
        }
    }
}

impl<'a> From<Attacks<'a>> for Vec<Action> {
    fn from(attacks: Attacks<'a>) -> Self {
        attacks.attacks
    }
}

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
