#![allow(warnings)]

use axum::response::Response;
use axum_server::tls_rustls::RustlsConfig;
use dotenvy::dotenv;
use env::{check_env, hostname, https, port, setup_cors, ssl_cert_key};
use error::ErrorResponse;
use prisma::PrismaClient;
use routes::routes;
use state::AppState;
use std::{net::SocketAddr, sync::Arc};
use tokio::net::TcpListener;
mod error;
mod helpers;
mod img_user;
mod prisma;
mod response;
mod state;
mod routes;
mod env;



pub type WebResult = std::result::Result<Response, ErrorResponse>;

#[tokio::main]
async fn main() {
    dotenv().ok();

        let client = PrismaClient::_builder()
        .build()
        .await
        .expect("Cannot connect to Mongodb");

    let client = Arc::new(client);

    let state = AppState::new(client).await;

    let mut routes = routes().layer(setup_cors()).with_state(state);

    let hostname = hostname();
    let port = port();


    if https() {
        let tls_config = {
            let (cert_path, key_path) = ssl_cert_key();
            RustlsConfig::from_pem_file(cert_path, key_path)
                .await
                .expect("Cannot find certifications to enable https")
        };

        let addr = SocketAddr::from(([0, 0, 0, 0], port));

        println!("Server started at https://{hostname}:{port}");

        axum_server::bind_rustls(addr, tls_config)
            .serve(routes.into_make_service())
            .await
            .expect("Server crashed")
    } else {
        let listener = TcpListener::bind(format!("0.0.0.0:{port}")).await.unwrap();

        println!("Server started at http://{hostname}:{port}");

        axum::serve(listener, routes).await.expect("Server crashed");
    }
}
