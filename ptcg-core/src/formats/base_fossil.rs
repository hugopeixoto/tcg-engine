use std::ops::Deref;
use crate::state::Card;
use crate::engine::{CardArchetype, Format, AttackingEffectsWhen};
use crate::sets::{base, fossil};
use crate::effect::CustomEffect;
use crate::custom_effects::*;

#[derive(Clone)]
pub struct BaseFossil {
    archetypes: std::rc::Rc<Vec<(String, Box<dyn CardArchetype>)>>,
    custom_effects: std::rc::Rc<Vec<(String, Box<dyn CustomEffect>)>>,
}

impl BaseFossil {
    pub fn new() -> Self {
        let mut cards = vec![];

        cards.extend(base::build());
        cards.extend(fossil::build());

        Self {
            archetypes: std::rc::Rc::new(cards),
            custom_effects: std::rc::Rc::new(vec![
                (PreventDamageDuringOpponentsTurn::identifier(), Box::new(PreventDamageDuringOpponentsTurn{})),
                (BlockAttachmentFromHand::identifier(), Box::new(BlockAttachmentFromHand{})),
                (RevengeKnockOut::identifier(), Box::new(RevengeKnockOut{})),
            ]),
        }
    }
}

impl Format for BaseFossil {
    fn behavior_from_id(&self, id: &String) -> &dyn CardArchetype {
        for (identifier, archetype) in self.archetypes.iter() {
            if identifier == id {
                return archetype.deref();
            }
        }

        panic!("Couldn't find card {}", id);
    }

    fn behavior(&self, card: &Card) -> &dyn CardArchetype {
        self.behavior_from_id(&card.archetype)
    }

    fn effect(&self, id: &String) -> &dyn CustomEffect {
        for (identifier, effect) in self.custom_effects.iter() {
            if identifier == id {
                return effect.deref();
            }
        }

        panic!("Couldn't find effect {}", id);
    }

    fn attacking_effects(&self) -> AttackingEffectsWhen {
        AttackingEffectsWhen::AfterWR
    }

    fn basic_for_stage2(&self, card: &Card) -> String {
        let stage1name = self.behavior(card).evolves_from().unwrap();

        for (_, archetype) in self.archetypes.iter() {
            if archetype.name() == stage1name {
                return archetype.evolves_from().unwrap();
            }
        }

        panic!("Couldn't find basic that matches card {}", card.archetype);
    }

    fn boxed_clone(&self) -> Box<dyn Format> {
        Box::new(self.clone())
    }
}
