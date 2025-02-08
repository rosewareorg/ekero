use ekero::prelude::*;

fn main() {
    clang_log::init(log::Level::Trace, "web");

    let mut app = App::new("0.0.0.0:8000", 20, ());

    app.set_default_handler(|ctx| {
        const BYTES: &[u8] = include_bytes!("html/404.html");

        let response = Response::new()
            .body(BYTES)
            .header("Content-Type", "text/html")
            .header("Content-Length", BYTES.len())
            .status_code(404);
        response.write_to(ctx)?;

        Ok(())
    });

    app.get("/", |ctx| {
        const BYTES: &[u8] = include_bytes!("html/root.html");

        let response = Response::new()
            .body(BYTES)
            .header("Content-Type", "text/html")
            .header("Content-Length", BYTES.len());

        response.write_to(ctx)?;

        Ok(())
    });

    app.poll_forever()
}
