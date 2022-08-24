use crate::*;
use crate::state::Type;
use crate::attack_builder::AttackBuilder;

#[derive(Default)]
pub struct DarkAlakazam1 {}
impl CardArchetype for DarkAlakazam1 {
    identifier!("Dark Alakazam (TR 1)");
    card_name!("Dark Alakazam");
    stage2!("Dark Kadabra");
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
    fn attacks(&self) -> Vec<Attack> {
        vec![
            Attack::new("Teleport Blast", Self::teleport_blast),
            Attack::new("Mind Shock", Self::mind_shock),
        ]
    }
}
impl DarkAlakazam1 {
    pub fn teleport_blast(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
    pub fn mind_shock(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
}

#[derive(Default)]
pub struct DarkArbok2 {}
impl CardArchetype for DarkArbok2 {
    identifier!("Dark Arbok (TR 2)");
    card_name!("Dark Arbok");
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
    fn attacks(&self) -> Vec<Attack> {
        vec![
            Attack::new("Stare", Self::stare),
            Attack::new("Poison Vapor", Self::poison_vapor),
        ]
    }
}
impl DarkArbok2 {
    pub fn stare(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
    pub fn poison_vapor(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
}

#[derive(Default)]
pub struct DarkBlastoise3 {}
impl CardArchetype for DarkBlastoise3 {
    identifier!("Dark Blastoise (TR 3)");
    card_name!("Dark Blastoise");
    stage2!("Dark Wartortle");
    hp!(70);
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
    fn attacks(&self) -> Vec<Attack> {
        vec![
            Attack::new("Hydrocannon", Self::hydrocannon),
            Attack::new("Rocket Tackle", Self::rocket_tackle),
        ]
    }
}
impl DarkBlastoise3 {
    pub fn hydrocannon(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
    pub fn rocket_tackle(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
}

#[derive(Default)]
pub struct DarkCharizard4 {}
impl CardArchetype for DarkCharizard4 {
    identifier!("Dark Charizard (TR 4)");
    card_name!("Dark Charizard");
    stage2!("Dark Charmeleon");
    hp!(80);
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
    fn attacks(&self) -> Vec<Attack> {
        vec![
            Attack::new("Nail Flick", Self::nail_flick),
            Attack::new("Continuous Fireball", Self::continuous_fireball),
        ]
    }
}
impl DarkCharizard4 {
    pub fn nail_flick(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Colorless])
            .damage(10)
    }
    pub fn continuous_fireball(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
}

#[derive(Default)]
pub struct DarkDragonite5 {}
impl CardArchetype for DarkDragonite5 {
    identifier!("Dark Dragonite (TR 5)");
    card_name!("Dark Dragonite");
    stage2!("Dark Dragonair");
    hp!(70);
    color!(Colorless);
    no_weakness!();
    resists!(Fighting, 30);
    retreat!(2);

    fn card_actions(&self, _player: Player, _card: &Card, _engine: &GameEngine) -> Vec<Action> {
        vec![]
    }
    fn execute(&self, _player: Player, _card: &Card, engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine.clone()
    }
    fn attacks(&self) -> Vec<Attack> {
        vec![
            Attack::new("Giant Tail", Self::giant_tail),
        ]
    }
}
impl DarkDragonite5 {
    pub fn giant_tail(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Colorless, Type::Colorless, Type::Colorless, Type::Colorless])
            .flip_a_coin()
            .if_heads(|e| e.damage(70))
    }
}

#[derive(Default)]
pub struct DarkDugtrio6 {}
impl CardArchetype for DarkDugtrio6 {
    identifier!("Dark Dugtrio (TR 6)");
    card_name!("Dark Dugtrio");
    stage1!("Diglett");
    hp!(50);
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
    fn attacks(&self) -> Vec<Attack> {
        vec![
            Attack::new("Knock Down", Self::knock_down),
        ]
    }
}
impl DarkDugtrio6 {
    pub fn knock_down(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
}

#[derive(Default)]
pub struct DarkGolbat7 {}
impl CardArchetype for DarkGolbat7 {
    identifier!("Dark Golbat (TR 7)");
    card_name!("Dark Golbat");
    stage1!("Zubat");
    hp!(50);
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
    fn attacks(&self) -> Vec<Attack> {
        vec![
            Attack::new("Flitter", Self::flitter),
        ]
    }
}
impl DarkGolbat7 {
    pub fn flitter(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
}

#[derive(Default)]
pub struct DarkGyarados8 {}
impl CardArchetype for DarkGyarados8 {
    identifier!("Dark Gyarados (TR 8)");
    card_name!("Dark Gyarados");
    stage1!("Magikarp");
    hp!(70);
    color!(Water);
    weak_to!(Grass);
    resists!(Fighting, 30);
    retreat!(2);

    fn card_actions(&self, _player: Player, _card: &Card, _engine: &GameEngine) -> Vec<Action> {
        vec![]
    }
    fn execute(&self, _player: Player, _card: &Card, engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine.clone()
    }
    fn attacks(&self) -> Vec<Attack> {
        vec![
            Attack::new("Ice Beam", Self::ice_beam),
        ]
    }
}
impl DarkGyarados8 {
    pub fn ice_beam(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Water, Type::Water, Type::Water])
            .flip_a_coin()
            .damage(30)
            .if_heads(|e| e.paralyze())
    }
}

#[derive(Default)]
pub struct DarkHypno9 {}
impl CardArchetype for DarkHypno9 {
    identifier!("Dark Hypno (TR 9)");
    card_name!("Dark Hypno");
    stage1!("Drowzee");
    hp!(60);
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
    fn attacks(&self) -> Vec<Attack> {
        vec![
            Attack::new("Psypunch", Self::psypunch),
            Attack::new("Bench Manipulation", Self::bench_manipulation),
        ]
    }
}
impl DarkHypno9 {
    pub fn psypunch(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Psychic])
            .damage(20)
    }
    pub fn bench_manipulation(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
}

#[derive(Default)]
pub struct DarkMachamp10 {}
impl CardArchetype for DarkMachamp10 {
    identifier!("Dark Machamp (TR 10)");
    card_name!("Dark Machamp");
    stage2!("Dark Machoke");
    hp!(70);
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
    fn attacks(&self) -> Vec<Attack> {
        vec![
            Attack::new("Mega Punch", Self::mega_punch),
            Attack::new("Fling", Self::fling),
        ]
    }
}
impl DarkMachamp10 {
    pub fn mega_punch(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Fighting, Type::Fighting])
            .damage(30)
    }
    pub fn fling(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
}

#[derive(Default)]
pub struct DarkMagneton11 {}
impl CardArchetype for DarkMagneton11 {
    identifier!("Dark Magneton (TR 11)");
    card_name!("Dark Magneton");
    stage1!("Magnemite");
    hp!(60);
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
    fn attacks(&self) -> Vec<Attack> {
        vec![
            Attack::new("Sonicboom", Self::sonicboom),
            Attack::new("Magnetic Lines", Self::magnetic_lines),
        ]
    }
}
impl DarkMagneton11 {
    pub fn sonicboom(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
    pub fn magnetic_lines(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
}

#[derive(Default)]
pub struct DarkSlowbro12 {}
impl CardArchetype for DarkSlowbro12 {
    identifier!("Dark Slowbro (TR 12)");
    card_name!("Dark Slowbro");
    stage1!("Slowpoke");
    hp!(60);
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
    fn attacks(&self) -> Vec<Attack> {
        vec![
            Attack::new("Fickle Attack", Self::fickle_attack),
        ]
    }
}
impl DarkSlowbro12 {
    pub fn fickle_attack(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Psychic, Type::Psychic])
            .flip_a_coin()
            .if_heads(|e| e.damage(40))
    }
}

#[derive(Default)]
pub struct DarkVileplume13 {}
impl CardArchetype for DarkVileplume13 {
    identifier!("Dark Vileplume (TR 13)");
    card_name!("Dark Vileplume");
    stage2!("Dark Gloom");
    hp!(60);
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
            Attack::new("Petal Whirlwind", Self::petal_whirlwind),
        ]
    }
}
impl DarkVileplume13 {
    pub fn petal_whirlwind(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
}

#[derive(Default)]
pub struct DarkWeezing14 {}
impl CardArchetype for DarkWeezing14 {
    identifier!("Dark Weezing (TR 14)");
    card_name!("Dark Weezing");
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
    fn attacks(&self) -> Vec<Attack> {
        vec![
            Attack::new("Mass Explosion", Self::mass_explosion),
            Attack::new("Stun Gas", Self::stun_gas),
        ]
    }
}
impl DarkWeezing14 {
    pub fn mass_explosion(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
    pub fn stun_gas(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
}

#[derive(Default)]
pub struct DarkAlakazam18 {}
impl CardArchetype for DarkAlakazam18 {
    identifier!("Dark Alakazam (TR 18)");
    card_name!("Dark Alakazam");
    stage2!("Dark Kadabra");
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
    fn attacks(&self) -> Vec<Attack> {
        vec![
            Attack::new("Teleport Blast", Self::teleport_blast),
            Attack::new("Mind Shock", Self::mind_shock),
        ]
    }
}
impl DarkAlakazam18 {
    pub fn teleport_blast(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
    pub fn mind_shock(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
}

#[derive(Default)]
pub struct DarkArbok19 {}
impl CardArchetype for DarkArbok19 {
    identifier!("Dark Arbok (TR 19)");
    card_name!("Dark Arbok");
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
    fn attacks(&self) -> Vec<Attack> {
        vec![
            Attack::new("Stare", Self::stare),
            Attack::new("Poison Vapor", Self::poison_vapor),
        ]
    }
}
impl DarkArbok19 {
    pub fn stare(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
    pub fn poison_vapor(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
}

#[derive(Default)]
pub struct DarkBlastoise20 {}
impl CardArchetype for DarkBlastoise20 {
    identifier!("Dark Blastoise (TR 20)");
    card_name!("Dark Blastoise");
    stage2!("Dark Wartortle");
    hp!(70);
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
    fn attacks(&self) -> Vec<Attack> {
        vec![
            Attack::new("Hydrocannon", Self::hydrocannon),
            Attack::new("Rocket Tackle", Self::rocket_tackle),
        ]
    }
}
impl DarkBlastoise20 {
    pub fn hydrocannon(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
    pub fn rocket_tackle(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
}

#[derive(Default)]
pub struct DarkCharizard21 {}
impl CardArchetype for DarkCharizard21 {
    identifier!("Dark Charizard (TR 21)");
    card_name!("Dark Charizard");
    stage2!("Dark Charmeleon");
    hp!(80);
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
    fn attacks(&self) -> Vec<Attack> {
        vec![
            Attack::new("Nail Flick", Self::nail_flick),
            Attack::new("Continuous Fireball", Self::continuous_fireball),
        ]
    }
}
impl DarkCharizard21 {
    pub fn nail_flick(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Colorless])
            .damage(10)
    }
    pub fn continuous_fireball(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
}

#[derive(Default)]
pub struct DarkDragonite22 {}
impl CardArchetype for DarkDragonite22 {
    identifier!("Dark Dragonite (TR 22)");
    card_name!("Dark Dragonite");
    stage2!("Dark Dragonair");
    hp!(70);
    color!(Colorless);
    no_weakness!();
    resists!(Fighting, 30);
    retreat!(2);

    fn card_actions(&self, _player: Player, _card: &Card, _engine: &GameEngine) -> Vec<Action> {
        vec![]
    }
    fn execute(&self, _player: Player, _card: &Card, engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine.clone()
    }
    fn attacks(&self) -> Vec<Attack> {
        vec![
            Attack::new("Giant Tail", Self::giant_tail),
        ]
    }
}
impl DarkDragonite22 {
    pub fn giant_tail(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Colorless, Type::Colorless, Type::Colorless, Type::Colorless])
            .flip_a_coin()
            .if_heads(|e| e.damage(70))
    }
}

#[derive(Default)]
pub struct DarkDugtrio23 {}
impl CardArchetype for DarkDugtrio23 {
    identifier!("Dark Dugtrio (TR 23)");
    card_name!("Dark Dugtrio");
    stage1!("Diglett");
    hp!(50);
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
    fn attacks(&self) -> Vec<Attack> {
        vec![
            Attack::new("Knock Down", Self::knock_down),
        ]
    }
}
impl DarkDugtrio23 {
    pub fn knock_down(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
}

#[derive(Default)]
pub struct DarkGolbat24 {}
impl CardArchetype for DarkGolbat24 {
    identifier!("Dark Golbat (TR 24)");
    card_name!("Dark Golbat");
    stage1!("Zubat");
    hp!(50);
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
    fn attacks(&self) -> Vec<Attack> {
        vec![
            Attack::new("Flitter", Self::flitter),
        ]
    }
}
impl DarkGolbat24 {
    pub fn flitter(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
}

#[derive(Default)]
pub struct DarkGyarados25 {}
impl CardArchetype for DarkGyarados25 {
    identifier!("Dark Gyarados (TR 25)");
    card_name!("Dark Gyarados");
    stage1!("Magikarp");
    hp!(70);
    color!(Water);
    weak_to!(Grass);
    resists!(Fighting, 30);
    retreat!(2);

    fn card_actions(&self, _player: Player, _card: &Card, _engine: &GameEngine) -> Vec<Action> {
        vec![]
    }
    fn execute(&self, _player: Player, _card: &Card, engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine.clone()
    }
    fn attacks(&self) -> Vec<Attack> {
        vec![
            Attack::new("Ice Beam", Self::ice_beam),
        ]
    }
}
impl DarkGyarados25 {
    pub fn ice_beam(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Water, Type::Water, Type::Water])
            .flip_a_coin()
            .damage(30)
            .if_heads(|e| e.paralyze())
    }
}

#[derive(Default)]
pub struct DarkHypno26 {}
impl CardArchetype for DarkHypno26 {
    identifier!("Dark Hypno (TR 26)");
    card_name!("Dark Hypno");
    stage1!("Drowzee");
    hp!(60);
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
    fn attacks(&self) -> Vec<Attack> {
        vec![
            Attack::new("Psypunch", Self::psypunch),
            Attack::new("Bench Manipulation", Self::bench_manipulation),
        ]
    }
}
impl DarkHypno26 {
    pub fn psypunch(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Psychic])
            .damage(20)
    }
    pub fn bench_manipulation(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
}

#[derive(Default)]
pub struct DarkMachamp27 {}
impl CardArchetype for DarkMachamp27 {
    identifier!("Dark Machamp (TR 27)");
    card_name!("Dark Machamp");
    stage2!("Dark Machoke");
    hp!(70);
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
    fn attacks(&self) -> Vec<Attack> {
        vec![
            Attack::new("Mega Punch", Self::mega_punch),
            Attack::new("Fling", Self::fling),
        ]
    }
}
impl DarkMachamp27 {
    pub fn mega_punch(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Fighting, Type::Fighting])
            .damage(30)
    }
    pub fn fling(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
}

#[derive(Default)]
pub struct DarkMagneton28 {}
impl CardArchetype for DarkMagneton28 {
    identifier!("Dark Magneton (TR 28)");
    card_name!("Dark Magneton");
    stage1!("Magnemite");
    hp!(60);
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
    fn attacks(&self) -> Vec<Attack> {
        vec![
            Attack::new("Sonicboom", Self::sonicboom),
            Attack::new("Magnetic Lines", Self::magnetic_lines),
        ]
    }
}
impl DarkMagneton28 {
    pub fn sonicboom(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
    pub fn magnetic_lines(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
}

#[derive(Default)]
pub struct DarkSlowbro29 {}
impl CardArchetype for DarkSlowbro29 {
    identifier!("Dark Slowbro (TR 29)");
    card_name!("Dark Slowbro");
    stage1!("Slowpoke");
    hp!(60);
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
    fn attacks(&self) -> Vec<Attack> {
        vec![
            Attack::new("Fickle Attack", Self::fickle_attack),
        ]
    }
}
impl DarkSlowbro29 {
    pub fn fickle_attack(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Psychic, Type::Psychic])
            .flip_a_coin()
            .if_heads(|e| e.damage(40))
    }
}

#[derive(Default)]
pub struct DarkVileplume30 {}
impl CardArchetype for DarkVileplume30 {
    identifier!("Dark Vileplume (TR 30)");
    card_name!("Dark Vileplume");
    stage2!("Dark Gloom");
    hp!(60);
    color!(Grass);
    weak_to!(Fighting);
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
            Attack::new("Petal Whirlwind", Self::petal_whirlwind),
        ]
    }
}
impl DarkVileplume30 {
    pub fn petal_whirlwind(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
}

#[derive(Default)]
pub struct DarkWeezing31 {}
impl CardArchetype for DarkWeezing31 {
    identifier!("Dark Weezing (TR 31)");
    card_name!("Dark Weezing");
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
    fn attacks(&self) -> Vec<Attack> {
        vec![
            Attack::new("Mass Explosion", Self::mass_explosion),
            Attack::new("Stun Gas", Self::stun_gas),
        ]
    }
}
impl DarkWeezing31 {
    pub fn mass_explosion(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
    pub fn stun_gas(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
}

#[derive(Default)]
pub struct DarkCharmeleon32 {}
impl CardArchetype for DarkCharmeleon32 {
    identifier!("Dark Charmeleon (TR 32)");
    card_name!("Dark Charmeleon");
    stage1!("Charmander");
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
    fn attacks(&self) -> Vec<Attack> {
        vec![
            Attack::new("Tail Slap", Self::tail_slap),
            Attack::new("Fireball", Self::fireball),
        ]
    }
}
impl DarkCharmeleon32 {
    pub fn tail_slap(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Colorless, Type::Colorless])
            .damage(20)
    }
    pub fn fireball(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
}

#[derive(Default)]
pub struct DarkDragonair33 {}
impl CardArchetype for DarkDragonair33 {
    identifier!("Dark Dragonair (TR 33)");
    card_name!("Dark Dragonair");
    stage1!("Dratini");
    hp!(60);
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
    fn attacks(&self) -> Vec<Attack> {
        vec![
            Attack::new("Tail Strike", Self::tail_strike),
        ]
    }
}
impl DarkDragonair33 {
    pub fn tail_strike(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Colorless, Type::Colorless, Type::Colorless])
            .flip_a_coin()
            .if_heads(|e| e.damage(40))
            .if_tails(|e| e.damage(20))
    }
}

#[derive(Default)]
pub struct DarkElectrode34 {}
impl CardArchetype for DarkElectrode34 {
    identifier!("Dark Electrode (TR 34)");
    card_name!("Dark Electrode");
    stage1!("Voltorb");
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
    fn attacks(&self) -> Vec<Attack> {
        vec![
            Attack::new("Rolling Tackle", Self::rolling_tackle),
            Attack::new("Energy Bomb", Self::energy_bomb),
        ]
    }
}
impl DarkElectrode34 {
    pub fn rolling_tackle(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Colorless])
            .damage(10)
    }
    pub fn energy_bomb(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
}

#[derive(Default)]
pub struct DarkFlareon35 {}
impl CardArchetype for DarkFlareon35 {
    identifier!("Dark Flareon (TR 35)");
    card_name!("Dark Flareon");
    stage1!("Eevee");
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
    fn attacks(&self) -> Vec<Attack> {
        vec![
            Attack::new("Rage", Self::rage),
            Attack::new("Playing with Fire", Self::playing_with_fire),
        ]
    }
}
impl DarkFlareon35 {
    pub fn rage(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
    pub fn playing_with_fire(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
}

#[derive(Default)]
pub struct DarkGloom36 {}
impl CardArchetype for DarkGloom36 {
    identifier!("Dark Gloom (TR 36)");
    card_name!("Dark Gloom");
    stage1!("Oddish");
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
    fn attacks(&self) -> Vec<Attack> {
        vec![
            Attack::new("Poisonpowder", Self::poisonpowder),
        ]
    }
}
impl DarkGloom36 {
    pub fn poisonpowder(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Grass, Type::Grass])
            .damage(10)
            .poison()
    }
}

#[derive(Default)]
pub struct DarkGolduck37 {}
impl CardArchetype for DarkGolduck37 {
    identifier!("Dark Golduck (TR 37)");
    card_name!("Dark Golduck");
    stage1!("Psyduck");
    hp!(60);
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
    fn attacks(&self) -> Vec<Attack> {
        vec![
            Attack::new("Third Eye", Self::third_eye),
            Attack::new("Super Psy", Self::super_psy),
        ]
    }
}
impl DarkGolduck37 {
    pub fn third_eye(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
    pub fn super_psy(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Psychic, Type::Psychic, Type::Colorless])
            .damage(50)
    }
}

#[derive(Default)]
pub struct DarkJolteon38 {}
impl CardArchetype for DarkJolteon38 {
    identifier!("Dark Jolteon (TR 38)");
    card_name!("Dark Jolteon");
    stage1!("Eevee");
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
            Attack::new("Lightning Flash", Self::lightning_flash),
            Attack::new("Thunder Attack", Self::thunder_attack),
        ]
    }
}
impl DarkJolteon38 {
    pub fn lightning_flash(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Lightning])
            .prevent_attack_on_a_flip_during_opponents_next_turn()
    }
    pub fn thunder_attack(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
}

#[derive(Default)]
pub struct DarkKadabra39 {}
impl CardArchetype for DarkKadabra39 {
    identifier!("Dark Kadabra (TR 39)");
    card_name!("Dark Kadabra");
    stage1!("Abra");
    hp!(50);
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
    fn attacks(&self) -> Vec<Attack> {
        vec![
            Attack::new("Mind Shock", Self::mind_shock),
        ]
    }
}
impl DarkKadabra39 {
    pub fn mind_shock(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
}

#[derive(Default)]
pub struct DarkMachoke40 {}
impl CardArchetype for DarkMachoke40 {
    identifier!("Dark Machoke (TR 40)");
    card_name!("Dark Machoke");
    stage1!("Machop");
    hp!(60);
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
    fn attacks(&self) -> Vec<Attack> {
        vec![
            Attack::new("Drag Off", Self::drag_off),
            Attack::new("Knock Back", Self::knock_back),
        ]
    }
}
impl DarkMachoke40 {
    pub fn drag_off(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
    pub fn knock_back(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Fighting, Type::Fighting, Type::Colorless])
            .damage(30)
            .switch_defending()
    }
}

#[derive(Default)]
pub struct DarkMuk41 {}
impl CardArchetype for DarkMuk41 {
    identifier!("Dark Muk (TR 41)");
    card_name!("Dark Muk");
    stage1!("Grimer");
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
    fn attacks(&self) -> Vec<Attack> {
        vec![
            Attack::new("Sludge Punch", Self::sludge_punch),
        ]
    }
}
impl DarkMuk41 {
    pub fn sludge_punch(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Grass, Type::Grass])
            .damage(20)
            .poison()
    }
}

#[derive(Default)]
pub struct DarkPersian42 {}
impl CardArchetype for DarkPersian42 {
    identifier!("Dark Persian (TR 42)");
    card_name!("Dark Persian");
    stage1!("Meowth");
    hp!(60);
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
            Attack::new("Fascinate", Self::fascinate),
            Attack::new("Poison Claws", Self::poison_claws),
        ]
    }
}
impl DarkPersian42 {
    pub fn fascinate(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
    pub fn poison_claws(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Colorless, Type::Colorless])
            .flip_a_coin()
            .damage(10)
            .if_heads(|e| e.poison())
    }
}

#[derive(Default)]
pub struct DarkPrimeape43 {}
impl CardArchetype for DarkPrimeape43 {
    identifier!("Dark Primeape (TR 43)");
    card_name!("Dark Primeape");
    stage1!("Mankey");
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
    fn attacks(&self) -> Vec<Attack> {
        vec![
            Attack::new("Frenzied Attack", Self::frenzied_attack),
        ]
    }
}
impl DarkPrimeape43 {
    pub fn frenzied_attack(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
}

#[derive(Default)]
pub struct DarkRapidash44 {}
impl CardArchetype for DarkRapidash44 {
    identifier!("Dark Rapidash (TR 44)");
    card_name!("Dark Rapidash");
    stage1!("Ponyta");
    hp!(60);
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
            Attack::new("Rear Kick", Self::rear_kick),
            Attack::new("Flame Pillar", Self::flame_pillar),
        ]
    }
}
impl DarkRapidash44 {
    pub fn rear_kick(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Colorless, Type::Colorless])
            .damage(20)
    }
    pub fn flame_pillar(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
}

#[derive(Default)]
pub struct DarkVaporeon45 {}
impl CardArchetype for DarkVaporeon45 {
    identifier!("Dark Vaporeon (TR 45)");
    card_name!("Dark Vaporeon");
    stage1!("Eevee");
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
    fn attacks(&self) -> Vec<Attack> {
        vec![
            Attack::new("Bite", Self::bite),
            Attack::new("Whirlpool", Self::whirlpool),
        ]
    }
}
impl DarkVaporeon45 {
    pub fn bite(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Colorless, Type::Colorless, Type::Colorless])
            .damage(30)
    }
    pub fn whirlpool(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Water, Type::Water, Type::Colorless])
            .discard_defending_energy_cards(&[Type::Any])
            .damage(20)
    }
}

#[derive(Default)]
pub struct DarkWartortle46 {}
impl CardArchetype for DarkWartortle46 {
    identifier!("Dark Wartortle (TR 46)");
    card_name!("Dark Wartortle");
    stage1!("Squirtle");
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
    fn attacks(&self) -> Vec<Attack> {
        vec![
            Attack::new("Doubleslap", Self::doubleslap),
            Attack::new("Mirror Shell", Self::mirror_shell),
        ]
    }
}
impl DarkWartortle46 {
    pub fn doubleslap(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Water])
            .flip_coins(2)
            .damage_per_heads(10)
    }
    pub fn mirror_shell(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
}

#[derive(Default)]
pub struct Magikarp47 {}
impl CardArchetype for Magikarp47 {
    identifier!("Magikarp (TR 47)");
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
    fn attacks(&self) -> Vec<Attack> {
        vec![
            Attack::new("Flop", Self::flop),
            Attack::new("Rapid Evolution", Self::rapid_evolution),
        ]
    }
}
impl Magikarp47 {
    pub fn flop(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Colorless])
            .damage(10)
    }
    pub fn rapid_evolution(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
}

#[derive(Default)]
pub struct Porygon48 {}
impl CardArchetype for Porygon48 {
    identifier!("Porygon (TR 48)");
    card_name!("Porygon");
    basic!();
    hp!(40);
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
            Attack::new("Conversion 1", Self::conversion_1),
            Attack::new("Psybeam", Self::psybeam),
        ]
    }
}
impl Porygon48 {
    pub fn conversion_1(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Colorless])
            .change_defending_weakness_except(&[Type::Colorless])
    }
    pub fn psybeam(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Colorless, Type::Colorless, Type::Colorless])
            .flip_a_coin()
            .damage(20)
            .if_heads(|e| e.confuse())
    }
}

#[derive(Default)]
pub struct Abra49 {}
impl CardArchetype for Abra49 {
    identifier!("Abra (TR 49)");
    card_name!("Abra");
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
            Attack::new("Vanish", Self::vanish),
            Attack::new("Psyshock", Self::psyshock),
        ]
    }
}
impl Abra49 {
    pub fn vanish(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
    pub fn psyshock(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Psychic])
            .flip_a_coin()
            .damage(10)
            .if_heads(|e| e.paralyze())
    }
}

#[derive(Default)]
pub struct Charmander50 {}
impl CardArchetype for Charmander50 {
    identifier!("Charmander (TR 50)");
    card_name!("Charmander");
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
    fn attacks(&self) -> Vec<Attack> {
        vec![
            Attack::new("Fire Tail", Self::fire_tail),
        ]
    }
}
impl Charmander50 {
    pub fn fire_tail(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Fire])
            .damage(20)
    }
}

#[derive(Default)]
pub struct DarkRaticate51 {}
impl CardArchetype for DarkRaticate51 {
    identifier!("Dark Raticate (TR 51)");
    card_name!("Dark Raticate");
    stage1!("Rattata");
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
            Attack::new("Gnaw", Self::gnaw),
            Attack::new("Hyper Fang", Self::hyper_fang),
        ]
    }
}
impl DarkRaticate51 {
    pub fn gnaw(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Colorless, Type::Colorless])
            .damage(20)
    }
    pub fn hyper_fang(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Colorless, Type::Colorless, Type::Colorless])
            .flip_a_coin()
            .if_heads(|e| e.damage(50))
    }
}

#[derive(Default)]
pub struct Diglett52 {}
impl CardArchetype for Diglett52 {
    identifier!("Diglett (TR 52)");
    card_name!("Diglett");
    basic!();
    hp!(40);
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
    fn attacks(&self) -> Vec<Attack> {
        vec![
            Attack::new("Dig Under", Self::dig_under),
            Attack::new("Scratch", Self::scratch),
        ]
    }
}
impl Diglett52 {
    pub fn dig_under(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
    pub fn scratch(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Fighting, Type::Colorless])
            .damage(20)
    }
}

#[derive(Default)]
pub struct Dratini53 {}
impl CardArchetype for Dratini53 {
    identifier!("Dratini (TR 53)");
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
    fn attacks(&self) -> Vec<Attack> {
        vec![
            Attack::new("Wrap", Self::wrap),
        ]
    }
}
impl Dratini53 {
    pub fn wrap(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Colorless, Type::Colorless])
            .flip_a_coin()
            .damage(10)
            .if_heads(|e| e.paralyze())
    }
}

#[derive(Default)]
pub struct Drowzee54 {}
impl CardArchetype for Drowzee54 {
    identifier!("Drowzee (TR 54)");
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
    fn attacks(&self) -> Vec<Attack> {
        vec![
            Attack::new("Nightmare", Self::nightmare),
        ]
    }
}
impl Drowzee54 {
    pub fn nightmare(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Psychic, Type::Colorless])
            .damage(10)
            .asleep()
    }
}

#[derive(Default)]
pub struct Eevee55 {}
impl CardArchetype for Eevee55 {
    identifier!("Eevee (TR 55)");
    card_name!("Eevee");
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
    fn attacks(&self) -> Vec<Attack> {
        vec![
            Attack::new("Tackle", Self::tackle),
            Attack::new("Sand-attack", Self::sand_attack),
        ]
    }
}
impl Eevee55 {
    pub fn tackle(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Colorless])
            .damage(10)
    }
    pub fn sand_attack(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Colorless, Type::Colorless])
            .prevent_attack_on_a_flip_during_opponents_next_turn()
    }
}

#[derive(Default)]
pub struct Ekans56 {}
impl CardArchetype for Ekans56 {
    identifier!("Ekans (TR 56)");
    card_name!("Ekans");
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
    fn attacks(&self) -> Vec<Attack> {
        vec![
            Attack::new("Bite", Self::bite),
            Attack::new("Poison Sting", Self::poison_sting),
        ]
    }
}
impl Ekans56 {
    pub fn bite(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Colorless])
            .damage(10)
    }
    pub fn poison_sting(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Grass, Type::Grass])
            .flip_a_coin()
            .damage(20)
            .if_heads(|e| e.poison())
    }
}

#[derive(Default)]
pub struct Grimer57 {}
impl CardArchetype for Grimer57 {
    identifier!("Grimer (TR 57)");
    card_name!("Grimer");
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
    fn attacks(&self) -> Vec<Attack> {
        vec![
            Attack::new("Poison Gas", Self::poison_gas),
            Attack::new("Sticky Hands", Self::sticky_hands),
        ]
    }
}
impl Grimer57 {
    pub fn poison_gas(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Grass])
            .asleep()
    }
    pub fn sticky_hands(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
}

#[derive(Default)]
pub struct Koffing58 {}
impl CardArchetype for Koffing58 {
    identifier!("Koffing (TR 58)");
    card_name!("Koffing");
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
    fn attacks(&self) -> Vec<Attack> {
        vec![
            Attack::new("Tackle", Self::tackle),
            Attack::new("Poison Gas", Self::poison_gas),
        ]
    }
}
impl Koffing58 {
    pub fn tackle(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Colorless])
            .damage(10)
    }
    pub fn poison_gas(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Grass, Type::Colorless])
            .flip_a_coin()
            .damage(20)
            .if_heads(|e| e.poison())
    }
}

#[derive(Default)]
pub struct Machop59 {}
impl CardArchetype for Machop59 {
    identifier!("Machop (TR 59)");
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
    fn attacks(&self) -> Vec<Attack> {
        vec![
            Attack::new("Punch", Self::punch),
            Attack::new("Kick", Self::kick),
        ]
    }
}
impl Machop59 {
    pub fn punch(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Colorless, Type::Colorless])
            .damage(20)
    }
    pub fn kick(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Fighting, Type::Colorless, Type::Colorless])
            .damage(30)
    }
}

#[derive(Default)]
pub struct Magnemite60 {}
impl CardArchetype for Magnemite60 {
    identifier!("Magnemite (TR 60)");
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
    fn attacks(&self) -> Vec<Attack> {
        vec![
            Attack::new("Tackle", Self::tackle),
            Attack::new("Magnetism", Self::magnetism),
        ]
    }
}
impl Magnemite60 {
    pub fn tackle(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Colorless, Type::Colorless])
            .damage(20)
    }
    pub fn magnetism(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
}

#[derive(Default)]
pub struct Mankey61 {}
impl CardArchetype for Mankey61 {
    identifier!("Mankey (TR 61)");
    card_name!("Mankey");
    basic!();
    hp!(40);
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
            Attack::new("Mischief", Self::mischief),
            Attack::new("Anger", Self::anger),
        ]
    }
}
impl Mankey61 {
    pub fn mischief(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
    pub fn anger(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Fighting, Type::Colorless])
            .flip_a_coin()
            .if_heads(|e| e.damage(40))
            .if_tails(|e| e.damage(20))
    }
}

#[derive(Default)]
pub struct Meowth62 {}
impl CardArchetype for Meowth62 {
    identifier!("Meowth (TR 62)");
    card_name!("Meowth");
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
    fn attacks(&self) -> Vec<Attack> {
        vec![
            Attack::new("Coin Hurl", Self::coin_hurl),
        ]
    }
}
impl Meowth62 {
    pub fn coin_hurl(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
}

#[derive(Default)]
pub struct Oddish63 {}
impl CardArchetype for Oddish63 {
    identifier!("Oddish (TR 63)");
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
            Attack::new("Sleep Powder", Self::sleep_powder),
            Attack::new("Poisonpowder", Self::poisonpowder),
        ]
    }
}
impl Oddish63 {
    pub fn sleep_powder(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Grass])
            .asleep()
    }
    pub fn poisonpowder(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Grass])
            .poison()
    }
}

#[derive(Default)]
pub struct Ponyta64 {}
impl CardArchetype for Ponyta64 {
    identifier!("Ponyta (TR 64)");
    card_name!("Ponyta");
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
    fn attacks(&self) -> Vec<Attack> {
        vec![
            Attack::new("Ember", Self::ember),
        ]
    }
}
impl Ponyta64 {
    pub fn ember(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Fire, Type::Colorless])
            .cost(|e| e.discard_attacking_energy_cards(&[Type::Fire]))
            .damage(30)
    }
}

#[derive(Default)]
pub struct Psyduck65 {}
impl CardArchetype for Psyduck65 {
    identifier!("Psyduck (TR 65)");
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
    fn attacks(&self) -> Vec<Attack> {
        vec![
            Attack::new("Dizziness", Self::dizziness),
            Attack::new("Water Gun", Self::water_gun),
        ]
    }
}
impl Psyduck65 {
    pub fn dizziness(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Psychic])
            .draw(1)
    }
    pub fn water_gun(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
}

#[derive(Default)]
pub struct Rattata66 {}
impl CardArchetype for Rattata66 {
    identifier!("Rattata (TR 66)");
    card_name!("Rattata");
    basic!();
    hp!(40);
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
            Attack::new("Quick Attack", Self::quick_attack),
        ]
    }
}
impl Rattata66 {
    pub fn quick_attack(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Colorless])
            .flip_a_coin()
            .if_heads(|e| e.damage(20))
            .if_tails(|e| e.damage(10))
    }
}

#[derive(Default)]
pub struct Slowpoke67 {}
impl CardArchetype for Slowpoke67 {
    identifier!("Slowpoke (TR 67)");
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
    fn attacks(&self) -> Vec<Attack> {
        vec![
            Attack::new("Afternoon Nap", Self::afternoon_nap),
            Attack::new("Headbutt", Self::headbutt),
        ]
    }
}
impl Slowpoke67 {
    pub fn afternoon_nap(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
    pub fn headbutt(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Psychic])
            .damage(10)
    }
}

#[derive(Default)]
pub struct Squirtle68 {}
impl CardArchetype for Squirtle68 {
    identifier!("Squirtle (TR 68)");
    card_name!("Squirtle");
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
    fn attacks(&self) -> Vec<Attack> {
        vec![
            Attack::new("Shell Attack", Self::shell_attack),
        ]
    }
}
impl Squirtle68 {
    pub fn shell_attack(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Colorless, Type::Colorless])
            .damage(20)
    }
}

#[derive(Default)]
pub struct Voltorb69 {}
impl CardArchetype for Voltorb69 {
    identifier!("Voltorb (TR 69)");
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
    fn attacks(&self) -> Vec<Attack> {
        vec![
            Attack::new("Speed Ball", Self::speed_ball),
        ]
    }
}
impl Voltorb69 {
    pub fn speed_ball(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Lightning])
            .damage(20)
    }
}

#[derive(Default)]
pub struct Zubat70 {}
impl CardArchetype for Zubat70 {
    identifier!("Zubat (TR 70)");
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
    fn attacks(&self) -> Vec<Attack> {
        vec![
            Attack::new("Ram", Self::ram),
            Attack::new("Bite", Self::bite),
        ]
    }
}
impl Zubat70 {
    pub fn ram(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Colorless])
            .damage(10)
    }
    pub fn bite(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Grass, Type::Colorless])
            .damage(20)
    }
}

#[derive(Default)]
pub struct DarkRaichu83 {}
impl CardArchetype for DarkRaichu83 {
    identifier!("Dark Raichu (TR 83)");
    card_name!("Dark Raichu");
    stage1!("Pikachu");
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
            Attack::new("Surprise Thunder", Self::surprise_thunder),
        ]
    }
}
impl DarkRaichu83 {
    pub fn surprise_thunder(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Lightning, Type::Lightning, Type::Lightning])
            .damage(30)
            .flip_a_coin()
            .if_heads(|e| e.flip_a_coin().if_heads(|e| e.each_opponents_bench(|e| e.damage(20))).if_tails(|e| e.each_own_bench(|e| e.damage(10))))
    }
}