use thiserror::Error;

#[derive(Debug, Error)]
pub enum CaculatorError {
    #[error("operator: {0} need {1} operands")]
    MissingOperand(char, u8),
    #[error("unsupported operator: {0}")]
    UnsupportedOperator(char),
    #[error("invalid expression: {0}")]
    InvalidExpression(String),
    #[error("divide by zero")]
    DivideByZero,
    #[error("unbalanced parenthesis")]
    UnBalancedParenthesis,
}
