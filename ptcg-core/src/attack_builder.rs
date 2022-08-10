use crate::engine::*;
use crate::state::*;

pub struct AttackBuilder<'a> {
    engine: GameEngine,
    dm: &'a mut dyn DecisionMaker,
    flips: Vec<Flips>,

    attack_cost: Vec<Type>,
    failed: bool,
    results: Vec<ActionResult>,
    original: GameEngine,
}

impl<'a> AttackBuilder<'a> {
    pub fn new(engine: GameEngine, dm: &'a mut dyn DecisionMaker) -> Self {
        let original = engine.clone();
        AttackBuilder {
            engine,
            dm,
            flips: vec![],

            attack_cost: vec![],
            results: vec![],
            failed: false,
            original,
        }
    }

    pub fn failed(&self) -> bool {
        self.failed
    }

    pub fn player(&self) -> Player {
        self.engine.player()
    }

    pub fn opponent(&self) -> Player {
        self.engine.opponent()
    }

    pub fn attacking(&self) -> &InPlayCard {
        self.engine.attacking()
    }

    pub fn defending(&self) -> &InPlayCard {
        self.engine.defending()
    }

    pub fn attack_cost(mut self, energy_requirements: &[Type]) -> Self {
        self.attack_cost = energy_requirements.iter().cloned().collect();

        if !self.engine.is_attack_energy_cost_met(self.engine.attacking(), energy_requirements) {
            self.failed = true;
        }
        self
    }

    pub fn flip_a_coin(mut self) -> Self {
        self.flips.push(self.dm.flip(1));
        self
    }

    pub fn flip_coins(mut self, how_many: usize) -> Self {
        self.flips.push(self.dm.flip(how_many));
        self
    }

    pub fn heads(&self) -> usize {
        self.flips.last().unwrap().heads()
    }

    pub fn damage(mut self, damage: usize) -> Self {
        self.engine = self.engine.damage(damage);
        self
    }

    pub fn damage_self(mut self, damage: usize) -> Self {
        self.engine = self.engine.damage_self(damage);
        self
    }

    pub fn if_heads<F>(self, f: F) -> Self where F: Fn(Self) -> Self {
        if self.heads() == 1 {
            f(self)
        } else {
            self
        }
    }

    pub fn if_tails<F>(self, f: F) -> Self where F: Fn(Self) -> Self {
        if self.heads() == 0 {
            f(self)
        } else {
            self
        }
    }

    pub fn then<F>(self, f: F) -> Self where F: Fn(Self) -> Self {
        f(self)
    }

    pub fn must<F>(mut self, f: F) -> Self where F: Fn(Self) -> Self {
        let idx = self.results.len();
        self = f(self);
        if !self.results[idx..].iter().all(|x| match x { ActionResult::Full => true, _ => false }) {
            self.failed = true;
        }
        self
    }

    pub fn defending_must_be_asleep(mut self) -> Self {
        if !self.defending().is_asleep() {
            self.failed = true;
        }
        self
    }

    pub fn each_own_bench<F>(mut self, f: F) -> Self where F: Fn(Self) -> Self {
        for target in self.engine.bench(self.player()) {
            self.engine = self.engine.push_target(self.attacking(), &target);
            self = f(self);
            self.engine = self.engine.pop_target();
        }
        self
    }

    pub fn each_opponents_bench<F>(mut self, f: F) -> Self where F: Fn(Self) -> Self {
        for target in self.engine.bench(self.opponent()) {
            self.engine = self.engine.push_target(self.attacking(), &target);
            self = f(self);
            self.engine = self.engine.pop_target();
        }
        self
    }

    pub fn with_effect(mut self, effect: Effect) -> Self {
        self.engine = self.engine.with_effect(effect);
        self
    }

    pub fn asleep(mut self) -> Self {
        self.engine = self.engine.asleep();
        self
    }

    pub fn confuse(mut self) -> Self {
        self.engine = self.engine.confuse();
        self
    }

    pub fn paralyze(mut self) -> Self {
        self.engine = self.engine.paralyze();
        self
    }

    pub fn poison(mut self) -> Self {
        self.engine = self.engine.poison();
        self
    }

    pub fn severe_poison(mut self, counters: usize) -> Self {
        self.engine = self.engine.severe_poison(counters);
        self
    }

    pub fn discard_defending_energy_cards(mut self, energy_requirements: &[Type]) -> Self {
        let (engine, result) = self.engine.discard_attached_energy_cards(self.player(), &self.engine.defending(), energy_requirements, self.dm);

        self.engine = engine;
        self.results.push(result);
        self
    }

    pub fn discard_all_attacking_energy_cards(mut self) -> Self {
        self.engine = self.engine.discard_all_attached_energy_cards(self.player(), &self.engine.attacking(), self.dm);
        self
    }

    pub fn discard_attacking_energy_cards(mut self, energy_requirements: &[Type]) -> Self {
        let (engine, result) = self.engine.discard_attached_energy_cards(self.player(), &self.engine.attacking(), energy_requirements, self.dm);

        self.engine = engine;
        self.results.push(result);
        self
    }

    pub fn heal_all_attacking(mut self) -> Self {
        self.engine = self.engine.heal_all(self.attacking());
        self
    }

    pub fn damage_per_heads(mut self, damage: usize) -> Self {
        self.engine = self.engine.damage(damage * self.heads());
        self
    }

    pub fn damage_plus_per_energy_card_on_defending(mut self, base_damage: usize, plus: usize) -> Self {
        self.engine = self.engine.damage(base_damage.saturating_add(self.defending().attached.iter().filter(|c| self.engine.is_energy(c.card())).count() * plus));
        self
    }

    pub fn damage_plus_per_damage_counter_on_defending(mut self, base_damage: usize, plus: usize) -> Self {
        self.engine = self.engine.damage(base_damage.saturating_add(self.engine.damage_counters_on(self.defending()) * plus));
        self
    }

    pub fn damage_per_damage_counter_on_itself(mut self, damage_per_counter: usize) -> Self {
        self.engine = self.engine.damage(self.engine.damage_counters_on(self.attacking()) * damage_per_counter);
        self
    }
    pub fn damage_minus_per_damage_counter_on_itself(mut self, base_damage: usize, minus: usize) -> Self {
        self.engine = self.engine.damage(base_damage.saturating_sub(self.engine.damage_counters_on(self.attacking()) * minus));
        self
    }

    pub fn damage_half_defending_remaining_hp(mut self) -> Self {
        self.engine = self.engine.damage(self.engine.remaining_hp(self.defending()).div_ceil(2));
        self
    }

    pub fn damage_plus_per_extra_energy_on_attacking(mut self, damage: usize, per_energy: usize, energy_type: Type, energy_limit: usize) -> Self {
        let mut additional = 0;
        let mut requirements = self.attack_cost.clone();
        while additional < energy_limit {
            requirements.push(energy_type.clone());
            if !self.engine.is_attack_energy_cost_met(self.attacking(), &requirements) {
                break;
            }
            additional += 1;
        }

        self.engine = self.engine.damage(damage.saturating_add(additional * per_energy));
        self
    }

    pub fn prevent_damage_during_opponents_next_turn(mut self) -> Self {
        self.engine = self.engine.with_effect(Effect {
            name: "NO_DAMAGE_DURING_OPPONENTS_NEXT_TURN".into(),
            source: EffectSource::Attack(self.player(), self.attacking().id),
            target: EffectTarget::InPlay(self.player(), self.attacking().id),
            consequence: EffectConsequence::BlockDamage,
            expires: EffectExpiration::EndOfTurn(self.opponent(), 0),
            system: false,
        });
        self
    }

    pub fn prevent_trainers_during_opponents_next_turn(mut self) -> Self {
        self.engine = self.engine.with_effect(Effect {
            name: "PSYDUCK_FO_HEADACHE_NO_TRAINERS".into(),
            source: EffectSource::Attack(self.player(), self.attacking().id),
            target: EffectTarget::Player(self.opponent()),
            consequence: EffectConsequence::BlockTrainerFromHand,
            expires: EffectExpiration::EndOfTurn(self.opponent(), 0),
            system: false,
        });
        self
    }

    pub fn knock_out_attacker_if_attacking_is_knocked_out_next_turn(self) -> Self {
        // self.engine = self.engine.with_effect(Effect {
        //     name: "KNOCK_OUT_IF_WE_ARE_KNOCKED_OUT".into(),
        //     source: EffectSource::Attack(self.player(), self.attacking().id),
        //     target: EffectTarget::InPlay(self.player(), self.attacking().id),
        //     consequence: TODO
        //     expires: EffectExpiration::EndOfTurn(self.opponent(), 0)
        //     system: false,
        // });
        self
    }
}

impl<'a> From<&AttackBuilder<'a>> for GameEngine {
    fn from(ab: &AttackBuilder<'a>) -> Self {
        if ab.failed {
            ab.original.clone()
        } else {
            ab.engine.clone()
        }
    }
}
