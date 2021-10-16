use worker::{Response, Result};

pub(crate) trait Service {
    fn help(status: Option<(String, u16)>) -> Response
    where
        Self: Sized;

    ///
    /// A service can be created with an optional body (generally, anything that requires something
    /// like an array is much simpler to pass in via JSON in the body than in the url).
    ///

    fn response(self) -> Result<Response>;
}
