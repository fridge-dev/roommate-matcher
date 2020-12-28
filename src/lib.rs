use std::error::Error;
pub use crate::logic::Assignment;
pub use crate::logic::MatchOutcome;

pub(crate) mod input;
pub(crate) mod logic;
pub(crate) mod model;

pub fn match_roommates_from_csv_lines(lines: Vec<String>) -> Result<MatchOutcome, Box<dyn Error>> {
    let people = input::parser::parse(lines)?;

    Ok(logic::match_roommates(people)?)
}
