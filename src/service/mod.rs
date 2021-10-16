use worker::{Response, Result};

pub mod fizzbuzz;
pub mod sort;

pub(crate) trait Service {
    type Body;

    fn help(status: Option<(String, u16)>) -> Response
    where
        Self: Sized;

    ///
    /// A service can be created with an optional body (generally, anything that requires something
    /// like an array is much simpler to pass in via JSON in the body than in the url).
    ///
    fn create(body: Option<Result<Self::Body>>, query: &str) -> std::result::Result<Self, Response>
    where
        Self: Sized;

    fn response(self) -> Result<Response>;
}
