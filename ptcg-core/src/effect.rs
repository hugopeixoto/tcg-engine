use crate::attack_builder::{AttackBuilder, AttackBuilderContext};
use crate::state::{Effect, EffectConsequence, EffectExpiration, EffectSource, EffectTarget, EffectParameter, Type};
use crate::state::{Player, InPlayCard};
use crate::engine::{GameEngine, Resistance, Weakness, Action};

#[derive(Default)]
pub struct AttackEffectBuilder {
    effect: Option<Box<dyn Fn(&AttackBuilderContext) -> EffectConsequence>>,
    source: Option<Box<dyn Fn(&AttackBuilderContext) -> EffectSource>>,
    target: Option<Box<dyn Fn(&AttackBuilderContext) -> EffectTarget>>,
    expires: Option<Box<dyn Fn(&AttackBuilderContext) -> EffectExpiration>>,
    parameters: Vec<Box<dyn Fn(&AttackBuilderContext) -> EffectParameter>>,
}

pub fn from_attack() -> AttackEffectBuilder {
    let mut builder = AttackEffectBuilder::default();

    builder.source = Some(Box::new(|ab: &AttackBuilderContext| {
        EffectSource::Attack(ab.player(), ab.attacking().id)
    }));

    builder
}

impl AttackEffectBuilder {
    pub fn on_attacking(mut self) -> Self {
        self.target = Some(Box::new(|ab: &AttackBuilderContext| {
            EffectTarget::InPlay(ab.player(), ab.attacking().id)
        }));

        self
    }

    pub fn on_defending(mut self) -> Self {
        self.target = Some(Box::new(|ab: &AttackBuilderContext| {
            EffectTarget::InPlay(ab.player(), ab.defending().id)
        }));

        self
    }

    pub fn until_opponents_end_of_turn(mut self) -> Self {
        self.expires = Some(Box::new(|ab: &AttackBuilderContext| {
            EffectExpiration::EndOfTurn(ab.opponent(), 0)
        }));

        self
    }

    pub fn while_active(mut self) -> Self {
        self.expires = Some(Box::new(|_ab| {
            EffectExpiration::DefendingPokemon
        }));

        self
    }

    pub fn type_parameter(mut self, t: Type) -> Self {
        self.parameters.push(Box::new(move |_ab| {
            EffectParameter::Type(t.clone())
        }));
        self
    }

    pub fn string_parameter(mut self, s: String) -> Self {
        self.parameters.push(Box::new(move |_ab| {
            EffectParameter::String(s.clone())
        }));
        self
    }

    pub fn custom_effect<T: CustomEffect>(mut self) -> Self {
        let identifier = T::identifier();
        self.effect = Some(Box::new(move |_ab: &AttackBuilderContext| {
            identifier.clone()
        }));
        self
    }

    pub fn apply<'a>(&self, mut builder: AttackBuilderContext<'a>) -> AttackBuilderContext<'a> {
        let target = (self.target.as_ref().unwrap())(&builder);
        let source = (self.source.as_ref().unwrap())(&builder);
        let expires = (self.expires.as_ref().unwrap())(&builder);
        let consequence = (self.effect.as_ref().unwrap())(&builder);
        let parameters = self.parameters.iter().map(|e| e(&builder)).collect();

        builder.engine = builder.engine.with_effect(Effect {
            consequence,
            expires,
            name: "batata".into(),
            source,
            target,
            parameters,
        });
        builder
    }
}

pub trait CustomEffect {
    fn identifier() -> String where Self: Sized;

    fn defending_damage(&self, _damage: usize) -> Option<usize> { None }
    fn attacking_damage(&self, _damage: usize) -> Option<usize> { None }
    fn get_resistance(&self, _effect: &Effect, _in_play: &InPlayCard, _engine: &GameEngine, _resistance: Resistance) -> Option<Resistance> { None }
    fn get_weakness(&self, _effect: &Effect, _in_play: &InPlayCard, _engine: &GameEngine, _weakness: Weakness) -> Option<Weakness> { None }
    fn get_attacks(&self, _effect: &Effect, _in_play: &InPlayCard, _engine: &GameEngine, _actions: Vec<Action>) -> Option<Vec<Action>> { None }

    fn on_affected(&self) -> Option<AttackBuilder> { None }
    fn on_turn_end(&self) -> Option<AttackBuilder> { None }
    fn on_would_be_knocked_out(&self, _effect: &Effect, _in_play: &InPlayCard, _engine: &GameEngine) -> Option<AttackBuilder> { None }
    fn on_knocked_out(&self, _effect: &Effect, _in_play: &InPlayCard, _engine: &GameEngine) -> Option<AttackBuilder> { None }
    fn on_trainer(&self) -> Option<AttackBuilder> { None }
    fn on_energy_attachment(&self, _effect: &Effect, _player: Player) -> Option<AttackBuilder> { None }
}
