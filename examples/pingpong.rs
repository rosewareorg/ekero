use ekero::prelude::*;

fn main() {
    clang_log::init(log::Level::Trace, "pingpong");

    let mut app = App::new("0.0.0.0:8000", 20, ());

    app.get("/ping", |ctx| {
        let response = Response::new()
            .body("pong")
            .status_code(202)
            .header("Content-Type", "text/plain");

        response.write_to(ctx)?;

        Ok(())
    });

    app.poll_forever()
}
