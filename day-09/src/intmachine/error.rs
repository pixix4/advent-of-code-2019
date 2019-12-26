#[derive(Debug)]
pub struct MachineError {
    pub message: String,
    pub reason: String,
}

impl From<std::io::Error> for MachineError {
    fn from(error: std::io::Error) -> Self {
        MachineError {
            message: "I/O error!".to_owned(),
            reason: format!("{}", error),
        }
    }
}

impl From<std::sync::mpsc::RecvError> for MachineError {
    fn from(error: std::sync::mpsc::RecvError) -> Self {
        MachineError {
            message: "Recv error!".to_owned(),
            reason: format!("{}", error),
        }
    }
}

impl From<std::sync::mpsc::SendError<i128>> for MachineError {
    fn from(error: std::sync::mpsc::SendError<i128>) -> Self {
        MachineError {
            message: "Send error!".to_owned(),
            reason: format!("{}", error),
        }
    }
}

#[must_use]
pub type MachineResult<T> = Result<T, MachineError>;
