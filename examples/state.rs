use ekero::prelude::*;

struct State {
    count: usize,
}

fn main() {
    clang_log::init(log::Level::Trace, "state");

    let mut app = App::new("0.0.0.0:8000", 20, State { count: 0 });

    app.get("/increment", |ctx| {
        let count = {
            let mut state = ctx.state_lock()?;
            let curr = state.count;
            state.count += 1;
            curr
        };

        let json = format!("{{\"count\": {count}}}");

        Ok(Response::new()
            .body(json)
            .status_code(200)
            .header("Content-Type", "application/json"))
    });

    app.get("/decrement", |ctx| {
        let count = {
            let mut state = ctx.state_lock()?;
            let curr = state.count;
            state.count -= 1;
            curr
        };

        let json = format!("{{\"count\": {count}}}");

        Ok(Response::new()
            .body(json)
            .status_code(200)
            .header("Content-Type", "application/json"))
    });

    app.poll_forever()
}
