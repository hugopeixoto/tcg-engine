use crate::engine::*;
use crate::state::*;
use crate::effect;
use crate::custom_effects;

pub struct AttackBuilderContext<'a> {
    pub engine: GameEngine,
    dm: &'a mut dyn DecisionMaker,
    flips: Vec<Flips>,

    attack_cost: Vec<Type>,
    prevented: bool,
    failed: bool,
    damage_done: usize,
    results: Vec<ActionResult>,
    original: GameEngine,
}

impl<'a> AttackBuilderContext<'a> {
    pub fn new(engine: GameEngine, dm: &'a mut dyn DecisionMaker) -> Self {
        let original = engine.clone();
        AttackBuilderContext {
            engine,
            dm,
            flips: vec![],

            attack_cost: vec![],
            results: vec![],
            prevented: false,
            failed: false,
            damage_done: 0,
            original,
        }
    }

    pub fn failed(&self) -> bool {
        self.failed
    }

    pub fn prevented(&self) -> bool {
        self.prevented
    }

    pub fn heads(&self) -> usize {
        self.flips.last().unwrap().heads()
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

    pub fn engine(&self) -> GameEngine {
        if self.failed {
            self.original.clone()
        } else {
            self.engine.clone()
        }
    }
}

pub struct AttackBuilder {
    operations: Vec<Box<dyn Fn(AttackBuilderContext) -> AttackBuilderContext>>,
}

impl AttackBuilder {
    pub fn new() -> Self {
        Self {
            operations: vec![],
        }
    }

    pub fn apply<'a>(&self, engine: GameEngine, dm: &'a mut dyn DecisionMaker) -> AttackBuilderContext<'a> {
        let mut ctx = AttackBuilderContext::new(engine, dm);

        for operation in self.operations.iter() {
            ctx = operation(ctx);
        }

        ctx
    }

    pub fn chain(mut self, other: AttackBuilder) -> Self {
        self.operations.extend(other.operations);
        self
    }

    // fluent api

    pub fn prevent(mut self) -> Self {
        self.operations.push(Box::new(move |mut builder| {
            builder.prevented = true;
            builder
        }));
        self
    }

    pub fn attack_cost(mut self, energy_requirements: &[Type]) -> Self {
        let cost = energy_requirements.iter().cloned().collect::<Vec<_>>();

        self.operations.push(Box::new(move |mut builder| {
            builder.attack_cost = cost.clone();

            if !builder.engine.is_attack_energy_cost_met(builder.engine.attacking(), &builder.attack_cost) {
                builder.failed = true;
            }
            builder
        }));

        self
    }

    pub fn flip_a_coin(mut self) -> Self {
        self.operations.push(Box::new(|mut builder| {
            builder.flips.push(builder.dm.flip(1));
            builder
        }));
        self
    }

    pub fn flip_coins(mut self, how_many: usize) -> Self {
        self.operations.push(Box::new(move |mut builder| {
            builder.flips.push(builder.dm.flip(how_many));
            builder
        }));
        self
    }

    pub fn damage(mut self, damage: usize) -> Self {
        self.operations.push(Box::new(move |mut builder| {
            (builder.engine, builder.damage_done) = builder.engine.damage(damage);
            builder
        }));
        self
    }

    pub fn damage_self(mut self, damage: usize) -> Self {
        self.operations.push(Box::new(move |mut builder| {
            (builder.engine, _) = builder.engine.damage_self(damage);
            builder
        }));
        self
    }

    fn wrap<F>(builder: AttackBuilderContext, f: F) -> AttackBuilderContext where F: Fn(Self) -> Self {
        let mut b2 = Self::new();
        b2 = f(b2);
        b2.apply(builder.engine, builder.dm)
    }

    pub fn if_heads<F>(mut self, f: F) -> Self where F: Fn(Self) -> Self + 'static {
        self.operations.push(Box::new(move |builder| {
            if builder.heads() == 1 {
                Self::wrap(builder, &f)
            } else {
                builder
            }
        }));
        self
    }

    pub fn if_tails<F>(mut self, f: F) -> Self where F: Fn(Self) -> Self + 'static {
        self.operations.push(Box::new(move |builder| {
            if builder.heads() == 0 {
                Self::wrap(builder, &f)
            } else {
                builder
            }
        }));
        self
    }

    pub fn if_did_damage<F>(mut self, f: F) -> Self where F: Fn(Self) -> Self + 'static {
        self.operations.push(Box::new(move |builder| {
            if builder.damage_done > 0 {
                Self::wrap(builder, &f)
            } else {
                builder
            }
        }));
        self
    }

    pub fn then<F>(mut self, f: F) -> Self where F: Fn(Self) -> Self + 'static {
        self.operations.push(Box::new(move |builder| {
            Self::wrap(builder, &f)
        }));
        self
    }

    pub fn must<F>(mut self, f: F) -> Self where F: Fn(Self) -> Self + 'static {
        self.operations.push(Box::new(move |mut builder| {
            let idx = builder.results.len();
            builder = Self::wrap(builder, &f);
            if !builder.results[idx..].iter().all(|x| match x { ActionResult::Full => true, _ => false }) {
                builder.failed = true;
            }
            builder
        }));
        self
    }

    pub fn defending_must_be_asleep(mut self) -> Self {
        self.operations.push(Box::new(move |mut builder| {
            if !builder.defending().is_asleep() {
                builder.failed = true;
            }
            builder
        }));
        self
    }

    pub fn each_own_bench<F>(mut self, f: F) -> Self where F: Fn(Self) -> Self + 'static {
        self.operations.push(Box::new(move |mut builder| {
            for target in builder.engine.bench(builder.player()) {
                builder.engine = builder.engine.push_target(builder.attacking(), &target);
                builder = Self::wrap(builder, &f);
                builder.engine = builder.engine.pop_target();
            }
            builder
        }));
        self
    }

    pub fn each_opponents_bench<F>(mut self, f: F) -> Self where F: Fn(Self) -> Self + 'static {
        self.operations.push(Box::new(move |mut builder| {
            for target in builder.engine.bench(builder.opponent()) {
                builder.engine = builder.engine.push_target(builder.attacking(), &target);
                builder = Self::wrap(builder, &f);
                builder.engine = builder.engine.pop_target();
            }
            builder
        }));
        self
    }

    pub fn with_effect(mut self, effect: Effect) -> Self {
        self.operations.push(Box::new(move |mut builder| {
            builder.engine = builder.engine.with_effect(effect.clone());
            builder
        }));
        self
    }

    pub fn asleep(mut self) -> Self {
        self.operations.push(Box::new(move |mut builder| {
            builder.engine = builder.engine.asleep(builder.engine.defending());
            builder
        }));
        self
    }

    pub fn confuse(mut self) -> Self {
        self.operations.push(Box::new(move |mut builder| {
            builder.engine = builder.engine.confuse(builder.engine.defending());
            builder
        }));
        self
    }

    pub fn paralyze(mut self) -> Self {
        self.operations.push(Box::new(move |mut builder| {
            builder.engine = builder.engine.paralyze(builder.engine.defending());
            builder
        }));
        self
    }

    pub fn poison(mut self) -> Self {
        self.operations.push(Box::new(move |mut builder| {
            builder.engine = builder.engine.poison(builder.engine.defending(), 1);
            builder
        }));
        self
    }

    pub fn severe_poison(mut self, counters: usize) -> Self {
        self.operations.push(Box::new(move |mut builder| {
            builder.engine = builder.engine.poison(builder.engine.defending(), counters);
            builder
        }));
        self
    }

    pub fn discard_defending_energy_cards(mut self, energy_requirements: &[Type]) -> Self {
        let energy_requirements = energy_requirements.iter().cloned().collect::<Vec<_>>();

        self.operations.push(Box::new(move |mut builder| {
            let (engine, result) = builder.engine.discard_attached_energy_cards(builder.player(), &builder.engine.defending(), &energy_requirements, builder.dm);

            builder.engine = engine;
            builder.results.push(result);
            builder
        }));
        self
    }

    pub fn discard_all_attacking_energy_cards(mut self) -> Self {
        self.operations.push(Box::new(move |mut builder| {
            builder.engine = builder.engine.discard_all_attached_energy_cards(builder.player(), &builder.engine.attacking(), builder.dm);
            builder
        }));
        self
    }

    pub fn discard_attacking_energy_cards(mut self, energy_requirements: &[Type]) -> Self {
        let energy_requirements = energy_requirements.iter().cloned().collect::<Vec<_>>();

        self.operations.push(Box::new(move |mut builder| {
            let (engine, result) = builder.engine.discard_attached_energy_cards(builder.player(), &builder.engine.attacking(), &energy_requirements, builder.dm);

            builder.engine = engine;
            builder.results.push(result);
            builder
        }));
        self
    }

    pub fn heal_all_attacking(mut self) -> Self {
        self.operations.push(Box::new(move |mut builder| {
            builder.engine = builder.engine.heal_all(builder.attacking());
            builder
        }));
        self
    }

    pub fn heal_attacking(mut self, damage: usize) -> Self {
        self.operations.push(Box::new(move |mut builder| {
            builder.engine = builder.engine.heal(builder.attacking(), damage);
            builder
        }));
        self
    }

    pub fn damage_per_heads(mut self, damage_per_heads: usize) -> Self {
        self.operations.push(Box::new(move |mut builder| {
            let damage = damage_per_heads * builder.heads();
            (builder.engine, builder.damage_done) = builder.engine.damage(damage);
            builder
        }));
        self
    }

    pub fn damage_plus_per_energy_card_on_defending(mut self, base_damage: usize, damage_per_energy_card: usize) -> Self {
        self.operations.push(Box::new(move |mut builder| {
            let energy_cards = builder.defending().attached.iter().filter(|c| builder.engine.is_energy(c.card())).count();
            let damage = base_damage + damage_per_energy_card * energy_cards;
            (builder.engine, builder.damage_done) = builder.engine.damage(damage);
            builder
        }));
        self
    }

    pub fn damage_plus_per_damage_counter_on_defending(mut self, base_damage: usize, damage_per_counter: usize) -> Self {
        self.operations.push(Box::new(move |mut builder| {
            let damage_counters = builder.engine.damage_counters_on(builder.defending());
            let damage = base_damage + damage_per_counter * damage_counters;
            (builder.engine, builder.damage_done) = builder.engine.damage(damage);
            builder
        }));
        self
    }

    pub fn damage_per_damage_counter_on_itself(mut self, damage_per_counter: usize) -> Self {
        self.operations.push(Box::new(move |mut builder| {
            let damage_counters = builder.engine.damage_counters_on(builder.attacking());
            let damage = damage_per_counter * damage_counters;
            (builder.engine, builder.damage_done) = builder.engine.damage(damage);
            builder
        }));
        self
    }
    pub fn damage_minus_per_damage_counter_on_itself(mut self, base_damage: usize, damage_per_counter: usize) -> Self {
        self.operations.push(Box::new(move |mut builder| {
            let damage_counters = builder.engine.damage_counters_on(builder.attacking());
            let damage = base_damage.saturating_sub(damage_counters * damage_per_counter);
            (builder.engine, builder.damage_done) = builder.engine.damage(damage);
            builder
        }));
        self
    }

    pub fn damage_half_defending_remaining_hp(mut self) -> Self {
        self.operations.push(Box::new(move |mut builder| {
            let remaining_hp = builder.engine.remaining_hp(builder.defending());
            let damage = remaining_hp.div_ceil(2);
            (builder.engine, builder.damage_done) = builder.engine.damage(damage);
            builder
        }));
        self
    }

    pub fn damage_plus_per_extra_energy_on_attacking(mut self, base_damage: usize, per_energy: usize, energy_type: Type, energy_limit: usize) -> Self {
        self.operations.push(Box::new(move |mut builder| {
            let mut additional = 0;
            let mut requirements = builder.attack_cost.clone();
            while additional < energy_limit {
                requirements.push(energy_type.clone());
                if !builder.engine.is_attack_energy_cost_met(builder.attacking(), &requirements) {
                    break;
                }
                additional += 1;
            }

            let damage = base_damage + additional * per_energy;
            (builder.engine, builder.damage_done) = builder.engine.damage(damage);
            builder
        }));
        self
    }

    pub fn switch_defending(mut self) -> Self {
        self.operations.push(Box::new(move |mut builder| {
            builder.engine = builder.engine.switch(builder.opponent(), builder.dm);
            builder
        }));
        self
    }

    pub fn gust_defending(mut self) -> Self {
        self.operations.push(Box::new(move |mut builder| {
            builder.engine = builder.engine.gust(builder.player(), builder.dm);
            builder
        }));
        self
    }

    pub fn knock_out_attacking(mut self) -> Self {
        self.operations.push(Box::new(move |mut builder| {
            builder.engine = builder.engine.knock_out(&builder.attacking().clone(), builder.dm);
            builder
        }));
        self
    }

    pub fn draw(mut self, how_many: usize) -> Self {
        self.operations.push(Box::new(move |mut builder| {
            builder.engine = builder.engine.draw(builder.engine.player(), how_many, builder.dm);
            builder
        }));
        self
    }

    pub fn change_attacking_resistance_except(mut self, exceptions: &[Type]) -> Self {
        let except_types = exceptions.iter().cloned().collect::<Vec<_>>();

        self.operations.push(Box::new(move |builder| {
            let mut types = builder.engine.format.available_types();
            types.retain(|t| !except_types.contains(t));
            let chosen_type = builder.dm.pick_type(builder.player(), &types);

            let effect = effect::from_attack()
                .on_attacking()
                .while_active()
                .type_parameter(chosen_type.clone())
                .custom_effect::<custom_effects::ChangeResistance>();

            effect.apply(builder)
        }));
        self
    }

    pub fn change_defending_weakness_except(mut self, exceptions: &[Type]) -> Self {
        let except_types = exceptions.iter().cloned().collect::<Vec<_>>();

        self.operations.push(Box::new(move |builder| {
            let mut types = builder.engine.format.available_types();
            types.retain(|t| !except_types.contains(t));
            let chosen_type = builder.dm.pick_type(builder.player(), &types);

            let effect = effect::from_attack()
                .on_defending()
                .while_active()
                .type_parameter(chosen_type.clone())
                .custom_effect::<custom_effects::ChangeWeakness>();

            effect.apply(builder)
        }));
        self
    }

    pub fn disable_defending_attack(mut self) -> Self {
        self.operations.push(Box::new(move |builder| {
            let attacks = builder.engine.attacks(builder.defending());

            let chosen = builder.dm.pick_action(builder.player(), &attacks);
            let chosen = match chosen {
                Action::Attack(_, _, name, _) => name,
                _ => { panic!("Picked an action that isn't an attack: {:?}", chosen); }
            };

            let effect = effect::from_attack()
                .on_defending()
                .until_opponents_end_of_turn()
                .string_parameter(chosen.clone())
                .custom_effect::<custom_effects::DisableAttack>();

            effect.apply(builder)
        }));

        self
    }

    pub fn prevent_damage_during_opponents_next_turn(mut self) -> Self {
        let effect = effect::from_attack()
            .on_attacking()
            .until_opponents_end_of_turn()
            .custom_effect::<custom_effects::PreventDamageDuringOpponentsTurn>();

        self.operations.push(Box::new(move |builder| effect.apply(builder)));
        self
    }

    pub fn prevent_up_to_damage_during_opponents_next_turn(mut self, up_to: usize) -> Self {
        let effect = effect::from_attack()
            .on_attacking()
            .until_opponents_end_of_turn()
            .usize_parameter(up_to)
            .custom_effect::<custom_effects::PreventUpToDamageDuringOpponentsTurn>();

        self.operations.push(Box::new(move |builder| effect.apply(builder)));
        self
    }

    pub fn prevent_damage_and_effects_during_opponents_next_turn(mut self) -> Self {
        let effect = effect::from_attack()
            .on_attacking()
            .until_opponents_end_of_turn()
            .custom_effect::<custom_effects::PreventDamageAndEffectsDuringOpponentsTurn>();

        self.operations.push(Box::new(move |builder| effect.apply(builder)));
        self
    }

    pub fn prevent_trainers_during_opponents_next_turn(mut self) -> Self {
        let effect = effect::from_attack()
            .on_defending()
            .until_opponents_end_of_turn()
            .custom_effect::<custom_effects::BlockTrainerFromHand>();

        self.operations.push(Box::new(move |builder| effect.apply(builder)));
        self
    }

    pub fn knock_out_attacker_if_attacking_is_knocked_out_next_turn(mut self) -> Self {
        let effect = effect::from_attack()
            .on_attacking()
            .until_opponents_end_of_turn()
            .custom_effect::<custom_effects::RevengeKnockOut>();

        self.operations.push(Box::new(move |builder| effect.apply(builder)));
        self
    }

    pub fn prevent_attack_on_a_flip_during_opponents_next_turn(mut self) -> Self {
        let effect = effect::from_attack()
            .on_defending()
            .until_opponents_end_of_turn()
            .custom_effect::<custom_effects::FlipToAttack>();

        self.operations.push(Box::new(move |builder| effect.apply(builder)));
        self
    }

    pub fn once_while_in_play(mut self) -> Self {
        self.operations.push(Box::new(move |builder| {
            let effect = effect::from_attack()
                .on_attacking()
                .while_in_play()
                .string_parameter(builder.engine.current_attack_name().unwrap())
                .custom_effect::<custom_effects::DisableAttack>();

            effect.apply(builder)
        }));

        self
    }
}
