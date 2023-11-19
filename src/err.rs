#[derive(Debug)]
pub enum Stage {
    Parse,
    Scan,
    Eval,
}

#[derive(Debug)]
pub struct Error {
    stage: Stage,
    line: u32,
    message: String,
}
impl Error {
    pub fn new(stage: Stage, line: u32, message: &str) -> Self {
        Self {
            stage,
            line,
            message: message.to_string(),
        }
    }
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let stage = match self.stage {
            Stage::Parse => "Parse",
            Stage::Scan => "Scan",
            Stage::Eval => "Eval",
        };
        write!(f, "[line {}] {} error: {} at", self.line, stage, self.message)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
