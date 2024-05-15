pub enum AssemblerError {
    ParseError(String),
    AstGenerationError(String),
    LoweringError(String),
    SerializationError(String),
}

pub type AssemblerResult<T> = std::result::Result<T, AssemblerError>;

impl std::fmt::Display for AssemblerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AssemblerError::ParseError(err) => write!(f, "parse error: {err}"),
            AssemblerError::AstGenerationError(err) => write!(f, "ast creation error: {err}"),
            AssemblerError::LoweringError(err) => write!(f, "lowering error: {err}"),
            AssemblerError::SerializationError(err) => write!(f, "serialization error: {err}"),
        }
    }
}

impl std::fmt::Debug for AssemblerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (self as &dyn std::fmt::Display).fmt(f)
    }
}
