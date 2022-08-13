#[macro_export]
macro_rules! identifier {
    ($id:literal) => {
        fn identifier(&self) -> String { $id.into() }
    }
}

#[macro_export]
macro_rules! card_name {
    ($name:literal) => {
        fn name(&self) -> String { $name.into() }
    }
}

#[macro_export]
macro_rules! retreat {
    ($cost:literal) => {
        fn retreat(&self) -> usize { $cost }
    }
}

#[macro_export]
macro_rules! basic {
    () => {
        fn stage(&self) -> Option<Stage> { Some(Stage::Basic) }
        fn evolves_from(&self) -> Option<String> { None }
    }
}

#[macro_export]
macro_rules! stage1 {
    ($from:literal) => {
        fn stage(&self) -> Option<Stage> { Some(Stage::Stage1) }
        fn evolves_from(&self) -> Option<String> { Some($from.into()) }
    }
}

#[macro_export]
macro_rules! stage2 {
    ($from:literal) => {
        fn stage(&self) -> Option<Stage> { Some(Stage::Stage2) }
        fn evolves_from(&self) -> Option<String> { Some($from.into()) }
    }
}

#[macro_export]
macro_rules! color {
    ($type:ident) => {
        fn pokemon_type(&self) -> Vec<Type> { vec![Type::$type] }
    };
    ($type0:ident, $type1:ident) => {
        fn pokemon_type(&self) -> Vec<Type> { vec![Type::$type0, Type::$type1] }
    };
}

#[macro_export]
macro_rules! hp {
    ($hp:literal) => {
        fn hp(&self, _card: &Card, _engine: &GameEngine) -> Option<usize> { Some($hp) }
    }
}

#[macro_export]
macro_rules! weak_to {
    ($type:ident) => {
        fn weakness(&self) -> Weakness { (2, vec![Type::$type]) }
    }
}

#[macro_export]
macro_rules! no_weakness {
    () => {
        fn weakness(&self) -> Weakness { (0, vec![]) }
    }
}

#[macro_export]
macro_rules! resists {
    ($type:ident, $amount:literal) => {
        fn resistance(&self) -> Resistance { ($amount, vec![Type::$type]) }
    }
}

#[macro_export]
macro_rules! no_resistance {
    () => {
        fn resistance(&self) -> Resistance { (0, vec![]) }
    }
}

#[macro_export]
macro_rules! not_a_pokemon {
    () => {
        fn stage(&self) -> Option<Stage> {
            None
        }
        fn evolves_from(&self) -> Option<String> {
            None
        }
        fn hp(&self, _card: &Card, _engine: &GameEngine) -> Option<usize> {
            None
        }
        fn weakness(&self) -> Weakness {
            (0, vec![])
        }
        fn resistance(&self) -> Resistance {
            (0, vec![])
        }
        fn pokemon_type(&self) -> Vec<Type> {
            vec![]
        }
        fn retreat(&self) -> usize {
            0
        }
    }
}
