use std::ops::Deref;
use crate::state::Card;
use crate::engine::{CardArchetype, Format, AttackingEffectsWhen};
use crate::sets::{base, fossil};

#[derive(Clone)]
pub struct BaseFossil {
    archetypes: std::rc::Rc<Vec<(String, Box<dyn CardArchetype>)>>,
}

impl BaseFossil {
    pub fn new() -> Self {
        let mut cards = vec![];

        cards.extend(base::build());
        cards.extend(fossil::build());

        Self { archetypes: std::rc::Rc::new(cards) }
    }
}

impl Format for BaseFossil {
    fn behavior(&self, card: &Card) -> &dyn CardArchetype {
        for (identifier, archetype) in self.archetypes.iter() {
            if *identifier == card.archetype {
                return archetype.deref();
            }
        }

        panic!("Couldn't find card {}", card.archetype);
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
