pub mod parser;
pub mod validator;

#[derive(thiserror::Error, Debug)]
pub enum InputError {
    #[error("Invalid line: {details:?}. Line: `{line:?}`")]
    BadLine {
        details: String,
        line: String,
    },
}
