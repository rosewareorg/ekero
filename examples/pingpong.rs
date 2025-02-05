use ekero::prelude::*;

fn main() {
    clang_log::init(log::Level::Trace, "pingpong");

    let mut app = App::new("0.0.0.0:8000", 20);

    app.add_handler("/ping", |mut ctx| {
        let response = Response::new().body(b"pong");
        response.write_to(&mut ctx)?;

        Ok(())
    });

    app.poll_forever()
}
