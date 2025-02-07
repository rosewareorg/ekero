use ekero::prelude::*;

fn main() {
    clang_log::init(log::Level::Trace, "pingpong");

    let mut app = App::new("0.0.0.0:8000", 20, ());

    app.get("/ping", |mut ctx| {
        let response = Response::new()
            .body(b"pong")
            .status_code(202)
            .header("Content-Type", b"text/plain")
            .header("Content-Length", b"4");

        response.write_to(&mut ctx)?;

        Ok(())
    });

    app.poll_forever()
}
