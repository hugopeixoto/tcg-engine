use crate::*;
use crate::attack_builder::AttackBuilder;
use crate::carddb::Attacks;

#[derive(Default)]
pub struct Aerodactyl1 {}
impl CardArchetype for Aerodactyl1 {
    identifier!("Aerodactyl (FO 1)");
    card_name!("Aerodactyl");
    stage1!("Mysterious Fossil");
    hp!(60);
    color!(Fighting);
    weak_to!(Grass);
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
            .register("Wing Attack", Self::wing_attack)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Aerodactyl1 {
    pub fn wing_attack(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Colorless, Type::Colorless, Type::Colorless])
            .damage(30)
    }
}

#[derive(Default)]
pub struct Articuno2 {}
impl CardArchetype for Articuno2 {
    identifier!("Articuno (FO 2)");
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
impl Articuno2 {
    pub fn freeze_dry(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Water, Type::Water, Type::Water])
            .flip_a_coin()
            .damage(30)
            .if_heads(|e| e.paralyze())
    }
    pub fn blizzard(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
}

#[derive(Default)]
pub struct Ditto3 {}
impl CardArchetype for Ditto3 {
    identifier!("Ditto (FO 3)");
    card_name!("Ditto");
    basic!();
    hp!(50);
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
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Ditto3 {
}

#[derive(Default)]
pub struct Dragonite4 {}
impl CardArchetype for Dragonite4 {
    identifier!("Dragonite (FO 4)");
    card_name!("Dragonite");
    stage2!("Dragonair");
    hp!(100);
    color!(Colorless);
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
            .register("Slam", Self::slam)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Dragonite4 {
    pub fn slam(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Colorless, Type::Colorless, Type::Colorless, Type::Colorless])
            .flip_coins(2)
            .damage_per_heads(40)
    }
}

#[derive(Default)]
pub struct Gengar5 {}
impl CardArchetype for Gengar5 {
    identifier!("Gengar (FO 5)");
    card_name!("Gengar");
    stage2!("Haunter");
    hp!(80);
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
            .register("Dark Mind", Self::dark_mind)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Gengar5 {
    pub fn dark_mind(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
}

#[derive(Default)]
pub struct Haunter6 {}
impl CardArchetype for Haunter6 {
    identifier!("Haunter (FO 6)");
    card_name!("Haunter");
    stage1!("Gastly");
    hp!(50);
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
            .register("Nightmare", Self::nightmare)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Haunter6 {
    pub fn nightmare(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Psychic, Type::Colorless])
            .damage(10)
            .asleep()
    }
}

#[derive(Default)]
pub struct Hitmonlee7 {}
impl CardArchetype for Hitmonlee7 {
    identifier!("Hitmonlee (FO 7)");
    card_name!("Hitmonlee");
    basic!();
    hp!(60);
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
            .register("Stretch Kick", Self::stretch_kick)
            .register("High Jump Kick", Self::high_jump_kick)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Hitmonlee7 {
    pub fn stretch_kick(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
    pub fn high_jump_kick(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Fighting, Type::Fighting, Type::Fighting])
            .damage(50)
    }
}

#[derive(Default)]
pub struct Hypno8 {}
impl CardArchetype for Hypno8 {
    identifier!("Hypno (FO 8)");
    card_name!("Hypno");
    stage1!("Drowzee");
    hp!(90);
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
            .register("Prophecy", Self::prophecy)
            .register("Dark Mind", Self::dark_mind)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Hypno8 {
    pub fn prophecy(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
    pub fn dark_mind(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
}

#[derive(Default)]
pub struct Kabutops9 {}
impl CardArchetype for Kabutops9 {
    identifier!("Kabutops (FO 9)");
    card_name!("Kabutops");
    stage2!("Kabuto");
    hp!(60);
    color!(Fighting);
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
            .register("Sharp Sickle", Self::sharp_sickle)
            .register("Absorb", Self::absorb)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Kabutops9 {
    pub fn sharp_sickle(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Fighting, Type::Fighting])
            .damage(30)
    }
    pub fn absorb(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
}

#[derive(Default)]
pub struct Lapras10 {}
impl CardArchetype for Lapras10 {
    identifier!("Lapras (FO 10)");
    card_name!("Lapras");
    basic!();
    hp!(80);
    color!(Water);
    weak_to!(Lightning);
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
            .register("Water Gun", Self::water_gun)
            .register("Confuse Ray", Self::confuse_ray)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Lapras10 {
    pub fn water_gun(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
    pub fn confuse_ray(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Water, Type::Water])
            .flip_a_coin()
            .damage(10)
            .if_heads(|e| e.confuse())
    }
}

#[derive(Default)]
pub struct Magneton11 {}
impl CardArchetype for Magneton11 {
    identifier!("Magneton (FO 11)");
    card_name!("Magneton");
    stage1!("Magnemite");
    hp!(80);
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
            .register("Sonicboom", Self::sonicboom)
            .register("Selfdestruct", Self::selfdestruct)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Magneton11 {
    pub fn sonicboom(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
    pub fn selfdestruct(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Lightning, Type::Lightning, Type::Lightning, Type::Lightning])
            .damage(100)
            .each_own_bench(|e| e.damage(20))
            .each_opponents_bench(|e| e.damage(20))
            .damage_self(100)
    }
}

#[derive(Default)]
pub struct Moltres12 {}
impl CardArchetype for Moltres12 {
    identifier!("Moltres (FO 12)");
    card_name!("Moltres");
    basic!();
    hp!(70);
    color!(Fire);
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
            .register("Wildfire", Self::wildfire)
            .register("Dive Bomb", Self::dive_bomb)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Moltres12 {
    pub fn wildfire(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
    pub fn dive_bomb(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Fire, Type::Fire, Type::Fire, Type::Fire])
            .flip_a_coin()
            .if_heads(|e| e.damage(80))
    }
}

#[derive(Default)]
pub struct Muk13 {}
impl CardArchetype for Muk13 {
    identifier!("Muk (FO 13)");
    card_name!("Muk");
    stage1!("Grimer");
    hp!(70);
    color!(Grass);
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
            .register("Sludge", Self::sludge)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Muk13 {
    pub fn sludge(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Grass, Type::Grass, Type::Grass])
            .flip_a_coin()
            .damage(30)
            .if_heads(|e| e.poison())
    }
}

#[derive(Default)]
pub struct Raichu14 {}
impl CardArchetype for Raichu14 {
    identifier!("Raichu (FO 14)");
    card_name!("Raichu");
    stage1!("Pikachu");
    hp!(90);
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
            .register("Gigashock", Self::gigashock)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Raichu14 {
    pub fn gigashock(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
}

#[derive(Default)]
pub struct Zapdos15 {}
impl CardArchetype for Zapdos15 {
    identifier!("Zapdos (FO 15)");
    card_name!("Zapdos");
    basic!();
    hp!(80);
    color!(Lightning);
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
            .register("Thunderstorm", Self::thunderstorm)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Zapdos15 {
    pub fn thunderstorm(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
}

#[derive(Default)]
pub struct Aerodactyl16 {}
impl CardArchetype for Aerodactyl16 {
    identifier!("Aerodactyl (FO 16)");
    card_name!("Aerodactyl");
    stage1!("Mysterious Fossil");
    hp!(60);
    color!(Fighting);
    weak_to!(Grass);
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
            .register("Wing Attack", Self::wing_attack)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Aerodactyl16 {
    pub fn wing_attack(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Colorless, Type::Colorless, Type::Colorless])
            .damage(30)
    }
}

#[derive(Default)]
pub struct Articuno17 {}
impl CardArchetype for Articuno17 {
    identifier!("Articuno (FO 17)");
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
impl Articuno17 {
    pub fn freeze_dry(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Water, Type::Water, Type::Water])
            .flip_a_coin()
            .damage(30)
            .if_heads(|e| e.paralyze())
    }
    pub fn blizzard(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
}

#[derive(Default)]
pub struct Ditto18 {}
impl CardArchetype for Ditto18 {
    identifier!("Ditto (FO 18)");
    card_name!("Ditto");
    basic!();
    hp!(50);
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
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Ditto18 {
}

#[derive(Default)]
pub struct Dragonite19 {}
impl CardArchetype for Dragonite19 {
    identifier!("Dragonite (FO 19)");
    card_name!("Dragonite");
    stage2!("Dragonair");
    hp!(100);
    color!(Colorless);
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
            .register("Slam", Self::slam)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Dragonite19 {
    pub fn slam(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Colorless, Type::Colorless, Type::Colorless, Type::Colorless])
            .flip_coins(2)
            .damage_per_heads(40)
    }
}

#[derive(Default)]
pub struct Gengar20 {}
impl CardArchetype for Gengar20 {
    identifier!("Gengar (FO 20)");
    card_name!("Gengar");
    stage2!("Haunter");
    hp!(80);
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
            .register("Dark Mind", Self::dark_mind)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Gengar20 {
    pub fn dark_mind(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
}

#[derive(Default)]
pub struct Haunter21 {}
impl CardArchetype for Haunter21 {
    identifier!("Haunter (FO 21)");
    card_name!("Haunter");
    stage1!("Gastly");
    hp!(50);
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
            .register("Nightmare", Self::nightmare)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Haunter21 {
    pub fn nightmare(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Psychic, Type::Colorless])
            .damage(10)
            .asleep()
    }
}

#[derive(Default)]
pub struct Hitmonlee22 {}
impl CardArchetype for Hitmonlee22 {
    identifier!("Hitmonlee (FO 22)");
    card_name!("Hitmonlee");
    basic!();
    hp!(60);
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
            .register("Stretch Kick", Self::stretch_kick)
            .register("High Jump Kick", Self::high_jump_kick)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Hitmonlee22 {
    pub fn stretch_kick(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
    pub fn high_jump_kick(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Fighting, Type::Fighting, Type::Fighting])
            .damage(50)
    }
}

#[derive(Default)]
pub struct Hypno23 {}
impl CardArchetype for Hypno23 {
    identifier!("Hypno (FO 23)");
    card_name!("Hypno");
    stage1!("Drowzee");
    hp!(90);
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
            .register("Prophecy", Self::prophecy)
            .register("Dark Mind", Self::dark_mind)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Hypno23 {
    pub fn prophecy(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
    pub fn dark_mind(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
}

#[derive(Default)]
pub struct Kabutops24 {}
impl CardArchetype for Kabutops24 {
    identifier!("Kabutops (FO 24)");
    card_name!("Kabutops");
    stage2!("Kabuto");
    hp!(60);
    color!(Fighting);
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
            .register("Sharp Sickle", Self::sharp_sickle)
            .register("Absorb", Self::absorb)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Kabutops24 {
    pub fn sharp_sickle(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Fighting, Type::Fighting])
            .damage(30)
    }
    pub fn absorb(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
}

#[derive(Default)]
pub struct Lapras25 {}
impl CardArchetype for Lapras25 {
    identifier!("Lapras (FO 25)");
    card_name!("Lapras");
    basic!();
    hp!(80);
    color!(Water);
    weak_to!(Lightning);
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
            .register("Water Gun", Self::water_gun)
            .register("Confuse Ray", Self::confuse_ray)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Lapras25 {
    pub fn water_gun(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
    pub fn confuse_ray(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Water, Type::Water])
            .flip_a_coin()
            .damage(10)
            .if_heads(|e| e.confuse())
    }
}

#[derive(Default)]
pub struct Magneton26 {}
impl CardArchetype for Magneton26 {
    identifier!("Magneton (FO 26)");
    card_name!("Magneton");
    stage1!("Magnemite");
    hp!(80);
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
            .register("Sonicboom", Self::sonicboom)
            .register("Selfdestruct", Self::selfdestruct)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Magneton26 {
    pub fn sonicboom(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
    pub fn selfdestruct(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Lightning, Type::Lightning, Type::Lightning, Type::Lightning])
            .damage(100)
            .each_own_bench(|e| e.damage(20))
            .each_opponents_bench(|e| e.damage(20))
            .damage_self(100)
    }
}

#[derive(Default)]
pub struct Moltres27 {}
impl CardArchetype for Moltres27 {
    identifier!("Moltres (FO 27)");
    card_name!("Moltres");
    basic!();
    hp!(70);
    color!(Fire);
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
            .register("Wildfire", Self::wildfire)
            .register("Dive Bomb", Self::dive_bomb)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Moltres27 {
    pub fn wildfire(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
    pub fn dive_bomb(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Fire, Type::Fire, Type::Fire, Type::Fire])
            .flip_a_coin()
            .if_heads(|e| e.damage(80))
    }
}

#[derive(Default)]
pub struct Muk28 {}
impl CardArchetype for Muk28 {
    identifier!("Muk (FO 28)");
    card_name!("Muk");
    stage1!("Grimer");
    hp!(70);
    color!(Grass);
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
            .register("Sludge", Self::sludge)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Muk28 {
    pub fn sludge(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Grass, Type::Grass, Type::Grass])
            .flip_a_coin()
            .damage(30)
            .if_heads(|e| e.poison())
    }
}

#[derive(Default)]
pub struct Raichu29 {}
impl CardArchetype for Raichu29 {
    identifier!("Raichu (FO 29)");
    card_name!("Raichu");
    stage1!("Pikachu");
    hp!(90);
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
            .register("Gigashock", Self::gigashock)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Raichu29 {
    pub fn gigashock(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
}

#[derive(Default)]
pub struct Zapdos30 {}
impl CardArchetype for Zapdos30 {
    identifier!("Zapdos (FO 30)");
    card_name!("Zapdos");
    basic!();
    hp!(80);
    color!(Lightning);
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
            .register("Thunderstorm", Self::thunderstorm)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Zapdos30 {
    pub fn thunderstorm(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
}

#[derive(Default)]
pub struct Arbok31 {}
impl CardArchetype for Arbok31 {
    identifier!("Arbok (FO 31)");
    card_name!("Arbok");
    stage1!("Ekans");
    hp!(60);
    color!(Grass);
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
            .register("Terror Strike", Self::terror_strike)
            .register("Poison Fang", Self::poison_fang)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Arbok31 {
    pub fn terror_strike(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
    pub fn poison_fang(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Grass, Type::Grass, Type::Colorless])
            .damage(20)
            .poison()
    }
}

#[derive(Default)]
pub struct Cloyster32 {}
impl CardArchetype for Cloyster32 {
    identifier!("Cloyster (FO 32)");
    card_name!("Cloyster");
    stage1!("Shellder");
    hp!(50);
    color!(Water);
    weak_to!(Lightning);
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
            .register("Clamp", Self::clamp)
            .register("Spike Cannon", Self::spike_cannon)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Cloyster32 {
    pub fn clamp(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
    pub fn spike_cannon(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Water, Type::Water])
            .flip_coins(2)
            .damage_per_heads(30)
    }
}

#[derive(Default)]
pub struct Gastly33 {}
impl CardArchetype for Gastly33 {
    identifier!("Gastly (FO 33)");
    card_name!("Gastly");
    basic!();
    hp!(50);
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
            .register("Lick", Self::lick)
            .register("Energy Conversion", Self::energy_conversion)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Gastly33 {
    pub fn lick(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Psychic])
            .flip_a_coin()
            .damage(10)
            .if_heads(|e| e.paralyze())
    }
    pub fn energy_conversion(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
}

#[derive(Default)]
pub struct Golbat34 {}
impl CardArchetype for Golbat34 {
    identifier!("Golbat (FO 34)");
    card_name!("Golbat");
    stage1!("Zubat");
    hp!(60);
    color!(Grass);
    weak_to!(Psychic);
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
            .register("Wing Attack", Self::wing_attack)
            .register("Leech Life", Self::leech_life)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Golbat34 {
    pub fn wing_attack(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Colorless, Type::Colorless, Type::Colorless])
            .damage(30)
    }
    pub fn leech_life(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
}

#[derive(Default)]
pub struct Golduck35 {}
impl CardArchetype for Golduck35 {
    identifier!("Golduck (FO 35)");
    card_name!("Golduck");
    stage1!("Psyduck");
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
            .register("Psyshock", Self::psyshock)
            .register("Hyper Beam", Self::hyper_beam)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Golduck35 {
    pub fn psyshock(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Psychic])
            .flip_a_coin()
            .damage(10)
            .if_heads(|e| e.paralyze())
    }
    pub fn hyper_beam(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Water, Type::Water, Type::Colorless])
            .discard_defending_energy_cards(&[Type::Any])
            .damage(20)
    }
}

#[derive(Default)]
pub struct Golem36 {}
impl CardArchetype for Golem36 {
    identifier!("Golem (FO 36)");
    card_name!("Golem");
    stage2!("Graveler");
    hp!(80);
    color!(Fighting);
    weak_to!(Grass);
    no_resistance!();
    retreat!(4);

    fn card_actions(&self, _player: Player, _card: &Card, _engine: &GameEngine) -> Vec<Action> {
        vec![]
    }
    fn execute(&self, _player: Player, _card: &Card, engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine.clone()
    }
    fn attacks(&self, player: Player, in_play: &InPlayCard, engine: &GameEngine) -> Vec<Action> {
        Attacks::new(player, in_play, engine)
            .register("Avalanche", Self::avalanche)
            .register("Selfdestruct", Self::selfdestruct)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Golem36 {
    pub fn avalanche(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Fighting, Type::Fighting, Type::Fighting, Type::Colorless])
            .damage(60)
    }
    pub fn selfdestruct(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Fighting, Type::Fighting, Type::Fighting, Type::Fighting])
            .damage(100)
            .each_own_bench(|e| e.damage(20))
            .each_opponents_bench(|e| e.damage(20))
            .damage_self(100)
    }
}

#[derive(Default)]
pub struct Graveler37 {}
impl CardArchetype for Graveler37 {
    identifier!("Graveler (FO 37)");
    card_name!("Graveler");
    stage1!("Geodude");
    hp!(60);
    color!(Fighting);
    weak_to!(Grass);
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
            .register("Harden", Self::harden)
            .register("Rock Throw", Self::rock_throw)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Graveler37 {
    pub fn harden(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
    pub fn rock_throw(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Fighting, Type::Fighting, Type::Colorless])
            .damage(40)
    }
}

#[derive(Default)]
pub struct Kingler38 {}
impl CardArchetype for Kingler38 {
    identifier!("Kingler (FO 38)");
    card_name!("Kingler");
    stage1!("Krabby");
    hp!(60);
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
            .register("Flail", Self::flail)
            .register("Crabhammer", Self::crabhammer)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Kingler38 {
    pub fn flail(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Water])
            .damage_per_damage_counter_on_itself(10)
    }
    pub fn crabhammer(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Water, Type::Water, Type::Colorless])
            .damage(40)
    }
}

#[derive(Default)]
pub struct Magmar39 {}
impl CardArchetype for Magmar39 {
    identifier!("Magmar (FO 39)");
    card_name!("Magmar");
    basic!();
    hp!(70);
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
            .register("Smokescreen", Self::smokescreen)
            .register("Smog", Self::smog)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Magmar39 {
    pub fn smokescreen(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Fire])
            .prevent_attack_on_a_flip_during_opponents_next_turn()
    }
    pub fn smog(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Fire, Type::Fire])
            .flip_a_coin()
            .damage(20)
            .if_heads(|e| e.poison())
    }
}

#[derive(Default)]
pub struct Omastar40 {}
impl CardArchetype for Omastar40 {
    identifier!("Omastar (FO 40)");
    card_name!("Omastar");
    stage2!("Omanyte");
    hp!(70);
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
            .register("Spike Cannon", Self::spike_cannon)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Omastar40 {
    pub fn water_gun(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
    pub fn spike_cannon(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Water, Type::Water])
            .flip_coins(2)
            .damage_per_heads(30)
    }
}

#[derive(Default)]
pub struct Sandslash41 {}
impl CardArchetype for Sandslash41 {
    identifier!("Sandslash (FO 41)");
    card_name!("Sandslash");
    stage1!("Sandshrew");
    hp!(70);
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
            .register("Slash", Self::slash)
            .register("Fury Swipes", Self::fury_swipes)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Sandslash41 {
    pub fn slash(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Colorless, Type::Colorless])
            .damage(20)
    }
    pub fn fury_swipes(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Fighting, Type::Fighting])
            .flip_coins(3)
            .damage_per_heads(20)
    }
}

#[derive(Default)]
pub struct Seadra42 {}
impl CardArchetype for Seadra42 {
    identifier!("Seadra (FO 42)");
    card_name!("Seadra");
    stage1!("Horsea");
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
            .register("Water Gun", Self::water_gun)
            .register("Agility", Self::agility)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Seadra42 {
    pub fn water_gun(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
    pub fn agility(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Water, Type::Colorless, Type::Colorless])
            .flip_a_coin()
            .if_heads(|e| e.prevent_damage_and_effects_during_opponents_next_turn())
    }
}

#[derive(Default)]
pub struct Slowbro43 {}
impl CardArchetype for Slowbro43 {
    identifier!("Slowbro (FO 43)");
    card_name!("Slowbro");
    stage1!("Slowpoke");
    hp!(60);
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
            .register("Psyshock", Self::psyshock)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Slowbro43 {
    pub fn psyshock(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Psychic, Type::Psychic])
            .flip_a_coin()
            .damage(20)
            .if_heads(|e| e.paralyze())
    }
}

#[derive(Default)]
pub struct Tentacruel44 {}
impl CardArchetype for Tentacruel44 {
    identifier!("Tentacruel (FO 44)");
    card_name!("Tentacruel");
    stage1!("Tentacool");
    hp!(60);
    color!(Water);
    weak_to!(Lightning);
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
            .register("Supersonic", Self::supersonic)
            .register("Jellyfish Sting", Self::jellyfish_sting)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Tentacruel44 {
    pub fn supersonic(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Water])
            .flip_a_coin()
            .if_heads(|e| e.confuse())
    }
    pub fn jellyfish_sting(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Water, Type::Water])
            .damage(10)
            .poison()
    }
}

#[derive(Default)]
pub struct Weezing45 {}
impl CardArchetype for Weezing45 {
    identifier!("Weezing (FO 45)");
    card_name!("Weezing");
    stage1!("Koffing");
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
            .register("Smog", Self::smog)
            .register("Selfdestruct", Self::selfdestruct)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Weezing45 {
    pub fn smog(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Grass, Type::Grass])
            .flip_a_coin()
            .damage(20)
            .if_heads(|e| e.poison())
    }
    pub fn selfdestruct(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Grass, Type::Grass, Type::Colorless])
            .damage(60)
            .each_own_bench(|e| e.damage(10))
            .each_opponents_bench(|e| e.damage(10))
            .damage_self(60)
    }
}

#[derive(Default)]
pub struct Ekans46 {}
impl CardArchetype for Ekans46 {
    identifier!("Ekans (FO 46)");
    card_name!("Ekans");
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
            .register("Spit Poison", Self::spit_poison)
            .register("Wrap", Self::wrap)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Ekans46 {
    pub fn spit_poison(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Grass])
            .flip_a_coin()
            .if_heads(|e| e.poison())
    }
    pub fn wrap(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Grass, Type::Colorless])
            .flip_a_coin()
            .damage(20)
            .if_heads(|e| e.paralyze())
    }
}

#[derive(Default)]
pub struct Geodude47 {}
impl CardArchetype for Geodude47 {
    identifier!("Geodude (FO 47)");
    card_name!("Geodude");
    basic!();
    hp!(50);
    color!(Fighting);
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
            .register("Stone Barrage", Self::stone_barrage)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Geodude47 {
    pub fn stone_barrage(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
}

#[derive(Default)]
pub struct Grimer48 {}
impl CardArchetype for Grimer48 {
    identifier!("Grimer (FO 48)");
    card_name!("Grimer");
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
            .register("Nasty Goo", Self::nasty_goo)
            .register("Minimize", Self::minimize)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Grimer48 {
    pub fn nasty_goo(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Colorless])
            .flip_a_coin()
            .damage(10)
            .if_heads(|e| e.paralyze())
    }
    pub fn minimize(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
}

#[derive(Default)]
pub struct Horsea49 {}
impl CardArchetype for Horsea49 {
    identifier!("Horsea (FO 49)");
    card_name!("Horsea");
    basic!();
    hp!(40);
    color!(Water);
    weak_to!(Lightning);
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
            .register("Smokescreen", Self::smokescreen)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Horsea49 {
    pub fn smokescreen(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Water])
            .prevent_attack_on_a_flip_during_opponents_next_turn()
    }
}

#[derive(Default)]
pub struct Kabuto50 {}
impl CardArchetype for Kabuto50 {
    identifier!("Kabuto (FO 50)");
    card_name!("Kabuto");
    stage1!("Mysterious Fossil");
    hp!(30);
    color!(Fighting);
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
            .register("Scratch", Self::scratch)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Kabuto50 {
    pub fn scratch(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Colorless])
            .damage(10)
    }
}

#[derive(Default)]
pub struct Krabby51 {}
impl CardArchetype for Krabby51 {
    identifier!("Krabby (FO 51)");
    card_name!("Krabby");
    basic!();
    hp!(50);
    color!(Water);
    weak_to!(Lightning);
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
            .register("Call for Family", Self::call_for_family)
            .register("Irongrip", Self::irongrip)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Krabby51 {
    pub fn call_for_family(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
    pub fn irongrip(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Water, Type::Colorless])
            .damage(20)
    }
}

#[derive(Default)]
pub struct Omanyte52 {}
impl CardArchetype for Omanyte52 {
    identifier!("Omanyte (FO 52)");
    card_name!("Omanyte");
    stage1!("Mysterious Fossil");
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
impl Omanyte52 {
    pub fn water_gun(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
}

#[derive(Default)]
pub struct Psyduck53 {}
impl CardArchetype for Psyduck53 {
    identifier!("Psyduck (FO 53)");
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
impl Psyduck53 {
    pub fn headache(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
    pub fn fury_swipes(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Water])
            .flip_coins(3)
            .damage_per_heads(10)
    }
}

#[derive(Default)]
pub struct Shellder54 {}
impl CardArchetype for Shellder54 {
    identifier!("Shellder (FO 54)");
    card_name!("Shellder");
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
            .register("Supersonic", Self::supersonic)
            .register("Hide in Shell", Self::hide_in_shell)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Shellder54 {
    pub fn supersonic(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Water])
            .flip_a_coin()
            .if_heads(|e| e.confuse())
    }
    pub fn hide_in_shell(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Water])
            .flip_a_coin()
            .if_heads(|e| e.prevent_damage_during_opponents_next_turn())
    }
}

#[derive(Default)]
pub struct Slowpoke55 {}
impl CardArchetype for Slowpoke55 {
    identifier!("Slowpoke (FO 55)");
    card_name!("Slowpoke");
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
            .register("Spacing Out", Self::spacing_out)
            .register("Scavenge", Self::scavenge)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Slowpoke55 {
    pub fn spacing_out(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
    pub fn scavenge(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
}

#[derive(Default)]
pub struct Tentacool56 {}
impl CardArchetype for Tentacool56 {
    identifier!("Tentacool (FO 56)");
    card_name!("Tentacool");
    basic!();
    hp!(30);
    color!(Water);
    weak_to!(Lightning);
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
            .register("Acid", Self::acid)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Tentacool56 {
    pub fn acid(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Water])
            .damage(10)
    }
}

#[derive(Default)]
pub struct Zubat57 {}
impl CardArchetype for Zubat57 {
    identifier!("Zubat (FO 57)");
    card_name!("Zubat");
    basic!();
    hp!(40);
    color!(Grass);
    weak_to!(Psychic);
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
            .register("Supersonic", Self::supersonic)
            .register("Leech Life", Self::leech_life)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Zubat57 {
    pub fn supersonic(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Colorless, Type::Colorless])
            .flip_a_coin()
            .if_heads(|e| e.confuse())
    }
    pub fn leech_life(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
}