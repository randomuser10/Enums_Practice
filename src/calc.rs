pub enum OperationType {
    Addition,
    Subtraction,
    Multiplication,
}

impl OperationType {
    pub fn get_sign(&self) -> &str {
        match self {
            OperationType::Addition => "+",
            OperationType::Subtraction => "-",
            OperationType::Multiplication => "*",
        }
    }

    pub fn perform(&self, x: i64, y: i64) -> Option<i64> {
        match self {
            OperationType::Addition => Some(x + y),
            OperationType::Subtraction => Some(x - y),
            OperationType::Multiplication => Some(x * y),
        }
    }
}
