use ekero::prelude::*;

fn main() {
    clang_log::init(log::Level::Trace, "web");

    let mut app = App::new("0.0.0.0:8000", 20, ());

    app.set_default_handler(|ctx| {
        const BYTES: &str = include_str!("html/404.html");

        let response = Response::new()
            .body(BYTES)
            .header("Content-Type", "text/html")
            .status_code(404);

        response.write_to(ctx)?;

        Ok(())
    });

    app.get("/", |ctx| {
        const BYTES: &str = include_str!("html/root.html");

        let response = Response::new()
            .body(BYTES)
            .header("Content-Type", "text/html");

        response.write_to(ctx)?;

        Ok(())
    });

    app.poll_forever()
}
