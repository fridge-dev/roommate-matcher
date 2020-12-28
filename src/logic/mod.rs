use crate::model::PersonData;
use std::collections::HashMap;

// TODO:3 include debug info
pub struct Assignment(String, String);

pub struct MatchOutcome {
    pub matches: Vec<Assignment>,
    pub unmatched: Vec<String>,
}

#[derive(thiserror::Error, Debug)]
pub enum MatchError {
    #[error("Placeholder")]
    Todo,
}

// Note on efficiency: This is horrible. There's probably a smart way to do this. But let's get
// a working version first.
//
// Assumption: any `preference` is present in person_name. TODO:1 impl validator.
pub fn match_roommates(people: Vec<PersonData>) -> Result<MatchOutcome, MatchError> {
    let mut unmatched_person_by_name = build_index(&people);
    let mut matches = Vec::with_capacity(unmatched_person_by_name.len() / 2);

    // Rule 1: Find all matches where people chose each other as their number 1 choice.
    for (_, person_data) in unmatched_person_by_name.iter() {
        if let Some(first_choice_name) = person_data.get_choice(0) {
            if let Some(first_choice_person) = unmatched_person_by_name.get(first_choice_name) {
                if let Some(first_choices_first_choice_name) = first_choice_person.get_choice(0) {
                    if person_data.person_name() == first_choices_first_choice_name {
                        // It's a match!
                        matches.push(Assignment(person_data.person_name().into(), first_choice_name.into()));
                        unmatched_person_by_name.remove(person_data.person_name());
                        unmatched_person_by_name.remove(first_choice_name);
                    }
                }
            }
        }
    }

    let outcome = MatchOutcome {
        matches,
        unmatched: unmatched_person_by_name.keys().map(|s| s.into()).collect(),
    };

    Ok(outcome)
}

// TODO:2 change to move instead of borrow?
// TODO:2 Or, don't expose raw data structure to algorithm
fn build_index(people: &Vec<PersonData>) -> HashMap<String, PersonData> {
    let mut map = HashMap::with_capacity(people.len());

    for person in people {
        // Assumption: person appears once. TODO:1 impl validator
        map.insert(person.person_name().into(), person.clone());
    }

    map
}
