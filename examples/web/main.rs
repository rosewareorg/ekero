use ekero::prelude::*;

fn main() {
    clang_log::init(log::Level::Trace, "web");

    let mut app = App::new("0.0.0.0:8000", 20);

    app.set_default_handler(|mut ctx| {
        const BYTES: &[u8] = include_bytes!("html/404.html");
        let fmt = format!("{}", BYTES.len());

        let response = Response::new()
            .body(BYTES)
            .header("Content-Type", b"text/html")
            .header("Content-Length", fmt.as_bytes())
            .status_code(404);
        response.write_to(&mut ctx)?;

        Ok(())
    });

    app.get("/", |mut ctx| {
        const BYTES: &[u8] = include_bytes!("html/root.html");
        let fmt = format!("{}", BYTES.len());

        let response = Response::new()
            .body(BYTES)
            .header("Content-Type", b"text/html")
            .header("Content-Length", fmt.as_bytes());
        response.write_to(&mut ctx)?;

        Ok(())
    });

    app.poll_forever()
}
