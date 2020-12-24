use crate::model::PersonData;
use crate::input::InputError;

/// validate performs some business logic validation for uniqueness, etc.
pub fn validate(people: &Vec<PersonData>) -> Result<(), InputError> {
    // TODO:1 impl validation
}