use crate::engine::*;
use crate::carddb::Pokemon;

mod pokemon;

pub use pokemon::*;

pub fn build() -> Vec<(String, Box<dyn CardArchetype>)> {
    vec![
        ("Articuno (FO 17)".into(), Pokemon::create::<Articuno17>()),
        ("Psyduck (FO 53)".into(), Pokemon::create::<Psyduck53>()),
    ]
}
