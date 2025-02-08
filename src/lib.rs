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
