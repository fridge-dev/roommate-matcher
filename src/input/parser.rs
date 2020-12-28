use crate::model::{PersonData, UnmatchedPeople};
use crate::input::InputError;
use std::collections::HashMap;
use std::hash::Hash;

/// `parse` expects data in the form of a CSV file with the schema:
///
/// `my name,first preference,second preference,third preference`
///
/// where all preferences are optional.
pub fn parse(lines: Vec<String>) -> Result<UnmatchedPeople, InputError> {
    if lines.is_empty() {
        return Err(InputError::NoData);
    }

    let people_data = parse_lines(lines)?;

    // Rule: Person names (left-most value) are unique.
    let people_by_name = map_with_unique_index(people_data, |person| person.person_name().into())
        .map_err(|duplicate_name| InputError::DuplicatePerson(duplicate_name))?;

    validate_preferences_exist_and_dont_refer_to_self(&people_by_name)?;

    Ok(UnmatchedPeople::new(people_by_name))
}

fn parse_lines(lines: Vec<String>) -> Result<Vec<PersonData>, InputError> {
    let mut people_data = Vec::with_capacity(lines.len());

    for line in lines {
        people_data.push(try_parse_line(line)?);
    }

    Ok(people_data)
}

fn try_parse_line(line: String) -> Result<PersonData, InputError> {
    let mut values_iter = line.split(",")
        .into_iter()
        .map(|v| v.trim().to_owned());

    // First value is person's name, and it is required. We filter empty str separately than the
    // iterator to ensure the first value before a "," is present.
    let raw_line_for_error = line.clone();
    let name = values_iter.next().filter(|v| !v.is_empty()).ok_or_else(|| {
        InputError::BadLine {
            details: "empty line".to_string(),
            line: raw_line_for_error,
        }
    })?;

    // Remaining values are optional. We treat lines like "my name,,," the same as "my name".
    // We also treat "my name,,other person," the same as "my name,other person"
    let preferences: Vec<_> = values_iter.filter(|v| !v.is_empty()).collect();

    Ok(PersonData::new(name, preferences))
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

fn validate_preferences_exist_and_dont_refer_to_self(people: &HashMap<String, PersonData>) -> Result<(), InputError> {
    for (person_name, person_data) in people {
        for choice_name in person_data.choices() {
            // Rule: You cannot choose yourself.
            if choice_name == person_name {
                return Err(InputError::SelfChoice(person_name.into()))
            }

            // Rule: Preferences must exist as a person name.
            if !people.contains_key(choice_name) {
                return Err(InputError::ChoseMissingPerson {
                    person_name: person_name.into(),
                    invalid_choice: choice_name.into(),
                })
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_ok_permutations() {
        let lines: Vec<_> = vec![
            "1a,1b,1c,1d",
            "2a,2b,2c,2d,",
            "3a,,,",
            "4a,",
            "5a",
            " 6a ",
            " 7a, ",
            " 8a, 8b ",
            " 9a,,,9b ",
        ].into_iter()
            .map(|l| l.into())
            .collect();

        let people = parse_lines(lines).unwrap();
        assert_eq!(people, vec![
            PersonData::new("1a".into(), vec!["1b".into(), "1c".into(), "1d".into()]),
            PersonData::new("2a".into(), vec!["2b".into(), "2c".into(), "2d".into()]),
            PersonData::new("3a".into(), vec![]),
            PersonData::new("4a".into(), vec![]),
            PersonData::new("5a".into(), vec![]),
            PersonData::new("6a".into(), vec![]),
            PersonData::new("7a".into(), vec![]),
            PersonData::new("8a".into(), vec!["8b".into()]),
            PersonData::new("9a".into(), vec!["9b".into()]),
        ]);
    }

    #[test]
    fn test_parse_err_permutations() {
        let invalid_files: Vec<Vec<String>> = vec![
            vec!["".into()],
            vec![" ".into()],
            vec![",".into()],
            vec![" , ".into()],
            vec![",name".into()],
        ];

        for invalid_file in invalid_files {
            parse_lines(invalid_file).unwrap_err();
        }
    }
}
