use thiserror::Error;

#[derive(Debug, Error)]
pub enum CaculatorError {
    #[error("invalid operand")]
    MissingOperand,
    #[error("unsupported operator: {0}")]
    UnsupportedOperator(char),
    #[error("invalid expression: {0}")]
    InvalidExpression(String),
    #[error("divide by zero")]
    DivideByZero,
}
