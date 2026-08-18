use thiserror::Error;

#[derive(Debug, Error)]
pub enum FractalError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Shader export error: {0}")]
    ShaderExport(String),
    #[error("Invalid formula: {0}")]
    InvalidFormula(String),
    #[error("Render error: {0}")]
    Render(String),
    #[error("Evaluation error: {0}")]
    Eval(String),
}

impl From<String> for FractalError {
    fn from(s: String) -> Self {
        FractalError::ShaderExport(s)
    }
}

pub type FractalResult<T> = Result<T, FractalError>;
