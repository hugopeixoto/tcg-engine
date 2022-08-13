pub trait CustomEffect {
    fn defending_damage(&self, _damage: usize) -> Option<usize> {
        None
    }

    fn attacking_damage(&self, _damage: usize) -> Option<usize> {
        None
    }

    fn on_affected(&self) -> () {
    }

    fn on_turn_end(&self) -> () {
    }

    fn on_knocked_out(&self) -> () {
    }

    fn on_trainer(&self) -> () {
    }
}

pub struct PreventDamageDuringOpponentsTurn {}
impl CustomEffect for PreventDamageDuringOpponentsTurn {
    fn defending_damage(&self, _damage: usize) -> Option<usize> {
        // TODO: if opponent's turn
        Some(0)
    }
}

pub struct PreventDamageAndEffectsDuringOpponentsTurn {}
impl CustomEffect for PreventDamageAndEffectsDuringOpponentsTurn {
    fn on_affected(&self) -> () {
    }
}
