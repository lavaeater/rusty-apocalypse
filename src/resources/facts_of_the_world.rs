use bevy::utils::HashMap;

pub enum Fact {
    StringFact(String),
    IntFact(i32),
    FloatFact(f32),
    BoolFact(bool),
    ListFact(Vec<Fact>),
}

pub struct FactsOfTheWorld {
    facts: HashMap<String, Fact>,
}

impl FactsOfTheWorld {
    pub fn get_fact(key: &str) -> Option<&Fact> {
        None
    }
}