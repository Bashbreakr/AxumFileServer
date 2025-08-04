use axum::{
    extract::Path,
    routing::get,
    Router,
};
use tower_http::services::ServeDir;
use local_ip_address::local_ip;

mod listfiles;

#[tokio::main]
async fn main() {
    let app = app();

    let ip = local_ip().unwrap();
    let addr = format!("{}:8080", ip);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .unwrap();

    println!("Server läuft unter http://{}", listener.local_addr().unwrap());

    axum::serve(listener, app)
        .await
        .unwrap();
}

fn app() -> Router {
    let base_path = "{*wildcard}".to_string(); //replace {*wildcard} with the folder containing the files to share

    Router::new()
        .route("/", get({
            let path = base_path.clone();
            move || async move {
                listfiles::listfiles(&path, "").await
            }
        }))
        .route("/browse/{*wildcard}", get({
            let path = base_path.clone();
            move |Path(subpath): Path<String>| {
                let full_path = format!("{}/{}", path, subpath);
                async move { listfiles::listfiles(&full_path, &subpath).await }
            }
        }))
        .nest_service("/staticfiles", ServeDir::new(&base_path))
        .nest_service("/static", ServeDir::new("static"))
}
