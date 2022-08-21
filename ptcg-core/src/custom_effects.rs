use crate::attack_builder::AttackBuilder;
use crate::effect::CustomEffect;
use crate::state::{Effect, Player, InPlayCard};
use crate::engine::GameEngine;

// TODO: the attackbuilder builds a bunch of objects, maybe it
// should be stored in the struct and reused here.

pub struct PreventDamageDuringOpponentsTurn {}
impl CustomEffect for PreventDamageDuringOpponentsTurn {
    fn identifier() -> String {
        "PREVENT_DAMAGE_DURING_OPPONENTS_TURN".into()
    }

    fn defending_damage(&self, _damage: usize) -> Option<usize> {
        // TODO: if opponent's turn
        Some(0)
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

    fn defending_damage(&self, _damage: usize) -> Option<usize> {
        // TODO: if opponent's turn
        Some(0)
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
        // effect target is knocked out?
        let this_pokemon_is_kod = effect.target.is_in_play(in_play);
        // there's an attacking pokemon?

        // TODO: Revenge knock out: Sky Return?
        // TODO: Revenge knock out: Quick shooting?
        println!("knocked out? {} && {}", engine.is_someone_attacking(), effect.target.is_player(engine.player()));
        if opponents_turn && this_pokemon_is_kod && engine.is_someone_attacking() {
            AttackBuilder::new()
                .knock_out_attacking()
                .into()
        } else {
            None
        }
    }
}
