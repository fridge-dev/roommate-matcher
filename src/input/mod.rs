pub mod parser;

#[derive(thiserror::Error, Debug)]
pub enum InputError {
    #[error("Lines in input file is empty")]
    NoData,

    #[error("Invalid line: {details:?}. Line: '{line:?}'")]
    BadLine {
        details: String,
        line: String,
    },

    #[error("Duplicate person with name '{0}'")]
    DuplicatePerson(String),

    #[error("Person '{person_name}' chose '{invalid_choice}' who does not exist")]
    ChoseMissingPerson {
        person_name: String,
        invalid_choice: String,
    },

    #[error("Person '{0}' chose themself as a preference lmao")]
    SelfChoice(String),
}

