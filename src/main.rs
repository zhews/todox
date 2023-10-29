use axum::{
    headers::{Cookie, HeaderMapExt},
    http::{Request, StatusCode},
    middleware::{self, Next},
    response::{IntoResponse, Redirect, Response},
    routing::get,
    Router, Server,
};
use deadpool_postgres::{Manager, ManagerConfig, Pool, RecyclingMethod};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use maud::{html, Markup, DOCTYPE};
use serde::{Deserialize, Serialize};
use std::{net::SocketAddr, process::exit};
use tokio_postgres::NoTls;

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    let auth_routes = Router::new()
        .route("/login", get(get_login).post(post_login))
        .route("/register", get(get_register).post(post_register));

    let mut postgres_config = tokio_postgres::Config::new();
    postgres_config.host("localhost");
    postgres_config.user("postgres");
    postgres_config.password("postgres");
    postgres_config.dbname("todox");
    let manager_config = ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
    };
    let manager = Manager::from_config(postgres_config, NoTls, manager_config);
    let Ok(pool) = Pool::builder(manager).max_size(20).build() else {
        println!("Failed to build database pool!");
        exit(1);
    };

    let app = Router::new()
        .route("/", get(get_index))
        .layer(middleware::from_fn(protect))
        .nest("/auth", auth_routes)
        .with_state(pool);

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn get_login() -> Markup {
    let content = html! {
        form hx-post="/auth/login" {
            input type="text" name="username" {}
            input type="password" name="password" {}
            button type="submit" { "Login" }
        }
    };
    page(content)
}

async fn post_login() -> Markup {
    let content = html! {
        p { "POST /auth/login" }
    };
    page(content)
}

async fn get_register() -> Markup {
    let content = html! {
        p { "GET /auth/register" }
    };
    page(content)
}

async fn post_register() -> Markup {
    let content = html! {
        p { "POST /auth/register" }
    };
    page(content)
}

async fn get_index() -> Markup {
    let content = html! {
        p { "Hello from TODOx!" }
    };
    page(content)
}

#[derive(Deserialize, Serialize)]
struct Claims {
    user_id: String,
}

async fn protect<B>(request: Request<B>, next: Next<B>) -> Response {
    let headers = request.headers();
    let Some(cookie_header): Option<Cookie> = headers.typed_get() else {
        return Redirect::temporary("/auth/login").into_response();
    };
    let Some(authentication_cookie) = cookie_header.get("authentication") else {
        return Redirect::temporary("/auth/login").into_response();
    };
    let Ok(_) = decode::<Claims>(
        authentication_cookie,
        &DecodingKey::from_secret("SECRET".as_bytes()),
        &Validation::new(Algorithm::HS256),
    ) else {
        return Redirect::temporary("/auth/login").into_response();
    };
    next.run(request).await
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
                meta name="viewport" content="width=device-width, initial-scale=1" {}
                script src=(HTMX_SRC) integrity=(HTMX_SRC_INTEGRITY) crossorigin="anonymous" {}
                title { "TODOx" }
            }
            body {
                (content)
            }
        }
    }
}
