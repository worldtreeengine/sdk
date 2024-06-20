use std::fmt::{Display, Formatter};
use serde::{Serialize, Serializer};
use crate::{Mark};

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ExpressionAtom {
    LogicalLiteral(bool),
    NumericLiteral(u32),
    Reference(String),
}

impl Serialize for ExpressionAtom {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        match self {
            Self::LogicalLiteral(b) => serializer.serialize_u32(if *b { 1 } else { 0 }),
            Self::NumericLiteral(n) => serializer.serialize_u32(*n),
            Self::Reference(s) => serializer.serialize_str(s),
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ExpressionOperator {
    // Grouping operators
    CloseParen,
    OpenParen,

    // Branch operators
    End,
    Else,
    Then,

    // Phrasal operators
    Either,
    Between,
    Minimum,
    Maximum,
    Of,
    Random,
    When,
    Unless,
    From,
    Comma,

    // Range operator
    To,

    // Conjunction operators
    And,
    Or,

    // Comparison operators
    Is,
    Equal,
    NotEqual,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,

    // Arithmetic operators
    Plus,
    Minus,
    Multiply,
    Divide,

    // Atomic operators
    Not,
    In,
}

impl Display for ExpressionOperator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ExpressionOperator::And => f.write_str("and"),
            ExpressionOperator::Or => f.write_str("or"),
            ExpressionOperator::Not => f.write_str("not"),
            ExpressionOperator::OpenParen => f.write_str("("),
            ExpressionOperator::CloseParen => f.write_str(")"),
            ExpressionOperator::Equal => f.write_str("="),
            ExpressionOperator::NotEqual => f.write_str("!="),
            ExpressionOperator::GreaterThan => f.write_str(">"),
            ExpressionOperator::GreaterThanOrEqual => f.write_str(">="),
            ExpressionOperator::LessThan => f.write_str("<"),
            ExpressionOperator::LessThanOrEqual => f.write_str("<="),
            ExpressionOperator::Plus => f.write_str("+"),
            ExpressionOperator::Minus => f.write_str("-"),
            ExpressionOperator::Multiply => f.write_str("*"),
            ExpressionOperator::Divide => f.write_str("/"),
            ExpressionOperator::End => f.write_str("end"),
            ExpressionOperator::When => f.write_str("when"),
            ExpressionOperator::Then => f.write_str("then"),
            ExpressionOperator::Else => f.write_str("else"),
            ExpressionOperator::Unless => f.write_str("unless"),
            ExpressionOperator::Comma => f.write_str(","),
            ExpressionOperator::Either => f.write_str("either"),
            ExpressionOperator::Between => f.write_str("between"),
            ExpressionOperator::Of => f.write_str("of"),
            ExpressionOperator::From => f.write_str("from"),
            ExpressionOperator::To => f.write_str("to"),
            ExpressionOperator::Minimum => f.write_str("minimum"),
            ExpressionOperator::Maximum => f.write_str("maximum"),
            ExpressionOperator::Random => f.write_str("random"),
            ExpressionOperator::Is => f.write_str("is"),
            ExpressionOperator::In => f.write_str("in"),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ExpressionToken {
    Atom(ExpressionAtom),
    Operator(ExpressionOperator),
    UnrecognizedToken,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct MarkedToken<T> {
    pub token: T,
    pub start_mark: Mark,
    pub end_mark: Mark,
}
