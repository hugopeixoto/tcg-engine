use crate::engine::*;
use crate::state::*;
use crate::effect;
use crate::custom_effects;

#[derive(PartialEq, Eq)]
enum Costs {
    All,
    IgnoreAllCosts,
    IgnoreEnergyCost,
    OnlyRequirements,
}

#[derive(PartialEq, Eq)]
enum OperationType {
    Normal,
    EnergyCost,
    OtherCost,
    OtherRequirement,
}

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

    pub fn this_pokemon(&self) -> &InPlayCard {
        self.engine.this_pokemon()
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

struct Operation {
    body: Box<dyn Fn(AttackBuilderContext) -> AttackBuilderContext>,
    optype: OperationType,
}

impl From<Box<dyn Fn(AttackBuilderContext) -> AttackBuilderContext>> for Operation {
    fn from(body: Box<dyn Fn(AttackBuilderContext) -> AttackBuilderContext>) -> Self {
        Self { body, optype: OperationType::Normal }
    }
}

pub struct AttackBuilder {
    operations: Vec<Operation>,
    costs: Costs,
}

impl AttackBuilder {
    pub fn new() -> Self {
        Self {
            operations: vec![],
            costs: Costs::All,
        }
    }

    pub fn apply<'a>(&self, engine: GameEngine, dm: &'a mut dyn DecisionMaker) -> AttackBuilderContext<'a> {
        self.chain(AttackBuilderContext::new(engine, dm))
    }

    pub fn chain<'a>(&self, mut ctx: AttackBuilderContext<'a>) -> AttackBuilderContext<'a> {
        for operation in self.operations.iter() {
            let run_this_operation = match self.costs {
                Costs::All => true,
                Costs::IgnoreAllCosts => operation.optype == OperationType::Normal,
                Costs::IgnoreEnergyCost => operation.optype != OperationType::EnergyCost,
                Costs::OnlyRequirements => operation.optype != OperationType::Normal,
            };

            if run_this_operation {
                ctx = (operation.body)(ctx);
            }
        }

        ctx
    }

    pub fn ignore_all_costs(mut self) -> Self {
        self.costs = Costs::IgnoreAllCosts;
        self
    }

    pub fn ignore_energy_cost(mut self) -> Self {
        self.costs = Costs::IgnoreEnergyCost;
        self
    }

    pub fn only_requirements(mut self) -> Self {
        self.costs = Costs::OnlyRequirements;
        self
    }

    pub fn add_operation<F>(mut self, body: F) -> Self where F: Fn(AttackBuilderContext) -> AttackBuilderContext + 'static {
        self.operations.push(Operation { body: Box::new(body), optype: OperationType::Normal });
        self
    }

    pub fn add_energy_cost_operation<F>(mut self, body: F) -> Self where F: Fn(AttackBuilderContext) -> AttackBuilderContext + 'static {
        self.operations.push(Operation { body: Box::new(body), optype: OperationType::EnergyCost });
        self
    }

    pub fn add_cost_operation<F>(mut self, body: F) -> Self where F: Fn(AttackBuilderContext) -> AttackBuilderContext + 'static {
        self.operations.push(Operation { body: Box::new(body), optype: OperationType::OtherCost });
        self
    }

    pub fn add_requirement_operation<F>(mut self, body: F) -> Self where F: Fn(AttackBuilderContext) -> AttackBuilderContext + 'static {
        self.operations.push(Operation { body: Box::new(body), optype: OperationType::OtherRequirement });
        self
    }

    // fluent api
    pub fn prevent(self) -> Self {
        self.add_operation(move |mut builder: AttackBuilderContext| {
            builder.prevented = true;
            builder
        })
    }

    pub fn attack_cost(self, energy_requirements: &[Type]) -> Self {
        let cost = energy_requirements.iter().cloned().collect::<Vec<_>>();

        self.add_energy_cost_operation(move |mut builder| {
            builder.attack_cost = cost.clone();

            if !builder.engine.is_attack_energy_cost_met(builder.engine.attacking(), &builder.attack_cost) {
                builder.failed = true;
            }
            builder
        })
    }

    pub fn flip_a_coin(self) -> Self {
        self.add_operation(|mut builder| {
            builder.flips.push(builder.dm.flip(1));
            builder
        })
    }

    pub fn flip_coins(self, how_many: usize) -> Self {
        self.add_operation(move |mut builder| {
            builder.flips.push(builder.dm.flip(how_many));
            builder
        })
    }

    pub fn damage(self, damage: usize) -> Self {
        self.add_operation(move |mut builder| {
            (builder.engine, builder.damage_done) = builder.engine.damage(damage);
            builder
        })
    }

    pub fn damage_self(self, damage: usize) -> Self {
        self.add_operation(move |mut builder| {
            (builder.engine, _) = builder.engine.damage_self(damage);
            builder
        })
    }

    fn wrap<F>(builder: AttackBuilderContext, f: F) -> AttackBuilderContext where F: Fn(Self) -> Self {
        f(Self::new()).chain(builder)
    }

    pub fn if_heads<F>(self, f: F) -> Self where F: Fn(Self) -> Self + 'static {
        self.add_operation(move |builder| {
            if builder.heads() == 1 {
                Self::wrap(builder, &f)
            } else {
                builder
            }
        })
    }

    pub fn if_tails<F>(self, f: F) -> Self where F: Fn(Self) -> Self + 'static {
        self.add_operation(move |builder| {
            if builder.heads() == 0 {
                Self::wrap(builder, &f)
            } else {
                builder
            }
        })
    }

    pub fn if_did_damage<F>(self, f: F) -> Self where F: Fn(Self) -> Self + 'static {
        self.add_operation(move |builder| {
            if builder.damage_done > 0 {
                Self::wrap(builder, &f)
            } else {
                builder
            }
        })
    }

    pub fn then<F>(self, f: F) -> Self where F: Fn(Self) -> Self + 'static {
        self.add_operation(move |builder| {
            Self::wrap(builder, &f)
        })
    }

    pub fn cost<F>(self, f: F) -> Self where F: Fn(Self) -> Self + 'static {
        self.add_cost_operation(move |mut builder| {
            let idx = builder.results.len();
            builder = Self::wrap(builder, &f);
            if !builder.results[idx..].iter().all(|x| match x { ActionResult::Full => true, _ => false }) {
                builder.failed = true;
            }
            builder
        })
    }

    pub fn must<F>(self, f: F) -> Self where F: Fn(Self) -> Self + 'static {
        self.add_requirement_operation(move |mut builder| {
            let idx = builder.results.len();
            builder = Self::wrap(builder, &f);
            if !builder.results[idx..].iter().all(|x| match x { ActionResult::Full => true, _ => false }) {
                builder.failed = true;
            }
            builder
        })
    }

    pub fn defending_must_be_asleep(self) -> Self {
        self.add_operation(move |mut builder| {
            if !builder.defending().is_asleep() {
                builder.failed = true;
            }
            builder
        })
    }

    pub fn each_own_bench<F>(self, f: F) -> Self where F: Fn(Self) -> Self + 'static {
        self.add_operation(move |mut builder| {
            for target in builder.engine.bench(builder.player()) {
                builder.engine = builder.engine.push_target(builder.attacking(), &target);
                builder = Self::wrap(builder, &f);
                builder.engine = builder.engine.pop_target();
            }
            builder
        })
    }

    pub fn each_opponents_bench<F>(self, f: F) -> Self where F: Fn(Self) -> Self + 'static {
        self.add_operation(move |mut builder| {
            for target in builder.engine.bench(builder.opponent()) {
                builder.engine = builder.engine.push_target(builder.attacking(), &target);
                builder = Self::wrap(builder, &f);
                builder.engine = builder.engine.pop_target();
            }
            builder
        })
    }

    pub fn with_effect(self, effect: Effect) -> Self {
        self.add_operation(move |mut builder| {
            builder.engine = builder.engine.with_effect(effect.clone());
            builder
        })
    }

    pub fn asleep(self) -> Self {
        self.add_operation(move |mut builder| {
            builder.engine = builder.engine.asleep(builder.engine.defending());
            builder
        })
    }

    pub fn confuse(self) -> Self {
        self.add_operation(move |mut builder| {
            builder.engine = builder.engine.confuse(builder.engine.defending());
            builder
        })
    }

    pub fn paralyze(self) -> Self {
        self.add_operation(move |mut builder| {
            builder.engine = builder.engine.paralyze(builder.engine.defending());
            builder
        })
    }

    pub fn poison(self) -> Self {
        self.add_operation(move |mut builder| {
            builder.engine = builder.engine.poison(builder.engine.defending(), 1);
            builder
        })
    }

    pub fn severe_poison(self, counters: usize) -> Self {
        self.add_operation(move |mut builder| {
            builder.engine = builder.engine.poison(builder.engine.defending(), counters);
            builder
        })
    }

    pub fn discard_defending_energy_cards(self, energy_requirements: &[Type]) -> Self {
        let energy_requirements = energy_requirements.iter().cloned().collect::<Vec<_>>();

        self.add_operation(move |mut builder| {
            let (engine, result) = builder.engine.discard_attached_energy_cards(builder.player(), &builder.engine.defending(), &energy_requirements, builder.dm);

            builder.engine = engine;
            builder.results.push(result);
            builder
        })
    }

    pub fn discard_all_attacking_energy_cards(self) -> Self {
        self.add_operation(move |mut builder| {
            builder.engine = builder.engine.discard_all_attached_energy_cards(builder.player(), &builder.engine.attacking(), builder.dm);
            builder
        })
    }

    pub fn discard_attacking_energy_cards(self, energy_requirements: &[Type]) -> Self {
        let energy_requirements = energy_requirements.iter().cloned().collect::<Vec<_>>();

        self.add_operation(move |mut builder| {
            let (engine, result) = builder.engine.discard_attached_energy_cards(builder.player(), &builder.engine.attacking(), &energy_requirements, builder.dm);

            builder.engine = engine;
            builder.results.push(result);
            builder
        })
    }

    pub fn heal_all_attacking(self) -> Self {
        self.add_operation(move |mut builder| {
            builder.engine = builder.engine.heal_all(builder.attacking());
            builder
        })
    }

    pub fn heal_attacking(self, damage: usize) -> Self {
        self.add_operation(move |mut builder| {
            builder.engine = builder.engine.heal(builder.attacking(), damage);
            builder
        })
    }

    pub fn damage_per_heads(self, damage_per_heads: usize) -> Self {
        self.add_operation(move |mut builder| {
            let damage = damage_per_heads * builder.heads();
            (builder.engine, builder.damage_done) = builder.engine.damage(damage);
            builder
        })
    }

    pub fn damage_plus_per_energy_card_on_defending(self, base_damage: usize, damage_per_energy_card: usize) -> Self {
        self.add_operation(move |mut builder| {
            let energy_cards = builder.defending().attached.iter().filter(|c| builder.engine.is_energy(c.card())).count();
            let damage = base_damage + damage_per_energy_card * energy_cards;
            (builder.engine, builder.damage_done) = builder.engine.damage(damage);
            builder
        })
    }

    pub fn damage_plus_per_damage_counter_on_defending(self, base_damage: usize, damage_per_counter: usize) -> Self {
        self.add_operation(move |mut builder| {
            let damage_counters = builder.engine.damage_counters_on(builder.defending());
            let damage = base_damage + damage_per_counter * damage_counters;
            (builder.engine, builder.damage_done) = builder.engine.damage(damage);
            builder
        })
    }

    pub fn damage_per_damage_counter_on_itself(self, damage_per_counter: usize) -> Self {
        self.add_operation(move |mut builder| {
            let damage_counters = builder.engine.damage_counters_on(builder.attacking());
            let damage = damage_per_counter * damage_counters;
            (builder.engine, builder.damage_done) = builder.engine.damage(damage);
            builder
        })
    }

    pub fn damage_minus_per_damage_counter_on_itself(self, base_damage: usize, damage_per_counter: usize) -> Self {
        self.add_operation(move |mut builder| {
            let damage_counters = builder.engine.damage_counters_on(builder.attacking());
            let damage = base_damage.saturating_sub(damage_counters * damage_per_counter);
            (builder.engine, builder.damage_done) = builder.engine.damage(damage);
            builder
        })
    }

    pub fn damage_half_defending_remaining_hp(self) -> Self {
        self.add_operation(move |mut builder| {
            let remaining_hp = builder.engine.remaining_hp(builder.defending());
            let damage = remaining_hp.div_ceil(2);
            (builder.engine, builder.damage_done) = builder.engine.damage(damage);
            builder
        })
    }

    pub fn damage_plus_per_extra_energy_on_attacking(self, base_damage: usize, per_energy: usize, energy_type: Type, energy_limit: usize) -> Self {
        self.add_operation(move |mut builder| {
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
        })
    }

    pub fn switch_defending(self) -> Self {
        self.add_operation(move |mut builder| {
            builder.engine = builder.engine.switch(builder.opponent(), builder.dm);
            builder
        })
    }

    pub fn gust_defending(self) -> Self {
        self.add_operation(move |mut builder| {
            builder.engine = builder.engine.gust(builder.player(), builder.dm);
            builder
        })
    }

    pub fn knock_out_attacking(self) -> Self {
        self.add_operation(move |mut builder| {
            builder.engine = builder.engine.knock_out(&builder.attacking().clone(), builder.dm);
            builder
        })
    }

    pub fn draw(self, how_many: usize) -> Self {
        self.add_operation(move |mut builder| {
            builder.engine = builder.engine.draw(builder.engine.player(), how_many, builder.dm);
            builder
        })
    }

    pub fn change_attacking_resistance_except(self, exceptions: &[Type]) -> Self {
        let except_types = exceptions.iter().cloned().collect::<Vec<_>>();

        self.add_operation(move |builder| {
            let mut types = builder.engine.format.available_types();
            types.retain(|t| !except_types.contains(t));
            let chosen_type = builder.dm.pick_type(builder.player(), &types);

            let effect = effect::from_attack()
                .on_attacking()
                .while_active()
                .type_parameter(chosen_type.clone())
                .custom_effect::<custom_effects::ChangeResistance>();

            effect.apply(builder)
        })
    }

    pub fn change_defending_weakness_except(self, exceptions: &[Type]) -> Self {
        let except_types = exceptions.iter().cloned().collect::<Vec<_>>();

        self.add_operation(move |builder| {
            let mut types = builder.engine.format.available_types();
            types.retain(|t| !except_types.contains(t));
            let chosen_type = builder.dm.pick_type(builder.player(), &types);

            let effect = effect::from_attack()
                .on_defending()
                .while_active()
                .type_parameter(chosen_type.clone())
                .custom_effect::<custom_effects::ChangeWeakness>();

            effect.apply(builder)
        })
    }

    pub fn disable_defending_attack(self) -> Self {
        self.add_operation(move |builder| {
            let attacks = builder.engine.attacks(builder.defending());

            let chosen = builder.dm.pick_attack(builder.player(), &attacks);

            let effect = effect::from_attack()
                .on_defending()
                .until_opponents_end_of_turn()
                .string_parameter(chosen.name().clone())
                .custom_effect::<custom_effects::DisableAttack>();

            effect.apply(builder)
        })
    }

    pub fn copy_defending_attack_without_costs(self) -> Self {
        self.add_operation(move |builder| {
            let attacks = builder.engine.attacks(builder.defending());
            let chosen = builder.dm.pick_attack(builder.player(), &attacks);
            let subbuilder = chosen.build();

            subbuilder.ignore_all_costs().chain(builder)
        })
    }

    pub fn disabled_under_special_conditions(self) -> Self {
        self.add_operation(move |mut builder| {
            if !builder.engine.poke_power_affected_by_special_condition(builder.this_pokemon()) {
                builder.failed = true;
            }
            builder
        })
    }

    pub fn move_own_damage_counter_without_ko(self) -> Self {
        self.add_operation(move |mut builder| {
            let possibilities = builder.engine
                .move_damage_counter_possibilities()
                .into_iter()
                .filter(|(from, to, counters)| {
                    from.owner == builder.player() &&
                    to.owner == builder.player() &&
                    builder.engine.remaining_hp(to) > counters * 10
                })
                .collect::<Vec<_>>();

            if possibilities.is_empty() {
                builder.failed = true;
                return builder;
            }

            let choice = builder.dm.pick_move_damage_counters(builder.player(), &possibilities);
            builder.engine = builder.engine.move_damage_counters(choice.0, choice.1, choice.2);
            builder
        })
    }

    pub fn attach_energy_from_hand(self, energy_type: Type, target_type: Type) -> Self {
        self.add_operation(move |mut builder| {
            let possibilities = builder.engine
                .attach_from_hand_possibilities()
                .into_iter()
                .filter(|(card, target)| {
                    card.owner == builder.player() &&
                    target.owner == builder.player() &&
                    builder.engine.is_basic_energy(card) &&
                    builder.engine.provides(card).contains(&energy_type) &&
                    builder.engine.pokemon_types(target).contains(&target_type)
                })
                .collect::<Vec<_>>();

            if possibilities.is_empty() {
                builder.failed = true;
                return builder;
            }

            let choice = builder.dm.pick_attach_from_hand(builder.player(), &possibilities);
            builder.engine = builder.engine.attach_from_hand(choice.0, choice.1);
            builder
        })
    }

    pub fn energy_burn(self, energy_type: Type) -> Self {
        self.add_operation(move |mut builder| {
            for energy in builder.this_pokemon().attached.clone().iter() {
                builder = effect::from_poke_power()
                    .until_end_of_turn()
                    .on_in_play_card(energy.card())
                    .type_parameter(energy_type.clone())
                    .custom_effect::<custom_effects::EnergyTypeTransform>()
                    .apply(builder);
            }

            builder
        })
    }

    pub fn prevent_damage_during_opponents_next_turn(self) -> Self {
        let effect = effect::from_attack()
            .on_attacking()
            .until_opponents_end_of_turn()
            .custom_effect::<custom_effects::PreventDamageDuringOpponentsTurn>();

        self.add_operation(move |builder| effect.apply(builder))
    }

    pub fn prevent_up_to_damage_during_opponents_next_turn(self, up_to: usize) -> Self {
        let effect = effect::from_attack()
            .on_attacking()
            .until_opponents_end_of_turn()
            .usize_parameter(up_to)
            .custom_effect::<custom_effects::PreventUpToDamageDuringOpponentsTurn>();

        self.add_operation(move |builder| effect.apply(builder))
    }

    pub fn prevent_damage_and_effects_during_opponents_next_turn(self) -> Self {
        let effect = effect::from_attack()
            .on_attacking()
            .until_opponents_end_of_turn()
            .custom_effect::<custom_effects::PreventDamageAndEffectsDuringOpponentsTurn>();

        self.add_operation(move |builder| effect.apply(builder))
    }

    pub fn prevent_trainers_during_opponents_next_turn(self) -> Self {
        let effect = effect::from_attack()
            .on_defending()
            .until_opponents_end_of_turn()
            .custom_effect::<custom_effects::BlockTrainerFromHand>();

        self.add_operation(move |builder| effect.apply(builder))
    }

    pub fn knock_out_attacker_if_attacking_is_knocked_out_next_turn(self) -> Self {
        let effect = effect::from_attack()
            .on_attacking()
            .until_opponents_end_of_turn()
            .custom_effect::<custom_effects::RevengeKnockOut>();

        self.add_operation(move |builder| effect.apply(builder))
    }

    pub fn prevent_attack_on_a_flip_during_opponents_next_turn(self) -> Self {
        let effect = effect::from_attack()
            .on_defending()
            .until_opponents_end_of_turn()
            .custom_effect::<custom_effects::FlipToAttack>();

        self.add_operation(move |builder| effect.apply(builder))
    }

    pub fn once_while_in_play(self) -> Self {
        self.add_operation(move |builder| {
            let effect = effect::from_attack()
                .on_attacking()
                .while_in_play()
                .string_parameter(builder.engine.current_attack_name().unwrap())
                .custom_effect::<custom_effects::DisableAttack>();

            effect.apply(builder)
        })
    }
}
