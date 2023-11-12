pub trait GenericErrorTrait {
    fn get_message(&self) -> String;
    fn get_code(&self) -> String;
}

#[derive(Debug)]
pub struct ErrorBody {
    message: String,
    code: String,
}

impl ErrorBody {
    pub fn new(message: String, code: &str) -> Self {
        ErrorBody {
            message,
            code: code.to_owned(),
        }
    }
}

impl GenericErrorTrait for ErrorBody {
    fn get_message(&self) -> String {
        self.message.to_owned()
    }
    fn get_code(&self) -> String {
        self.code.to_owned()
    }
}

#[derive(Debug)]
pub enum GenericError {
    Repository(ErrorBody),
    Service(ErrorBody),
    Web(ErrorBody),
}
