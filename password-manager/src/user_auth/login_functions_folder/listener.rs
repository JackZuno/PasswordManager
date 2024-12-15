use axum::Router;
use std::net::SocketAddr;
use tokio::sync::mpsc;

pub async fn start_listener(tx: mpsc::Sender<String>) {
    let app = Router::new().route("/callback", axum::routing::get(move |query: axum::extract::Query<std::collections::HashMap<String, String>>| {
        let tx = tx.clone();
        async move {
            if let Some(code) = query.get("code") {
                // Send the code back through the channel
                tx.send(code.clone()).await.unwrap();
                // println!("Authorization code received: {}", code);
                "Callback received! You can close this window."
            } else {
                "No code found in query parameters."
            }
        }
    }));

    // Set up the listener
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    // Start the server
    axum::serve(listener, app.into_make_service()).await.unwrap();
}
