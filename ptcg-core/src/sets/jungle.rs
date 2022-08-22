use crate::engine::*;
use crate::carddb::Pokemon;

mod pokemon;

pub use pokemon::*;

macro_rules! mk_pokemon {
    ($vec: expr, $($name: ident),+) => {
        $({
                let boxed = Pokemon::create::<$name>();
                $vec.push((boxed.identifier(), boxed));
        })+
    }
}

pub fn build() -> Vec<(String, Box<dyn CardArchetype>)> {
    let mut entries = vec![];

    mk_pokemon!(
        entries,
        Kangaskhan5, Kangaskhan21
    );

    entries
}
