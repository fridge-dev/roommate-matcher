use crate::model::UnmatchedPeople;
use std::fmt::{Display, Formatter};
use core::fmt;

// TODO:3 include debug info
#[derive(Debug)]
pub struct Assignment(String, String);

#[derive(Debug)]
pub struct MatchOutcome {
    pub matches: Vec<Assignment>,
    pub unmatched: Vec<String>,
}

impl Display for MatchOutcome {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut lines: Vec<String> = Vec::new();

        lines.push("-- Matches --".into());
        self.matches
            .iter()
            .map(|assignment| format!("{} & {}", assignment.0, assignment.1))
            .for_each(|line| lines.push(line));

        lines.push("".into());
        lines.push("-- Unmatched --".into());
        for p in self.unmatched.iter() {
            lines.push(p.into());
        }

        f.write_str(&lines.join("\n"))
    }
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

    // Match Rule 1: Match people who chose each other as #1 choice.
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

    // TODO:1.5 dynamically update choices to remove matched people from unmatched peoples' choices.
    // TODO:1.5 add condition to check for early termination that all remaining unmatched people have no valid choices remaining.

    let outcome = MatchOutcome {
        matches,
        unmatched: unmatched_people.drain_all_names(),
    };

    Ok(outcome)
}
