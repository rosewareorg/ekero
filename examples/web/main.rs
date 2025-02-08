use ekero::prelude::*;

fn main() {
    clang_log::init(log::Level::Trace, "web");

    let mut app = App::new("0.0.0.0:8000", 20, ());

    app.set_default_handler(|_ctx| {
        const BYTES: &str = include_str!("html/404.html");

        Ok(Response::new()
            .body(BYTES)
            .header("Content-Type", "text/html")
            .status_code(404))
    });

    app.get("/", |_ctx| {
        const BYTES: &str = include_str!("html/root.html");

        Ok(Response::new()
            .body(BYTES)
            .header("Content-Type", "text/html"))
    });

    app.poll_forever()
}
