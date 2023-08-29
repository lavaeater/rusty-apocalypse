use bevy::prelude::{Resource};
use bevy::utils::HashMap;

#[derive(Debug, PartialEq)]
pub enum Fact {
    StringFact(String),
    IntFact(i32),
    FloatFact(f32),
    BoolFact(bool),
    ListFact(Vec<Fact>),
}

trait FactTrait {
    fn value<T>(&self) -> Option<T>;
}

impl FactTrait for String {
    fn value<String>(&self) -> Option<&str> {
        Some(self.clone())
    }
}

#[derive(Debug, Resource)]
pub struct FactsOfTheWorld {
    pub facts: HashMap<String, Fact>,
}

impl FactsOfTheWorld {
    pub fn is_true(&self, key: &str) -> bool {
        match self.facts.get(key) {
            Some(fact) => {
                match fact {
                    Fact::BoolFact(b) => *b,
                    _ => false,
                }
            },
            None => false,
        }
    }

    pub fn is_equal(&self, key: &str, fact: Fact) -> bool {
        match self.facts.get(key) {
            Some(f) => {
                f == &fact
            },
            None => false,
        }
    }

    pub fn larger_than(&self, key: &str, fact: Fact) -> bool {
        match self.facts.get(key) {
            Some(stored_fact) => {
                match fact {
                    Fact::IntFact(i) => {
                        match stored_fact {
                            Fact::IntFact(j) => i > *j,
                            Fact::FloatFact(j) => i as f32 > *j,
                            _ => false,
                        }
                    },
                    Fact::FloatFact(f) => {
                        match stored_fact {
                            Fact::IntFact(j) => f as i32 > *j,
                            Fact::FloatFact(j) => f > *j,
                            _ => false,
                        }
                    },
                    _ => false,
                }
            }
            None => false,
        }
    }

    pub fn contains(&self, key: &str, fact: Fact) -> bool {
        match self.facts.get(key) {
            Some(stored_fact) => {
                match stored_fact {
                    Fact::ListFact(list) => {
                        list.contains(&fact)
                    },
                    _ => false,
                }
            }
            None => false,
        }
    }

    pub fn smaller_than(&self, key: &str, fact: Fact) -> bool {
        match self.facts.get(key) {
            Some(stored_fact) => {
                match fact {
                    Fact::IntFact(i) => {
                        match stored_fact {
                            Fact::IntFact(j) => *j > i,
                            Fact::FloatFact(j) => *j > i as f32,
                            _ => false,
                        }
                    },
                    Fact::FloatFact(f) => {
                        match stored_fact {
                            Fact::IntFact(j) => *j > f as i32,
                            Fact::FloatFact(j) => *j >f,
                            _ => false,
                        }
                    },
                    _ => false,
                }
            }
            None => false,
        }
    }

    pub fn get_fact(&self, key: &str) -> Option<&Fact> {
        self.facts.get(key)
    }

    pub fn store_fact(&mut self, key: &str, fact: Fact) {
        self.facts.insert(key.to_string(), fact);
    }

    pub fn update_fact(&mut self, key: &str, fact: Fact) {
        self.facts.insert(key.to_string(), fact);
    }
}