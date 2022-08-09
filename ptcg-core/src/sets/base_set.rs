use crate::engine::*;
use crate::carddb::{Pokemon, Trainer};

mod pokemon;
mod energies;
mod trainers;

pub use pokemon::*;
pub use energies::*;
pub use trainers::*;

macro_rules! mk_pokemon {
    ($vec: expr, $set: expr, $($name: ident),+) => {
        $({
                let boxed = Pokemon::create::<$name>();
                $vec.push((format!("{} ({} {})", boxed.name(), $set, $vec.len() + 1), boxed))
        })+
    }
}

macro_rules! mk_trainer {
    ($vec: expr, $set: expr, $($name: ident),+) => {
        $({
                let boxed = Trainer::create::<$name>();
                $vec.push((format!("{} ({} {})", boxed.name(), $set, $vec.len() + 1), boxed))
        })+
    }
}


pub fn build() -> Vec<(String, Box<dyn CardArchetype>)> {
    let mut entries = vec![];

    mk_pokemon!(
        entries, "BS",
        Alakazam, Blastoise, Chansey, Charizard, Clefairy,
        Gyarados, Hitmonchan, Machamp, Magneton, Mewtwo,
        Mewtwo, Ninetales, Poliwrath, Raichu, Venusaur, Zapdos,
        Beedrill, Dragonair, Dugtrio, Electabuzz, Electrode,
        Pidgeotto, Arcanine, Charmeleon, Dewgong, Dratini,
        FarfetchD, Growlithe, Haunter, Ivysaur, Jynx, Kadabra,
        Kakuna, Machoke, Magikarp, Magmar, Nidorino, Poliwhirl,
        Porygon, Raticate, Seel, Wartortle, Abra, Bulbasaur,
        Caterpie, Charmander, Diglett, Doduo, Drowzee, Gastly,
        Koffing, Machop, Magnemite, Metapod, NidoranM, Onix,
        Pidgey, Pikachu, Poliwag, Ponyta, Rattata, Sandshrew,
        Squirtle, Starmie, Staryu, Tangela, Voltorb, Vulpix,
        Weedle
    );

    mk_trainer!(
        entries, "BS",
        ClefairyDoll, ComputerSearch, DevolutionSpray,
        ImpostorProfessorOak, ItemFinder, Lass, PokemonBreeder,
        PokemonTrader, ScoopUp, SuperEnergyRemoval, Defender,
        EnergyRetrieval, FullHeal, Maintenance, PlusPower,
        PokemonCenter, PokemonFlute, Pokedex, ProfessorOak,
        Revive, SuperPotion, Bill, EnergyRemoval, GustOfWind,
        Potion, Switch
    );

    entries.push(("Double Colorless Energy (BS 96)".into(), Box::new(DoubleColorlessEnergy::default())));
    entries.push(("Fighting Energy (BS 97)"  .into(), BasicEnergy::create("Fighting Energy", Type::Fighting)));
    entries.push(("Fire Energy (BS 98)"      .into(), BasicEnergy::create("Fire Energy", Type::Fire)));
    entries.push(("Grass Energy (BS 99)"     .into(), BasicEnergy::create("Grass Energy", Type::Grass)));
    entries.push(("Lightning Energy (BS 100)".into(), BasicEnergy::create("Lightning Energy", Type::Lightning)));
    entries.push(("Psychic Energy (BS 101)"  .into(), BasicEnergy::create("Psychic Energy", Type::Psychic)));
    entries.push(("Water Energy (BS 102)"    .into(), BasicEnergy::create("Water Energy", Type::Water)));

    entries
}
