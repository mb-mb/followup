use std::fmt;

#[derive(Debug, Clone)]
pub enum DomainError {
    NotFound,
    ValidationError(String),
    DatabaseError(String),
}

impl fmt::Display for DomainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFound => write!(f, "Registro não encontrado"),
            Self::ValidationError(msg) => write!(f, "Erro de validação: {msg}"),
            Self::DatabaseError(msg) => write!(f, "Erro de banco de dados: {msg}"),
        }
    }
}

impl std::error::Error for DomainError {}
