use crate::attack_builder::AttackBuilder;
use crate::effect::CustomEffect;
use crate::state::{Effect, Player, InPlayCard};
use crate::engine::{GameEngine, Resistance, Weakness, Action};

// TODO: the attackbuilder builds a bunch of objects, maybe it
// should be stored in the struct and reused here.

pub struct PreventDamageDuringOpponentsTurn {}
impl CustomEffect for PreventDamageDuringOpponentsTurn {
    fn identifier() -> String {
        "PREVENT_DAMAGE_DURING_OPPONENTS_TURN".into()
    }

    fn defending_damage(&self, effect: &Effect, in_play: &InPlayCard, engine: &GameEngine, _damage: usize) -> Option<usize> {
        let opponents_turn = !effect.target.is_player(engine.player());
        let this_pokemon = effect.target.is_in_play(in_play);

        if opponents_turn && this_pokemon {
            Some(0)
        } else {
            None
        }
    }
}

pub struct PreventDamageAndEffectsDuringOpponentsTurn {}
impl CustomEffect for PreventDamageAndEffectsDuringOpponentsTurn {
    fn identifier() -> String {
        "PREVENT_DAMAGE_AND_EFFECTS_DURING_OPPONENTS_TURN".into()
    }

    fn on_affected(&self) -> Option<AttackBuilder> {
        AttackBuilder::new()
            .prevent()
            .into()
    }

    fn defending_damage(&self, effect: &Effect, in_play: &InPlayCard, engine: &GameEngine, _damage: usize) -> Option<usize> {
        let opponents_turn = !effect.target.is_player(engine.player());
        let this_pokemon = effect.target.is_in_play(in_play);

        if opponents_turn && this_pokemon {
            Some(0)
        } else {
            None
        }
    }
}

pub struct BlockTrainerFromHand {}
impl CustomEffect for BlockTrainerFromHand {
    fn identifier() -> String {
        "BLOCK_TRAINER_FROM_HAND".into()
    }

    fn on_trainer(&self) -> Option<AttackBuilder> {
        AttackBuilder::new()
            .prevent()
            .into()
    }
}

pub struct BlockAttachmentFromHand {}
impl CustomEffect for BlockAttachmentFromHand {
    fn identifier() -> String {
        "BLOCK_ATTACHMENT_FROM_HAND".into()
    }

    fn on_energy_attachment(&self, effect: &Effect, player: Player) -> Option<AttackBuilder> {
        if effect.target.is_player(player) {
            AttackBuilder::new()
                .prevent()
                .into()
        } else {
            None
        }
    }
}

pub struct RevengeKnockOut {}
impl CustomEffect for RevengeKnockOut {
    fn identifier() -> String {
        "REVENGE_KNOCK_OUT".into()
    }

    fn on_knocked_out(&self, effect: &Effect, in_play: &InPlayCard, engine: &GameEngine) -> Option<AttackBuilder> {
        let opponents_turn = !effect.target.is_player(engine.player());
        let this_pokemon = effect.target.is_in_play(in_play);
        // there's an attacking pokemon?

        // TODO: Revenge knock out: Sky Return?
        // TODO: Revenge knock out: Quick shooting?
        println!("knocked out? {} && {}", engine.is_someone_attacking(), effect.target.is_player(engine.player()));
        if opponents_turn && this_pokemon && engine.is_someone_attacking() {
            AttackBuilder::new()
                .knock_out_attacking()
                .into()
        } else {
            None
        }
    }
}

pub struct ChangeResistance {}
impl CustomEffect for ChangeResistance {
    fn identifier() -> String {
        "CHANGE_RESISTANCE".into()
    }

    fn get_resistance(&self, effect: &Effect, in_play: &InPlayCard, _engine: &GameEngine, resistance: Resistance) -> Option<Resistance> {
        let chosen = effect.get_parameter_type(0).unwrap();
        let this_pokemon = effect.target.is_in_play(in_play);

        if this_pokemon {
            Some((resistance.0, vec![chosen]))
        } else {
            None
        }
    }
}

pub struct ChangeWeakness {}
impl CustomEffect for ChangeWeakness {
    fn identifier() -> String {
        "CHANGE_WEAKNESS".into()
    }

    fn get_weakness(&self, effect: &Effect, in_play: &InPlayCard, _engine: &GameEngine, weakness: Weakness) -> Option<Weakness> {
        let chosen = effect.get_parameter_type(0).unwrap();
        let this_pokemon = effect.target.is_in_play(in_play);

        if this_pokemon {
            Some((weakness.0, vec![chosen]))
        } else {
            None
        }
    }
}

pub struct DisableAttack {}
impl CustomEffect for DisableAttack {
    fn identifier() -> String {
        "DISABLE_ATTACK".into()
    }

    fn get_attacks(&self, effect: &Effect, in_play: &InPlayCard, _engine: &GameEngine, actions: Vec<Action>) -> Option<Vec<Action>> {
        let disabled = effect.get_parameter_string(0).unwrap();
        let this_pokemon = effect.target.is_in_play(in_play);

        if this_pokemon {
            Some(actions.into_iter().filter(|a| match a { Action::Attack(_, _, name, _) => *name != disabled, _ => true }).collect())
        } else {
            None
        }
    }
}

pub struct FlipToAttack {}
impl CustomEffect for FlipToAttack {
    fn identifier() -> String {
        "FLIP_TO_ATTACK".into()
    }

    fn on_attempt_to_attack(&self, effect: &Effect, in_play: &InPlayCard, _engine: &GameEngine) -> Option<AttackBuilder> {
        let this_pokemon = effect.target.is_in_play(in_play);

        if this_pokemon {
            AttackBuilder::new()
                .flip_a_coin()
                .if_tails(|e| e.prevent())
                .into()
        } else {
            None
        }
    }
}
