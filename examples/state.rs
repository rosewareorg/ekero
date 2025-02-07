use ekero::prelude::*;

struct State {
    count: usize,
}

fn main() {
    clang_log::init(log::Level::Trace, "state");

    let mut app = App::new("0.0.0.0:8000", 20, State { count: 0 });

    app.get("/increment", |mut ctx| {
        let count = match ctx.lock_state() {
            Ok(mut count_guard) => {
                count_guard.count += 1;
                count_guard.count - 1
            }
            _ => 0,
        };

        let json = format!("{{\"count\": {count}");
        let json_size = format!("{}", json.len());

        let response = Response::new()
            .body(json.as_bytes())
            .status_code(200)
            .header("Content-Type", b"application/json")
            .header("Content-Length", json_size.as_bytes());

        response.write_to(&mut ctx)?;

        Ok(())
    });

    app.poll_forever()
}
