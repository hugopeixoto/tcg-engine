use crate::*;
use crate::carddb::Attacks;
use crate::attack_builder::AttackBuilder;

#[derive(Default)]
pub struct Articuno {}
impl CardArchetype for Articuno {
    card_name!("Articuno");
    basic!();
    hp!(70);
    color!(Water);
    no_weakness!();
    resists!(Fighting, 30);
    retreat!(2);

    fn card_actions(&self, _player: Player, _card: &Card, _engine: &GameEngine) -> Vec<Action> {
        vec![]
    }
    fn execute(&self, _player: Player, _card: &Card, engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine.clone()
    }
    fn attacks(&self, player: Player, in_play: &InPlayCard, engine: &GameEngine) -> Vec<Action> {
        Attacks::new(player, in_play, engine)
            .register("Freeze Dry", Self::freeze_dry)
            .register("Blizzard", Self::blizzard)
            .into()
    }
    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Articuno {
    pub fn freeze_dry(builder: AttackBuilder) -> AttackBuilder {
        builder
            .flip_a_coin()
            .damage(30)
            .if_heads(|b| b.paralyze())
    }

    pub fn blizzard(builder: AttackBuilder) -> AttackBuilder {
        builder
            .flip_a_coin()
            .damage(50)
            .if_heads(|b| b.each_opponents_bench(|b2| b2.damage(10)))
            .if_tails(|b| b.each_own_bench(|b2| b2.damage(10)))
    }
}

#[derive(Default)]
pub struct Psyduck {}
impl CardArchetype for Psyduck {
    card_name!("Psyduck");
    basic!();
    hp!(50);
    color!(Water);
    weak_to!(Lightning);
    no_resistance!();
    retreat!(1);

    fn card_actions(&self, _player: Player, _card: &Card, _engine: &GameEngine) -> Vec<Action> {
        vec![]
    }
    fn execute(&self, _player: Player, _card: &Card, engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine.clone()
    }
    fn attacks(&self, player: Player, in_play: &InPlayCard, engine: &GameEngine) -> Vec<Action> {
        Attacks::new(player, in_play, engine)
            .register("Headache", Self::headache)
            .register("Fury Swipes", Self::fury_swipes)
            .into()
    }
    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Psyduck {
    pub fn headache(builder: AttackBuilder) -> AttackBuilder {
        builder
            .prevent_trainers_during_opponents_next_turn()
    }
    pub fn fury_swipes(builder: AttackBuilder) -> AttackBuilder {
        builder
            .flip_coins(3)
            .damage_per_heads(10)
    }
}
