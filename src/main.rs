use axum::{routing::get, Router, Server};
use maud::{html, Markup, DOCTYPE};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let app = Router::new().route("/", get(index));
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn index() -> Markup {
    let content = html! {
        p { "Hello from TODOx!" }
    };
    page(content)
}

fn page(content: Markup) -> Markup {
    const HTMX_SRC: &str = "https://unpkg.com/htmx.org@1.9.6";
    const HTMX_SRC_INTEGRITY: &str =
        "sha384-FhXw7b6AlE/jyjlZH5iHa/tTe9EpJ1Y55RjcgPbjeWMskSxZt1v9qkxLJWNJaGni";
    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="UTF-8" {}
                meta name="viewport" content="width=device-width, initial-scale=1"
                script src=(HTMX_SRC) integrity=(HTMX_SRC_INTEGRITY) crossorigin="anonymous" {}
                title { "TODOx" }
            }
            body {
                (content)
            }
        }
    }
}
