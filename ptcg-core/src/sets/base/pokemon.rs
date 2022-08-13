use crate::*;
use crate::attack_builder::AttackBuilder;
use crate::carddb::Attacks;

#[derive(Default)]
pub struct Alakazam1 {}
impl CardArchetype for Alakazam1 {
    identifier!("Alakazam (BS 1)");
    card_name!("Alakazam");
    stage2!("Kadabra");
    hp!(80);
    color!(Psychic);
    weak_to!(Psychic);
    no_resistance!();
    retreat!(3);

    fn card_actions(&self, _player: Player, _card: &Card, _engine: &GameEngine) -> Vec<Action> {
        vec![]
    }
    fn execute(&self, _player: Player, _card: &Card, engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine.clone()
    }
    fn attacks(&self, player: Player, in_play: &InPlayCard, engine: &GameEngine) -> Vec<Action> {
        Attacks::new(player, in_play, engine)
            .register("Confuse Ray", Self::confuse_ray)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Alakazam1 {
    pub fn confuse_ray(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Psychic, Type::Psychic, Type::Psychic])
            .flip_a_coin()
            .damage(30)
            .if_heads(|e| e.confuse())
    }
}

#[derive(Default)]
pub struct Blastoise2 {}
impl CardArchetype for Blastoise2 {
    identifier!("Blastoise (BS 2)");
    card_name!("Blastoise");
    stage2!("Wartortle");
    hp!(100);
    color!(Water);
    weak_to!(Lightning);
    no_resistance!();
    retreat!(3);

    fn card_actions(&self, _player: Player, _card: &Card, _engine: &GameEngine) -> Vec<Action> {
        vec![]
    }
    fn execute(&self, _player: Player, _card: &Card, engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine.clone()
    }
    fn attacks(&self, player: Player, in_play: &InPlayCard, engine: &GameEngine) -> Vec<Action> {
        Attacks::new(player, in_play, engine)
            .register("Hydro Pump", Self::hydro_pump)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Blastoise2 {
    pub fn hydro_pump(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Water, Type::Water, Type::Water])
            .damage_plus_per_extra_energy_on_attacking(40, 10, Type::Water, 2)
    }
}

#[derive(Default)]
pub struct Chansey3 {}
impl CardArchetype for Chansey3 {
    identifier!("Chansey (BS 3)");
    card_name!("Chansey");
    basic!();
    hp!(120);
    color!(Colorless);
    weak_to!(Fighting);
    resists!(Psychic, 30);
    retreat!(1);

    fn card_actions(&self, _player: Player, _card: &Card, _engine: &GameEngine) -> Vec<Action> {
        vec![]
    }
    fn execute(&self, _player: Player, _card: &Card, engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine.clone()
    }
    fn attacks(&self, player: Player, in_play: &InPlayCard, engine: &GameEngine) -> Vec<Action> {
        Attacks::new(player, in_play, engine)
            .register("Scrunch", Self::scrunch)
            .register("Double-edge", Self::double_edge)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Chansey3 {
    pub fn scrunch(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Colorless, Type::Colorless])
            .flip_a_coin()
            .if_heads(|e| e.prevent_damage_during_opponents_next_turn())
    }
    pub fn double_edge(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Colorless, Type::Colorless, Type::Colorless, Type::Colorless])
            .damage(80)
            .damage_self(80)
    }
}

#[derive(Default)]
pub struct Charizard4 {}
impl CardArchetype for Charizard4 {
    identifier!("Charizard (BS 4)");
    card_name!("Charizard");
    stage2!("Charmeleon");
    hp!(120);
    color!(Fire);
    weak_to!(Water);
    resists!(Fighting, 30);
    retreat!(3);

    fn card_actions(&self, _player: Player, _card: &Card, _engine: &GameEngine) -> Vec<Action> {
        vec![]
    }
    fn execute(&self, _player: Player, _card: &Card, engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine.clone()
    }
    fn attacks(&self, player: Player, in_play: &InPlayCard, engine: &GameEngine) -> Vec<Action> {
        Attacks::new(player, in_play, engine)
            .register("Fire Spin", Self::fire_spin)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Charizard4 {
    pub fn fire_spin(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Fire, Type::Fire, Type::Fire, Type::Fire])
            .must(|e| e.discard_attacking_energy_cards(&[Type::Any, Type::Any]))
            .damage(100)
    }
}

#[derive(Default)]
pub struct Clefairy5 {}
impl CardArchetype for Clefairy5 {
    identifier!("Clefairy (BS 5)");
    card_name!("Clefairy");
    basic!();
    hp!(40);
    color!(Colorless);
    weak_to!(Fighting);
    resists!(Psychic, 30);
    retreat!(1);

    fn card_actions(&self, _player: Player, _card: &Card, _engine: &GameEngine) -> Vec<Action> {
        vec![]
    }
    fn execute(&self, _player: Player, _card: &Card, engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine.clone()
    }
    fn attacks(&self, player: Player, in_play: &InPlayCard, engine: &GameEngine) -> Vec<Action> {
        Attacks::new(player, in_play, engine)
            .register("Sing", Self::sing)
            .register("Metronome", Self::metronome)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Clefairy5 {
    pub fn sing(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Colorless])
            .flip_a_coin()
            .if_heads(|e| e.asleep())
    }
    pub fn metronome(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
}

#[derive(Default)]
pub struct Gyarados6 {}
impl CardArchetype for Gyarados6 {
    identifier!("Gyarados (BS 6)");
    card_name!("Gyarados");
    stage1!("Magikarp");
    hp!(100);
    color!(Water);
    weak_to!(Grass);
    resists!(Fighting, 30);
    retreat!(3);

    fn card_actions(&self, _player: Player, _card: &Card, _engine: &GameEngine) -> Vec<Action> {
        vec![]
    }
    fn execute(&self, _player: Player, _card: &Card, engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine.clone()
    }
    fn attacks(&self, player: Player, in_play: &InPlayCard, engine: &GameEngine) -> Vec<Action> {
        Attacks::new(player, in_play, engine)
            .register("Dragon Rage", Self::dragon_rage)
            .register("Bubblebeam", Self::bubblebeam)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Gyarados6 {
    pub fn dragon_rage(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Water, Type::Water, Type::Water])
            .damage(50)
    }
    pub fn bubblebeam(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Water, Type::Water, Type::Water, Type::Water])
            .flip_a_coin()
            .damage(40)
            .if_heads(|e| e.paralyze())
    }
}

#[derive(Default)]
pub struct Hitmonchan7 {}
impl CardArchetype for Hitmonchan7 {
    identifier!("Hitmonchan (BS 7)");
    card_name!("Hitmonchan");
    basic!();
    hp!(70);
    color!(Fighting);
    weak_to!(Psychic);
    no_resistance!();
    retreat!(2);

    fn card_actions(&self, _player: Player, _card: &Card, _engine: &GameEngine) -> Vec<Action> {
        vec![]
    }
    fn execute(&self, _player: Player, _card: &Card, engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine.clone()
    }
    fn attacks(&self, player: Player, in_play: &InPlayCard, engine: &GameEngine) -> Vec<Action> {
        Attacks::new(player, in_play, engine)
            .register("Jab", Self::jab)
            .register("Special Punch", Self::special_punch)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Hitmonchan7 {
    pub fn jab(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Fighting])
            .damage(20)
    }
    pub fn special_punch(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Fighting, Type::Fighting, Type::Colorless])
            .damage(40)
    }
}

#[derive(Default)]
pub struct Machamp8 {}
impl CardArchetype for Machamp8 {
    identifier!("Machamp (BS 8)");
    card_name!("Machamp");
    stage2!("Machoke");
    hp!(100);
    color!(Fighting);
    weak_to!(Psychic);
    no_resistance!();
    retreat!(3);

    fn card_actions(&self, _player: Player, _card: &Card, _engine: &GameEngine) -> Vec<Action> {
        vec![]
    }
    fn execute(&self, _player: Player, _card: &Card, engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine.clone()
    }
    fn attacks(&self, player: Player, in_play: &InPlayCard, engine: &GameEngine) -> Vec<Action> {
        Attacks::new(player, in_play, engine)
            .register("Seismic Toss", Self::seismic_toss)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Machamp8 {
    pub fn seismic_toss(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Fighting, Type::Fighting, Type::Fighting, Type::Colorless])
            .damage(60)
    }
}

#[derive(Default)]
pub struct Magneton9 {}
impl CardArchetype for Magneton9 {
    identifier!("Magneton (BS 9)");
    card_name!("Magneton");
    stage1!("Magnemite");
    hp!(60);
    color!(Lightning);
    weak_to!(Fighting);
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
            .register("Thunder Wave", Self::thunder_wave)
            .register("Selfdestruct", Self::selfdestruct)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Magneton9 {
    pub fn thunder_wave(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Lightning, Type::Lightning, Type::Colorless])
            .flip_a_coin()
            .damage(30)
            .if_heads(|e| e.paralyze())
    }
    pub fn selfdestruct(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Lightning, Type::Lightning, Type::Colorless, Type::Colorless])
            .damage(80)
            .each_own_bench(|e| e.damage(20))
            .each_opponents_bench(|e| e.damage(20))
            .damage_self(80)
    }
}

#[derive(Default)]
pub struct Mewtwo10 {}
impl CardArchetype for Mewtwo10 {
    identifier!("Mewtwo (BS 10)");
    card_name!("Mewtwo");
    basic!();
    hp!(60);
    color!(Psychic);
    weak_to!(Psychic);
    no_resistance!();
    retreat!(3);

    fn card_actions(&self, _player: Player, _card: &Card, _engine: &GameEngine) -> Vec<Action> {
        vec![]
    }
    fn execute(&self, _player: Player, _card: &Card, engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine.clone()
    }
    fn attacks(&self, player: Player, in_play: &InPlayCard, engine: &GameEngine) -> Vec<Action> {
        Attacks::new(player, in_play, engine)
            .register("Psychic", Self::psychic)
            .register("Barrier", Self::barrier)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Mewtwo10 {
    pub fn psychic(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Psychic, Type::Colorless])
            .damage_plus_per_energy_card_on_defending(10, 10)
    }
    pub fn barrier(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Psychic, Type::Psychic])
            .must(|e| e.discard_attacking_energy_cards(&[Type::Psychic]))
            .prevent_damage_and_effects_during_opponents_next_turn()
    }
}

#[derive(Default)]
pub struct Nidoking11 {}
impl CardArchetype for Nidoking11 {
    identifier!("Nidoking (BS 11)");
    card_name!("Nidoking");
    stage2!("Nidorino");
    hp!(90);
    color!(Grass);
    weak_to!(Psychic);
    no_resistance!();
    retreat!(3);

    fn card_actions(&self, _player: Player, _card: &Card, _engine: &GameEngine) -> Vec<Action> {
        vec![]
    }
    fn execute(&self, _player: Player, _card: &Card, engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine.clone()
    }
    fn attacks(&self, player: Player, in_play: &InPlayCard, engine: &GameEngine) -> Vec<Action> {
        Attacks::new(player, in_play, engine)
            .register("Thrash", Self::thrash)
            .register("Toxic", Self::toxic)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Nidoking11 {
    pub fn thrash(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Grass, Type::Colorless, Type::Colorless])
            .flip_a_coin()
            .if_heads(|e| e.damage(40))
            .if_tails(|e| e.damage(30).damage_self(10))
    }
    pub fn toxic(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Grass, Type::Grass, Type::Grass])
            .damage(20)
            .severe_poison(2)
    }
}

#[derive(Default)]
pub struct Ninetales12 {}
impl CardArchetype for Ninetales12 {
    identifier!("Ninetales (BS 12)");
    card_name!("Ninetales");
    stage1!("Vulpix");
    hp!(80);
    color!(Fire);
    weak_to!(Water);
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
            .register("Lure", Self::lure)
            .register("Fire Blast", Self::fire_blast)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Ninetales12 {
    pub fn lure(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Colorless, Type::Colorless])
            .gust_defending()
    }
    pub fn fire_blast(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Fire, Type::Fire, Type::Fire, Type::Fire])
            .must(|e| e.discard_attacking_energy_cards(&[Type::Fire]))
            .damage(80)
    }
}

#[derive(Default)]
pub struct Poliwrath13 {}
impl CardArchetype for Poliwrath13 {
    identifier!("Poliwrath (BS 13)");
    card_name!("Poliwrath");
    stage2!("Poliwhirl");
    hp!(90);
    color!(Water);
    weak_to!(Grass);
    no_resistance!();
    retreat!(3);

    fn card_actions(&self, _player: Player, _card: &Card, _engine: &GameEngine) -> Vec<Action> {
        vec![]
    }
    fn execute(&self, _player: Player, _card: &Card, engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine.clone()
    }
    fn attacks(&self, player: Player, in_play: &InPlayCard, engine: &GameEngine) -> Vec<Action> {
        Attacks::new(player, in_play, engine)
            .register("Water Gun", Self::water_gun)
            .register("Whirlpool", Self::whirlpool)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Poliwrath13 {
    pub fn water_gun(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Water, Type::Water, Type::Colorless])
            .damage_plus_per_extra_energy_on_attacking(30, 10, Type::Water, 2)
    }
    pub fn whirlpool(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Water, Type::Water, Type::Colorless, Type::Colorless])
            .discard_defending_energy_cards(&[Type::Any])
            .damage(40)
    }
}

#[derive(Default)]
pub struct Raichu14 {}
impl CardArchetype for Raichu14 {
    identifier!("Raichu (BS 14)");
    card_name!("Raichu");
    stage1!("Pikachu");
    hp!(80);
    color!(Lightning);
    weak_to!(Fighting);
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
            .register("Agility", Self::agility)
            .register("Thunder", Self::thunder)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Raichu14 {
    pub fn agility(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Lightning, Type::Colorless, Type::Colorless])
            .flip_a_coin()
            .if_heads(|e| e.prevent_damage_and_effects_during_opponents_next_turn())
    }
    pub fn thunder(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Lightning, Type::Lightning, Type::Lightning, Type::Colorless])
            .flip_a_coin()
            .damage(60)
            .if_tails(|e| e.damage_self(30))
    }
}

#[derive(Default)]
pub struct Venusaur15 {}
impl CardArchetype for Venusaur15 {
    identifier!("Venusaur (BS 15)");
    card_name!("Venusaur");
    stage2!("Ivysaur");
    hp!(100);
    color!(Grass);
    weak_to!(Fire);
    no_resistance!();
    retreat!(2);

    fn card_actions(&self, _player: Player, _card: &Card, _engine: &GameEngine) -> Vec<Action> {
        vec![]
    }
    fn execute(&self, _player: Player, _card: &Card, engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine.clone()
    }
    fn attacks(&self, player: Player, in_play: &InPlayCard, engine: &GameEngine) -> Vec<Action> {
        Attacks::new(player, in_play, engine)
            .register("Solarbeam", Self::solarbeam)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Venusaur15 {
    pub fn solarbeam(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Grass, Type::Grass, Type::Grass, Type::Grass])
            .damage(60)
    }
}

#[derive(Default)]
pub struct Zapdos16 {}
impl CardArchetype for Zapdos16 {
    identifier!("Zapdos (BS 16)");
    card_name!("Zapdos");
    basic!();
    hp!(90);
    color!(Lightning);
    no_weakness!();
    resists!(Fighting, 30);
    retreat!(3);

    fn card_actions(&self, _player: Player, _card: &Card, _engine: &GameEngine) -> Vec<Action> {
        vec![]
    }
    fn execute(&self, _player: Player, _card: &Card, engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine.clone()
    }
    fn attacks(&self, player: Player, in_play: &InPlayCard, engine: &GameEngine) -> Vec<Action> {
        Attacks::new(player, in_play, engine)
            .register("Thunder", Self::thunder)
            .register("Thunderbolt", Self::thunderbolt)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Zapdos16 {
    pub fn thunder(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Lightning, Type::Lightning, Type::Lightning, Type::Colorless])
            .flip_a_coin()
            .damage(60)
            .if_tails(|e| e.damage_self(30))
    }
    pub fn thunderbolt(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Lightning, Type::Lightning, Type::Lightning, Type::Lightning])
            .must(|e| e.discard_all_attacking_energy_cards())
            .damage(100)
    }
}

#[derive(Default)]
pub struct Beedrill17 {}
impl CardArchetype for Beedrill17 {
    identifier!("Beedrill (BS 17)");
    card_name!("Beedrill");
    stage2!("Kakuna");
    hp!(80);
    color!(Grass);
    weak_to!(Fire);
    resists!(Fighting, 30);
    retreat!(0);

    fn card_actions(&self, _player: Player, _card: &Card, _engine: &GameEngine) -> Vec<Action> {
        vec![]
    }
    fn execute(&self, _player: Player, _card: &Card, engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine.clone()
    }
    fn attacks(&self, player: Player, in_play: &InPlayCard, engine: &GameEngine) -> Vec<Action> {
        Attacks::new(player, in_play, engine)
            .register("Twineedle", Self::twineedle)
            .register("Poison Sting", Self::poison_sting)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Beedrill17 {
    pub fn twineedle(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Colorless, Type::Colorless, Type::Colorless])
            .flip_coins(2)
            .damage_per_heads(30)
    }
    pub fn poison_sting(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Grass, Type::Grass, Type::Grass])
            .flip_a_coin()
            .damage(40)
            .if_heads(|e| e.poison())
    }
}

#[derive(Default)]
pub struct Dragonair18 {}
impl CardArchetype for Dragonair18 {
    identifier!("Dragonair (BS 18)");
    card_name!("Dragonair");
    stage1!("Dratini");
    hp!(80);
    color!(Colorless);
    no_weakness!();
    resists!(Psychic, 30);
    retreat!(2);

    fn card_actions(&self, _player: Player, _card: &Card, _engine: &GameEngine) -> Vec<Action> {
        vec![]
    }
    fn execute(&self, _player: Player, _card: &Card, engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine.clone()
    }
    fn attacks(&self, player: Player, in_play: &InPlayCard, engine: &GameEngine) -> Vec<Action> {
        Attacks::new(player, in_play, engine)
            .register("Slam", Self::slam)
            .register("Hyper Beam", Self::hyper_beam)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Dragonair18 {
    pub fn slam(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Colorless, Type::Colorless, Type::Colorless])
            .flip_coins(2)
            .damage_per_heads(30)
    }
    pub fn hyper_beam(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Colorless, Type::Colorless, Type::Colorless, Type::Colorless])
            .discard_defending_energy_cards(&[Type::Any])
            .damage(20)
    }
}

#[derive(Default)]
pub struct Dugtrio19 {}
impl CardArchetype for Dugtrio19 {
    identifier!("Dugtrio (BS 19)");
    card_name!("Dugtrio");
    stage1!("Diglett");
    hp!(70);
    color!(Fighting);
    weak_to!(Grass);
    resists!(Lightning, 30);
    retreat!(2);

    fn card_actions(&self, _player: Player, _card: &Card, _engine: &GameEngine) -> Vec<Action> {
        vec![]
    }
    fn execute(&self, _player: Player, _card: &Card, engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine.clone()
    }
    fn attacks(&self, player: Player, in_play: &InPlayCard, engine: &GameEngine) -> Vec<Action> {
        Attacks::new(player, in_play, engine)
            .register("Slash", Self::slash)
            .register("Earthquake", Self::earthquake)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Dugtrio19 {
    pub fn slash(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Fighting, Type::Fighting, Type::Colorless])
            .damage(40)
    }
    pub fn earthquake(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Fighting, Type::Fighting, Type::Fighting, Type::Fighting])
            .damage(70)
            .each_own_bench(|e| e.damage(10))
    }
}

#[derive(Default)]
pub struct Electabuzz20 {}
impl CardArchetype for Electabuzz20 {
    identifier!("Electabuzz (BS 20)");
    card_name!("Electabuzz");
    basic!();
    hp!(70);
    color!(Lightning);
    weak_to!(Fighting);
    no_resistance!();
    retreat!(2);

    fn card_actions(&self, _player: Player, _card: &Card, _engine: &GameEngine) -> Vec<Action> {
        vec![]
    }
    fn execute(&self, _player: Player, _card: &Card, engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine.clone()
    }
    fn attacks(&self, player: Player, in_play: &InPlayCard, engine: &GameEngine) -> Vec<Action> {
        Attacks::new(player, in_play, engine)
            .register("Thundershock", Self::thundershock)
            .register("Thunderpunch", Self::thunderpunch)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Electabuzz20 {
    pub fn thundershock(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Lightning])
            .flip_a_coin()
            .damage(10)
            .if_heads(|e| e.paralyze())
    }
    pub fn thunderpunch(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Lightning, Type::Colorless])
            .flip_a_coin()
            .if_heads(|e| e.damage(40))
            .if_tails(|e| e.damage(30).damage_self(10))
    }
}

#[derive(Default)]
pub struct Electrode21 {}
impl CardArchetype for Electrode21 {
    identifier!("Electrode (BS 21)");
    card_name!("Electrode");
    stage1!("Voltorb");
    hp!(80);
    color!(Lightning);
    weak_to!(Fighting);
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
            .register("Electric Shock", Self::electric_shock)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Electrode21 {
    pub fn electric_shock(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Lightning, Type::Lightning, Type::Lightning])
            .flip_a_coin()
            .damage(50)
            .if_tails(|e| e.damage_self(10))
    }
}

#[derive(Default)]
pub struct Pidgeotto22 {}
impl CardArchetype for Pidgeotto22 {
    identifier!("Pidgeotto (BS 22)");
    card_name!("Pidgeotto");
    stage1!("Pidgey");
    hp!(60);
    color!(Colorless);
    weak_to!(Lightning);
    resists!(Fighting, 30);
    retreat!(1);

    fn card_actions(&self, _player: Player, _card: &Card, _engine: &GameEngine) -> Vec<Action> {
        vec![]
    }
    fn execute(&self, _player: Player, _card: &Card, engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine.clone()
    }
    fn attacks(&self, player: Player, in_play: &InPlayCard, engine: &GameEngine) -> Vec<Action> {
        Attacks::new(player, in_play, engine)
            .register("Whirlwind", Self::whirlwind)
            .register("Mirror Move", Self::mirror_move)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Pidgeotto22 {
    pub fn whirlwind(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Colorless, Type::Colorless])
            .damage(20)
            .switch_defending()
    }
    pub fn mirror_move(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
}

#[derive(Default)]
pub struct Arcanine23 {}
impl CardArchetype for Arcanine23 {
    identifier!("Arcanine (BS 23)");
    card_name!("Arcanine");
    stage1!("Growlithe");
    hp!(100);
    color!(Fire);
    weak_to!(Water);
    no_resistance!();
    retreat!(3);

    fn card_actions(&self, _player: Player, _card: &Card, _engine: &GameEngine) -> Vec<Action> {
        vec![]
    }
    fn execute(&self, _player: Player, _card: &Card, engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine.clone()
    }
    fn attacks(&self, player: Player, in_play: &InPlayCard, engine: &GameEngine) -> Vec<Action> {
        Attacks::new(player, in_play, engine)
            .register("Flamethrower", Self::flamethrower)
            .register("Take Down", Self::take_down)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Arcanine23 {
    pub fn flamethrower(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Fire, Type::Fire, Type::Colorless])
            .must(|e| e.discard_attacking_energy_cards(&[Type::Fire]))
            .damage(50)
    }
    pub fn take_down(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Fire, Type::Fire, Type::Colorless, Type::Colorless])
            .damage(80)
            .damage_self(30)
    }
}

#[derive(Default)]
pub struct Charmeleon24 {}
impl CardArchetype for Charmeleon24 {
    identifier!("Charmeleon (BS 24)");
    card_name!("Charmeleon");
    stage1!("Charmander");
    hp!(80);
    color!(Fire);
    weak_to!(Water);
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
            .register("Slash", Self::slash)
            .register("Flamethrower", Self::flamethrower)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Charmeleon24 {
    pub fn slash(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Colorless, Type::Colorless, Type::Colorless])
            .damage(30)
    }
    pub fn flamethrower(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Fire, Type::Fire, Type::Colorless])
            .must(|e| e.discard_attacking_energy_cards(&[Type::Fire]))
            .damage(50)
    }
}

#[derive(Default)]
pub struct Dewgong25 {}
impl CardArchetype for Dewgong25 {
    identifier!("Dewgong (BS 25)");
    card_name!("Dewgong");
    stage1!("Seel");
    hp!(80);
    color!(Water);
    weak_to!(Lightning);
    no_resistance!();
    retreat!(3);

    fn card_actions(&self, _player: Player, _card: &Card, _engine: &GameEngine) -> Vec<Action> {
        vec![]
    }
    fn execute(&self, _player: Player, _card: &Card, engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine.clone()
    }
    fn attacks(&self, player: Player, in_play: &InPlayCard, engine: &GameEngine) -> Vec<Action> {
        Attacks::new(player, in_play, engine)
            .register("Aurora Beam", Self::aurora_beam)
            .register("Ice Beam", Self::ice_beam)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Dewgong25 {
    pub fn aurora_beam(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Water, Type::Water, Type::Colorless])
            .damage(50)
    }
    pub fn ice_beam(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Water, Type::Water, Type::Colorless, Type::Colorless])
            .flip_a_coin()
            .damage(30)
            .if_heads(|e| e.paralyze())
    }
}

#[derive(Default)]
pub struct Dratini26 {}
impl CardArchetype for Dratini26 {
    identifier!("Dratini (BS 26)");
    card_name!("Dratini");
    basic!();
    hp!(40);
    color!(Colorless);
    no_weakness!();
    resists!(Psychic, 30);
    retreat!(1);

    fn card_actions(&self, _player: Player, _card: &Card, _engine: &GameEngine) -> Vec<Action> {
        vec![]
    }
    fn execute(&self, _player: Player, _card: &Card, engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine.clone()
    }
    fn attacks(&self, player: Player, in_play: &InPlayCard, engine: &GameEngine) -> Vec<Action> {
        Attacks::new(player, in_play, engine)
            .register("Pound", Self::pound)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Dratini26 {
    pub fn pound(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Colorless])
            .damage(10)
    }
}

#[derive(Default)]
pub struct FarfetchD27 {}
impl CardArchetype for FarfetchD27 {
    identifier!("Farfetch'd (BS 27)");
    card_name!("Farfetch'd");
    basic!();
    hp!(50);
    color!(Colorless);
    weak_to!(Lightning);
    resists!(Fighting, 30);
    retreat!(1);

    fn card_actions(&self, _player: Player, _card: &Card, _engine: &GameEngine) -> Vec<Action> {
        vec![]
    }
    fn execute(&self, _player: Player, _card: &Card, engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine.clone()
    }
    fn attacks(&self, player: Player, in_play: &InPlayCard, engine: &GameEngine) -> Vec<Action> {
        Attacks::new(player, in_play, engine)
            .register("Leek Slap", Self::leek_slap)
            .register("Pot Smash", Self::pot_smash)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl FarfetchD27 {
    pub fn leek_slap(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
    pub fn pot_smash(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Colorless, Type::Colorless, Type::Colorless])
            .damage(30)
    }
}

#[derive(Default)]
pub struct Growlithe28 {}
impl CardArchetype for Growlithe28 {
    identifier!("Growlithe (BS 28)");
    card_name!("Growlithe");
    basic!();
    hp!(60);
    color!(Fire);
    weak_to!(Water);
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
            .register("Flare", Self::flare)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Growlithe28 {
    pub fn flare(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Fire, Type::Colorless])
            .damage(20)
    }
}

#[derive(Default)]
pub struct Haunter29 {}
impl CardArchetype for Haunter29 {
    identifier!("Haunter (BS 29)");
    card_name!("Haunter");
    stage1!("Gastly");
    hp!(60);
    color!(Psychic);
    no_weakness!();
    resists!(Fighting, 30);
    retreat!(1);

    fn card_actions(&self, _player: Player, _card: &Card, _engine: &GameEngine) -> Vec<Action> {
        vec![]
    }
    fn execute(&self, _player: Player, _card: &Card, engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine.clone()
    }
    fn attacks(&self, player: Player, in_play: &InPlayCard, engine: &GameEngine) -> Vec<Action> {
        Attacks::new(player, in_play, engine)
            .register("Hypnosis", Self::hypnosis)
            .register("Dream Eater", Self::dream_eater)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Haunter29 {
    pub fn hypnosis(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Psychic])
            .asleep()
    }
    pub fn dream_eater(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Psychic, Type::Psychic])
            .defending_must_be_asleep()
            .damage(50)
    }
}

#[derive(Default)]
pub struct Ivysaur30 {}
impl CardArchetype for Ivysaur30 {
    identifier!("Ivysaur (BS 30)");
    card_name!("Ivysaur");
    stage1!("Bulbasaur");
    hp!(60);
    color!(Grass);
    weak_to!(Fire);
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
            .register("Vine Whip", Self::vine_whip)
            .register("Poisonpowder", Self::poisonpowder)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Ivysaur30 {
    pub fn vine_whip(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Grass, Type::Colorless, Type::Colorless])
            .damage(30)
    }
    pub fn poisonpowder(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Grass, Type::Grass, Type::Grass])
            .damage(20)
            .poison()
    }
}

#[derive(Default)]
pub struct Jynx31 {}
impl CardArchetype for Jynx31 {
    identifier!("Jynx (BS 31)");
    card_name!("Jynx");
    basic!();
    hp!(70);
    color!(Psychic);
    weak_to!(Psychic);
    no_resistance!();
    retreat!(2);

    fn card_actions(&self, _player: Player, _card: &Card, _engine: &GameEngine) -> Vec<Action> {
        vec![]
    }
    fn execute(&self, _player: Player, _card: &Card, engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine.clone()
    }
    fn attacks(&self, player: Player, in_play: &InPlayCard, engine: &GameEngine) -> Vec<Action> {
        Attacks::new(player, in_play, engine)
            .register("Doubleslap", Self::doubleslap)
            .register("Meditate", Self::meditate)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Jynx31 {
    pub fn doubleslap(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Psychic])
            .flip_coins(2)
            .damage_per_heads(10)
    }
    pub fn meditate(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Psychic, Type::Psychic, Type::Colorless])
            .damage_plus_per_damage_counter_on_defending(20, 10)
    }
}

#[derive(Default)]
pub struct Kadabra32 {}
impl CardArchetype for Kadabra32 {
    identifier!("Kadabra (BS 32)");
    card_name!("Kadabra");
    stage1!("Abra");
    hp!(60);
    color!(Psychic);
    weak_to!(Psychic);
    no_resistance!();
    retreat!(3);

    fn card_actions(&self, _player: Player, _card: &Card, _engine: &GameEngine) -> Vec<Action> {
        vec![]
    }
    fn execute(&self, _player: Player, _card: &Card, engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine.clone()
    }
    fn attacks(&self, player: Player, in_play: &InPlayCard, engine: &GameEngine) -> Vec<Action> {
        Attacks::new(player, in_play, engine)
            .register("Recover", Self::recover)
            .register("Super Psy", Self::super_psy)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Kadabra32 {
    pub fn recover(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Psychic, Type::Psychic])
            .must(|e| e.discard_attacking_energy_cards(&[Type::Psychic]))
            .heal_all_attacking()
    }
    pub fn super_psy(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Psychic, Type::Psychic, Type::Colorless])
            .damage(50)
    }
}

#[derive(Default)]
pub struct Kakuna33 {}
impl CardArchetype for Kakuna33 {
    identifier!("Kakuna (BS 33)");
    card_name!("Kakuna");
    stage1!("Weedle");
    hp!(80);
    color!(Grass);
    weak_to!(Fire);
    no_resistance!();
    retreat!(2);

    fn card_actions(&self, _player: Player, _card: &Card, _engine: &GameEngine) -> Vec<Action> {
        vec![]
    }
    fn execute(&self, _player: Player, _card: &Card, engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine.clone()
    }
    fn attacks(&self, player: Player, in_play: &InPlayCard, engine: &GameEngine) -> Vec<Action> {
        Attacks::new(player, in_play, engine)
            .register("Stiffen", Self::stiffen)
            .register("Poisonpowder", Self::poisonpowder)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Kakuna33 {
    pub fn stiffen(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Colorless, Type::Colorless])
            .flip_a_coin()
            .if_heads(|e| e.prevent_damage_during_opponents_next_turn())
    }
    pub fn poisonpowder(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Grass, Type::Grass])
            .flip_a_coin()
            .damage(20)
            .if_heads(|e| e.poison())
    }
}

#[derive(Default)]
pub struct Machoke34 {}
impl CardArchetype for Machoke34 {
    identifier!("Machoke (BS 34)");
    card_name!("Machoke");
    stage1!("Machop");
    hp!(80);
    color!(Fighting);
    weak_to!(Psychic);
    no_resistance!();
    retreat!(3);

    fn card_actions(&self, _player: Player, _card: &Card, _engine: &GameEngine) -> Vec<Action> {
        vec![]
    }
    fn execute(&self, _player: Player, _card: &Card, engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine.clone()
    }
    fn attacks(&self, player: Player, in_play: &InPlayCard, engine: &GameEngine) -> Vec<Action> {
        Attacks::new(player, in_play, engine)
            .register("Karate Chop", Self::karate_chop)
            .register("Submission", Self::submission)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Machoke34 {
    pub fn karate_chop(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Fighting, Type::Fighting, Type::Colorless])
            .damage_minus_per_damage_counter_on_itself(50, 10)
    }
    pub fn submission(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Fighting, Type::Fighting, Type::Colorless, Type::Colorless])
            .damage(60)
            .damage_self(20)
    }
}

#[derive(Default)]
pub struct Magikarp35 {}
impl CardArchetype for Magikarp35 {
    identifier!("Magikarp (BS 35)");
    card_name!("Magikarp");
    basic!();
    hp!(30);
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
            .register("Tackle", Self::tackle)
            .register("Flail", Self::flail)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Magikarp35 {
    pub fn tackle(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Colorless])
            .damage(10)
    }
    pub fn flail(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Water])
            .damage_per_damage_counter_on_itself(10)
    }
}

#[derive(Default)]
pub struct Magmar36 {}
impl CardArchetype for Magmar36 {
    identifier!("Magmar (BS 36)");
    card_name!("Magmar");
    basic!();
    hp!(50);
    color!(Fire);
    weak_to!(Water);
    no_resistance!();
    retreat!(2);

    fn card_actions(&self, _player: Player, _card: &Card, _engine: &GameEngine) -> Vec<Action> {
        vec![]
    }
    fn execute(&self, _player: Player, _card: &Card, engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine.clone()
    }
    fn attacks(&self, player: Player, in_play: &InPlayCard, engine: &GameEngine) -> Vec<Action> {
        Attacks::new(player, in_play, engine)
            .register("Fire Punch", Self::fire_punch)
            .register("Flamethrower", Self::flamethrower)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Magmar36 {
    pub fn fire_punch(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Fire, Type::Fire])
            .damage(30)
    }
    pub fn flamethrower(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Fire, Type::Fire, Type::Colorless])
            .must(|e| e.discard_attacking_energy_cards(&[Type::Fire]))
            .damage(50)
    }
}

#[derive(Default)]
pub struct Nidorino37 {}
impl CardArchetype for Nidorino37 {
    identifier!("Nidorino (BS 37)");
    card_name!("Nidorino");
    stage1!("Nidoran ♂");
    hp!(60);
    color!(Grass);
    weak_to!(Psychic);
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
            .register("Double Kick", Self::double_kick)
            .register("Horn Drill", Self::horn_drill)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Nidorino37 {
    pub fn double_kick(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Grass, Type::Colorless, Type::Colorless])
            .flip_coins(2)
            .damage_per_heads(30)
    }
    pub fn horn_drill(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Grass, Type::Grass, Type::Colorless, Type::Colorless])
            .damage(50)
    }
}

#[derive(Default)]
pub struct Poliwhirl38 {}
impl CardArchetype for Poliwhirl38 {
    identifier!("Poliwhirl (BS 38)");
    card_name!("Poliwhirl");
    stage1!("Poliwag");
    hp!(60);
    color!(Water);
    weak_to!(Grass);
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
            .register("Amnesia", Self::amnesia)
            .register("Doubleslap", Self::doubleslap)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Poliwhirl38 {
    pub fn amnesia(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
    pub fn doubleslap(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Water, Type::Water, Type::Colorless])
            .flip_coins(2)
            .damage_per_heads(30)
    }
}

#[derive(Default)]
pub struct Porygon39 {}
impl CardArchetype for Porygon39 {
    identifier!("Porygon (BS 39)");
    card_name!("Porygon");
    basic!();
    hp!(30);
    color!(Colorless);
    weak_to!(Fighting);
    resists!(Psychic, 30);
    retreat!(1);

    fn card_actions(&self, _player: Player, _card: &Card, _engine: &GameEngine) -> Vec<Action> {
        vec![]
    }
    fn execute(&self, _player: Player, _card: &Card, engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine.clone()
    }
    fn attacks(&self, player: Player, in_play: &InPlayCard, engine: &GameEngine) -> Vec<Action> {
        Attacks::new(player, in_play, engine)
            .register("Conversion 1", Self::conversion_1)
            .register("Conversion 2", Self::conversion_2)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Porygon39 {
    pub fn conversion_1(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
    pub fn conversion_2(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
}

#[derive(Default)]
pub struct Raticate40 {}
impl CardArchetype for Raticate40 {
    identifier!("Raticate (BS 40)");
    card_name!("Raticate");
    stage1!("Rattata");
    hp!(60);
    color!(Colorless);
    weak_to!(Fighting);
    resists!(Psychic, 30);
    retreat!(1);

    fn card_actions(&self, _player: Player, _card: &Card, _engine: &GameEngine) -> Vec<Action> {
        vec![]
    }
    fn execute(&self, _player: Player, _card: &Card, engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine.clone()
    }
    fn attacks(&self, player: Player, in_play: &InPlayCard, engine: &GameEngine) -> Vec<Action> {
        Attacks::new(player, in_play, engine)
            .register("Bite", Self::bite)
            .register("Super Fang", Self::super_fang)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Raticate40 {
    pub fn bite(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Colorless])
            .damage(20)
    }
    pub fn super_fang(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Colorless, Type::Colorless, Type::Colorless])
            .damage_half_defending_remaining_hp()
    }
}

#[derive(Default)]
pub struct Seel41 {}
impl CardArchetype for Seel41 {
    identifier!("Seel (BS 41)");
    card_name!("Seel");
    basic!();
    hp!(60);
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
            .register("Headbutt", Self::headbutt)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Seel41 {
    pub fn headbutt(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Water])
            .damage(10)
    }
}

#[derive(Default)]
pub struct Wartortle42 {}
impl CardArchetype for Wartortle42 {
    identifier!("Wartortle (BS 42)");
    card_name!("Wartortle");
    stage1!("Squirtle");
    hp!(70);
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
            .register("Withdraw", Self::withdraw)
            .register("Bite", Self::bite)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Wartortle42 {
    pub fn withdraw(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Water, Type::Colorless])
            .flip_a_coin()
            .if_heads(|e| e.prevent_damage_during_opponents_next_turn())
    }
    pub fn bite(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Water, Type::Colorless, Type::Colorless])
            .damage(40)
    }
}

#[derive(Default)]
pub struct Abra43 {}
impl CardArchetype for Abra43 {
    identifier!("Abra (BS 43)");
    card_name!("Abra");
    basic!();
    hp!(30);
    color!(Psychic);
    weak_to!(Psychic);
    no_resistance!();
    retreat!(0);

    fn card_actions(&self, _player: Player, _card: &Card, _engine: &GameEngine) -> Vec<Action> {
        vec![]
    }
    fn execute(&self, _player: Player, _card: &Card, engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine.clone()
    }
    fn attacks(&self, player: Player, in_play: &InPlayCard, engine: &GameEngine) -> Vec<Action> {
        Attacks::new(player, in_play, engine)
            .register("Psyshock", Self::psyshock)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Abra43 {
    pub fn psyshock(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Psychic])
            .flip_a_coin()
            .damage(10)
            .if_heads(|e| e.paralyze())
    }
}

#[derive(Default)]
pub struct Bulbasaur44 {}
impl CardArchetype for Bulbasaur44 {
    identifier!("Bulbasaur (BS 44)");
    card_name!("Bulbasaur");
    basic!();
    hp!(40);
    color!(Grass);
    weak_to!(Fire);
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
            .register("Leech Seed", Self::leech_seed)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Bulbasaur44 {
    pub fn leech_seed(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Grass, Type::Grass])
            .damage(20)
            .if_did_damage(|e| e.heal_attacking(10))
    }
}

#[derive(Default)]
pub struct Caterpie45 {}
impl CardArchetype for Caterpie45 {
    identifier!("Caterpie (BS 45)");
    card_name!("Caterpie");
    basic!();
    hp!(40);
    color!(Grass);
    weak_to!(Fire);
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
            .register("String Shot", Self::string_shot)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Caterpie45 {
    pub fn string_shot(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Grass])
            .flip_a_coin()
            .damage(10)
            .if_heads(|e| e.paralyze())
    }
}

#[derive(Default)]
pub struct Charmander46 {}
impl CardArchetype for Charmander46 {
    identifier!("Charmander (BS 46)");
    card_name!("Charmander");
    basic!();
    hp!(50);
    color!(Fire);
    weak_to!(Water);
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
            .register("Scratch", Self::scratch)
            .register("Ember", Self::ember)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Charmander46 {
    pub fn scratch(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Colorless])
            .damage(10)
    }
    pub fn ember(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Fire, Type::Colorless])
            .must(|e| e.discard_attacking_energy_cards(&[Type::Fire]))
            .damage(30)
    }
}

#[derive(Default)]
pub struct Diglett47 {}
impl CardArchetype for Diglett47 {
    identifier!("Diglett (BS 47)");
    card_name!("Diglett");
    basic!();
    hp!(30);
    color!(Fighting);
    weak_to!(Grass);
    resists!(Lightning, 30);
    retreat!(0);

    fn card_actions(&self, _player: Player, _card: &Card, _engine: &GameEngine) -> Vec<Action> {
        vec![]
    }
    fn execute(&self, _player: Player, _card: &Card, engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine.clone()
    }
    fn attacks(&self, player: Player, in_play: &InPlayCard, engine: &GameEngine) -> Vec<Action> {
        Attacks::new(player, in_play, engine)
            .register("Dig", Self::dig)
            .register("Mud Slap", Self::mud_slap)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Diglett47 {
    pub fn dig(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Fighting])
            .damage(10)
    }
    pub fn mud_slap(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Fighting, Type::Fighting])
            .damage(30)
    }
}

#[derive(Default)]
pub struct Doduo48 {}
impl CardArchetype for Doduo48 {
    identifier!("Doduo (BS 48)");
    card_name!("Doduo");
    basic!();
    hp!(50);
    color!(Colorless);
    weak_to!(Lightning);
    resists!(Fighting, 30);
    retreat!(0);

    fn card_actions(&self, _player: Player, _card: &Card, _engine: &GameEngine) -> Vec<Action> {
        vec![]
    }
    fn execute(&self, _player: Player, _card: &Card, engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine.clone()
    }
    fn attacks(&self, player: Player, in_play: &InPlayCard, engine: &GameEngine) -> Vec<Action> {
        Attacks::new(player, in_play, engine)
            .register("Fury Attack", Self::fury_attack)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Doduo48 {
    pub fn fury_attack(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Colorless])
            .flip_coins(2)
            .damage_per_heads(10)
    }
}

#[derive(Default)]
pub struct Drowzee49 {}
impl CardArchetype for Drowzee49 {
    identifier!("Drowzee (BS 49)");
    card_name!("Drowzee");
    basic!();
    hp!(50);
    color!(Psychic);
    weak_to!(Psychic);
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
            .register("Pound", Self::pound)
            .register("Confuse Ray", Self::confuse_ray)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Drowzee49 {
    pub fn pound(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Colorless])
            .damage(10)
    }
    pub fn confuse_ray(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Psychic, Type::Psychic])
            .flip_a_coin()
            .damage(10)
            .if_heads(|e| e.confuse())
    }
}

#[derive(Default)]
pub struct Gastly50 {}
impl CardArchetype for Gastly50 {
    identifier!("Gastly (BS 50)");
    card_name!("Gastly");
    basic!();
    hp!(30);
    color!(Psychic);
    no_weakness!();
    resists!(Fighting, 30);
    retreat!(0);

    fn card_actions(&self, _player: Player, _card: &Card, _engine: &GameEngine) -> Vec<Action> {
        vec![]
    }
    fn execute(&self, _player: Player, _card: &Card, engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine.clone()
    }
    fn attacks(&self, player: Player, in_play: &InPlayCard, engine: &GameEngine) -> Vec<Action> {
        Attacks::new(player, in_play, engine)
            .register("Sleeping Gas", Self::sleeping_gas)
            .register("Destiny Bond", Self::destiny_bond)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Gastly50 {
    pub fn sleeping_gas(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Psychic])
            .flip_a_coin()
            .if_heads(|e| e.asleep())
    }
    pub fn destiny_bond(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Psychic, Type::Colorless])
            .knock_out_attacker_if_attacking_is_knocked_out_next_turn()
    }
}

#[derive(Default)]
pub struct Koffing51 {}
impl CardArchetype for Koffing51 {
    identifier!("Koffing (BS 51)");
    card_name!("Koffing");
    basic!();
    hp!(50);
    color!(Grass);
    weak_to!(Psychic);
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
            .register("Foul Gas", Self::foul_gas)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Koffing51 {
    pub fn foul_gas(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Grass, Type::Grass])
            .flip_a_coin()
            .damage(10)
            .if_heads(|e| e.poison())
            .if_tails(|e| e.confuse())
    }
}

#[derive(Default)]
pub struct Machop52 {}
impl CardArchetype for Machop52 {
    identifier!("Machop (BS 52)");
    card_name!("Machop");
    basic!();
    hp!(50);
    color!(Fighting);
    weak_to!(Psychic);
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
            .register("Low Kick", Self::low_kick)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Machop52 {
    pub fn low_kick(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Fighting])
            .damage(20)
    }
}

#[derive(Default)]
pub struct Magnemite53 {}
impl CardArchetype for Magnemite53 {
    identifier!("Magnemite (BS 53)");
    card_name!("Magnemite");
    basic!();
    hp!(40);
    color!(Lightning);
    weak_to!(Fighting);
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
            .register("Thunder Wave", Self::thunder_wave)
            .register("Selfdestruct", Self::selfdestruct)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Magnemite53 {
    pub fn thunder_wave(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Lightning])
            .flip_a_coin()
            .damage(10)
            .if_heads(|e| e.paralyze())
    }
    pub fn selfdestruct(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Lightning, Type::Colorless])
            .damage(40)
            .each_own_bench(|e| e.damage(10))
            .each_opponents_bench(|e| e.damage(10))
            .damage_self(40)
    }
}

#[derive(Default)]
pub struct Metapod54 {}
impl CardArchetype for Metapod54 {
    identifier!("Metapod (BS 54)");
    card_name!("Metapod");
    stage1!("Caterpie");
    hp!(70);
    color!(Grass);
    weak_to!(Fire);
    no_resistance!();
    retreat!(2);

    fn card_actions(&self, _player: Player, _card: &Card, _engine: &GameEngine) -> Vec<Action> {
        vec![]
    }
    fn execute(&self, _player: Player, _card: &Card, engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine.clone()
    }
    fn attacks(&self, player: Player, in_play: &InPlayCard, engine: &GameEngine) -> Vec<Action> {
        Attacks::new(player, in_play, engine)
            .register("Stiffen", Self::stiffen)
            .register("Stun Spore", Self::stun_spore)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Metapod54 {
    pub fn stiffen(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Colorless, Type::Colorless])
            .flip_a_coin()
            .if_heads(|e| e.prevent_damage_during_opponents_next_turn())
    }
    pub fn stun_spore(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Grass, Type::Grass])
            .flip_a_coin()
            .damage(20)
            .if_heads(|e| e.paralyze())
    }
}

#[derive(Default)]
pub struct NidoranM55 {}
impl CardArchetype for NidoranM55 {
    identifier!("Nidoran ♂ (BS 55)");
    card_name!("Nidoran ♂");
    basic!();
    hp!(40);
    color!(Grass);
    weak_to!(Psychic);
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
            .register("Horn Hazard", Self::horn_hazard)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl NidoranM55 {
    pub fn horn_hazard(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Grass])
            .flip_a_coin()
            .if_heads(|e| e.damage(30))
    }
}

#[derive(Default)]
pub struct Onix56 {}
impl CardArchetype for Onix56 {
    identifier!("Onix (BS 56)");
    card_name!("Onix");
    basic!();
    hp!(90);
    color!(Fighting);
    weak_to!(Grass);
    no_resistance!();
    retreat!(3);

    fn card_actions(&self, _player: Player, _card: &Card, _engine: &GameEngine) -> Vec<Action> {
        vec![]
    }
    fn execute(&self, _player: Player, _card: &Card, engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine.clone()
    }
    fn attacks(&self, player: Player, in_play: &InPlayCard, engine: &GameEngine) -> Vec<Action> {
        Attacks::new(player, in_play, engine)
            .register("Rock Throw", Self::rock_throw)
            .register("Harden", Self::harden)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Onix56 {
    pub fn rock_throw(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Fighting])
            .damage(10)
    }
    pub fn harden(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
}

#[derive(Default)]
pub struct Pidgey57 {}
impl CardArchetype for Pidgey57 {
    identifier!("Pidgey (BS 57)");
    card_name!("Pidgey");
    basic!();
    hp!(40);
    color!(Colorless);
    weak_to!(Lightning);
    resists!(Fighting, 30);
    retreat!(1);

    fn card_actions(&self, _player: Player, _card: &Card, _engine: &GameEngine) -> Vec<Action> {
        vec![]
    }
    fn execute(&self, _player: Player, _card: &Card, engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine.clone()
    }
    fn attacks(&self, player: Player, in_play: &InPlayCard, engine: &GameEngine) -> Vec<Action> {
        Attacks::new(player, in_play, engine)
            .register("Whirlwind", Self::whirlwind)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Pidgey57 {
    pub fn whirlwind(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Colorless, Type::Colorless])
            .damage(10)
            .switch_defending()
    }
}

#[derive(Default)]
pub struct Pikachu58 {}
impl CardArchetype for Pikachu58 {
    identifier!("Pikachu (BS 58)");
    card_name!("Pikachu");
    basic!();
    hp!(40);
    color!(Lightning);
    weak_to!(Fighting);
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
            .register("Gnaw", Self::gnaw)
            .register("Thunder Jolt", Self::thunder_jolt)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Pikachu58 {
    pub fn gnaw(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Colorless])
            .damage(10)
    }
    pub fn thunder_jolt(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Lightning, Type::Colorless])
            .flip_a_coin()
            .damage(30)
            .if_tails(|e| e.damage_self(10))
    }
}

#[derive(Default)]
pub struct Poliwag59 {}
impl CardArchetype for Poliwag59 {
    identifier!("Poliwag (BS 59)");
    card_name!("Poliwag");
    basic!();
    hp!(40);
    color!(Water);
    weak_to!(Grass);
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
            .register("Water Gun", Self::water_gun)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Poliwag59 {
    pub fn water_gun(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Water])
            .damage_plus_per_extra_energy_on_attacking(10, 10, Type::Water, 2)
    }
}

#[derive(Default)]
pub struct Ponyta60 {}
impl CardArchetype for Ponyta60 {
    identifier!("Ponyta (BS 60)");
    card_name!("Ponyta");
    basic!();
    hp!(40);
    color!(Fire);
    weak_to!(Water);
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
            .register("Smash Kick", Self::smash_kick)
            .register("Flame Tail", Self::flame_tail)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Ponyta60 {
    pub fn smash_kick(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Colorless, Type::Colorless])
            .damage(20)
    }
    pub fn flame_tail(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Fire, Type::Fire])
            .damage(30)
    }
}

#[derive(Default)]
pub struct Rattata61 {}
impl CardArchetype for Rattata61 {
    identifier!("Rattata (BS 61)");
    card_name!("Rattata");
    basic!();
    hp!(30);
    color!(Colorless);
    weak_to!(Fighting);
    resists!(Psychic, 30);
    retreat!(0);

    fn card_actions(&self, _player: Player, _card: &Card, _engine: &GameEngine) -> Vec<Action> {
        vec![]
    }
    fn execute(&self, _player: Player, _card: &Card, engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine.clone()
    }
    fn attacks(&self, player: Player, in_play: &InPlayCard, engine: &GameEngine) -> Vec<Action> {
        Attacks::new(player, in_play, engine)
            .register("Bite", Self::bite)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Rattata61 {
    pub fn bite(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Colorless])
            .damage(20)
    }
}

#[derive(Default)]
pub struct Sandshrew62 {}
impl CardArchetype for Sandshrew62 {
    identifier!("Sandshrew (BS 62)");
    card_name!("Sandshrew");
    basic!();
    hp!(40);
    color!(Fighting);
    weak_to!(Grass);
    resists!(Lightning, 30);
    retreat!(1);

    fn card_actions(&self, _player: Player, _card: &Card, _engine: &GameEngine) -> Vec<Action> {
        vec![]
    }
    fn execute(&self, _player: Player, _card: &Card, engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine.clone()
    }
    fn attacks(&self, player: Player, in_play: &InPlayCard, engine: &GameEngine) -> Vec<Action> {
        Attacks::new(player, in_play, engine)
            .register("Sand-attack", Self::sand_attack)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Sandshrew62 {
    pub fn sand_attack(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Fighting])
            .prevent_attack_on_a_flip_during_opponents_next_turn()
    }
}

#[derive(Default)]
pub struct Squirtle63 {}
impl CardArchetype for Squirtle63 {
    identifier!("Squirtle (BS 63)");
    card_name!("Squirtle");
    basic!();
    hp!(40);
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
            .register("Bubble", Self::bubble)
            .register("Withdraw", Self::withdraw)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Squirtle63 {
    pub fn bubble(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Water])
            .flip_a_coin()
            .damage(10)
            .if_heads(|e| e.paralyze())
    }
    pub fn withdraw(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Water, Type::Colorless])
            .flip_a_coin()
            .if_heads(|e| e.prevent_damage_during_opponents_next_turn())
    }
}

#[derive(Default)]
pub struct Starmie64 {}
impl CardArchetype for Starmie64 {
    identifier!("Starmie (BS 64)");
    card_name!("Starmie");
    stage1!("Staryu");
    hp!(60);
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
            .register("Recover", Self::recover)
            .register("Star Freeze", Self::star_freeze)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Starmie64 {
    pub fn recover(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Water, Type::Water])
            .must(|e| e.discard_attacking_energy_cards(&[Type::Water]))
            .heal_all_attacking()
    }
    pub fn star_freeze(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Water, Type::Colorless, Type::Colorless])
            .flip_a_coin()
            .damage(20)
            .if_heads(|e| e.paralyze())
    }
}

#[derive(Default)]
pub struct Staryu65 {}
impl CardArchetype for Staryu65 {
    identifier!("Staryu (BS 65)");
    card_name!("Staryu");
    basic!();
    hp!(40);
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
            .register("Slap", Self::slap)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Staryu65 {
    pub fn slap(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Water])
            .damage(20)
    }
}

#[derive(Default)]
pub struct Tangela66 {}
impl CardArchetype for Tangela66 {
    identifier!("Tangela (BS 66)");
    card_name!("Tangela");
    basic!();
    hp!(50);
    color!(Grass);
    weak_to!(Fire);
    no_resistance!();
    retreat!(2);

    fn card_actions(&self, _player: Player, _card: &Card, _engine: &GameEngine) -> Vec<Action> {
        vec![]
    }
    fn execute(&self, _player: Player, _card: &Card, engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine.clone()
    }
    fn attacks(&self, player: Player, in_play: &InPlayCard, engine: &GameEngine) -> Vec<Action> {
        Attacks::new(player, in_play, engine)
            .register("Bind", Self::bind)
            .register("Poisonpowder", Self::poisonpowder)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Tangela66 {
    pub fn bind(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Grass, Type::Colorless])
            .flip_a_coin()
            .damage(20)
            .if_heads(|e| e.paralyze())
    }
    pub fn poisonpowder(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Grass, Type::Grass, Type::Grass])
            .damage(20)
            .poison()
    }
}

#[derive(Default)]
pub struct Voltorb67 {}
impl CardArchetype for Voltorb67 {
    identifier!("Voltorb (BS 67)");
    card_name!("Voltorb");
    basic!();
    hp!(40);
    color!(Lightning);
    weak_to!(Fighting);
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
            .register("Tackle", Self::tackle)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Voltorb67 {
    pub fn tackle(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Colorless])
            .damage(10)
    }
}

#[derive(Default)]
pub struct Vulpix68 {}
impl CardArchetype for Vulpix68 {
    identifier!("Vulpix (BS 68)");
    card_name!("Vulpix");
    basic!();
    hp!(50);
    color!(Fire);
    weak_to!(Water);
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
            .register("Confuse Ray", Self::confuse_ray)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Vulpix68 {
    pub fn confuse_ray(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Fire, Type::Fire])
            .flip_a_coin()
            .damage(10)
            .if_heads(|e| e.confuse())
    }
}

#[derive(Default)]
pub struct Weedle69 {}
impl CardArchetype for Weedle69 {
    identifier!("Weedle (BS 69)");
    card_name!("Weedle");
    basic!();
    hp!(40);
    color!(Grass);
    weak_to!(Fire);
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
            .register("Poison Sting", Self::poison_sting)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Weedle69 {
    pub fn poison_sting(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Grass])
            .flip_a_coin()
            .damage(10)
            .if_heads(|e| e.poison())
    }
}