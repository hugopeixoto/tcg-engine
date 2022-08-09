use crate::state::*;
use crate::engine::*;
use crate::*;

#[derive(Default)]
pub struct DoubleColorlessEnergy {}
impl CardArchetype for DoubleColorlessEnergy {
    card_name!("Double Colorless Energy");
    not_a_pokemon!();

    fn card_actions(&self, player: Player, card: &Card, engine: &GameEngine) -> Vec<Action> {
        if !engine.attachment_from_hand_targets(player, card).is_empty() {
            vec![Action::AttachFromHand(player, card.clone())]
        } else {
            vec![]
        }
    }
    fn execute(&self, player: Player, card: &Card, engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameEngine {
        let targets = engine.attachment_from_hand_targets(player, card);
        let target = dm.pick_in_play(player, 1, &targets)[0];

        engine
            .attach_from_hand(player, card, target)
            .with_effect(Effect {
                source: EffectSource::Energy(player, card.clone()),
                target: EffectTarget::Player(player),
                expires: EffectExpiration::EndOfTurn(player, 0),
                consequence: EffectConsequence::BlockAttachmentFromHand,
                name: "ENERGY_ATTACH_FOR_TURN".into(),
                system: true,
            })
    }
    fn attacks(&self, _player: Player, _in_play: &InPlayCard, _engine: &GameEngine) -> Vec<Action> {
        vec![]
    }
    fn provides(&self) -> Vec<Type> {
        vec![Type::Colorless, Type::Colorless]
    }
}

pub struct BasicEnergy {
    name: String,
    energy_type: Type,
}
impl BasicEnergy {
    pub fn create(name: &str, energy_type: Type) -> Box<dyn CardArchetype> {
        Box::new(BasicEnergy { name: name.into(), energy_type })
    }
}
impl CardArchetype for BasicEnergy {
    not_a_pokemon!();
    fn name(&self) -> String {
        self.name.clone()
    }

    fn card_actions(&self, player: Player, card: &Card, engine: &GameEngine) -> Vec<Action> {
        if !engine.attachment_from_hand_targets(player, card).is_empty() {
            vec![Action::AttachFromHand(player, card.clone())]
        } else {
            vec![]
        }
    }

    fn execute(&self, player: Player, card: &Card, engine: &GameEngine, dm: &mut dyn DecisionMaker) -> GameEngine {
        let targets = engine.attachment_from_hand_targets(player, card);
        let target = dm.pick_in_play(player, 1, &targets)[0];

        engine
            .attach_from_hand(player, card, target)
            .with_effect(Effect {
                source: EffectSource::Energy(player, card.clone()),
                target: EffectTarget::Player(player),
                expires: EffectExpiration::EndOfTurn(player, 0),
                consequence: EffectConsequence::BlockAttachmentFromHand,
                name: "ENERGY_ATTACH_FOR_TURN".into(),
                system: true,
            })
    }

    fn attacks(&self, _player: Player, _in_play: &InPlayCard, _engine: &GameEngine) -> Vec<Action> {
        vec![]
    }

    fn provides(&self) -> Vec<Type> {
        vec![self.energy_type.clone()]
    }
}
