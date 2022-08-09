use crate::engine::*;
use crate::carddb::Pokemon;

mod pokemon;

pub use pokemon::*;

pub fn find(archetype: &str) -> Option<Box<dyn CardArchetype>> {
    Some(match archetype {
        "Articuno (FO 17)" => Pokemon::create::<Articuno>(),
        "Psyduck (FO 53)"  => Pokemon::create::<Psyduck>(),
        _                  => { return None; },
    })
}
