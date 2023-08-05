use axum::extract::Path;
use axum::{routing::get, Router};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let app = Router::new()
        .route("/", get(hello))
        .route("/:name", get(hello_name));

    axum::Server::bind(
        &"0.0.0.0:8000"
            .parse()
            .expect("unable to parse binding address"),
    )
    .serve(app.into_make_service())
    .await
    .expect("unable to bind service");

    Ok(())
}

async fn hello() -> &'static str {
    "Hello, World!\n"
}

async fn hello_name(Path(name): Path<String>) -> String {
    format!("Hello {}\n", name)
}
