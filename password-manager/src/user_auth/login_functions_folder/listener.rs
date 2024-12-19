use axum::{response::Html, Router};
use std::net::SocketAddr;
use tokio::sync::mpsc;

pub async fn start_listener(tx: mpsc::Sender<String>) {
    let app = Router::new().route(
        "/callback",
        axum::routing::get(move |query: axum::extract::Query<std::collections::HashMap<String, String>>| {
            let tx = tx.clone();
            async move {
                if let Some(code) = query.get("code") {
                    // Send the code back through the channel
                    tx.send(code.clone()).await.unwrap();

                    Html(
                        r#"
                        <!DOCTYPE html>
                        <html lang="en">
                        <head>
                            <meta charset="UTF-8">
                            <meta name="viewport" content="width=device-width, initial-scale=1.0">
                            <title>Authorization Successful</title>
                            <style>
                                body {
                                    font-family: 'Arial', sans-serif;
                                    display: flex;
                                    justify-content: center;
                                    align-items: center;
                                    height: 100vh;
                                    margin: 0;
                                    background: linear-gradient(135deg, #344955, #232F34);
                                    color: #fff;
                                    text-align: center;
                                }
                                .container {
                                    background: rgba(255, 255, 255, 0.05);
                                    padding: 2rem 3rem;
                                    border-radius: 12px;
                                    box-shadow: 0px 6px 20px rgba(0, 0, 0, 0.4);
                                    animation: fadeIn 0.8s ease-in-out;
                                }
                                h1 {
                                    font-size: 2.4rem;
                                    margin-bottom: 1rem;
                                    color: #00C853;
                                }
                                p {
                                    font-size: 1.2rem;
                                    line-height: 1.6;
                                    margin-bottom: 1rem;
                                    color: #D5D5D5;
                                }
                                a {
                                    display: inline-block;
                                    margin-top: 1.5rem;
                                    padding: 0.8rem 2rem;
                                    font-size: 1.1rem;
                                    font-weight: bold;
                                    color: #00C853;
                                    background: #fff;
                                    border-radius: 8px;
                                    text-decoration: none;
                                    transition: all 0.3s ease-in-out;
                                }
                                a:hover {
                                    background: #00C853;
                                    color: #fff;
                                    transform: scale(1.05);
                                }
                                @keyframes fadeIn {
                                    from {
                                        opacity: 0;
                                        transform: translateY(-20px);
                                    }
                                    to {
                                        opacity: 1;
                                        transform: translateY(0);
                                    }
                                }
                            </style>
                        </head>
                        <body>
                            <div class="container">
                                <h1>Authorization Successful</h1>
                                <p>Thank you for authorizing the password manager. You can now close this window and return to the application to continue.</p>
                            </div>
                        </body>
                        </html>
                        "#,
                    )
                } else {
                    Html(
                        r#"
                        <!DOCTYPE html>
                        <html lang="en">
                        <head>
                            <meta charset="UTF-8">
                            <meta name="viewport" content="width=device-width, initial-scale=1.0">
                            <title>Authorization Failed</title>
                            <style>
                                body {
                                    font-family: 'Arial', sans-serif;
                                    display: flex;
                                    justify-content: center;
                                    align-items: center;
                                    height: 100vh;
                                    margin: 0;
                                    background: linear-gradient(135deg, #5E35B1, #311B92);
                                    color: #fff;
                                    text-align: center;
                                }
                                .container {
                                    background: rgba(255, 255, 255, 0.05);
                                    padding: 2rem 3rem;
                                    border-radius: 12px;
                                    box-shadow: 0px 6px 20px rgba(0, 0, 0, 0.4);
                                    animation: fadeIn 0.8s ease-in-out;
                                }
                                h1 {
                                    font-size: 2.4rem;
                                    margin-bottom: 1rem;
                                    color: #FF5252;
                                }
                                p {
                                    font-size: 1.2rem;
                                    line-height: 1.6;
                                    margin-bottom: 1rem;
                                    color: #D5D5D5;
                                }
                                a {
                                    display: inline-block;
                                    margin-top: 1.5rem;
                                    padding: 0.8rem 2rem;
                                    font-size: 1.1rem;
                                    font-weight: bold;
                                    color: #FF5252;
                                    background: #fff;
                                    border-radius: 8px;
                                    text-decoration: none;
                                    transition: all 0.3s ease-in-out;
                                }
                                a:hover {
                                    background: #FF5252;
                                    color: #fff;
                                    transform: scale(1.05);
                                }
                                @keyframes fadeIn {
                                    from {
                                        opacity: 0;
                                        transform: translateY(-20px);
                                    }
                                    to {
                                        opacity: 1;
                                        transform: translateY(0);
                                    }
                                }
                            </style>
                        </head>
                        <body>
                            <div class="container">
                                <h1>Authorization Failed</h1>
                                <p>It looks like something went wrong. Please try again or contact support for assistance.</p>
                            </div>
                        </body>
                        </html>
                        "#,
                    )
                }
            }
        }),
    );

    // Set up the listener
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    // Start the server
    axum::serve(listener, app.into_make_service()).await.unwrap();
}
