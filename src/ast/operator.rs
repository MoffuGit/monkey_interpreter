use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PrefixOperator {
    Not,
    Negative,
}

impl Display for PrefixOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PrefixOperator::Not => write!(f, "!"),
            PrefixOperator::Negative => write!(f, "-"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InfixOperator {
    Add,
    Sub,
    Mul,
    Div,
    Equal,
    NotEqual,
    GreaterThan,
    LessThan,
    Modulo,
    GreaterThanOrEqual,
    LessThanOrEqual,
}

impl Display for InfixOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InfixOperator::Add => write!(f, "+"),
            InfixOperator::Sub => write!(f, "-"),
            InfixOperator::Mul => write!(f, "*"),
            InfixOperator::Div => write!(f, "/"),
            InfixOperator::Equal => write!(f, "=="),
            InfixOperator::NotEqual => write!(f, "!="),
            InfixOperator::GreaterThan => write!(f, ">"),
            InfixOperator::LessThan => write!(f, "<"),
            InfixOperator::Modulo => write!(f, "%"),
            InfixOperator::GreaterThanOrEqual => write!(f, ">="),
            InfixOperator::LessThanOrEqual => write!(f, "<="),
        }
    }
}
