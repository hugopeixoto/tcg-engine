use crate::state::*;
use crate::engine::*;
use crate::*;

#[derive(Default)]
pub struct DoubleColorlessEnergy {}
impl CardArchetype for DoubleColorlessEnergy {
    identifier!("Double Colorless Energy (BS 96)");
    card_name!("Double Colorless Energy");
    not_a_pokemon!();

    fn attachable_as_energy_for_turn(&self, _card: &Card, _engine: &GameEngine) -> bool {
        true
    }

    fn provides(&self) -> Vec<Type> {
        vec![Type::Colorless, Type::Colorless]
    }
}

pub struct BasicEnergy {
    identifier: String,
    name: String,
    energy_type: Type,
}
impl BasicEnergy {
    pub fn create(identifier: &str, name: &str, energy_type: Type) -> Box<dyn CardArchetype> {
        Box::new(BasicEnergy {
            identifier: identifier.into(),
            name: name.into(),
            energy_type,
        })
    }
}
impl CardArchetype for BasicEnergy {
    not_a_pokemon!();
    fn identifier(&self) -> String {
        self.identifier.clone()
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn attachable_as_energy_for_turn(&self, _card: &Card, _engine: &GameEngine) -> bool {
        true
    }

    fn provides(&self) -> Vec<Type> {
        vec![self.energy_type.clone()]
    }
}
