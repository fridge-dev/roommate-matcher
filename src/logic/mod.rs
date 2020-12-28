use crate::model::UnmatchedPeople;

// TODO:3 include debug info
pub struct Assignment(String, String);

pub struct MatchOutcome {
    pub matches: Vec<Assignment>,
    pub unmatched: Vec<String>,
}

#[derive(thiserror::Error, Debug)]
pub enum MatchError {
    #[allow(dead_code)] // TODO remove and use real err type
    #[error("Placeholder")]
    Todo,
}

// Note on efficiency: This is horrible. There's probably a smart way to do this. But let's get
// a working version first.
pub fn match_roommates(mut unmatched_people: UnmatchedPeople) -> Result<MatchOutcome, MatchError> {
    let mut matches = Vec::with_capacity(unmatched_people.count() / 2);

    // Rule 1: Find all matches where people chose each other as their number 1 choice.
    for current_person in unmatched_people.iterator() {
        if let Some(first_choice_name) = current_person.choice(0) {
            if let Some(first_choice_person) = unmatched_people.get(first_choice_name) {
                if let Some(first_choices_first_choice_name) = first_choice_person.choice(0) {
                    if current_person.person_name() == first_choices_first_choice_name {
                        // It's a match!
                        matches.push(Assignment(current_person.person_name().into(), first_choice_name.into()));
                        unmatched_people.remove(current_person.person_name());
                        unmatched_people.remove(first_choice_name);
                    }
                }
            }
        }
    }

    // TODO:1 Add more rules.

    let outcome = MatchOutcome {
        matches,
        unmatched: unmatched_people.drain_all_names(),
    };

    Ok(outcome)
}
