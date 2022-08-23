use crate::*;
use crate::state::Type;
use crate::attack_builder::AttackBuilder;

#[derive(Default)]
pub struct Clefable1 {}
impl CardArchetype for Clefable1 {
    identifier!("Clefable (JU 1)");
    card_name!("Clefable");
    stage1!("Clefairy");
    hp!(70);
    color!(Colorless);
    weak_to!(Fighting);
    resists!(Psychic, 30);
    retreat!(2);

    fn card_actions(&self, _player: Player, _card: &Card, _engine: &GameEngine) -> Vec<Action> {
        vec![]
    }
    fn execute(&self, _player: Player, _card: &Card, engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine.clone()
    }
    fn attacks(&self) -> Vec<Attack> {
      vec![
        Attack::new("Metronome", Self::metronome),
        Attack::new("Minimize", Self::minimize),
      ]
    }
}
impl Clefable1 {
    pub fn metronome(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Colorless])
            .copy_defending_attack_without_costs()
    }
    pub fn minimize(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
}

#[derive(Default)]
pub struct Electrode2 {}
impl CardArchetype for Electrode2 {
    identifier!("Electrode (JU 2)");
    card_name!("Electrode");
    stage1!("Voltorb");
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
    fn attacks(&self) -> Vec<Attack> {
      vec![
        Attack::new("Tackle", Self::tackle),
        Attack::new("Chain Lightning", Self::chain_lightning),
      ]
    }
}
impl Electrode2 {
    pub fn tackle(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Colorless, Type::Colorless])
            .damage(20)
    }
    pub fn chain_lightning(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
}

#[derive(Default)]
pub struct Flareon3 {}
impl CardArchetype for Flareon3 {
    identifier!("Flareon (JU 3)");
    card_name!("Flareon");
    stage1!("Eevee");
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
    fn attacks(&self) -> Vec<Attack> {
      vec![
        Attack::new("Quick Attack", Self::quick_attack),
        Attack::new("Flamethrower", Self::flamethrower),
      ]
    }
}
impl Flareon3 {
    pub fn quick_attack(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Colorless, Type::Colorless])
            .flip_a_coin()
            .if_heads(|e| e.damage(30))
            .if_tails(|e| e.damage(10))
    }
    pub fn flamethrower(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Fire, Type::Fire, Type::Colorless, Type::Colorless])
            .cost(|e| e.discard_attacking_energy_cards(&[Type::Fire]))
            .damage(60)
    }
}

#[derive(Default)]
pub struct Jolteon4 {}
impl CardArchetype for Jolteon4 {
    identifier!("Jolteon (JU 4)");
    card_name!("Jolteon");
    stage1!("Eevee");
    hp!(70);
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
    fn attacks(&self) -> Vec<Attack> {
      vec![
        Attack::new("Quick Attack", Self::quick_attack),
        Attack::new("Pin Missile", Self::pin_missile),
      ]
    }
}
impl Jolteon4 {
    pub fn quick_attack(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Colorless, Type::Colorless])
            .flip_a_coin()
            .if_heads(|e| e.damage(30))
            .if_tails(|e| e.damage(10))
    }
    pub fn pin_missile(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Lightning, Type::Lightning, Type::Colorless])
            .flip_coins(4)
            .damage_per_heads(20)
    }
}

#[derive(Default)]
pub struct Kangaskhan5 {}
impl CardArchetype for Kangaskhan5 {
    identifier!("Kangaskhan (JU 5)");
    card_name!("Kangaskhan");
    basic!();
    hp!(90);
    color!(Colorless);
    weak_to!(Fighting);
    resists!(Psychic, 30);
    retreat!(3);

    fn card_actions(&self, _player: Player, _card: &Card, _engine: &GameEngine) -> Vec<Action> {
        vec![]
    }
    fn execute(&self, _player: Player, _card: &Card, engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine.clone()
    }
    fn attacks(&self) -> Vec<Attack> {
      vec![
        Attack::new("Fetch", Self::fetch),
        Attack::new("Comet Punch", Self::comet_punch),
      ]
    }
}
impl Kangaskhan5 {
    pub fn fetch(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Colorless])
            .draw(1)
    }
    pub fn comet_punch(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Colorless, Type::Colorless, Type::Colorless, Type::Colorless])
            .flip_coins(4)
            .damage_per_heads(20)
    }
}

#[derive(Default)]
pub struct MrMime6 {}
impl CardArchetype for MrMime6 {
    identifier!("Mr. Mime (JU 6)");
    card_name!("Mr. Mime");
    basic!();
    hp!(40);
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
    fn attacks(&self) -> Vec<Attack> {
      vec![
        Attack::new("Meditate", Self::meditate),
      ]
    }
}
impl MrMime6 {
    pub fn meditate(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Psychic, Type::Colorless])
            .damage_plus_per_damage_counter_on_defending(10, 10)
    }
}

#[derive(Default)]
pub struct Nidoqueen7 {}
impl CardArchetype for Nidoqueen7 {
    identifier!("Nidoqueen (JU 7)");
    card_name!("Nidoqueen");
    stage2!("Nidorina");
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
    fn attacks(&self) -> Vec<Attack> {
      vec![
        Attack::new("Boyfriends", Self::boyfriends),
        Attack::new("Mega Punch", Self::mega_punch),
      ]
    }
}
impl Nidoqueen7 {
    pub fn boyfriends(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
    pub fn mega_punch(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Grass, Type::Grass, Type::Colorless, Type::Colorless])
            .damage(50)
    }
}

#[derive(Default)]
pub struct Pidgeot8 {}
impl CardArchetype for Pidgeot8 {
    identifier!("Pidgeot (JU 8)");
    card_name!("Pidgeot");
    stage2!("Pidgeotto");
    hp!(80);
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
    fn attacks(&self) -> Vec<Attack> {
      vec![
        Attack::new("Wing Attack", Self::wing_attack),
        Attack::new("Hurricane", Self::hurricane),
      ]
    }
}
impl Pidgeot8 {
    pub fn wing_attack(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Colorless, Type::Colorless])
            .damage(20)
    }
    pub fn hurricane(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
}

#[derive(Default)]
pub struct Pinsir9 {}
impl CardArchetype for Pinsir9 {
    identifier!("Pinsir (JU 9)");
    card_name!("Pinsir");
    basic!();
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
    fn attacks(&self) -> Vec<Attack> {
      vec![
        Attack::new("Irongrip", Self::irongrip),
        Attack::new("Guillotine", Self::guillotine),
      ]
    }
}
impl Pinsir9 {
    pub fn irongrip(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Grass, Type::Grass])
            .flip_a_coin()
            .damage(20)
            .if_heads(|e| e.paralyze())
    }
    pub fn guillotine(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Grass, Type::Grass, Type::Colorless, Type::Colorless])
            .damage(50)
    }
}

#[derive(Default)]
pub struct Scyther10 {}
impl CardArchetype for Scyther10 {
    identifier!("Scyther (JU 10)");
    card_name!("Scyther");
    basic!();
    hp!(70);
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
    fn attacks(&self) -> Vec<Attack> {
      vec![
        Attack::new("Swords Dance", Self::swords_dance),
        Attack::new("Slash", Self::slash),
      ]
    }
}
impl Scyther10 {
    pub fn swords_dance(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
    pub fn slash(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Colorless, Type::Colorless, Type::Colorless])
            .damage(30)
    }
}

#[derive(Default)]
pub struct Snorlax11 {}
impl CardArchetype for Snorlax11 {
    identifier!("Snorlax (JU 11)");
    card_name!("Snorlax");
    basic!();
    hp!(90);
    color!(Colorless);
    weak_to!(Fighting);
    resists!(Psychic, 30);
    retreat!(4);

    fn card_actions(&self, _player: Player, _card: &Card, _engine: &GameEngine) -> Vec<Action> {
        vec![]
    }
    fn execute(&self, _player: Player, _card: &Card, engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine.clone()
    }
    fn attacks(&self) -> Vec<Attack> {
      vec![
        Attack::new("Body Slam", Self::body_slam),
      ]
    }
}
impl Snorlax11 {
    pub fn body_slam(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Colorless, Type::Colorless, Type::Colorless, Type::Colorless])
            .flip_a_coin()
            .damage(30)
            .if_heads(|e| e.paralyze())
    }
}

#[derive(Default)]
pub struct Vaporeon12 {}
impl CardArchetype for Vaporeon12 {
    identifier!("Vaporeon (JU 12)");
    card_name!("Vaporeon");
    stage1!("Eevee");
    hp!(80);
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
    fn attacks(&self) -> Vec<Attack> {
      vec![
        Attack::new("Quick Attack", Self::quick_attack),
        Attack::new("Water Gun", Self::water_gun),
      ]
    }
}
impl Vaporeon12 {
    pub fn quick_attack(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Colorless, Type::Colorless])
            .flip_a_coin()
            .if_heads(|e| e.damage(30))
            .if_tails(|e| e.damage(10))
    }
    pub fn water_gun(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Water, Type::Water, Type::Colorless])
            .damage_plus_per_extra_energy_on_attacking(30, 10, Type::Water, 2)
    }
}

#[derive(Default)]
pub struct Venomoth13 {}
impl CardArchetype for Venomoth13 {
    identifier!("Venomoth (JU 13)");
    card_name!("Venomoth");
    stage1!("Venonat");
    hp!(70);
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
    fn attacks(&self) -> Vec<Attack> {
      vec![
        Attack::new("Venom Powder", Self::venom_powder),
      ]
    }
}
impl Venomoth13 {
    pub fn venom_powder(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
}

#[derive(Default)]
pub struct Victreebel14 {}
impl CardArchetype for Victreebel14 {
    identifier!("Victreebel (JU 14)");
    card_name!("Victreebel");
    stage2!("Weepinbell");
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
    fn attacks(&self) -> Vec<Attack> {
      vec![
        Attack::new("Lure", Self::lure),
        Attack::new("Acid", Self::acid),
      ]
    }
}
impl Victreebel14 {
    pub fn lure(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Grass])
            .gust_defending()
    }
    pub fn acid(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
}

#[derive(Default)]
pub struct Vileplume15 {}
impl CardArchetype for Vileplume15 {
    identifier!("Vileplume (JU 15)");
    card_name!("Vileplume");
    stage2!("Gloom");
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
    fn attacks(&self) -> Vec<Attack> {
      vec![
        Attack::new("Petal Dance", Self::petal_dance),
      ]
    }
}
impl Vileplume15 {
    pub fn petal_dance(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Grass, Type::Grass, Type::Grass])
            .flip_coins(3)
            .damage_per_heads(40)
    }
}

#[derive(Default)]
pub struct Wigglytuff16 {}
impl CardArchetype for Wigglytuff16 {
    identifier!("Wigglytuff (JU 16)");
    card_name!("Wigglytuff");
    stage1!("Jigglypuff");
    hp!(80);
    color!(Colorless);
    weak_to!(Fighting);
    resists!(Psychic, 30);
    retreat!(2);

    fn card_actions(&self, _player: Player, _card: &Card, _engine: &GameEngine) -> Vec<Action> {
        vec![]
    }
    fn execute(&self, _player: Player, _card: &Card, engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine.clone()
    }
    fn attacks(&self) -> Vec<Attack> {
      vec![
        Attack::new("Lullaby", Self::lullaby),
        Attack::new("Do the Wave", Self::do_the_wave),
      ]
    }
}
impl Wigglytuff16 {
    pub fn lullaby(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Colorless])
            .asleep()
    }
    pub fn do_the_wave(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
}

#[derive(Default)]
pub struct Clefable17 {}
impl CardArchetype for Clefable17 {
    identifier!("Clefable (JU 17)");
    card_name!("Clefable");
    stage1!("Clefairy");
    hp!(70);
    color!(Colorless);
    weak_to!(Fighting);
    resists!(Psychic, 30);
    retreat!(2);

    fn card_actions(&self, _player: Player, _card: &Card, _engine: &GameEngine) -> Vec<Action> {
        vec![]
    }
    fn execute(&self, _player: Player, _card: &Card, engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine.clone()
    }
    fn attacks(&self) -> Vec<Attack> {
      vec![
        Attack::new("Metronome", Self::metronome),
        Attack::new("Minimize", Self::minimize),
      ]
    }
}
impl Clefable17 {
    pub fn metronome(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Colorless])
            .copy_defending_attack_without_costs()
    }
    pub fn minimize(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
}

#[derive(Default)]
pub struct Electrode18 {}
impl CardArchetype for Electrode18 {
    identifier!("Electrode (JU 18)");
    card_name!("Electrode");
    stage1!("Voltorb");
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
    fn attacks(&self) -> Vec<Attack> {
      vec![
        Attack::new("Tackle", Self::tackle),
        Attack::new("Chain Lightning", Self::chain_lightning),
      ]
    }
}
impl Electrode18 {
    pub fn tackle(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Colorless, Type::Colorless])
            .damage(20)
    }
    pub fn chain_lightning(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
}

#[derive(Default)]
pub struct Flareon19 {}
impl CardArchetype for Flareon19 {
    identifier!("Flareon (JU 19)");
    card_name!("Flareon");
    stage1!("Eevee");
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
    fn attacks(&self) -> Vec<Attack> {
      vec![
        Attack::new("Quick Attack", Self::quick_attack),
        Attack::new("Flamethrower", Self::flamethrower),
      ]
    }
}
impl Flareon19 {
    pub fn quick_attack(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Colorless, Type::Colorless])
            .flip_a_coin()
            .if_heads(|e| e.damage(30))
            .if_tails(|e| e.damage(10))
    }
    pub fn flamethrower(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Fire, Type::Fire, Type::Colorless, Type::Colorless])
            .cost(|e| e.discard_attacking_energy_cards(&[Type::Fire]))
            .damage(60)
    }
}

#[derive(Default)]
pub struct Jolteon20 {}
impl CardArchetype for Jolteon20 {
    identifier!("Jolteon (JU 20)");
    card_name!("Jolteon");
    stage1!("Eevee");
    hp!(70);
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
    fn attacks(&self) -> Vec<Attack> {
      vec![
        Attack::new("Quick Attack", Self::quick_attack),
        Attack::new("Pin Missile", Self::pin_missile),
      ]
    }
}
impl Jolteon20 {
    pub fn quick_attack(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Colorless, Type::Colorless])
            .flip_a_coin()
            .if_heads(|e| e.damage(30))
            .if_tails(|e| e.damage(10))
    }
    pub fn pin_missile(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Lightning, Type::Lightning, Type::Colorless])
            .flip_coins(4)
            .damage_per_heads(20)
    }
}

#[derive(Default)]
pub struct Kangaskhan21 {}
impl CardArchetype for Kangaskhan21 {
    identifier!("Kangaskhan (JU 21)");
    card_name!("Kangaskhan");
    basic!();
    hp!(90);
    color!(Colorless);
    weak_to!(Fighting);
    resists!(Psychic, 30);
    retreat!(3);

    fn card_actions(&self, _player: Player, _card: &Card, _engine: &GameEngine) -> Vec<Action> {
        vec![]
    }
    fn execute(&self, _player: Player, _card: &Card, engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine.clone()
    }
    fn attacks(&self) -> Vec<Attack> {
      vec![
        Attack::new("Fetch", Self::fetch),
        Attack::new("Comet Punch", Self::comet_punch),
      ]
    }
}
impl Kangaskhan21 {
    pub fn fetch(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Colorless])
            .draw(1)
    }
    pub fn comet_punch(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Colorless, Type::Colorless, Type::Colorless, Type::Colorless])
            .flip_coins(4)
            .damage_per_heads(20)
    }
}

#[derive(Default)]
pub struct MrMime22 {}
impl CardArchetype for MrMime22 {
    identifier!("Mr. Mime (JU 22)");
    card_name!("Mr. Mime");
    basic!();
    hp!(40);
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
    fn attacks(&self) -> Vec<Attack> {
      vec![
        Attack::new("Meditate", Self::meditate),
      ]
    }
}
impl MrMime22 {
    pub fn meditate(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Psychic, Type::Colorless])
            .damage_plus_per_damage_counter_on_defending(10, 10)
    }
}

#[derive(Default)]
pub struct Nidoqueen23 {}
impl CardArchetype for Nidoqueen23 {
    identifier!("Nidoqueen (JU 23)");
    card_name!("Nidoqueen");
    stage2!("Nidorina");
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
    fn attacks(&self) -> Vec<Attack> {
      vec![
        Attack::new("Boyfriends", Self::boyfriends),
        Attack::new("Mega Punch", Self::mega_punch),
      ]
    }
}
impl Nidoqueen23 {
    pub fn boyfriends(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
    pub fn mega_punch(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Grass, Type::Grass, Type::Colorless, Type::Colorless])
            .damage(50)
    }
}

#[derive(Default)]
pub struct Pidgeot24 {}
impl CardArchetype for Pidgeot24 {
    identifier!("Pidgeot (JU 24)");
    card_name!("Pidgeot");
    stage2!("Pidgeotto");
    hp!(80);
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
    fn attacks(&self) -> Vec<Attack> {
      vec![
        Attack::new("Wing Attack", Self::wing_attack),
        Attack::new("Hurricane", Self::hurricane),
      ]
    }
}
impl Pidgeot24 {
    pub fn wing_attack(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Colorless, Type::Colorless])
            .damage(20)
    }
    pub fn hurricane(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
}

#[derive(Default)]
pub struct Pinsir25 {}
impl CardArchetype for Pinsir25 {
    identifier!("Pinsir (JU 25)");
    card_name!("Pinsir");
    basic!();
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
    fn attacks(&self) -> Vec<Attack> {
      vec![
        Attack::new("Irongrip", Self::irongrip),
        Attack::new("Guillotine", Self::guillotine),
      ]
    }
}
impl Pinsir25 {
    pub fn irongrip(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Grass, Type::Grass])
            .flip_a_coin()
            .damage(20)
            .if_heads(|e| e.paralyze())
    }
    pub fn guillotine(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Grass, Type::Grass, Type::Colorless, Type::Colorless])
            .damage(50)
    }
}

#[derive(Default)]
pub struct Scyther26 {}
impl CardArchetype for Scyther26 {
    identifier!("Scyther (JU 26)");
    card_name!("Scyther");
    basic!();
    hp!(70);
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
    fn attacks(&self) -> Vec<Attack> {
      vec![
        Attack::new("Swords Dance", Self::swords_dance),
        Attack::new("Slash", Self::slash),
      ]
    }
}
impl Scyther26 {
    pub fn swords_dance(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
    pub fn slash(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Colorless, Type::Colorless, Type::Colorless])
            .damage(30)
    }
}

#[derive(Default)]
pub struct Snorlax27 {}
impl CardArchetype for Snorlax27 {
    identifier!("Snorlax (JU 27)");
    card_name!("Snorlax");
    basic!();
    hp!(90);
    color!(Colorless);
    weak_to!(Fighting);
    resists!(Psychic, 30);
    retreat!(4);

    fn card_actions(&self, _player: Player, _card: &Card, _engine: &GameEngine) -> Vec<Action> {
        vec![]
    }
    fn execute(&self, _player: Player, _card: &Card, engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine.clone()
    }
    fn attacks(&self) -> Vec<Attack> {
      vec![
        Attack::new("Body Slam", Self::body_slam),
      ]
    }
}
impl Snorlax27 {
    pub fn body_slam(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Colorless, Type::Colorless, Type::Colorless, Type::Colorless])
            .flip_a_coin()
            .damage(30)
            .if_heads(|e| e.paralyze())
    }
}

#[derive(Default)]
pub struct Vaporeon28 {}
impl CardArchetype for Vaporeon28 {
    identifier!("Vaporeon (JU 28)");
    card_name!("Vaporeon");
    stage1!("Eevee");
    hp!(80);
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
    fn attacks(&self) -> Vec<Attack> {
      vec![
        Attack::new("Quick Attack", Self::quick_attack),
        Attack::new("Water Gun", Self::water_gun),
      ]
    }
}
impl Vaporeon28 {
    pub fn quick_attack(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Colorless, Type::Colorless])
            .flip_a_coin()
            .if_heads(|e| e.damage(30))
            .if_tails(|e| e.damage(10))
    }
    pub fn water_gun(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Water, Type::Water, Type::Colorless])
            .damage_plus_per_extra_energy_on_attacking(30, 10, Type::Water, 2)
    }
}

#[derive(Default)]
pub struct Venomoth29 {}
impl CardArchetype for Venomoth29 {
    identifier!("Venomoth (JU 29)");
    card_name!("Venomoth");
    stage1!("Venonat");
    hp!(70);
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
    fn attacks(&self) -> Vec<Attack> {
      vec![
        Attack::new("Venom Powder", Self::venom_powder),
      ]
    }
}
impl Venomoth29 {
    pub fn venom_powder(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
}

#[derive(Default)]
pub struct Victreebel30 {}
impl CardArchetype for Victreebel30 {
    identifier!("Victreebel (JU 30)");
    card_name!("Victreebel");
    stage2!("Weepinbell");
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
    fn attacks(&self) -> Vec<Attack> {
      vec![
        Attack::new("Lure", Self::lure),
        Attack::new("Acid", Self::acid),
      ]
    }
}
impl Victreebel30 {
    pub fn lure(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Grass])
            .gust_defending()
    }
    pub fn acid(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
}

#[derive(Default)]
pub struct Vileplume31 {}
impl CardArchetype for Vileplume31 {
    identifier!("Vileplume (JU 31)");
    card_name!("Vileplume");
    stage2!("Gloom");
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
    fn attacks(&self) -> Vec<Attack> {
      vec![
        Attack::new("Petal Dance", Self::petal_dance),
      ]
    }
}
impl Vileplume31 {
    pub fn petal_dance(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Grass, Type::Grass, Type::Grass])
            .flip_coins(3)
            .damage_per_heads(40)
    }
}

#[derive(Default)]
pub struct Wigglytuff32 {}
impl CardArchetype for Wigglytuff32 {
    identifier!("Wigglytuff (JU 32)");
    card_name!("Wigglytuff");
    stage1!("Jigglypuff");
    hp!(80);
    color!(Colorless);
    weak_to!(Fighting);
    resists!(Psychic, 30);
    retreat!(2);

    fn card_actions(&self, _player: Player, _card: &Card, _engine: &GameEngine) -> Vec<Action> {
        vec![]
    }
    fn execute(&self, _player: Player, _card: &Card, engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine.clone()
    }
    fn attacks(&self) -> Vec<Attack> {
      vec![
        Attack::new("Lullaby", Self::lullaby),
        Attack::new("Do the Wave", Self::do_the_wave),
      ]
    }
}
impl Wigglytuff32 {
    pub fn lullaby(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Colorless])
            .asleep()
    }
    pub fn do_the_wave(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
}

#[derive(Default)]
pub struct Butterfree33 {}
impl CardArchetype for Butterfree33 {
    identifier!("Butterfree (JU 33)");
    card_name!("Butterfree");
    stage2!("Metapod");
    hp!(70);
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
    fn attacks(&self) -> Vec<Attack> {
      vec![
        Attack::new("Whirlwind", Self::whirlwind),
        Attack::new("Mega Drain", Self::mega_drain),
      ]
    }
}
impl Butterfree33 {
    pub fn whirlwind(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Colorless, Type::Colorless])
            .damage(20)
            .switch_defending()
    }
    pub fn mega_drain(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
}

#[derive(Default)]
pub struct Dodrio34 {}
impl CardArchetype for Dodrio34 {
    identifier!("Dodrio (JU 34)");
    card_name!("Dodrio");
    stage1!("Doduo");
    hp!(70);
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
    fn attacks(&self) -> Vec<Attack> {
      vec![
        Attack::new("Rage", Self::rage),
      ]
    }
}
impl Dodrio34 {
    pub fn rage(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
}

#[derive(Default)]
pub struct Exeggutor35 {}
impl CardArchetype for Exeggutor35 {
    identifier!("Exeggutor (JU 35)");
    card_name!("Exeggutor");
    stage1!("Exeggcute");
    hp!(80);
    color!(Grass);
    weak_to!(Fire);
    no_resistance!();
    retreat!(3);

    fn card_actions(&self, _player: Player, _card: &Card, _engine: &GameEngine) -> Vec<Action> {
        vec![]
    }
    fn execute(&self, _player: Player, _card: &Card, engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine.clone()
    }
    fn attacks(&self) -> Vec<Attack> {
      vec![
        Attack::new("Teleport", Self::teleport),
        Attack::new("Big Eggsplosion", Self::big_eggsplosion),
      ]
    }
}
impl Exeggutor35 {
    pub fn teleport(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
    pub fn big_eggsplosion(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
}

#[derive(Default)]
pub struct Fearow36 {}
impl CardArchetype for Fearow36 {
    identifier!("Fearow (JU 36)");
    card_name!("Fearow");
    stage1!("Spearow");
    hp!(70);
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
    fn attacks(&self) -> Vec<Attack> {
      vec![
        Attack::new("Agility", Self::agility),
        Attack::new("Drill Peck", Self::drill_peck),
      ]
    }
}
impl Fearow36 {
    pub fn agility(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Colorless, Type::Colorless, Type::Colorless])
            .flip_a_coin()
            .if_heads(|e| e.prevent_damage_and_effects_during_opponents_next_turn())
    }
    pub fn drill_peck(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Colorless, Type::Colorless, Type::Colorless, Type::Colorless])
            .damage(40)
    }
}

#[derive(Default)]
pub struct Gloom37 {}
impl CardArchetype for Gloom37 {
    identifier!("Gloom (JU 37)");
    card_name!("Gloom");
    stage1!("Oddish");
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
    fn attacks(&self) -> Vec<Attack> {
      vec![
        Attack::new("Poisonpowder", Self::poisonpowder),
        Attack::new("Foul Odor", Self::foul_odor),
      ]
    }
}
impl Gloom37 {
    pub fn poisonpowder(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Grass])
            .poison()
    }
    pub fn foul_odor(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
}

#[derive(Default)]
pub struct Lickitung38 {}
impl CardArchetype for Lickitung38 {
    identifier!("Lickitung (JU 38)");
    card_name!("Lickitung");
    basic!();
    hp!(90);
    color!(Colorless);
    weak_to!(Fighting);
    resists!(Psychic, 30);
    retreat!(3);

    fn card_actions(&self, _player: Player, _card: &Card, _engine: &GameEngine) -> Vec<Action> {
        vec![]
    }
    fn execute(&self, _player: Player, _card: &Card, engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine.clone()
    }
    fn attacks(&self) -> Vec<Attack> {
      vec![
        Attack::new("Tongue Wrap", Self::tongue_wrap),
        Attack::new("Supersonic", Self::supersonic),
      ]
    }
}
impl Lickitung38 {
    pub fn tongue_wrap(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Colorless])
            .flip_a_coin()
            .damage(10)
            .if_heads(|e| e.paralyze())
    }
    pub fn supersonic(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Colorless, Type::Colorless])
            .flip_a_coin()
            .if_heads(|e| e.confuse())
    }
}

#[derive(Default)]
pub struct Marowak39 {}
impl CardArchetype for Marowak39 {
    identifier!("Marowak (JU 39)");
    card_name!("Marowak");
    stage1!("Cubone");
    hp!(60);
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
    fn attacks(&self) -> Vec<Attack> {
      vec![
        Attack::new("Bonemerang", Self::bonemerang),
        Attack::new("Call for Friend", Self::call_for_friend),
      ]
    }
}
impl Marowak39 {
    pub fn bonemerang(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Fighting, Type::Fighting])
            .flip_coins(2)
            .damage_per_heads(30)
    }
    pub fn call_for_friend(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
}

#[derive(Default)]
pub struct Nidorina40 {}
impl CardArchetype for Nidorina40 {
    identifier!("Nidorina (JU 40)");
    card_name!("Nidorina");
    stage1!("Nidoran ♀");
    hp!(70);
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
    fn attacks(&self) -> Vec<Attack> {
      vec![
        Attack::new("Supersonic", Self::supersonic),
        Attack::new("Double Kick", Self::double_kick),
      ]
    }
}
impl Nidorina40 {
    pub fn supersonic(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Grass])
            .flip_a_coin()
            .if_heads(|e| e.confuse())
    }
    pub fn double_kick(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Grass, Type::Colorless, Type::Colorless])
            .flip_coins(2)
            .damage_per_heads(30)
    }
}

#[derive(Default)]
pub struct Parasect41 {}
impl CardArchetype for Parasect41 {
    identifier!("Parasect (JU 41)");
    card_name!("Parasect");
    stage1!("Paras");
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
    fn attacks(&self) -> Vec<Attack> {
      vec![
        Attack::new("Spore", Self::spore),
        Attack::new("Slash", Self::slash),
      ]
    }
}
impl Parasect41 {
    pub fn spore(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Grass, Type::Grass])
            .asleep()
    }
    pub fn slash(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Colorless, Type::Colorless, Type::Colorless])
            .damage(30)
    }
}

#[derive(Default)]
pub struct Persian42 {}
impl CardArchetype for Persian42 {
    identifier!("Persian (JU 42)");
    card_name!("Persian");
    stage1!("Meowth");
    hp!(70);
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
    fn attacks(&self) -> Vec<Attack> {
      vec![
        Attack::new("Scratch", Self::scratch),
        Attack::new("Pounce", Self::pounce),
      ]
    }
}
impl Persian42 {
    pub fn scratch(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Colorless, Type::Colorless])
            .damage(20)
    }
    pub fn pounce(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
}

#[derive(Default)]
pub struct Primeape43 {}
impl CardArchetype for Primeape43 {
    identifier!("Primeape (JU 43)");
    card_name!("Primeape");
    stage1!("Mankey");
    hp!(70);
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
    fn attacks(&self) -> Vec<Attack> {
      vec![
        Attack::new("Fury Swipes", Self::fury_swipes),
        Attack::new("Tantrum", Self::tantrum),
      ]
    }
}
impl Primeape43 {
    pub fn fury_swipes(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Fighting, Type::Fighting])
            .flip_coins(3)
            .damage_per_heads(20)
    }
    pub fn tantrum(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
}

#[derive(Default)]
pub struct Rapidash44 {}
impl CardArchetype for Rapidash44 {
    identifier!("Rapidash (JU 44)");
    card_name!("Rapidash");
    stage1!("Ponyta");
    hp!(70);
    color!(Fire);
    weak_to!(Water);
    no_resistance!();
    retreat!(0);

    fn card_actions(&self, _player: Player, _card: &Card, _engine: &GameEngine) -> Vec<Action> {
        vec![]
    }
    fn execute(&self, _player: Player, _card: &Card, engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine.clone()
    }
    fn attacks(&self) -> Vec<Attack> {
      vec![
        Attack::new("Stomp", Self::stomp),
        Attack::new("Agility", Self::agility),
      ]
    }
}
impl Rapidash44 {
    pub fn stomp(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Colorless, Type::Colorless])
            .flip_a_coin()
            .if_heads(|e| e.damage(30))
            .if_tails(|e| e.damage(20))
    }
    pub fn agility(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Fire, Type::Fire, Type::Colorless])
            .flip_a_coin()
            .if_heads(|e| e.prevent_damage_and_effects_during_opponents_next_turn())
    }
}

#[derive(Default)]
pub struct Rhydon45 {}
impl CardArchetype for Rhydon45 {
    identifier!("Rhydon (JU 45)");
    card_name!("Rhydon");
    stage1!("Rhyhorn");
    hp!(100);
    color!(Fighting);
    weak_to!(Grass);
    resists!(Lightning, 30);
    retreat!(3);

    fn card_actions(&self, _player: Player, _card: &Card, _engine: &GameEngine) -> Vec<Action> {
        vec![]
    }
    fn execute(&self, _player: Player, _card: &Card, engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine.clone()
    }
    fn attacks(&self) -> Vec<Attack> {
      vec![
        Attack::new("Horn Attack", Self::horn_attack),
        Attack::new("Ram", Self::ram),
      ]
    }
}
impl Rhydon45 {
    pub fn horn_attack(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Fighting, Type::Colorless, Type::Colorless])
            .damage(30)
    }
    pub fn ram(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
}

#[derive(Default)]
pub struct Seaking46 {}
impl CardArchetype for Seaking46 {
    identifier!("Seaking (JU 46)");
    card_name!("Seaking");
    stage1!("Goldeen");
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
    fn attacks(&self) -> Vec<Attack> {
      vec![
        Attack::new("Horn Attack", Self::horn_attack),
        Attack::new("Waterfall", Self::waterfall),
      ]
    }
}
impl Seaking46 {
    pub fn horn_attack(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Water])
            .damage(10)
    }
    pub fn waterfall(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Water, Type::Colorless])
            .damage(30)
    }
}

#[derive(Default)]
pub struct Tauros47 {}
impl CardArchetype for Tauros47 {
    identifier!("Tauros (JU 47)");
    card_name!("Tauros");
    basic!();
    hp!(60);
    color!(Colorless);
    weak_to!(Fighting);
    resists!(Psychic, 30);
    retreat!(2);

    fn card_actions(&self, _player: Player, _card: &Card, _engine: &GameEngine) -> Vec<Action> {
        vec![]
    }
    fn execute(&self, _player: Player, _card: &Card, engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine.clone()
    }
    fn attacks(&self) -> Vec<Attack> {
      vec![
        Attack::new("Stomp", Self::stomp),
        Attack::new("Rampage", Self::rampage),
      ]
    }
}
impl Tauros47 {
    pub fn stomp(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Colorless, Type::Colorless])
            .flip_a_coin()
            .if_heads(|e| e.damage(30))
            .if_tails(|e| e.damage(20))
    }
    pub fn rampage(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
}

#[derive(Default)]
pub struct Weepinbell48 {}
impl CardArchetype for Weepinbell48 {
    identifier!("Weepinbell (JU 48)");
    card_name!("Weepinbell");
    stage1!("Bellsprout");
    hp!(70);
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
    fn attacks(&self) -> Vec<Attack> {
      vec![
        Attack::new("Poisonpowder", Self::poisonpowder),
        Attack::new("Razor Leaf", Self::razor_leaf),
      ]
    }
}
impl Weepinbell48 {
    pub fn poisonpowder(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Grass])
            .flip_a_coin()
            .damage(10)
            .if_heads(|e| e.poison())
    }
    pub fn razor_leaf(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Grass, Type::Grass])
            .damage(30)
    }
}

#[derive(Default)]
pub struct Bellsprout49 {}
impl CardArchetype for Bellsprout49 {
    identifier!("Bellsprout (JU 49)");
    card_name!("Bellsprout");
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
    fn attacks(&self) -> Vec<Attack> {
      vec![
        Attack::new("Vine Whip", Self::vine_whip),
        Attack::new("Call for Family", Self::call_for_family),
      ]
    }
}
impl Bellsprout49 {
    pub fn vine_whip(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Grass])
            .damage(10)
    }
    pub fn call_for_family(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
}

#[derive(Default)]
pub struct Cubone50 {}
impl CardArchetype for Cubone50 {
    identifier!("Cubone (JU 50)");
    card_name!("Cubone");
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
    fn attacks(&self) -> Vec<Attack> {
      vec![
        Attack::new("Snivel", Self::snivel),
        Attack::new("Rage", Self::rage),
      ]
    }
}
impl Cubone50 {
    pub fn snivel(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
    pub fn rage(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
}

#[derive(Default)]
pub struct Eevee51 {}
impl CardArchetype for Eevee51 {
    identifier!("Eevee (JU 51)");
    card_name!("Eevee");
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
    fn attacks(&self) -> Vec<Attack> {
      vec![
        Attack::new("Tail Wag", Self::tail_wag),
        Attack::new("Quick Attack", Self::quick_attack),
      ]
    }
}
impl Eevee51 {
    pub fn tail_wag(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
    pub fn quick_attack(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Colorless, Type::Colorless])
            .flip_a_coin()
            .if_heads(|e| e.damage(30))
            .if_tails(|e| e.damage(10))
    }
}

#[derive(Default)]
pub struct Exeggcute52 {}
impl CardArchetype for Exeggcute52 {
    identifier!("Exeggcute (JU 52)");
    card_name!("Exeggcute");
    basic!();
    hp!(50);
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
    fn attacks(&self) -> Vec<Attack> {
      vec![
        Attack::new("Hypnosis", Self::hypnosis),
        Attack::new("Leech Seed", Self::leech_seed),
      ]
    }
}
impl Exeggcute52 {
    pub fn hypnosis(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Psychic])
            .asleep()
    }
    pub fn leech_seed(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Grass, Type::Grass])
            .damage(20)
            .if_did_damage(|e| e.heal_attacking(10))
    }
}

#[derive(Default)]
pub struct Goldeen53 {}
impl CardArchetype for Goldeen53 {
    identifier!("Goldeen (JU 53)");
    card_name!("Goldeen");
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
    fn attacks(&self) -> Vec<Attack> {
      vec![
        Attack::new("Horn Attack", Self::horn_attack),
      ]
    }
}
impl Goldeen53 {
    pub fn horn_attack(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Water])
            .damage(10)
    }
}

#[derive(Default)]
pub struct Jigglypuff54 {}
impl CardArchetype for Jigglypuff54 {
    identifier!("Jigglypuff (JU 54)");
    card_name!("Jigglypuff");
    basic!();
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
    fn attacks(&self) -> Vec<Attack> {
      vec![
        Attack::new("Lullaby", Self::lullaby),
        Attack::new("Pound", Self::pound),
      ]
    }
}
impl Jigglypuff54 {
    pub fn lullaby(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Colorless])
            .asleep()
    }
    pub fn pound(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Colorless, Type::Colorless])
            .damage(20)
    }
}

#[derive(Default)]
pub struct Mankey55 {}
impl CardArchetype for Mankey55 {
    identifier!("Mankey (JU 55)");
    card_name!("Mankey");
    basic!();
    hp!(30);
    color!(Fighting);
    weak_to!(Psychic);
    no_resistance!();
    retreat!(0);

    fn card_actions(&self, _player: Player, _card: &Card, _engine: &GameEngine) -> Vec<Action> {
        vec![]
    }
    fn execute(&self, _player: Player, _card: &Card, engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine.clone()
    }
    fn attacks(&self) -> Vec<Attack> {
      vec![
        Attack::new("Scratch", Self::scratch),
      ]
    }
}
impl Mankey55 {
    pub fn scratch(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Colorless])
            .damage(10)
    }
}

#[derive(Default)]
pub struct Meowth56 {}
impl CardArchetype for Meowth56 {
    identifier!("Meowth (JU 56)");
    card_name!("Meowth");
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
    fn attacks(&self) -> Vec<Attack> {
      vec![
        Attack::new("Pay Day", Self::pay_day),
      ]
    }
}
impl Meowth56 {
    pub fn pay_day(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Colorless, Type::Colorless])
            .flip_a_coin()
            .damage(10)
            .if_heads(|e| e.draw(1))
    }
}

#[derive(Default)]
pub struct NidoranF57 {}
impl CardArchetype for NidoranF57 {
    identifier!("Nidoran ♀ (JU 57)");
    card_name!("Nidoran ♀");
    basic!();
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
    fn attacks(&self) -> Vec<Attack> {
      vec![
        Attack::new("Fury Swipes", Self::fury_swipes),
        Attack::new("Call for Family", Self::call_for_family),
      ]
    }
}
impl NidoranF57 {
    pub fn fury_swipes(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Grass])
            .flip_coins(3)
            .damage_per_heads(10)
    }
    pub fn call_for_family(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
}

#[derive(Default)]
pub struct Oddish58 {}
impl CardArchetype for Oddish58 {
    identifier!("Oddish (JU 58)");
    card_name!("Oddish");
    basic!();
    hp!(50);
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
    fn attacks(&self) -> Vec<Attack> {
      vec![
        Attack::new("Stun Spore", Self::stun_spore),
        Attack::new("Sprout", Self::sprout),
      ]
    }
}
impl Oddish58 {
    pub fn stun_spore(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Grass])
            .flip_a_coin()
            .damage(10)
            .if_heads(|e| e.paralyze())
    }
    pub fn sprout(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
}

#[derive(Default)]
pub struct Paras59 {}
impl CardArchetype for Paras59 {
    identifier!("Paras (JU 59)");
    card_name!("Paras");
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
    fn attacks(&self) -> Vec<Attack> {
      vec![
        Attack::new("Scratch", Self::scratch),
        Attack::new("Spore", Self::spore),
      ]
    }
}
impl Paras59 {
    pub fn scratch(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Colorless, Type::Colorless])
            .damage(20)
    }
    pub fn spore(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Grass, Type::Grass])
            .asleep()
    }
}

#[derive(Default)]
pub struct Pikachu60 {}
impl CardArchetype for Pikachu60 {
    identifier!("Pikachu (JU 60)");
    card_name!("Pikachu");
    basic!();
    hp!(50);
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
    fn attacks(&self) -> Vec<Attack> {
      vec![
        Attack::new("Spark", Self::spark),
      ]
    }
}
impl Pikachu60 {
    pub fn spark(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
}

#[derive(Default)]
pub struct Rhyhorn61 {}
impl CardArchetype for Rhyhorn61 {
    identifier!("Rhyhorn (JU 61)");
    card_name!("Rhyhorn");
    basic!();
    hp!(70);
    color!(Fighting);
    weak_to!(Grass);
    resists!(Lightning, 30);
    retreat!(3);

    fn card_actions(&self, _player: Player, _card: &Card, _engine: &GameEngine) -> Vec<Action> {
        vec![]
    }
    fn execute(&self, _player: Player, _card: &Card, engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine.clone()
    }
    fn attacks(&self) -> Vec<Attack> {
      vec![
        Attack::new("Leer", Self::leer),
        Attack::new("Horn Attack", Self::horn_attack),
      ]
    }
}
impl Rhyhorn61 {
    pub fn leer(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
    pub fn horn_attack(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Fighting, Type::Colorless, Type::Colorless])
            .damage(30)
    }
}

#[derive(Default)]
pub struct Spearow62 {}
impl CardArchetype for Spearow62 {
    identifier!("Spearow (JU 62)");
    card_name!("Spearow");
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
    fn attacks(&self) -> Vec<Attack> {
      vec![
        Attack::new("Peck", Self::peck),
        Attack::new("Mirror Move", Self::mirror_move),
      ]
    }
}
impl Spearow62 {
    pub fn peck(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Colorless])
            .damage(10)
    }
    pub fn mirror_move(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
}

#[derive(Default)]
pub struct Venonat63 {}
impl CardArchetype for Venonat63 {
    identifier!("Venonat (JU 63)");
    card_name!("Venonat");
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
    fn attacks(&self) -> Vec<Attack> {
      vec![
        Attack::new("Stun Spore", Self::stun_spore),
        Attack::new("Leech Life", Self::leech_life),
      ]
    }
}
impl Venonat63 {
    pub fn stun_spore(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Grass])
            .flip_a_coin()
            .damage(10)
            .if_heads(|e| e.paralyze())
    }
    pub fn leech_life(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
}