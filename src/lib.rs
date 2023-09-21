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

pub trait ContextMessage : Sized {
    #[track_caller]
    fn context<S: Into<String>>(self, text: S) -> Self {
        let caller_location = std::panic::Location::caller();

        let message = ErrorMessage{
            message: text.into(),
            location: caller_location.to_string(),
        };

        self.attach_context_message(message)
    }

    fn attach_context_message(self, message: ErrorMessage) -> Self;
}

impl <R, E: Debug> ContextMessage for std::result::Result<R, Error<E>> {
    fn attach_context_message(self, message: ErrorMessage) -> Self {
        match self {
            Ok(value) => Ok(value),
            Err(mut err) => {
                err.context.push(message);
                Err(err)
            }
        }
    }
}

impl <R, E : Debug> From<Error<E>> for Result<R, Error<E>> {
    fn from(value: Error<E>) -> Self {
        Err(value)
    }
}

impl <E : Debug> From<E> for Error<E> {
    fn from(value: E) -> Self {
        Error::new(value)
    }
}

impl <T: Debug> Debug for Error<T> {
    #[track_caller]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.error)?;

        writeln!(f, "\n\nCaused by:")?;
        for item in self.context.iter().rev() {
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

