use crate::engine::*;
use crate::carddb::{Pokemon, Trainer};

mod pokemon;
mod energies;
mod trainers;

pub use pokemon::*;
pub use energies::*;
pub use trainers::*;

macro_rules! mk_pokemon {
    ($vec: expr, $($name: ident),+) => {
        $({
                let boxed = Pokemon::create::<$name>();
                $vec.push((boxed.identifier(), boxed));
        })+
    }
}

macro_rules! mk_trainer {
    ($vec: expr, $($name: ident),+) => {
        $({
                let boxed = Trainer::create::<$name>();
                $vec.push((boxed.identifier(), boxed));
        })+
    }
}

pub fn build() -> Vec<(String, Box<dyn CardArchetype>)> {
    let mut entries = vec![];

    mk_pokemon!(
        entries,
        Alakazam1, Blastoise2, Chansey3, Charizard4, Clefairy5,
        Gyarados6, Hitmonchan7, Machamp8, Magneton9, Mewtwo10,
        Ninetales12, Poliwrath13, Raichu14, Venusaur15, Zapdos16,
        Beedrill17, Dragonair18, Dugtrio19, Electabuzz20, Electrode21,
        Pidgeotto22, Arcanine23, Charmeleon24, Dewgong25, Dratini26,
        FarfetchD27, Growlithe28, Haunter29, Ivysaur30, Jynx31, Kadabra32,
        Kakuna33, Machoke34, Magikarp35, Magmar36, Nidorino37, Poliwhirl38,
        Porygon39, Raticate40, Seel41, Wartortle42, Abra43, Bulbasaur44,
        Caterpie45, Charmander46, Diglett47, Doduo48, Drowzee49, Gastly50,
        Koffing51, Machop52, Magnemite53, Metapod54, NidoranM55, Onix56,
        Pidgey57, Pikachu58, Poliwag59, Ponyta60, Rattata61, Sandshrew62,
        Squirtle63, Starmie64, Staryu65, Tangela66, Voltorb67, Vulpix68,
        Weedle69
    );

    mk_trainer!(
        entries,
        ClefairyDoll70, ComputerSearch71, DevolutionSpray72,
        ImpostorProfessorOak73, ItemFinder74, Lass75, PokemonBreeder76,
        PokemonTrader77, ScoopUp78, SuperEnergyRemoval79, Defender80,
        EnergyRetrieval81, FullHeal82, Maintenance83, PlusPower84,
        PokemonCenter85, PokemonFlute86, Pokedex87, ProfessorOak88,
        Revive89, SuperPotion90, Bill90, EnergyRemoval91, GustOfWind92,
        Potion93, Switch94
    );

    entries.push(("Double Colorless Energy (BS 96)".into(), Box::new(DoubleColorlessEnergy::default())));
    entries.push(("Fighting Energy (BS 97)"  .into(), BasicEnergy::create("Fighting Energy (BS 97)", "Fighting Energy", Type::Fighting)));
    entries.push(("Fire Energy (BS 98)"      .into(), BasicEnergy::create("Fire Energy (BS 98)", "Fire Energy", Type::Fire)));
    entries.push(("Grass Energy (BS 99)"     .into(), BasicEnergy::create("Grass Energy (BS 99)", "Grass Energy", Type::Grass)));
    entries.push(("Lightning Energy (BS 100)".into(), BasicEnergy::create("Lightning Energy (BS 100)", "Lightning Energy", Type::Lightning)));
    entries.push(("Psychic Energy (BS 101)"  .into(), BasicEnergy::create("Psychic Energy (BS 101)", "Psychic Energy", Type::Psychic)));
    entries.push(("Water Energy (BS 102)"    .into(), BasicEnergy::create("Water Energy (BS 102)", "Water Energy", Type::Water)));

    entries
}
