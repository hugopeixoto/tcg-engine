use crate::*;
use crate::carddb::Attacks;

#[derive(Default)]
pub struct Alakazam {}
impl CardArchetype for Alakazam {
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
            .register("Confuse Ray", &[Type::Psychic, Type::Psychic, Type::Psychic], Self::confuse_ray)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Alakazam {
    pub fn confuse_ray(engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameEngine {
        let heads = dm.flip(1).heads() == 1;
        engine
            .damage(30)
            .then_if(heads, |e| e.confuse())
    }
}

#[derive(Default)]
pub struct Blastoise {}
impl CardArchetype for Blastoise {
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
            .register("Hydro Pump", &[Type::Water, Type::Water, Type::Water], Self::hydro_pump)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Blastoise {
    pub fn hydro_pump(_engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        unimplemented!();
    }
}

#[derive(Default)]
pub struct Chansey {}
impl CardArchetype for Chansey {
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
            .register("Scrunch", &[Type::Colorless, Type::Colorless], Self::scrunch)
            .register("Double-edge", &[Type::Colorless, Type::Colorless, Type::Colorless, Type::Colorless], Self::double_edge)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Chansey {
    pub fn scrunch(engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameEngine {
        let heads = dm.flip(1).heads() == 1;
        engine
            .then_if(heads, |e| e.with_effect(Effect {
                name: "NO_DAMAGE_DURING_OPPONENTS_NEXT_TURN".into(),
                source: EffectSource::Attack(e.player(), e.attacking().id),
                target: EffectTarget::InPlay(e.player(), e.attacking().id),
                consequence: EffectConsequence::BlockDamage,
                expires: EffectExpiration::EndOfTurn(e.opponent(), 0),
                system: false,
            })
        )
    }
    pub fn double_edge(engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine
            .damage(80)
            .damage_self(80)
    }
}

#[derive(Default)]
pub struct Charizard {}
impl CardArchetype for Charizard {
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
            .register("Fire Spin", &[Type::Fire, Type::Fire, Type::Fire, Type::Fire], Self::fire_spin)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Charizard {
    pub fn fire_spin(_engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        unimplemented!();
    }
}

#[derive(Default)]
pub struct Clefairy {}
impl CardArchetype for Clefairy {
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
            .register("Sing", &[Type::Colorless], Self::sing)
            .register("Metronome", &[Type::Colorless, Type::Colorless, Type::Colorless], Self::metronome)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Clefairy {
    pub fn sing(engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameEngine {
        let heads = dm.flip(1).heads() == 1;
        engine
            .then_if(heads, |e| e.asleep())
    }
    pub fn metronome(_engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        unimplemented!();
    }
}

#[derive(Default)]
pub struct Gyarados {}
impl CardArchetype for Gyarados {
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
            .register("Dragon Rage", &[Type::Water, Type::Water, Type::Water], Self::dragon_rage)
            .register("Bubblebeam", &[Type::Water, Type::Water, Type::Water, Type::Water], Self::bubblebeam)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Gyarados {
    pub fn dragon_rage(engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine
            .damage(50)
    }
    pub fn bubblebeam(engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameEngine {
        let heads = dm.flip(1).heads() == 1;
        engine
            .damage(40)
            .then_if(heads, |e| e.paralyze())
    }
}

#[derive(Default)]
pub struct Hitmonchan {}
impl CardArchetype for Hitmonchan {
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
            .register("Jab", &[Type::Fighting], Self::jab)
            .register("Special Punch", &[Type::Fighting, Type::Fighting, Type::Colorless], Self::special_punch)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Hitmonchan {
    pub fn jab(engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine
            .damage(20)
    }
    pub fn special_punch(engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine
            .damage(40)
    }
}

#[derive(Default)]
pub struct Machamp {}
impl CardArchetype for Machamp {
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
            .register("Seismic Toss", &[Type::Fighting, Type::Fighting, Type::Fighting, Type::Colorless], Self::seismic_toss)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Machamp {
    pub fn seismic_toss(engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine
            .damage(60)
    }
}

#[derive(Default)]
pub struct Magneton {}
impl CardArchetype for Magneton {
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
            .register("Thunder Wave", &[Type::Lightning, Type::Lightning, Type::Colorless], Self::thunder_wave)
            .register("Selfdestruct", &[Type::Lightning, Type::Lightning, Type::Colorless, Type::Colorless], Self::selfdestruct)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Magneton {
    pub fn thunder_wave(engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameEngine {
        let heads = dm.flip(1).heads() == 1;
        engine
            .damage(30)
            .then_if(heads, |e| e.paralyze())
    }
    pub fn selfdestruct(engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine
            .damage(80)
            .then(|e| e.target_all(e.bench(e.player()), |e2| e2.damage(20)))
            .then(|e| e.target_all(e.bench(e.opponent()), |e2| e2.damage(20)))
            .damage_self(80)
    }
}

#[derive(Default)]
pub struct Mewtwo {}
impl CardArchetype for Mewtwo {
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
            .register("Psychic", &[Type::Psychic, Type::Colorless], Self::psychic)
            .register("Barrier", &[Type::Psychic, Type::Psychic], Self::barrier)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Mewtwo {
    pub fn psychic(_engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        unimplemented!();
    }
    pub fn barrier(_engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        unimplemented!();
    }
}

#[derive(Default)]
pub struct Nidoking {}
impl CardArchetype for Nidoking {
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
            .register("Thrash", &[Type::Grass, Type::Colorless, Type::Colorless], Self::thrash)
            .register("Toxic", &[Type::Grass, Type::Grass, Type::Grass], Self::toxic)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Nidoking {
    pub fn thrash(engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameEngine {
        let heads = dm.flip(1).heads() == 1;
        engine
            .then_if(heads, |e| e.damage(40))
            .then_if(!heads, |e| e.damage(30).damage_self(10))
    }
    pub fn toxic(engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine
            .damage(20)
            .severe_poison(2)
    }
}

#[derive(Default)]
pub struct Ninetales {}
impl CardArchetype for Ninetales {
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
            .register("Lure", &[Type::Colorless, Type::Colorless], Self::lure)
            .register("Fire Blast", &[Type::Fire, Type::Fire, Type::Fire, Type::Fire], Self::fire_blast)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Ninetales {
    pub fn lure(_engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        unimplemented!();
    }
    pub fn fire_blast(engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameEngine {
        engine
            .discard_attached_energies(engine.player(), engine.attacking(), &[Type::Fire], dm)
            .damage(80)
    }
}

#[derive(Default)]
pub struct Poliwrath {}
impl CardArchetype for Poliwrath {
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
            .register("Water Gun", &[Type::Water, Type::Water, Type::Colorless], Self::water_gun)
            .register("Whirlpool", &[Type::Water, Type::Water, Type::Colorless, Type::Colorless], Self::whirlpool)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Poliwrath {
    pub fn water_gun(_engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        unimplemented!();
    }
    pub fn whirlpool(_engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        unimplemented!();
    }
}

#[derive(Default)]
pub struct Raichu {}
impl CardArchetype for Raichu {
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
            .register("Agility", &[Type::Lightning, Type::Colorless, Type::Colorless], Self::agility)
            .register("Thunder", &[Type::Lightning, Type::Lightning, Type::Lightning, Type::Colorless], Self::thunder)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Raichu {
    pub fn agility(_engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        unimplemented!();
    }
    pub fn thunder(engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameEngine {
        let heads = dm.flip(1).heads() == 1;
        engine
            .damage(60)
            .then_if(!heads, |e| e.damage_self(30))
    }
}

#[derive(Default)]
pub struct Venusaur {}
impl CardArchetype for Venusaur {
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
            .register("Solarbeam", &[Type::Grass, Type::Grass, Type::Grass, Type::Grass], Self::solarbeam)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Venusaur {
    pub fn solarbeam(engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine
            .damage(60)
    }
}

#[derive(Default)]
pub struct Zapdos {}
impl CardArchetype for Zapdos {
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
            .register("Thunder", &[Type::Lightning, Type::Lightning, Type::Lightning, Type::Colorless], Self::thunder)
            .register("Thunderbolt", &[Type::Lightning, Type::Lightning, Type::Lightning, Type::Lightning], Self::thunderbolt)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Zapdos {
    pub fn thunder(engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameEngine {
        let heads = dm.flip(1).heads() == 1;
        engine
            .damage(60)
            .then_if(!heads, |e| e.damage_self(30))
    }
    pub fn thunderbolt(engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameEngine {
        engine
            .discard_all_attached_energies(engine.player(), engine.attacking(), dm)
            .damage(100)
    }
}

#[derive(Default)]
pub struct Beedrill {}
impl CardArchetype for Beedrill {
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
            .register("Twineedle", &[Type::Colorless, Type::Colorless, Type::Colorless], Self::twineedle)
            .register("Poison Sting", &[Type::Grass, Type::Grass, Type::Grass], Self::poison_sting)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Beedrill {
    pub fn twineedle(engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameEngine {
        let flip_results = dm.flip(2);
        engine
            .damage(30 * flip_results.heads())
    }
    pub fn poison_sting(engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameEngine {
        let heads = dm.flip(1).heads() == 1;
        engine
            .damage(40)
            .then_if(heads, |e| e.poison())
    }
}

#[derive(Default)]
pub struct Dragonair {}
impl CardArchetype for Dragonair {
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
            .register("Slam", &[Type::Colorless, Type::Colorless, Type::Colorless], Self::slam)
            .register("Hyper Beam", &[Type::Colorless, Type::Colorless, Type::Colorless, Type::Colorless], Self::hyper_beam)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Dragonair {
    pub fn slam(engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameEngine {
        let flip_results = dm.flip(2);
        engine
            .damage(30 * flip_results.heads())
    }
    pub fn hyper_beam(_engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        unimplemented!();
    }
}

#[derive(Default)]
pub struct Dugtrio {}
impl CardArchetype for Dugtrio {
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
            .register("Slash", &[Type::Fighting, Type::Fighting, Type::Colorless], Self::slash)
            .register("Earthquake", &[Type::Fighting, Type::Fighting, Type::Fighting, Type::Fighting], Self::earthquake)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Dugtrio {
    pub fn slash(engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine
            .damage(40)
    }
    pub fn earthquake(engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine
            .damage(70)
            .then(|e| e.target_all(e.bench(e.player()), |e2| e2.damage(10)))
    }
}

#[derive(Default)]
pub struct Electabuzz {}
impl CardArchetype for Electabuzz {
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
            .register("Thundershock", &[Type::Lightning], Self::thundershock)
            .register("Thunderpunch", &[Type::Lightning, Type::Colorless], Self::thunderpunch)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Electabuzz {
    pub fn thundershock(engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameEngine {
        let heads = dm.flip(1).heads() == 1;
        engine
            .damage(10)
            .then_if(heads, |e| e.paralyze())
    }
    pub fn thunderpunch(engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameEngine {
        let heads = dm.flip(1).heads() == 1;
        engine
            .then_if(heads, |e| e.damage(40))
            .then_if(!heads, |e| e.damage(30).damage_self(10))
    }
}

#[derive(Default)]
pub struct Electrode {}
impl CardArchetype for Electrode {
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
            .register("Electric Shock", &[Type::Lightning, Type::Lightning, Type::Lightning], Self::electric_shock)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Electrode {
    pub fn electric_shock(engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameEngine {
        let heads = dm.flip(1).heads() == 1;
        engine
            .damage(50)
            .then_if(!heads, |e| e.damage_self(10))
    }
}

#[derive(Default)]
pub struct Pidgeotto {}
impl CardArchetype for Pidgeotto {
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
            .register("Whirlwind", &[Type::Colorless, Type::Colorless], Self::whirlwind)
            .register("Mirror Move", &[Type::Colorless, Type::Colorless, Type::Colorless], Self::mirror_move)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Pidgeotto {
    pub fn whirlwind(_engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        unimplemented!();
    }
    pub fn mirror_move(_engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        unimplemented!();
    }
}

#[derive(Default)]
pub struct Arcanine {}
impl CardArchetype for Arcanine {
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
            .register("Flamethrower", &[Type::Fire, Type::Fire, Type::Colorless], Self::flamethrower)
            .register("Take Down", &[Type::Fire, Type::Fire, Type::Colorless, Type::Colorless], Self::take_down)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Arcanine {
    pub fn flamethrower(engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameEngine {
        engine
            .discard_attached_energies(engine.player(), engine.attacking(), &[Type::Fire], dm)
            .damage(50)
    }
    pub fn take_down(engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine
            .damage(80)
            .damage_self(30)
    }
}

#[derive(Default)]
pub struct Charmeleon {}
impl CardArchetype for Charmeleon {
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
            .register("Slash", &[Type::Colorless, Type::Colorless, Type::Colorless], Self::slash)
            .register("Flamethrower", &[Type::Fire, Type::Fire, Type::Colorless], Self::flamethrower)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Charmeleon {
    pub fn slash(engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine
            .damage(30)
    }
    pub fn flamethrower(engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameEngine {
        engine
            .discard_attached_energies(engine.player(), engine.attacking(), &[Type::Fire], dm)
            .damage(50)
    }
}

#[derive(Default)]
pub struct Dewgong {}
impl CardArchetype for Dewgong {
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
            .register("Aurora Beam", &[Type::Water, Type::Water, Type::Colorless], Self::aurora_beam)
            .register("Ice Beam", &[Type::Water, Type::Water, Type::Colorless, Type::Colorless], Self::ice_beam)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Dewgong {
    pub fn aurora_beam(engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine
            .damage(50)
    }
    pub fn ice_beam(engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameEngine {
        let heads = dm.flip(1).heads() == 1;
        engine
            .damage(30)
            .then_if(heads, |e| e.paralyze())
    }
}

#[derive(Default)]
pub struct Dratini {}
impl CardArchetype for Dratini {
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
            .register("Pound", &[Type::Colorless], Self::pound)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Dratini {
    pub fn pound(engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine
            .damage(10)
    }
}

#[derive(Default)]
pub struct FarfetchD {}
impl CardArchetype for FarfetchD {
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
            .register("Leek Slap", &[Type::Colorless], Self::leek_slap)
            .register("Pot Smash", &[Type::Colorless, Type::Colorless, Type::Colorless], Self::pot_smash)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl FarfetchD {
    pub fn leek_slap(_engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        unimplemented!();
    }
    pub fn pot_smash(engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine
            .damage(30)
    }
}

#[derive(Default)]
pub struct Growlithe {}
impl CardArchetype for Growlithe {
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
            .register("Flare", &[Type::Fire, Type::Colorless], Self::flare)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Growlithe {
    pub fn flare(engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine
            .damage(20)
    }
}

#[derive(Default)]
pub struct Haunter {}
impl CardArchetype for Haunter {
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
            .register("Hypnosis", &[Type::Psychic], Self::hypnosis)
            .register("Dream Eater", &[Type::Psychic, Type::Psychic], Self::dream_eater)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Haunter {
    pub fn hypnosis(engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine
            .asleep()
    }
    pub fn dream_eater(_engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        unimplemented!();
    }
}

#[derive(Default)]
pub struct Ivysaur {}
impl CardArchetype for Ivysaur {
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
            .register("Vine Whip", &[Type::Grass, Type::Colorless, Type::Colorless], Self::vine_whip)
            .register("Poisonpowder", &[Type::Grass, Type::Grass, Type::Grass], Self::poisonpowder)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Ivysaur {
    pub fn vine_whip(engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine
            .damage(30)
    }
    pub fn poisonpowder(engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine
            .damage(20)
            .poison()
    }
}

#[derive(Default)]
pub struct Jynx {}
impl CardArchetype for Jynx {
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
            .register("Doubleslap", &[Type::Psychic], Self::doubleslap)
            .register("Meditate", &[Type::Psychic, Type::Psychic, Type::Colorless], Self::meditate)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Jynx {
    pub fn doubleslap(engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameEngine {
        let flip_results = dm.flip(2);
        engine
            .damage(10 * flip_results.heads())
    }
    pub fn meditate(_engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        unimplemented!();
    }
}

#[derive(Default)]
pub struct Kadabra {}
impl CardArchetype for Kadabra {
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
            .register("Recover", &[Type::Psychic, Type::Psychic], Self::recover)
            .register("Super Psy", &[Type::Psychic, Type::Psychic, Type::Colorless], Self::super_psy)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Kadabra {
    pub fn recover(_engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        unimplemented!();
    }
    pub fn super_psy(engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine
            .damage(50)
    }
}

#[derive(Default)]
pub struct Kakuna {}
impl CardArchetype for Kakuna {
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
            .register("Stiffen", &[Type::Colorless, Type::Colorless], Self::stiffen)
            .register("Poisonpowder", &[Type::Grass, Type::Grass], Self::poisonpowder)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Kakuna {
    pub fn stiffen(engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameEngine {
        let heads = dm.flip(1).heads() == 1;
        engine
            .then_if(heads, |e| e.with_effect(Effect {
                name: "NO_DAMAGE_DURING_OPPONENTS_NEXT_TURN".into(),
                source: EffectSource::Attack(e.player(), e.attacking().id),
                target: EffectTarget::InPlay(e.player(), e.attacking().id),
                consequence: EffectConsequence::BlockDamage,
                expires: EffectExpiration::EndOfTurn(e.opponent(), 0),
                system: false,
            })
        )
    }
    pub fn poisonpowder(engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameEngine {
        let heads = dm.flip(1).heads() == 1;
        engine
            .damage(20)
            .then_if(heads, |e| e.poison())
    }
}

#[derive(Default)]
pub struct Machoke {}
impl CardArchetype for Machoke {
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
            .register("Karate Chop", &[Type::Fighting, Type::Fighting, Type::Colorless], Self::karate_chop)
            .register("Submission", &[Type::Fighting, Type::Fighting, Type::Colorless, Type::Colorless], Self::submission)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Machoke {
    pub fn karate_chop(engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine
            .damage(50usize.saturating_sub(engine.damage_counters_on(engine.attacking()) * 10))
    }
    pub fn submission(engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine
            .damage(60)
            .damage_self(20)
    }
}

#[derive(Default)]
pub struct Magikarp {}
impl CardArchetype for Magikarp {
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
            .register("Tackle", &[Type::Colorless], Self::tackle)
            .register("Flail", &[Type::Water], Self::flail)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Magikarp {
    pub fn tackle(engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine
            .damage(10)
    }
    pub fn flail(_engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        unimplemented!();
    }
}

#[derive(Default)]
pub struct Magmar {}
impl CardArchetype for Magmar {
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
            .register("Fire Punch", &[Type::Fire, Type::Fire], Self::fire_punch)
            .register("Flamethrower", &[Type::Fire, Type::Fire, Type::Colorless], Self::flamethrower)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Magmar {
    pub fn fire_punch(engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine
            .damage(30)
    }
    pub fn flamethrower(engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameEngine {
        engine
            .discard_attached_energies(engine.player(), engine.attacking(), &[Type::Fire], dm)
            .damage(50)
    }
}

#[derive(Default)]
pub struct Nidorino {}
impl CardArchetype for Nidorino {
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
            .register("Double Kick", &[Type::Grass, Type::Colorless, Type::Colorless], Self::double_kick)
            .register("Horn Drill", &[Type::Grass, Type::Grass, Type::Colorless, Type::Colorless], Self::horn_drill)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Nidorino {
    pub fn double_kick(engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameEngine {
        let flip_results = dm.flip(2);
        engine
            .damage(30 * flip_results.heads())
    }
    pub fn horn_drill(engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine
            .damage(50)
    }
}

#[derive(Default)]
pub struct Poliwhirl {}
impl CardArchetype for Poliwhirl {
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
            .register("Amnesia", &[Type::Water, Type::Water], Self::amnesia)
            .register("Doubleslap", &[Type::Water, Type::Water, Type::Colorless], Self::doubleslap)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Poliwhirl {
    pub fn amnesia(_engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        unimplemented!();
    }
    pub fn doubleslap(engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameEngine {
        let flip_results = dm.flip(2);
        engine
            .damage(30 * flip_results.heads())
    }
}

#[derive(Default)]
pub struct Porygon {}
impl CardArchetype for Porygon {
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
            .register("Conversion 1", &[Type::Colorless], Self::conversion_1)
            .register("Conversion 2", &[Type::Colorless, Type::Colorless], Self::conversion_2)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Porygon {
    pub fn conversion_1(_engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        unimplemented!();
    }
    pub fn conversion_2(_engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        unimplemented!();
    }
}

#[derive(Default)]
pub struct Raticate {}
impl CardArchetype for Raticate {
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
            .register("Bite", &[Type::Colorless], Self::bite)
            .register("Super Fang", &[Type::Colorless, Type::Colorless, Type::Colorless], Self::super_fang)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Raticate {
    pub fn bite(engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine
            .damage(20)
    }
    pub fn super_fang(_engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        unimplemented!();
    }
}

#[derive(Default)]
pub struct Seel {}
impl CardArchetype for Seel {
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
            .register("Headbutt", &[Type::Water], Self::headbutt)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Seel {
    pub fn headbutt(engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine
            .damage(10)
    }
}

#[derive(Default)]
pub struct Wartortle {}
impl CardArchetype for Wartortle {
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
            .register("Withdraw", &[Type::Water, Type::Colorless], Self::withdraw)
            .register("Bite", &[Type::Water, Type::Colorless, Type::Colorless], Self::bite)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Wartortle {
    pub fn withdraw(engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameEngine {
        let heads = dm.flip(1).heads() == 1;
        engine
            .then_if(heads, |e| e.with_effect(Effect {
                name: "NO_DAMAGE_DURING_OPPONENTS_NEXT_TURN".into(),
                source: EffectSource::Attack(e.player(), e.attacking().id),
                target: EffectTarget::InPlay(e.player(), e.attacking().id),
                consequence: EffectConsequence::BlockDamage,
                expires: EffectExpiration::EndOfTurn(e.opponent(), 0),
                system: false,
            })
        )
    }
    pub fn bite(engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine
            .damage(40)
    }
}

#[derive(Default)]
pub struct Abra {}
impl CardArchetype for Abra {
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
            .register("Psyshock", &[Type::Psychic], Self::psyshock)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Abra {
    pub fn psyshock(engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameEngine {
        let heads = dm.flip(1).heads() == 1;
        engine
            .damage(10)
            .then_if(heads, |e| e.paralyze())
    }
}

#[derive(Default)]
pub struct Bulbasaur {}
impl CardArchetype for Bulbasaur {
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
            .register("Leech Seed", &[Type::Grass, Type::Grass], Self::leech_seed)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Bulbasaur {
    pub fn leech_seed(_engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        unimplemented!();
    }
}

#[derive(Default)]
pub struct Caterpie {}
impl CardArchetype for Caterpie {
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
            .register("String Shot", &[Type::Grass], Self::string_shot)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Caterpie {
    pub fn string_shot(engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameEngine {
        let heads = dm.flip(1).heads() == 1;
        engine
            .damage(10)
            .then_if(heads, |e| e.paralyze())
    }
}

#[derive(Default)]
pub struct Charmander {}
impl CardArchetype for Charmander {
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
            .register("Scratch", &[Type::Colorless], Self::scratch)
            .register("Ember", &[Type::Fire, Type::Colorless], Self::ember)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Charmander {
    pub fn scratch(engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine
            .damage(10)
    }
    pub fn ember(engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameEngine {
        engine
            .discard_attached_energies(engine.player(), engine.attacking(), &[Type::Fire], dm)
            .damage(30)
    }
}

#[derive(Default)]
pub struct Diglett {}
impl CardArchetype for Diglett {
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
            .register("Dig", &[Type::Fighting], Self::dig)
            .register("Mud Slap", &[Type::Fighting, Type::Fighting], Self::mud_slap)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Diglett {
    pub fn dig(engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine
            .damage(10)
    }
    pub fn mud_slap(engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine
            .damage(30)
    }
}

#[derive(Default)]
pub struct Doduo {}
impl CardArchetype for Doduo {
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
            .register("Fury Attack", &[Type::Colorless], Self::fury_attack)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Doduo {
    pub fn fury_attack(engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameEngine {
        let flip_results = dm.flip(2);
        engine
            .damage(10 * flip_results.heads())
    }
}

#[derive(Default)]
pub struct Drowzee {}
impl CardArchetype for Drowzee {
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
            .register("Pound", &[Type::Colorless], Self::pound)
            .register("Confuse Ray", &[Type::Psychic, Type::Psychic], Self::confuse_ray)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Drowzee {
    pub fn pound(engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine
            .damage(10)
    }
    pub fn confuse_ray(engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameEngine {
        let heads = dm.flip(1).heads() == 1;
        engine
            .damage(10)
            .then_if(heads, |e| e.confuse())
    }
}

#[derive(Default)]
pub struct Gastly {}
impl CardArchetype for Gastly {
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
            .register("Sleeping Gas", &[Type::Psychic], Self::sleeping_gas)
            .register("Destiny Bond", &[Type::Psychic, Type::Colorless], Self::destiny_bond)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Gastly {
    pub fn sleeping_gas(engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameEngine {
        let heads = dm.flip(1).heads() == 1;
        engine
            .then_if(heads, |e| e.asleep())
    }
    pub fn destiny_bond(_engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        unimplemented!();
    }
}

#[derive(Default)]
pub struct Koffing {}
impl CardArchetype for Koffing {
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
            .register("Foul Gas", &[Type::Grass, Type::Grass], Self::foul_gas)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Koffing {
    pub fn foul_gas(engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameEngine {
        let heads = dm.flip(1).heads() == 1;
        engine
            .damage(10)
            .then_if(heads, |e| e.poison())
            .then_if(!heads, |e| e.confuse())
    }
}

#[derive(Default)]
pub struct Machop {}
impl CardArchetype for Machop {
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
            .register("Low Kick", &[Type::Fighting], Self::low_kick)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Machop {
    pub fn low_kick(engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine
            .damage(20)
    }
}

#[derive(Default)]
pub struct Magnemite {}
impl CardArchetype for Magnemite {
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
            .register("Thunder Wave", &[Type::Lightning], Self::thunder_wave)
            .register("Selfdestruct", &[Type::Lightning, Type::Colorless], Self::selfdestruct)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Magnemite {
    pub fn thunder_wave(engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameEngine {
        let heads = dm.flip(1).heads() == 1;
        engine
            .damage(10)
            .then_if(heads, |e| e.paralyze())
    }
    pub fn selfdestruct(engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine
            .damage(40)
            .then(|e| e.target_all(e.bench(e.player()), |e2| e2.damage(10)))
            .then(|e| e.target_all(e.bench(e.opponent()), |e2| e2.damage(10)))
            .damage_self(40)
    }
}

#[derive(Default)]
pub struct Metapod {}
impl CardArchetype for Metapod {
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
            .register("Stiffen", &[Type::Colorless, Type::Colorless], Self::stiffen)
            .register("Stun Spore", &[Type::Grass, Type::Grass], Self::stun_spore)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Metapod {
    pub fn stiffen(engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameEngine {
        let heads = dm.flip(1).heads() == 1;
        engine
            .then_if(heads, |e| e.with_effect(Effect {
                name: "NO_DAMAGE_DURING_OPPONENTS_NEXT_TURN".into(),
                source: EffectSource::Attack(e.player(), e.attacking().id),
                target: EffectTarget::InPlay(e.player(), e.attacking().id),
                consequence: EffectConsequence::BlockDamage,
                expires: EffectExpiration::EndOfTurn(e.opponent(), 0),
                system: false,
            })
        )
    }
    pub fn stun_spore(engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameEngine {
        let heads = dm.flip(1).heads() == 1;
        engine
            .damage(20)
            .then_if(heads, |e| e.paralyze())
    }
}

#[derive(Default)]
pub struct NidoranM {}
impl CardArchetype for NidoranM {
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
            .register("Horn Hazard", &[Type::Grass], Self::horn_hazard)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl NidoranM {
    pub fn horn_hazard(_engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        unimplemented!();
    }
}

#[derive(Default)]
pub struct Onix {}
impl CardArchetype for Onix {
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
            .register("Rock Throw", &[Type::Fighting], Self::rock_throw)
            .register("Harden", &[Type::Fighting, Type::Fighting], Self::harden)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Onix {
    pub fn rock_throw(engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine
            .damage(10)
    }
    pub fn harden(_engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        unimplemented!();
    }
}

#[derive(Default)]
pub struct Pidgey {}
impl CardArchetype for Pidgey {
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
            .register("Whirlwind", &[Type::Colorless, Type::Colorless], Self::whirlwind)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Pidgey {
    pub fn whirlwind(_engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        unimplemented!();
    }
}

#[derive(Default)]
pub struct Pikachu {}
impl CardArchetype for Pikachu {
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
            .register("Gnaw", &[Type::Colorless], Self::gnaw)
            .register("Thunder Jolt", &[Type::Lightning, Type::Colorless], Self::thunder_jolt)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Pikachu {
    pub fn gnaw(engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine
            .damage(10)
    }
    pub fn thunder_jolt(engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameEngine {
        let heads = dm.flip(1).heads() == 1;
        engine
            .damage(30)
            .then_if(!heads, |e| e.damage_self(10))
    }
}

#[derive(Default)]
pub struct Poliwag {}
impl CardArchetype for Poliwag {
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
            .register("Water Gun", &[Type::Water], Self::water_gun)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Poliwag {
    pub fn water_gun(_engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        unimplemented!();
    }
}

#[derive(Default)]
pub struct Ponyta {}
impl CardArchetype for Ponyta {
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
            .register("Smash Kick", &[Type::Colorless, Type::Colorless], Self::smash_kick)
            .register("Flame Tail", &[Type::Fire, Type::Fire], Self::flame_tail)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Ponyta {
    pub fn smash_kick(engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine
            .damage(20)
    }
    pub fn flame_tail(engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine
            .damage(30)
    }
}

#[derive(Default)]
pub struct Rattata {}
impl CardArchetype for Rattata {
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
            .register("Bite", &[Type::Colorless], Self::bite)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Rattata {
    pub fn bite(engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine
            .damage(20)
    }
}

#[derive(Default)]
pub struct Sandshrew {}
impl CardArchetype for Sandshrew {
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
            .register("Sand-attack", &[Type::Fighting], Self::sand_attack)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Sandshrew {
    pub fn sand_attack(_engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        unimplemented!();
    }
}

#[derive(Default)]
pub struct Squirtle {}
impl CardArchetype for Squirtle {
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
            .register("Bubble", &[Type::Water], Self::bubble)
            .register("Withdraw", &[Type::Water, Type::Colorless], Self::withdraw)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Squirtle {
    pub fn bubble(engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameEngine {
        let heads = dm.flip(1).heads() == 1;
        engine
            .damage(10)
            .then_if(heads, |e| e.paralyze())
    }
    pub fn withdraw(engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameEngine {
        let heads = dm.flip(1).heads() == 1;
        engine
            .then_if(heads, |e| e.with_effect(Effect {
                name: "NO_DAMAGE_DURING_OPPONENTS_NEXT_TURN".into(),
                source: EffectSource::Attack(e.player(), e.attacking().id),
                target: EffectTarget::InPlay(e.player(), e.attacking().id),
                consequence: EffectConsequence::BlockDamage,
                expires: EffectExpiration::EndOfTurn(e.opponent(), 0),
                system: false,
            })
        )
    }
}

#[derive(Default)]
pub struct Starmie {}
impl CardArchetype for Starmie {
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
            .register("Recover", &[Type::Water, Type::Water], Self::recover)
            .register("Star Freeze", &[Type::Water, Type::Colorless, Type::Colorless], Self::star_freeze)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Starmie {
    pub fn recover(_engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        unimplemented!();
    }
    pub fn star_freeze(engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameEngine {
        let heads = dm.flip(1).heads() == 1;
        engine
            .damage(20)
            .then_if(heads, |e| e.paralyze())
    }
}

#[derive(Default)]
pub struct Staryu {}
impl CardArchetype for Staryu {
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
            .register("Slap", &[Type::Water], Self::slap)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Staryu {
    pub fn slap(engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine
            .damage(20)
    }
}

#[derive(Default)]
pub struct Tangela {}
impl CardArchetype for Tangela {
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
            .register("Bind", &[Type::Grass, Type::Colorless], Self::bind)
            .register("Poisonpowder", &[Type::Grass, Type::Grass, Type::Grass], Self::poisonpowder)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Tangela {
    pub fn bind(engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameEngine {
        let heads = dm.flip(1).heads() == 1;
        engine
            .damage(20)
            .then_if(heads, |e| e.paralyze())
    }
    pub fn poisonpowder(engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine
            .damage(20)
            .poison()
    }
}

#[derive(Default)]
pub struct Voltorb {}
impl CardArchetype for Voltorb {
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
            .register("Tackle", &[Type::Colorless], Self::tackle)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Voltorb {
    pub fn tackle(engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine
            .damage(10)
    }
}

#[derive(Default)]
pub struct Vulpix {}
impl CardArchetype for Vulpix {
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
            .register("Confuse Ray", &[Type::Fire, Type::Fire], Self::confuse_ray)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Vulpix {
    pub fn confuse_ray(engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameEngine {
        let heads = dm.flip(1).heads() == 1;
        engine
            .damage(10)
            .then_if(heads, |e| e.confuse())
    }
}

#[derive(Default)]
pub struct Weedle {}
impl CardArchetype for Weedle {
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
            .register("Poison Sting", &[Type::Grass], Self::poison_sting)
            .into()
    }

    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Weedle {
    pub fn poison_sting(engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameEngine {
        let heads = dm.flip(1).heads() == 1;
        engine
            .damage(10)
            .then_if(heads, |e| e.poison())
    }
}
