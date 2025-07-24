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
}
