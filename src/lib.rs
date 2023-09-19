use std::fmt::Debug;

pub struct Error<T: Debug> {
    error: T,
    context: Vec<ErrorMessage>
}

pub struct ErrorMessage {
    message: String,
    location: String,
}

impl <T: Debug> Error<T> {
    pub fn new(error: T) -> Self {
        Error{
            error,
            context: Vec::new(),
        }
    }

    #[track_caller]
    pub fn context<S: Into<String>>(mut self, text: S) -> Self {
        let caller_location = std::panic::Location::caller();

        let message = ErrorMessage{
            message: text.into(),
            location: caller_location.to_string(),
        };

        self.context.push(message);
        self
    }
}

impl <T: Debug> Debug for Error<T> {
    #[track_caller]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.error)?;

        writeln!(f, "\n\nCaused by:")?;
        for item in self.context.iter() {
            writeln!(f, "- {:?}", item)?;
        }

        write!(f, "\n")
    }
}

impl Debug for ErrorMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.message, self.location)
    }
}

