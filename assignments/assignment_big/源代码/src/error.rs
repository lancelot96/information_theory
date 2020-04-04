use failure::Fail;

#[derive(Debug, Fail)]
pub enum CodingError {
    #[fail(display = "NoSuchLetterError")]
    NoSuchLetterError(char),

    #[fail(display = "CodeBookTooShort")]
    CodeBookTooShort,

    #[fail(display = "ParsingError")]
    ParsingError,
}
