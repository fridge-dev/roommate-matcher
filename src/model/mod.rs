use std::collections::HashMap;
use std::vec;
use std::slice::Iter;

/// PersonData represents the input data corresponding with a single person.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PersonData {
    person_name: String,
    // Keep this modeling private in case I want to change it.
    // Note: may be empty
    preferences: Vec<String>,
}

impl PersonData {
    #[allow(dead_code)]
    pub fn with_no_preferences(person_name: String) -> Self {
        PersonData {
            person_name,
            preferences: Vec::new(),
        }
    }

    #[allow(dead_code)]
    pub fn with_preferences(person_name: String, preferences: Vec<String>) -> Self {
        PersonData {
            person_name,
            preferences: preferences,
        }
    }

    pub fn new(person_name: String, preferences: Vec<String>) -> Self {
        PersonData {
            person_name,
            preferences,
        }
    }

    pub fn person_name(&self) -> &str {
        &self.person_name
    }

    pub fn choice(&self, index: usize) -> Option<&String> {
        self.preferences.get(index)
    }

    pub fn choices(&self) -> Iter<'_, String> {
        self.preferences.iter()
    }
}

/// UnmatchedPeople is meant to be the primary data model of the application logic. It provides
/// a convenient data access API for the `logic` mod to use.
pub struct UnmatchedPeople {
    person_by_name: HashMap<String, PersonData>,
}

impl UnmatchedPeople {
    pub fn new(person_by_name: HashMap<String, PersonData>) -> Self {
        UnmatchedPeople {
            person_by_name
        }
    }

    pub fn count(&self) -> usize {
        self.person_by_name.len()
    }

    pub fn get(&self, person_name: &str) -> Option<&PersonData> {
        self.person_by_name.get(person_name)
    }

    pub fn remove(&mut self, person_name: &str) {
        self.person_by_name.remove(person_name);
    }

    pub fn iterator(&self) -> UnmatchedPeopleIterator {
        // I don't understand how to write an iterator.
        let persons: Vec<PersonData> = self.person_by_name
            .values()
            .map(|v| v.to_owned())
            .collect();

        UnmatchedPeopleIterator {
            delegate: persons.into_iter(),
        }
    }

    pub fn drain_all_names(&mut self) -> Vec<String> {
        self.person_by_name.drain()
            .map(|(k, _)| k.into())
            .collect()
    }
}

// I need a custom iterator type because I want to remove from the underlying collection (the hashmap)
// while I'm iterating through it. I also don't want to unnecessarily expose the key type in the
// iterator, as this leaks that I'm using a HashMap as my data model.
pub struct UnmatchedPeopleIterator {
    delegate: vec::IntoIter<PersonData>,
}

impl Iterator for UnmatchedPeopleIterator {
    type Item = PersonData;

    fn next(&mut self) -> Option<Self::Item> {
        self.delegate.next()
    }
}