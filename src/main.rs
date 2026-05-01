use askama::Template;
use axum::{
    Router,
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::{get, post},
};
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "character_sheet=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let router = Router::new()
        .route("/", get(home))
        .route("/calculate", post(calculate));

    info!("Starting server on port 3000");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, router).await.unwrap();
}

async fn home() -> impl IntoResponse {
    let template = HomepageTemplate {};
    HtmlTemplate(template)
}

struct StatField {
    id: String,
    value: i32,
}

#[derive(Template)]
#[template(path = "stat_response.html")]
struct StatResponseTemplate {
    fields: Vec<StatField>,
}

async fn calculate(body: String) -> impl IntoResponse {
    let args: Vec<&str> = body.split('=').collect();
    let val: i32 = args[1].parse().unwrap_or(0);

    let template = StatResponseTemplate {
        fields: vec![
            StatField {
                id: "strength-modifier".to_string(),
                value: (val - 10) / 2,
            },
            StatField {
                id: "acrobatics-modifier".to_string(),
                value: (val - 10) / 2 + 11,
            },
        ],
    };
    HtmlTemplate(template)
}

#[derive(Template)]
#[template(path = "homepage.html")]
struct HomepageTemplate;

/// A wrapper type that we'll use to encapsulate HTML parsed by askama into valid HTML for axum to serve.
struct HtmlTemplate<T>(T);

/// Allows us to convert Askama HTML templates into valid HTML for axum to serve in the response.
impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        // Attempt to render the template with askama
        match self.0.render() {
            // If we're able to successfully parse and aggregate the template, serve it
            Ok(html) => Html(html).into_response(),
            // If we're not, return an error or some bit of fallback HTML
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {}", err),
            )
                .into_response(),
        }
    }
}
