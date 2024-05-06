use worker::{Response, Result};

pub trait Service {
    const NAME: &'static str;

    fn error(message: &str, status_code: u16) -> Result<Response>;

    fn help() -> Result<Response>
    where
        Self: Sized;

    ///
    /// A service can be created with an optional body (generally, anything that requires something
    /// like an array is much simpler to pass in via JSON in the body than in the url).
    ///
    fn response(self) -> Result<Response>;
}
