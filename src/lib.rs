/// a rust crate for simple http routing
///
/// ```rust
/// use ekero::prelude::*;
///
/// let mut app = App::new("0.0.0.0:8000", 20, ());
///
///    app.get("/ping", |_ctx| {
///        Ok(Response::new()
///            .body("pong")
///            .status_code(202)
///            .header("Content-Type", "text/plain"))
/// });
///
///    app.poll_forever()
/// ```
///
pub mod resource;

pub mod context;
pub mod request;
pub mod response;

pub mod app;
pub mod handler;
pub mod threadpool;

pub(crate) mod errors;

pub mod prelude {
    pub use super::{
        app::App, context::Context, handler::Handler, request::Request, request::*,
        resource::Resource, response::Response,
    };
}
