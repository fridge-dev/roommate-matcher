use crate::model::PersonData;
use crate::input::InputError;
use std::collections::HashMap;

/// validate performs some business logic validation for uniqueness, etc.
pub fn validate(people: &HashMap<String, PersonData>) -> Result<(), InputError> {
    // TODO:1 impl validation
    if people.is_empty() {
        panic!("unimplemented")
    }
    Ok(())
}