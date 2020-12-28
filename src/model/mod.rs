use std::collections::HashMap;
use crate::input::{InputError, validator};
use std::vec;
use std::hash::Hash;

/// PersonData represents the input data corresponding with a single person.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PersonData {
    person_name: String,
    // Keep this modeling private in case I want to change it.
    preferences: Option<Vec<String>>
}

impl PersonData {
    #[allow(dead_code)]
    pub fn with_no_preferences(person_name: String) -> Self {
        PersonData {
            person_name,
            preferences: None,
        }
    }

    #[allow(dead_code)]
    pub fn with_preferences(person_name: String, preferences: Vec<String>) -> Self {
        PersonData {
            person_name,
            preferences: Some(preferences),
        }
    }

    pub fn new(person_name: String, preferences: Vec<String>) -> Self {
        PersonData {
            person_name,
            preferences: Some(preferences).filter(|p| !p.is_empty()),
        }
    }

    pub fn person_name(&self) -> &str {
        &self.person_name
    }

    pub fn get_choice(&self, index: usize) -> Option<&String> {
        match &self.preferences {
            None => None,
            Some(pref) => pref.get(index),
        }
    }
}

/// UnmatchedPeople is meant to be the primary data model of the application logic. It provides
/// a convenient data access API for the `logic` mod to use.
pub struct UnmatchedPeople {
    person_by_name: HashMap<String, PersonData>,
}

impl UnmatchedPeople {
}

impl UnmatchedPeople {
    /// try_create tries to create a valid instance of `UnmatchedPeople`. This bakes some business
    /// logic into the model, but it ensures that an instance with illegal state cannot be
    /// constructed.
    pub fn try_create(people: Vec<PersonData>) -> Result<Self, InputError> {
        // Rule: Person names (left-most value) are unique.
        let people_by_name = map_with_unique_index(people, |person| person.person_name.clone())
            .map_err(|duplicate_name| InputError::DuplicatePerson(duplicate_name))?;

        validator::validate(&people_by_name)?;

        Ok(Self::new(people_by_name))
    }

    fn new(person_by_name: HashMap<String, PersonData>) -> Self {
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

/// Returns a HashMap that is guaranteed to have uniquely indexed all of the values. If duplicate is
/// present, the key for the duplicate is returned as an Err.
fn map_with_unique_index<K, V, F>(values: Vec<V>, key_for_value: F) -> Result<HashMap<K, V>, K>
where
    K: Hash + Eq,
    F: Fn(&V) -> K,
{
    let mut map = HashMap::with_capacity(values.len());

    for v in values {
        if let Some(duplicate) = map.insert(key_for_value(&v), v) {
            return Err(key_for_value(&duplicate));
        }
    }

    Ok(map)
}