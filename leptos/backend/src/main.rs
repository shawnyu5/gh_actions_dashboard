use axum::{routing::post, Router};
use leptos::*;

#[tokio::main]
async fn main() {
    let conf = get_configuration(None).await.unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    // Generate the list of routes in your Leptos App
    // let routes = generate_route_list(|cx| view! { cx, <App/> }).await;

    let app = Router::new()
        .route("/api/*fn_name", post(leptos_axum::handle_server_fns))
        // .leptos_routes(&leptos_options, routes, |cx| view! { cx, <App/> })
        // .fallback(file_and_error_handler)
        .with_state(leptos_options);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    log!("listening on http://{}", &addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[server(Hello, "/api")]
async fn hello() -> Result<(), ServerFnError> {
    println!("Hello, World from server!");
    return Ok(());
}

// pub async fn file_and_error_handler(
// uri: Uri,
// State(options): State<LeptosOptions>,
// req: Request<Body>,
// ) -> AxumResponse {
// let root = options.site_root.clone();
// let res = get_static_file(uri.clone(), &root).await.unwrap();

// if res.status() == StatusCode::OK {
// res.into_response()
// } else {
// let handler =
// leptos_axum::render_app_to_stream(options.to_owned(), move |cx| view! { cx, <App/> });
// handler(req).await.into_response()
// }
// }

// async fn get_static_file(uri: Uri, root: &str) -> Result<Response<BoxBody>, (StatusCode, String)> {
// let req = Request::builder()
// .uri(uri.clone())
// .body(Body::empty())
// .unwrap();
// // `ServeDir` implements `tower::Service` so we can call it with `tower::ServiceExt::oneshot`
// // This path is relative to the cargo root
// match ServeDir::new(root).oneshot(req).await {
// Ok(res) => Ok(res.map(boxed)),
// Err(err) => Err((
// StatusCode::INTERNAL_SERVER_ERROR,
// format!("Something went wrong: {err}"),
// )),
// }
// }
