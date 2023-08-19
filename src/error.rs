use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParserError {
    #[error("Syntax error: no name found for function (this should not happen)")]
    NoFunctionName,
    #[error("Syntax error: no equals sing found '='")]
    NoEquals,
    #[error("Syntax error: too many equals signs")]
    EqualsCount,
    #[error("Syntax error: invalid token '{0}'")]
    InvalidToken(String),
    #[error("Syntax error: invalid operator '{0}'")]
    InvalidOperator(String),
    #[error("Syntax error: unknown constant '{0}'")]
    UnknownConstant(String),
}

#[derive(Debug, Error)]
pub enum EvaluatorError {
    #[error("Syntax error: can't find function with the name '{0}'")]
    UnknownFunction(String),
    #[error("Error while parsing: {0}")]
    ParseFailure(ParserError),
    #[error("Equality found in evaluator")]
    EqualityInEval,
}
