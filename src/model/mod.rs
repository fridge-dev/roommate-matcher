/// PersonData represents the input data corresponding with a single person.
#[derive(Debug, Eq, PartialEq)]
pub struct PersonData {
    person_name: String,
    // Keep this modeling private in case I want to change it.
    preferences: Option<Vec<String>>
}

#[allow(dead_code)]
pub struct Assignment(String, String);

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
}
