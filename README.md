# rust_actix
Demo of building a simple web server using Rust

# installation
Clone this repository and run `cargo build`.

# usage
1. Open terminal in the project root.
1. Run `RUST_LOG=debug ./target/debug/rust_actix`.
1. Open `http://127.0.0.1:8080/` in your browser. You should see Hello world! page. In your terminal, you should see logs.
1. Try `http://127.0.0.1:8080/hey` to showcase GET request.
1. Try `http://127.0.0.1:8080/form` to showcase PUT requests. Fill in some names and click all three submit buttons. Check the browser and the terminal for logs.
1. Try `http://127.0.0.1:8080/json` for a REST API.
