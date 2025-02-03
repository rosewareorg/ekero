# ekerö
a rust crate for simple http routing

```rust
let mut app = App::new("0.0.0.0:8000", 20);

app.add_handler("/ping", |mut ctx| {
    let response = Response::new().body(Some(b"pong".to_vec()));
    response.write_to(&mut ctx)?;

    Ok(())
});

app.poll_forever()
```
