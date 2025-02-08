# ekerö
a rust crate for simple http routing

```rust
let mut app = App::new("0.0.0.0:8000", 20, ());

    app.get("/ping", |_ctx| {
        Ok(Response::new()
            .body("pong")
            .status_code(202)
            .header("Content-Type", "text/plain"))
    });

    app.poll_forever()
```
