use Error::*;

pub enum Error {
    NoArgument,
    UnknownArgument,
    TooManyArguments,
    UnknownValue,
    FileNotFound,
    IOError,
    NonCanonicalizablePath,
    BaseNameNotFound,
    ParentDirNotFound,
    StringConversionFailure,
    SerdeJsonConversionFailure,
}

impl Error {
    fn message(&self) -> &str {
        match self {
            NoArgument => "No argument provided",
            UnknownArgument => "Unknown argument",
            TooManyArguments => "Too many arguments",
            UnknownValue => "Unknown value",
            FileNotFound => "File not found",
            IOError => "Unable to access filesystem",
            NonCanonicalizablePath => "Unable to retrieve absolute path",
            BaseNameNotFound => "Unable to retrieve base name",
            ParentDirNotFound => "Unable to retrieve parent dir",
            StringConversionFailure => "Error while converting to string",
            SerdeJsonConversionFailure => "Unable to retrieve json file content",
        }
    }

    pub fn abort(&self) -> ! {
        panic!("{}", self.message());
    }
}

pub trait Catch<T> {
    fn catch(self, err: Error) -> T;
}

impl<T, E> Catch<T> for Result<T, E> {
    fn catch(self, err: Error) -> T {
        match self {
            Ok(t) => t,
            Err(_) => err.abort(),
        }
    }
}

impl<T> Catch<T> for Option<T> {
    fn catch(self, err: Error) -> T {
        match self {
            Some(t) => t,
            None => err.abort(),
        }
    }
}
