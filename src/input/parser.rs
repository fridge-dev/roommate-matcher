use crate::model::PersonData;
use crate::input::InputError;

/// `parse` expects data in the form of a CSV file with the schema:
///
/// `my name,first preference,second preference,third preference`
///
/// where all preferences are optional.
pub fn parse(lines: Vec<String>) -> Result<Vec<PersonData>, InputError> {
    let mut people_data = Vec::with_capacity(lines.len());

    for line in lines {
        people_data.push(try_parse_line(line)?);
    }

    Ok(people_data)
}

// TODO:2 Fail if missing first choice but has subsequent choices.
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
    let preferences: Vec<_> = values_iter.filter(|v| !v.is_empty()).collect();

    Ok(PersonData::new(name, preferences))
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
        ].into_iter()
            .map(|l| l.into())
            .collect();

        let people = parse(lines).unwrap();
        assert_eq!(people, vec![
            PersonData::new("1a".into(), vec!["1b".into(), "1c".into(), "1d".into()]),
            PersonData::new("2a".into(), vec!["2b".into(), "2c".into(), "2d".into()]),
            PersonData::new("3a".into(), vec![]),
            PersonData::new("4a".into(), vec![]),
            PersonData::new("5a".into(), vec![]),
            PersonData::new("6a".into(), vec![]),
            PersonData::new("7a".into(), vec![]),
            PersonData::new("8a".into(), vec!["8b".into()]),
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
            parse(invalid_file).unwrap_err();
        }
    }
}
