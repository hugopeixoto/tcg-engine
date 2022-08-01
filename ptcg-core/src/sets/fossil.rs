use crate::*;

struct Attacks<'a> {
    player: Player,
    in_play: &'a InPlayCard,
    engine: &'a GameEngine,
    attacks: Vec<Action>,
}

impl<'a> Attacks<'a> {
    pub fn new(player: Player, in_play: &'a InPlayCard, engine: &'a GameEngine) -> Self {
        Attacks {
            player,
            in_play,
            engine: engine,
            attacks: vec![],
        }
    }

    pub fn register(mut self, name: &str, energy_requirements: &[Type], f: fn(&GameEngine, &mut dyn DecisionMaker) -> GameEngine) -> Attacks<'a> {
        if self.engine.is_attack_energy_cost_met(self.in_play, energy_requirements) {
            self.attacks.push(Action::Attack(self.player, self.in_play.clone(), name.into(), Box::new(RFA::new(f))));
        }

        self
    }
}

impl<'a> From<Attacks<'a>> for Vec<Action> {
    fn from(attacks: Attacks<'a>) -> Self {
        attacks.attacks
    }
}

#[derive(Default)]
pub struct Articuno {}
impl CardArchetype for Articuno {
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
            .register("Freeze Dry", &[Type::Water, Type::Water, Type::Water], Self::freeze_dry)
            .register("Blizzard", &[Type::Water, Type::Water, Type::Water, Type::Water], Self::blizzard)
            .into()
    }
    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Articuno {
    pub fn freeze_dry(engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameEngine {
        let paralyzed = dm.flip(1).heads() == 1;

        engine.damage(30).then_if(paralyzed, GameEngine::paralyze)
    }

    pub fn blizzard(engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameEngine {
        let whose_bench = if dm.flip(1).heads() == 1 {
            engine.opponent()
        } else {
            engine.player()
        };

        engine
            .damage(50)
            .then(|e| e.target_all(e.bench(whose_bench), |e2| e2.damage(10)))
    }
}

#[derive(Default)]
pub struct Psyduck {}
impl CardArchetype for Psyduck {
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
            .register("Headache", &[Type::Psychic], Self::headache)
            .register("Fury Swipes", &[Type::Water], Self::fury_swipes)
            .into()
    }
    fn provides(&self) -> Vec<Type> {
        vec![]
    }
}
impl Psyduck {
    pub fn headache(engine: &GameEngine, _dm: &mut dyn DecisionMaker) -> GameEngine {
        engine.with_effect(Effect {
            name: "PSYDUCK_FO_HEADACHE_NO_TRAINERS".into(),
            source: EffectSource::Attack(engine.player(), engine.attacking().id),
            target: EffectTarget::Player(engine.opponent()),
            consequence: EffectConsequence::BlockTrainerFromHand,
            expires: EffectExpiration::EndOfTurn(engine.opponent(), 0),
        })
    }
    pub fn fury_swipes(engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameEngine {
        let damage = 10 * dm.flip(3).heads();

        engine.damage(damage)
    }
}
