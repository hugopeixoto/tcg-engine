use crate::*;
use crate::state::Type;
use crate::attack_builder::AttackBuilder;

#[derive(Default)]
pub struct Alakazam1 {}
impl CardArchetype for Alakazam1 {
    identifier!("Alakazam (B2 1)");
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
    fn attacks(&self) -> Vec<Attack> {
        vec![
            Attack::new("Confuse Ray", Self::confuse_ray),
        ]
    }
    fn poke_powers(&self) -> Vec<Attack> {
        vec![
            Attack::new("Damage Swap", Self::damage_swap),
        ]
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
    pub fn damage_swap(builder: AttackBuilder) -> AttackBuilder {
        builder
            .move_own_damage_counter_without_ko()
    }
}

#[derive(Default)]
pub struct Blastoise2 {}
impl CardArchetype for Blastoise2 {
    identifier!("Blastoise (B2 2)");
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
    fn attacks(&self) -> Vec<Attack> {
        vec![
            Attack::new("Hydro Pump", Self::hydro_pump),
        ]
    }
    fn poke_powers(&self) -> Vec<Attack> {
        vec![
            Attack::new("Rain Dance", Self::rain_dance),
        ]
    }
}
impl Blastoise2 {
    pub fn hydro_pump(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
    pub fn rain_dance(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attach_energy_from_hand(Type::Water, Type::Water)
    }
}

#[derive(Default)]
pub struct Chansey3 {}
impl CardArchetype for Chansey3 {
    identifier!("Chansey (B2 3)");
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
    fn attacks(&self) -> Vec<Attack> {
        vec![
            Attack::new("Scrunch", Self::scrunch),
            Attack::new("Double-edge", Self::double_edge),
        ]
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
    identifier!("Charizard (B2 4)");
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
    fn attacks(&self) -> Vec<Attack> {
        vec![
            Attack::new("Fire Spin", Self::fire_spin),
        ]
    }
    fn poke_powers(&self) -> Vec<Attack> {
        vec![
            Attack::new("Energy Burn", Self::energy_burn),
        ]
    }
}
impl Charizard4 {
    pub fn fire_spin(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Fire, Type::Fire, Type::Fire, Type::Fire])
            .cost(|e| e.discard_attacking_energy_cards(&[Type::Any, Type::Any]))
            .damage(100)
    }
    pub fn energy_burn(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
}

#[derive(Default)]
pub struct Clefable5 {}
impl CardArchetype for Clefable5 {
    identifier!("Clefable (B2 5)");
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
impl Clefable5 {
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
pub struct Clefairy6 {}
impl CardArchetype for Clefairy6 {
    identifier!("Clefairy (B2 6)");
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
    fn attacks(&self) -> Vec<Attack> {
        vec![
            Attack::new("Sing", Self::sing),
            Attack::new("Metronome", Self::metronome),
        ]
    }
}
impl Clefairy6 {
    pub fn sing(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Colorless])
            .flip_a_coin()
            .if_heads(|e| e.asleep())
    }
    pub fn metronome(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Colorless, Type::Colorless, Type::Colorless])
            .copy_defending_attack_without_costs()
    }
}

#[derive(Default)]
pub struct Gyarados7 {}
impl CardArchetype for Gyarados7 {
    identifier!("Gyarados (B2 7)");
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
    fn attacks(&self) -> Vec<Attack> {
        vec![
            Attack::new("Dragon Rage", Self::dragon_rage),
            Attack::new("Bubblebeam", Self::bubblebeam),
        ]
    }
}
impl Gyarados7 {
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
pub struct Hitmonchan8 {}
impl CardArchetype for Hitmonchan8 {
    identifier!("Hitmonchan (B2 8)");
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
    fn attacks(&self) -> Vec<Attack> {
        vec![
            Attack::new("Jab", Self::jab),
            Attack::new("Special Punch", Self::special_punch),
        ]
    }
}
impl Hitmonchan8 {
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
pub struct Magneton9 {}
impl CardArchetype for Magneton9 {
    identifier!("Magneton (B2 9)");
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
    fn attacks(&self) -> Vec<Attack> {
        vec![
            Attack::new("Thunder Wave", Self::thunder_wave),
            Attack::new("Selfdestruct", Self::selfdestruct),
        ]
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
    pub fn selfdestruct(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
}

#[derive(Default)]
pub struct Mewtwo10 {}
impl CardArchetype for Mewtwo10 {
    identifier!("Mewtwo (B2 10)");
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
    fn attacks(&self) -> Vec<Attack> {
        vec![
            Attack::new("Psychic", Self::psychic),
            Attack::new("Barrier", Self::barrier),
        ]
    }
}
impl Mewtwo10 {
    pub fn psychic(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Psychic, Type::Colorless])
            .damage_plus_per_energy_card_on_defending(10, 10)
    }
    pub fn barrier(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
}

#[derive(Default)]
pub struct Nidoking11 {}
impl CardArchetype for Nidoking11 {
    identifier!("Nidoking (B2 11)");
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
    fn attacks(&self) -> Vec<Attack> {
        vec![
            Attack::new("Thrash", Self::thrash),
            Attack::new("Toxic", Self::toxic),
        ]
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
pub struct Nidoqueen12 {}
impl CardArchetype for Nidoqueen12 {
    identifier!("Nidoqueen (B2 12)");
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
impl Nidoqueen12 {
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
pub struct Ninetales13 {}
impl CardArchetype for Ninetales13 {
    identifier!("Ninetales (B2 13)");
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
    fn attacks(&self) -> Vec<Attack> {
        vec![
            Attack::new("Lure", Self::lure),
            Attack::new("Fire Blast", Self::fire_blast),
        ]
    }
}
impl Ninetales13 {
    pub fn lure(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
    pub fn fire_blast(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Fire, Type::Fire, Type::Fire, Type::Fire])
            .cost(|e| e.discard_attacking_energy_cards(&[Type::Fire]))
            .damage(80)
    }
}

#[derive(Default)]
pub struct Pidgeot14 {}
impl CardArchetype for Pidgeot14 {
    identifier!("Pidgeot (B2 14)");
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
impl Pidgeot14 {
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
pub struct Poliwrath15 {}
impl CardArchetype for Poliwrath15 {
    identifier!("Poliwrath (B2 15)");
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
    fn attacks(&self) -> Vec<Attack> {
        vec![
            Attack::new("Water Gun", Self::water_gun),
            Attack::new("Whirlpool", Self::whirlpool),
        ]
    }
}
impl Poliwrath15 {
    pub fn water_gun(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
    pub fn whirlpool(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Water, Type::Water, Type::Colorless, Type::Colorless])
            .discard_defending_energy_cards(&[Type::Any])
            .damage(40)
    }
}

#[derive(Default)]
pub struct Raichu16 {}
impl CardArchetype for Raichu16 {
    identifier!("Raichu (B2 16)");
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
    fn attacks(&self) -> Vec<Attack> {
        vec![
            Attack::new("Agility", Self::agility),
            Attack::new("Thunder", Self::thunder),
        ]
    }
}
impl Raichu16 {
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
pub struct Scyther17 {}
impl CardArchetype for Scyther17 {
    identifier!("Scyther (B2 17)");
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
impl Scyther17 {
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
pub struct Venusaur18 {}
impl CardArchetype for Venusaur18 {
    identifier!("Venusaur (B2 18)");
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
    fn attacks(&self) -> Vec<Attack> {
        vec![
            Attack::new("Solarbeam", Self::solarbeam),
        ]
    }
    fn poke_powers(&self) -> Vec<Attack> {
        vec![
            Attack::new("Energy Trans", Self::energy_trans),
        ]
    }
}
impl Venusaur18 {
    pub fn solarbeam(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Grass, Type::Grass, Type::Grass, Type::Grass])
            .damage(60)
    }
    pub fn energy_trans(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
}

#[derive(Default)]
pub struct Wigglytuff19 {}
impl CardArchetype for Wigglytuff19 {
    identifier!("Wigglytuff (B2 19)");
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
impl Wigglytuff19 {
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
pub struct Zapdos20 {}
impl CardArchetype for Zapdos20 {
    identifier!("Zapdos (B2 20)");
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
    fn attacks(&self) -> Vec<Attack> {
        vec![
            Attack::new("Thunder", Self::thunder),
            Attack::new("Thunderbolt", Self::thunderbolt),
        ]
    }
}
impl Zapdos20 {
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
            .cost(|e| e.discard_all_attacking_energy_cards())
            .damage(100)
    }
}

#[derive(Default)]
pub struct Beedrill21 {}
impl CardArchetype for Beedrill21 {
    identifier!("Beedrill (B2 21)");
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
    fn attacks(&self) -> Vec<Attack> {
        vec![
            Attack::new("Twineedle", Self::twineedle),
            Attack::new("Poison Sting", Self::poison_sting),
        ]
    }
}
impl Beedrill21 {
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
pub struct Dragonair22 {}
impl CardArchetype for Dragonair22 {
    identifier!("Dragonair (B2 22)");
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
    fn attacks(&self) -> Vec<Attack> {
        vec![
            Attack::new("Slam", Self::slam),
            Attack::new("Hyper Beam", Self::hyper_beam),
        ]
    }
}
impl Dragonair22 {
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
pub struct Dugtrio23 {}
impl CardArchetype for Dugtrio23 {
    identifier!("Dugtrio (B2 23)");
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
    fn attacks(&self) -> Vec<Attack> {
        vec![
            Attack::new("Slash", Self::slash),
            Attack::new("Earthquake", Self::earthquake),
        ]
    }
}
impl Dugtrio23 {
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
pub struct Electabuzz24 {}
impl CardArchetype for Electabuzz24 {
    identifier!("Electabuzz (B2 24)");
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
    fn attacks(&self) -> Vec<Attack> {
        vec![
            Attack::new("Thundershock", Self::thundershock),
            Attack::new("Thunderpunch", Self::thunderpunch),
        ]
    }
}
impl Electabuzz24 {
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
pub struct Electrode25 {}
impl CardArchetype for Electrode25 {
    identifier!("Electrode (B2 25)");
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
    fn attacks(&self) -> Vec<Attack> {
        vec![
            Attack::new("Electric Shock", Self::electric_shock),
        ]
    }
}
impl Electrode25 {
    pub fn electric_shock(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Lightning, Type::Lightning, Type::Lightning])
            .flip_a_coin()
            .damage(50)
            .if_tails(|e| e.damage_self(10))
    }
}

#[derive(Default)]
pub struct Kangaskhan26 {}
impl CardArchetype for Kangaskhan26 {
    identifier!("Kangaskhan (B2 26)");
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
impl Kangaskhan26 {
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
pub struct MrMime27 {}
impl CardArchetype for MrMime27 {
    identifier!("Mr. Mime (B2 27)");
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
impl MrMime27 {
    pub fn meditate(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Psychic, Type::Colorless])
            .damage_plus_per_damage_counter_on_defending(10, 10)
    }
}

#[derive(Default)]
pub struct Pidgeotto28 {}
impl CardArchetype for Pidgeotto28 {
    identifier!("Pidgeotto (B2 28)");
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
    fn attacks(&self) -> Vec<Attack> {
        vec![
            Attack::new("Whirlwind", Self::whirlwind),
            Attack::new("Mirror Move", Self::mirror_move),
        ]
    }
}
impl Pidgeotto28 {
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
pub struct Pinsir29 {}
impl CardArchetype for Pinsir29 {
    identifier!("Pinsir (B2 29)");
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
impl Pinsir29 {
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
pub struct Snorlax30 {}
impl CardArchetype for Snorlax30 {
    identifier!("Snorlax (B2 30)");
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
impl Snorlax30 {
    pub fn body_slam(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Colorless, Type::Colorless, Type::Colorless, Type::Colorless])
            .flip_a_coin()
            .damage(30)
            .if_heads(|e| e.paralyze())
    }
}

#[derive(Default)]
pub struct Venomoth31 {}
impl CardArchetype for Venomoth31 {
    identifier!("Venomoth (B2 31)");
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
impl Venomoth31 {
    pub fn venom_powder(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
}

#[derive(Default)]
pub struct Victreebel32 {}
impl CardArchetype for Victreebel32 {
    identifier!("Victreebel (B2 32)");
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
impl Victreebel32 {
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
pub struct Arcanine33 {}
impl CardArchetype for Arcanine33 {
    identifier!("Arcanine (B2 33)");
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
    fn attacks(&self) -> Vec<Attack> {
        vec![
            Attack::new("Flamethrower", Self::flamethrower),
            Attack::new("Take Down", Self::take_down),
        ]
    }
}
impl Arcanine33 {
    pub fn flamethrower(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Fire, Type::Fire, Type::Colorless])
            .cost(|e| e.discard_attacking_energy_cards(&[Type::Fire]))
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
pub struct Butterfree34 {}
impl CardArchetype for Butterfree34 {
    identifier!("Butterfree (B2 34)");
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
impl Butterfree34 {
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
pub struct Charmeleon35 {}
impl CardArchetype for Charmeleon35 {
    identifier!("Charmeleon (B2 35)");
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
    fn attacks(&self) -> Vec<Attack> {
        vec![
            Attack::new("Slash", Self::slash),
            Attack::new("Flamethrower", Self::flamethrower),
        ]
    }
}
impl Charmeleon35 {
    pub fn slash(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Colorless, Type::Colorless, Type::Colorless])
            .damage(30)
    }
    pub fn flamethrower(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Fire, Type::Fire, Type::Colorless])
            .cost(|e| e.discard_attacking_energy_cards(&[Type::Fire]))
            .damage(50)
    }
}

#[derive(Default)]
pub struct Dewgong36 {}
impl CardArchetype for Dewgong36 {
    identifier!("Dewgong (B2 36)");
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
    fn attacks(&self) -> Vec<Attack> {
        vec![
            Attack::new("Aurora Beam", Self::aurora_beam),
            Attack::new("Ice Beam", Self::ice_beam),
        ]
    }
}
impl Dewgong36 {
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
pub struct Dodrio37 {}
impl CardArchetype for Dodrio37 {
    identifier!("Dodrio (B2 37)");
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
impl Dodrio37 {
    pub fn rage(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
}

#[derive(Default)]
pub struct Dratini38 {}
impl CardArchetype for Dratini38 {
    identifier!("Dratini (B2 38)");
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
            Attack::new("Pound", Self::pound),
        ]
    }
}
impl Dratini38 {
    pub fn pound(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Colorless])
            .damage(10)
    }
}

#[derive(Default)]
pub struct Exeggutor39 {}
impl CardArchetype for Exeggutor39 {
    identifier!("Exeggutor (B2 39)");
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
impl Exeggutor39 {
    pub fn teleport(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
    pub fn big_eggsplosion(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
}

#[derive(Default)]
pub struct FarfetchD40 {}
impl CardArchetype for FarfetchD40 {
    identifier!("Farfetch'd (B2 40)");
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
    fn attacks(&self) -> Vec<Attack> {
        vec![
            Attack::new("Leek Slap", Self::leek_slap),
            Attack::new("Pot Smash", Self::pot_smash),
        ]
    }
}
impl FarfetchD40 {
    pub fn leek_slap(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Colorless])
            .flip_a_coin()
            .if_heads(|e| e.damage(30))
            .once_while_in_play()
    }
    pub fn pot_smash(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Colorless, Type::Colorless, Type::Colorless])
            .damage(30)
    }
}

#[derive(Default)]
pub struct Fearow41 {}
impl CardArchetype for Fearow41 {
    identifier!("Fearow (B2 41)");
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
impl Fearow41 {
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
pub struct Growlithe42 {}
impl CardArchetype for Growlithe42 {
    identifier!("Growlithe (B2 42)");
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
    fn attacks(&self) -> Vec<Attack> {
        vec![
            Attack::new("Flare", Self::flare),
        ]
    }
}
impl Growlithe42 {
    pub fn flare(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Fire, Type::Colorless])
            .damage(20)
    }
}

#[derive(Default)]
pub struct Haunter43 {}
impl CardArchetype for Haunter43 {
    identifier!("Haunter (B2 43)");
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
    fn attacks(&self) -> Vec<Attack> {
        vec![
            Attack::new("Hypnosis", Self::hypnosis),
            Attack::new("Dream Eater", Self::dream_eater),
        ]
    }
}
impl Haunter43 {
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
pub struct Ivysaur44 {}
impl CardArchetype for Ivysaur44 {
    identifier!("Ivysaur (B2 44)");
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
    fn attacks(&self) -> Vec<Attack> {
        vec![
            Attack::new("Vine Whip", Self::vine_whip),
            Attack::new("Poisonpowder", Self::poisonpowder),
        ]
    }
}
impl Ivysaur44 {
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
pub struct Jynx45 {}
impl CardArchetype for Jynx45 {
    identifier!("Jynx (B2 45)");
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
    fn attacks(&self) -> Vec<Attack> {
        vec![
            Attack::new("Doubleslap", Self::doubleslap),
            Attack::new("Meditate", Self::meditate),
        ]
    }
}
impl Jynx45 {
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
pub struct Kadabra46 {}
impl CardArchetype for Kadabra46 {
    identifier!("Kadabra (B2 46)");
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
    fn attacks(&self) -> Vec<Attack> {
        vec![
            Attack::new("Recover", Self::recover),
            Attack::new("Super Psy", Self::super_psy),
        ]
    }
}
impl Kadabra46 {
    pub fn recover(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Psychic, Type::Psychic])
            .cost(|e| e.discard_attacking_energy_cards(&[Type::Psychic]))
            .heal_all_attacking()
    }
    pub fn super_psy(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Psychic, Type::Psychic, Type::Colorless])
            .damage(50)
    }
}

#[derive(Default)]
pub struct Kakuna47 {}
impl CardArchetype for Kakuna47 {
    identifier!("Kakuna (B2 47)");
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
    fn attacks(&self) -> Vec<Attack> {
        vec![
            Attack::new("Stiffen", Self::stiffen),
            Attack::new("Poisonpowder", Self::poisonpowder),
        ]
    }
}
impl Kakuna47 {
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
pub struct Lickitung48 {}
impl CardArchetype for Lickitung48 {
    identifier!("Lickitung (B2 48)");
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
impl Lickitung48 {
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
pub struct Machoke49 {}
impl CardArchetype for Machoke49 {
    identifier!("Machoke (B2 49)");
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
    fn attacks(&self) -> Vec<Attack> {
        vec![
            Attack::new("Karate Chop", Self::karate_chop),
            Attack::new("Submission", Self::submission),
        ]
    }
}
impl Machoke49 {
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
pub struct Magikarp50 {}
impl CardArchetype for Magikarp50 {
    identifier!("Magikarp (B2 50)");
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
            Attack::new("Tackle", Self::tackle),
            Attack::new("Flail", Self::flail),
        ]
    }
}
impl Magikarp50 {
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
pub struct Magmar51 {}
impl CardArchetype for Magmar51 {
    identifier!("Magmar (B2 51)");
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
    fn attacks(&self) -> Vec<Attack> {
        vec![
            Attack::new("Fire Punch", Self::fire_punch),
            Attack::new("Flamethrower", Self::flamethrower),
        ]
    }
}
impl Magmar51 {
    pub fn fire_punch(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Fire, Type::Fire])
            .damage(30)
    }
    pub fn flamethrower(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Fire, Type::Fire, Type::Colorless])
            .cost(|e| e.discard_attacking_energy_cards(&[Type::Fire]))
            .damage(50)
    }
}

#[derive(Default)]
pub struct Marowak52 {}
impl CardArchetype for Marowak52 {
    identifier!("Marowak (B2 52)");
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
impl Marowak52 {
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
pub struct Nidorina53 {}
impl CardArchetype for Nidorina53 {
    identifier!("Nidorina (B2 53)");
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
impl Nidorina53 {
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
pub struct Nidorino54 {}
impl CardArchetype for Nidorino54 {
    identifier!("Nidorino (B2 54)");
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
    fn attacks(&self) -> Vec<Attack> {
        vec![
            Attack::new("Double Kick", Self::double_kick),
            Attack::new("Horn Drill", Self::horn_drill),
        ]
    }
}
impl Nidorino54 {
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
pub struct Parasect55 {}
impl CardArchetype for Parasect55 {
    identifier!("Parasect (B2 55)");
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
impl Parasect55 {
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
pub struct Persian56 {}
impl CardArchetype for Persian56 {
    identifier!("Persian (B2 56)");
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
impl Persian56 {
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
pub struct Poliwhirl57 {}
impl CardArchetype for Poliwhirl57 {
    identifier!("Poliwhirl (B2 57)");
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
    fn attacks(&self) -> Vec<Attack> {
        vec![
            Attack::new("Amnesia", Self::amnesia),
            Attack::new("Doubleslap", Self::doubleslap),
        ]
    }
}
impl Poliwhirl57 {
    pub fn amnesia(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Water, Type::Water])
            .disable_defending_attack()
    }
    pub fn doubleslap(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Water, Type::Water, Type::Colorless])
            .flip_coins(2)
            .damage_per_heads(30)
    }
}

#[derive(Default)]
pub struct Raticate58 {}
impl CardArchetype for Raticate58 {
    identifier!("Raticate (B2 58)");
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
    fn attacks(&self) -> Vec<Attack> {
        vec![
            Attack::new("Bite", Self::bite),
            Attack::new("Super Fang", Self::super_fang),
        ]
    }
}
impl Raticate58 {
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
pub struct Rhydon59 {}
impl CardArchetype for Rhydon59 {
    identifier!("Rhydon (B2 59)");
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
impl Rhydon59 {
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
pub struct Seaking60 {}
impl CardArchetype for Seaking60 {
    identifier!("Seaking (B2 60)");
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
impl Seaking60 {
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
pub struct Seel61 {}
impl CardArchetype for Seel61 {
    identifier!("Seel (B2 61)");
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
    fn attacks(&self) -> Vec<Attack> {
        vec![
            Attack::new("Headbutt", Self::headbutt),
        ]
    }
}
impl Seel61 {
    pub fn headbutt(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Water])
            .damage(10)
    }
}

#[derive(Default)]
pub struct Tauros62 {}
impl CardArchetype for Tauros62 {
    identifier!("Tauros (B2 62)");
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
impl Tauros62 {
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
pub struct Wartortle63 {}
impl CardArchetype for Wartortle63 {
    identifier!("Wartortle (B2 63)");
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
    fn attacks(&self) -> Vec<Attack> {
        vec![
            Attack::new("Withdraw", Self::withdraw),
            Attack::new("Bite", Self::bite),
        ]
    }
}
impl Wartortle63 {
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
pub struct Weepinbell64 {}
impl CardArchetype for Weepinbell64 {
    identifier!("Weepinbell (B2 64)");
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
impl Weepinbell64 {
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
pub struct Abra65 {}
impl CardArchetype for Abra65 {
    identifier!("Abra (B2 65)");
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
    fn attacks(&self) -> Vec<Attack> {
        vec![
            Attack::new("Psyshock", Self::psyshock),
        ]
    }
}
impl Abra65 {
    pub fn psyshock(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Psychic])
            .flip_a_coin()
            .damage(10)
            .if_heads(|e| e.paralyze())
    }
}

#[derive(Default)]
pub struct Bellsprout66 {}
impl CardArchetype for Bellsprout66 {
    identifier!("Bellsprout (B2 66)");
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
impl Bellsprout66 {
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
pub struct Bulbasaur67 {}
impl CardArchetype for Bulbasaur67 {
    identifier!("Bulbasaur (B2 67)");
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
    fn attacks(&self) -> Vec<Attack> {
        vec![
            Attack::new("Leech Seed", Self::leech_seed),
        ]
    }
}
impl Bulbasaur67 {
    pub fn leech_seed(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Grass, Type::Grass])
            .damage(20)
            .if_did_damage(|e| e.heal_attacking(10))
    }
}

#[derive(Default)]
pub struct Caterpie68 {}
impl CardArchetype for Caterpie68 {
    identifier!("Caterpie (B2 68)");
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
    fn attacks(&self) -> Vec<Attack> {
        vec![
            Attack::new("String Shot", Self::string_shot),
        ]
    }
}
impl Caterpie68 {
    pub fn string_shot(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Grass])
            .flip_a_coin()
            .damage(10)
            .if_heads(|e| e.paralyze())
    }
}

#[derive(Default)]
pub struct Charmander69 {}
impl CardArchetype for Charmander69 {
    identifier!("Charmander (B2 69)");
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
    fn attacks(&self) -> Vec<Attack> {
        vec![
            Attack::new("Scratch", Self::scratch),
            Attack::new("Ember", Self::ember),
        ]
    }
}
impl Charmander69 {
    pub fn scratch(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Colorless])
            .damage(10)
    }
    pub fn ember(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Fire, Type::Colorless])
            .cost(|e| e.discard_attacking_energy_cards(&[Type::Fire]))
            .damage(30)
    }
}

#[derive(Default)]
pub struct Cubone70 {}
impl CardArchetype for Cubone70 {
    identifier!("Cubone (B2 70)");
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
impl Cubone70 {
    pub fn snivel(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
    pub fn rage(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
}

#[derive(Default)]
pub struct Diglett71 {}
impl CardArchetype for Diglett71 {
    identifier!("Diglett (B2 71)");
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
    fn attacks(&self) -> Vec<Attack> {
        vec![
            Attack::new("Dig", Self::dig),
            Attack::new("Mud Slap", Self::mud_slap),
        ]
    }
}
impl Diglett71 {
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
pub struct Doduo72 {}
impl CardArchetype for Doduo72 {
    identifier!("Doduo (B2 72)");
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
    fn attacks(&self) -> Vec<Attack> {
        vec![
            Attack::new("Fury Attack", Self::fury_attack),
        ]
    }
}
impl Doduo72 {
    pub fn fury_attack(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Colorless])
            .flip_coins(2)
            .damage_per_heads(10)
    }
}

#[derive(Default)]
pub struct Drowzee73 {}
impl CardArchetype for Drowzee73 {
    identifier!("Drowzee (B2 73)");
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
            Attack::new("Pound", Self::pound),
            Attack::new("Confuse Ray", Self::confuse_ray),
        ]
    }
}
impl Drowzee73 {
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
pub struct Exeggcute74 {}
impl CardArchetype for Exeggcute74 {
    identifier!("Exeggcute (B2 74)");
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
impl Exeggcute74 {
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
pub struct Gastly75 {}
impl CardArchetype for Gastly75 {
    identifier!("Gastly (B2 75)");
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
    fn attacks(&self) -> Vec<Attack> {
        vec![
            Attack::new("Sleeping Gas", Self::sleeping_gas),
            Attack::new("Destiny Bond", Self::destiny_bond),
        ]
    }
}
impl Gastly75 {
    pub fn sleeping_gas(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Psychic])
            .flip_a_coin()
            .if_heads(|e| e.asleep())
    }
    pub fn destiny_bond(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Psychic, Type::Colorless])
            .cost(|e| e.discard_attacking_energy_cards(&[Type::Psychic]))
            .knock_out_attacker_if_attacking_is_knocked_out_next_turn()
    }
}

#[derive(Default)]
pub struct Goldeen76 {}
impl CardArchetype for Goldeen76 {
    identifier!("Goldeen (B2 76)");
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
impl Goldeen76 {
    pub fn horn_attack(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Water])
            .damage(10)
    }
}

#[derive(Default)]
pub struct Jigglypuff77 {}
impl CardArchetype for Jigglypuff77 {
    identifier!("Jigglypuff (B2 77)");
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
impl Jigglypuff77 {
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
pub struct Machop78 {}
impl CardArchetype for Machop78 {
    identifier!("Machop (B2 78)");
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
            Attack::new("Low Kick", Self::low_kick),
        ]
    }
}
impl Machop78 {
    pub fn low_kick(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Fighting])
            .damage(20)
    }
}

#[derive(Default)]
pub struct Magnemite79 {}
impl CardArchetype for Magnemite79 {
    identifier!("Magnemite (B2 79)");
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
            Attack::new("Thunder Wave", Self::thunder_wave),
            Attack::new("Selfdestruct", Self::selfdestruct),
        ]
    }
}
impl Magnemite79 {
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
pub struct Meowth80 {}
impl CardArchetype for Meowth80 {
    identifier!("Meowth (B2 80)");
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
impl Meowth80 {
    pub fn pay_day(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Colorless, Type::Colorless])
            .flip_a_coin()
            .damage(10)
            .if_heads(|e| e.draw(1))
    }
}

#[derive(Default)]
pub struct Metapod81 {}
impl CardArchetype for Metapod81 {
    identifier!("Metapod (B2 81)");
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
    fn attacks(&self) -> Vec<Attack> {
        vec![
            Attack::new("Stiffen", Self::stiffen),
            Attack::new("Stun Spore", Self::stun_spore),
        ]
    }
}
impl Metapod81 {
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
pub struct NidoranF82 {}
impl CardArchetype for NidoranF82 {
    identifier!("Nidoran ♀ (B2 82)");
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
impl NidoranF82 {
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
pub struct NidoranM83 {}
impl CardArchetype for NidoranM83 {
    identifier!("Nidoran ♂ (B2 83)");
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
    fn attacks(&self) -> Vec<Attack> {
        vec![
            Attack::new("Horn Hazard", Self::horn_hazard),
        ]
    }
}
impl NidoranM83 {
    pub fn horn_hazard(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Grass])
            .flip_a_coin()
            .if_heads(|e| e.damage(30))
    }
}

#[derive(Default)]
pub struct Onix84 {}
impl CardArchetype for Onix84 {
    identifier!("Onix (B2 84)");
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
    fn attacks(&self) -> Vec<Attack> {
        vec![
            Attack::new("Rock Throw", Self::rock_throw),
            Attack::new("Harden", Self::harden),
        ]
    }
}
impl Onix84 {
    pub fn rock_throw(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Fighting])
            .damage(10)
    }
    pub fn harden(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Fighting, Type::Fighting])
            .prevent_up_to_damage_during_opponents_next_turn(30)
    }
}

#[derive(Default)]
pub struct Paras85 {}
impl CardArchetype for Paras85 {
    identifier!("Paras (B2 85)");
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
impl Paras85 {
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
pub struct Pidgey86 {}
impl CardArchetype for Pidgey86 {
    identifier!("Pidgey (B2 86)");
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
    fn attacks(&self) -> Vec<Attack> {
        vec![
            Attack::new("Whirlwind", Self::whirlwind),
        ]
    }
}
impl Pidgey86 {
    pub fn whirlwind(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Colorless, Type::Colorless])
            .damage(10)
            .switch_defending()
    }
}

#[derive(Default)]
pub struct Pikachu87 {}
impl CardArchetype for Pikachu87 {
    identifier!("Pikachu (B2 87)");
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
    fn attacks(&self) -> Vec<Attack> {
        vec![
            Attack::new("Gnaw", Self::gnaw),
            Attack::new("Thunder Jolt", Self::thunder_jolt),
        ]
    }
}
impl Pikachu87 {
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
pub struct Poliwag88 {}
impl CardArchetype for Poliwag88 {
    identifier!("Poliwag (B2 88)");
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
    fn attacks(&self) -> Vec<Attack> {
        vec![
            Attack::new("Water Gun", Self::water_gun),
        ]
    }
}
impl Poliwag88 {
    pub fn water_gun(_builder: AttackBuilder) -> AttackBuilder {
        unimplemented!();
    }
}

#[derive(Default)]
pub struct Rattata89 {}
impl CardArchetype for Rattata89 {
    identifier!("Rattata (B2 89)");
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
    fn attacks(&self) -> Vec<Attack> {
        vec![
            Attack::new("Bite", Self::bite),
        ]
    }
}
impl Rattata89 {
    pub fn bite(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Colorless])
            .damage(20)
    }
}

#[derive(Default)]
pub struct Rhyhorn90 {}
impl CardArchetype for Rhyhorn90 {
    identifier!("Rhyhorn (B2 90)");
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
impl Rhyhorn90 {
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
pub struct Sandshrew91 {}
impl CardArchetype for Sandshrew91 {
    identifier!("Sandshrew (B2 91)");
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
    fn attacks(&self) -> Vec<Attack> {
        vec![
            Attack::new("Sand-attack", Self::sand_attack),
        ]
    }
}
impl Sandshrew91 {
    pub fn sand_attack(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Fighting])
            .prevent_attack_on_a_flip_during_opponents_next_turn()
    }
}

#[derive(Default)]
pub struct Spearow92 {}
impl CardArchetype for Spearow92 {
    identifier!("Spearow (B2 92)");
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
impl Spearow92 {
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
pub struct Squirtle93 {}
impl CardArchetype for Squirtle93 {
    identifier!("Squirtle (B2 93)");
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
    fn attacks(&self) -> Vec<Attack> {
        vec![
            Attack::new("Bubble", Self::bubble),
            Attack::new("Withdraw", Self::withdraw),
        ]
    }
}
impl Squirtle93 {
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
pub struct Starmie94 {}
impl CardArchetype for Starmie94 {
    identifier!("Starmie (B2 94)");
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
    fn attacks(&self) -> Vec<Attack> {
        vec![
            Attack::new("Recover", Self::recover),
            Attack::new("Star Freeze", Self::star_freeze),
        ]
    }
}
impl Starmie94 {
    pub fn recover(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Water, Type::Water])
            .cost(|e| e.discard_attacking_energy_cards(&[Type::Water]))
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
pub struct Staryu95 {}
impl CardArchetype for Staryu95 {
    identifier!("Staryu (B2 95)");
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
    fn attacks(&self) -> Vec<Attack> {
        vec![
            Attack::new("Slap", Self::slap),
        ]
    }
}
impl Staryu95 {
    pub fn slap(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Water])
            .damage(20)
    }
}

#[derive(Default)]
pub struct Tangela96 {}
impl CardArchetype for Tangela96 {
    identifier!("Tangela (B2 96)");
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
    fn attacks(&self) -> Vec<Attack> {
        vec![
            Attack::new("Bind", Self::bind),
            Attack::new("Poisonpowder", Self::poisonpowder),
        ]
    }
}
impl Tangela96 {
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
pub struct Venonat97 {}
impl CardArchetype for Venonat97 {
    identifier!("Venonat (B2 97)");
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
impl Venonat97 {
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

#[derive(Default)]
pub struct Voltorb98 {}
impl CardArchetype for Voltorb98 {
    identifier!("Voltorb (B2 98)");
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
            Attack::new("Tackle", Self::tackle),
        ]
    }
}
impl Voltorb98 {
    pub fn tackle(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Colorless])
            .damage(10)
    }
}

#[derive(Default)]
pub struct Vulpix99 {}
impl CardArchetype for Vulpix99 {
    identifier!("Vulpix (B2 99)");
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
    fn attacks(&self) -> Vec<Attack> {
        vec![
            Attack::new("Confuse Ray", Self::confuse_ray),
        ]
    }
}
impl Vulpix99 {
    pub fn confuse_ray(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Fire, Type::Fire])
            .flip_a_coin()
            .damage(10)
            .if_heads(|e| e.confuse())
    }
}

#[derive(Default)]
pub struct Weedle100 {}
impl CardArchetype for Weedle100 {
    identifier!("Weedle (B2 100)");
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
    fn attacks(&self) -> Vec<Attack> {
        vec![
            Attack::new("Poison Sting", Self::poison_sting),
        ]
    }
}
impl Weedle100 {
    pub fn poison_sting(builder: AttackBuilder) -> AttackBuilder {
        builder
            .attack_cost(&[Type::Grass])
            .flip_a_coin()
            .damage(10)
            .if_heads(|e| e.poison())
    }
}