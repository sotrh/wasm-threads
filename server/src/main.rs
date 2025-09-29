use std::net::SocketAddr;

use axum::{
    Router,
    http::{HeaderName, header::HeaderValue},
};
use axum_server::tls_rustls::RustlsConfig;
use rcgen::{CertifiedKey, generate_simple_self_signed};
use tokio::io::AsyncWriteExt;
use tower_http::{services::ServeDir, set_header::SetResponseHeaderLayer};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = secrets().await?;

    let app = Router::new()
        .fallback_service(ServeDir::new("."))
        .layer(SetResponseHeaderLayer::if_not_present(
            HeaderName::from_static("cross-origin-opener-policy"),
            HeaderValue::from_static("same-origin"),
        ))
        .layer(SetResponseHeaderLayer::if_not_present(
            HeaderName::from_static("cross-origin-embedder-policy"),
            HeaderValue::from_static("same-origin"),
        ));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8443));

    println!("Listening on https://{}", addr);

    axum_server::bind_rustls(addr, config)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

async fn secrets() -> anyhow::Result<RustlsConfig> {
    let cert_path = "target/secrets/cert.pem";
    let key_path = "target/secrets/key.pem";

    if !std::fs::exists(cert_path).unwrap_or(false) || !std::fs::exists(key_path).unwrap_or(false) {
        tokio::fs::create_dir_all("target/secrets/").await?;

        let subject_alt_names = vec!["localhost".to_string()];
        let CertifiedKey { cert, signing_key } = generate_simple_self_signed(subject_alt_names)?;

        let cert_pem = cert.pem();
        let key_pem = signing_key.serialize_pem();

        let mut cert_file = tokio::fs::File::create(cert_path).await?;
        cert_file.write_all(cert_pem.as_bytes()).await?;
        println!("Generated cert.pem");

        let mut key_file = tokio::fs::File::create(key_path).await?;
        key_file.write_all(key_pem.as_bytes()).await?;
        println!("Generated key.pem");
    }

    Ok(RustlsConfig::from_pem_file(cert_path, key_path).await?)
}
